//! Tests for the EHCI register descriptions and root hub helpers.
use rover_uefi::protocols::usb::ehci::{
    FrameListEntry, FrameListSize, HcCapabilities, HcCapabilityFlags, ScheduleElementType,
    ScheduleLink,
};
use rover_uefi::protocols::usb::host_controller::root_hub::{is_connected, is_enabled};
use rover_uefi::protocols::usb::host_controller::usb2::UsbPortStatus;

#[test]
fn schedule_link_queue_head_roundtrip() {
    let link = ScheduleLink::queue_head(0x2000);
    let encoded = link.encode();
    assert_eq!(encoded & 1, 0);
    assert_eq!(encoded & 0xFFFF_FFE0, 0x2000);
    let decoded = ScheduleLink::decode(encoded);
    assert_eq!(decoded.address, 0x2000);
    assert!(!decoded.terminate);
    assert_eq!(decoded.element_type, ScheduleElementType::QueueHead);
}

#[test]
fn schedule_link_terminated_sets_bit_zero() {
    let encoded = ScheduleLink::terminated().encode();
    assert_eq!(encoded & 1, 1);
    assert!(ScheduleLink::decode(encoded).terminate);
}

#[test]
fn schedule_link_masks_low_address_bits() {
    // The address occupies bits 5 to 31; bits 0 to 2 are the link
    // type and terminator flags.
    let link = ScheduleLink::queue_head(0xFFFF_FFFF);
    assert_eq!(link.encode() & 0xFFFF_FFE0, 0xFFFF_FFE0);
    assert_eq!(ScheduleLink::decode(link.encode()).address, 0xFFFF_FFE0);
}

#[test]
fn frame_list_entry_roundtrip() {
    let entry = FrameListEntry::queue_head(0x4000);
    let decoded = FrameListEntry::decode(entry.encode());
    assert_eq!(decoded, entry);
    assert!(FrameListEntry::decode(FrameListEntry::empty().encode()).terminate);
}

#[test]
fn frame_list_sizes_roundtrip() {
    for size in [
        FrameListSize::Entries1024,
        FrameListSize::Entries512,
        FrameListSize::Entries256,
        FrameListSize::Reserved,
    ] {
        assert_eq!(FrameListSize::from_raw(size.to_raw()), size);
        assert!(size.entry_count() > 0);
        assert!(size.index_bits() > 0);
    }
}

#[test]
fn ehci_capabilities_parse() {
    let mut registers = [0_u8; 0x40];
    registers[0] = 0x40;
    registers[2..4].copy_from_slice(&0x0100_u16.to_le_bytes());
    // Port count 2, port power control, three ports per companion,
    // one companion controller, debug port 2.
    registers[4..8].copy_from_slice(&0x0020_1312_u32.to_le_bytes());
    // 64-bit addressing, programmable frame list, threshold 3, and the
    // full-frame threshold flag.
    registers[8..12].copy_from_slice(&0x0000_00B3_u32.to_le_bytes());

    let Some(capabilities) = HcCapabilities::parse(&registers) else {
        panic!("the capability block must parse");
    };
    assert_eq!(capabilities.cap_length, 0x40);
    assert_eq!(capabilities.hci_version, 0x0100);
    assert_eq!(capabilities.port_count, 2);
    assert_eq!(capabilities.ports_per_companion, 3);
    assert_eq!(capabilities.companion_controller_count, 1);
    assert_eq!(capabilities.debug_port, 2);
    assert_eq!(capabilities.isochronous_scheduling_threshold, 3);
    assert!(
        capabilities
            .flags
            .contains(HcCapabilityFlags::PORT_POWER_CONTROL)
    );
    assert!(
        capabilities
            .flags
            .contains(HcCapabilityFlags::ADDRESSING_64_BIT)
    );
    assert!(
        capabilities
            .flags
            .contains(HcCapabilityFlags::PROGRAMMABLE_FRAME_LIST)
    );
    assert!(
        capabilities
            .flags
            .contains(HcCapabilityFlags::ISOCHRONOUS_THRESHOLD_FULL_FRAME)
    );
}

#[test]
fn ehci_capabilities_reject_short_blocks() {
    assert!(HcCapabilities::parse(&[]).is_none());
    assert!(HcCapabilities::parse(&[0_u8; 11]).is_none());
}

#[test]
fn root_hub_port_bits() {
    let mut status = UsbPortStatus {
        port_status: 0,
        port_change_status: 0,
    };
    assert!(!is_connected(&status));
    assert!(!is_enabled(&status));
    status.port_status = 0x0001;
    assert!(is_connected(&status));
    assert!(!is_enabled(&status));
    status.port_status = 0x0002;
    assert!(!is_connected(&status));
    assert!(is_enabled(&status));
    status.port_status = 0xFFFF;
    assert!(is_connected(&status));
    assert!(is_enabled(&status));
}
