//! Tests for HID report descriptor parsing and report decoding.
use rover_uefi::protocols::usb::hid::descriptor::ReportDescriptor;
use rover_uefi::protocols::usb::hid::report::FieldValue;

/// A three-button mouse with relative X and Y axes.
const MOUSE_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x02, // Usage (Mouse)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x01, //   Usage (Pointer)
    0xA1, 0x00, //   Collection (Physical)
    0x05, 0x09, //     Usage Page (Buttons)
    0x19, 0x01, //     Usage Minimum (1)
    0x29, 0x03, //     Usage Maximum (3)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x95, 0x03, //     Report Count (3)
    0x75, 0x01, //     Report Size (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    0x95, 0x01, //     Report Count (1)
    0x75, 0x05, //     Report Size (5)
    0x81, 0x03, //     Input (Constant)
    0x05, 0x01, //     Usage Page (Generic Desktop)
    0x09, 0x30, //     Usage (X)
    0x09, 0x31, //     Usage (Y)
    0x15, 0x81, //     Logical Minimum (-127)
    0x25, 0x7F, //     Logical Maximum (127)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x02, //     Report Count (2)
    0x81, 0x06, //     Input (Data, Variable, Relative)
    0xC0, //   End Collection
    0xC0, // End Collection
];

#[test]
fn descriptor_parses_and_has_fields() {
    let Some(descriptor) = ReportDescriptor::<64>::parse(MOUSE_DESCRIPTOR) else {
        panic!("the mouse descriptor must parse");
    };
    assert!(!descriptor.is_empty());
    assert!(!descriptor.has_report_ids());
    assert_eq!(descriptor.items().len(), descriptor.len());
    assert!(descriptor.fields().count() > 0);
}

#[test]
fn empty_descriptor_is_accepted_but_empty() {
    let Some(descriptor) = ReportDescriptor::<8>::parse(&[]) else {
        panic!("an empty descriptor parses to an empty item list");
    };
    assert!(descriptor.is_empty());
    assert_eq!(descriptor.len(), 0);
    assert!(descriptor.fields().next().is_none());
}

#[test]
fn truncated_descriptor_is_rejected() {
    // The final usage item declares one byte of data that is missing.
    assert!(ReportDescriptor::<8>::parse(&[0x05, 0x01, 0x09]).is_none());
    // A short item with a two byte payload that is missing.
    assert!(ReportDescriptor::<8>::parse(&[0x06, 0x01]).is_none());
}

#[test]
fn descriptor_longer_than_the_item_capacity_is_rejected() {
    let mut bytes = [0_u8; 64];
    // Each pair is a two byte usage item, so 32 items exceed a
    // capacity of 16.
    for index in 0..32_usize {
        bytes[index * 2] = 0x05;
        bytes[index * 2 + 1] = 0x01;
    }
    assert!(ReportDescriptor::<16>::parse(&bytes).is_none());
}

#[test]
fn report_decodes_buttons_and_axes() {
    let Some(descriptor) = ReportDescriptor::<64>::parse(MOUSE_DESCRIPTOR) else {
        panic!("the mouse descriptor must parse");
    };
    // All three buttons pressed, X = 16, Y = -16.
    let report = [0xFF_u8, 0x10, 0xF0];
    let fields: Vec<_> = descriptor.parse_report(&report).collect();
    // Three button bits, five padding bits, and one two-value axis
    // field that carries X and Y.
    assert_eq!(fields.len(), 3);
    assert!(fields.iter().all(|field| field.report_id == 0));

    let buttons = fields[0].value;
    assert_eq!(buttons.len(), 3);
    assert_eq!(buttons.get(0), Some(1));
    assert_eq!(buttons.get(1), Some(1));
    assert_eq!(buttons.get(2), Some(1));
    assert_eq!(buttons.get(3), None);

    // The constant padding field carries the upper five bits of 0xFF.
    assert_eq!(fields[1].value, FieldValue::Scalar(31));

    let axes = fields[2].value;
    assert_eq!(axes.len(), 2);
    assert_eq!(axes.get(0), Some(16));
    assert_eq!(axes.get(1), Some(-16));
    assert!(fields[2].signed);
}

#[test]
fn short_report_yields_no_field() {
    let Some(descriptor) = ReportDescriptor::<64>::parse(MOUSE_DESCRIPTOR) else {
        panic!("the mouse descriptor must parse");
    };
    // The axes are missing from this report.
    let fields: Vec<_> = descriptor.parse_report(&[0x01]).collect();
    assert!(fields.len() < 3);
}

#[test]
fn report_identifiers_select_matching_fields() {
    // Report ID 1 followed by one eight-bit X axis.
    let descriptor_bytes = [
        0x05_u8, 0x01, 0x85, 0x01, 0x09, 0x30, 0x15, 0x00, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x01,
        0x81, 0x02,
    ];
    let Some(descriptor) = ReportDescriptor::<32>::parse(&descriptor_bytes) else {
        panic!("the descriptor with a report identifier must parse");
    };
    assert!(descriptor.has_report_ids());

    let report = [0x01_u8, 0x05];
    let iter = descriptor.parse_report(&report);
    assert_eq!(iter.report_id(), Some(1));
    let fields: Vec<_> = iter.collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].report_id, 1);

    // A report of a different identifier yields no decoded field.
    let other = [0x02_u8, 0x05];
    assert_eq!(descriptor.parse_report(&other).count(), 0);
}
