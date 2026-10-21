//! Tests for graphics output helpers.
//!
//! The tests cover GUIDs, layouts, pixel
//! formats, and pure validation helpers.

use core::mem::{align_of, offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::graphics::blt::BltOperation;
use rover_uefi::protocols::graphics::guid::{
    EDID_ACTIVE_GUID, EDID_DISCOVERED_GUID, GRAPHICS_OUTPUT_GUID,
};
use rover_uefi::protocols::graphics::mode::EfiGraphicsOutputModeInformation;
use rover_uefi::protocols::graphics::pixel::{
    EfiGraphicsOutputBltPixel, EfiPixelBitmask, PixelFormat,
};
use rover_uefi::protocols::graphics::protocol::{
    EfiEdidActiveProtocol, EfiEdidDiscoveredProtocol, EfiGraphicsOutputProtocol,
    EfiGraphicsOutputProtocolBlt, EfiGraphicsOutputProtocolQueryMode,
    EfiGraphicsOutputProtocolSetMode,
};
use rover_uefi::protocols::graphics::validation::{
    blt_buffer_size, framebuffer_size, pixel_offset, rectangle_in_display, validate_mode_index,
};

#[test]
fn guid_values() {
    assert_eq!(
        GRAPHICS_OUTPUT_GUID.to_bytes(),
        [
            0xDE, 0xA9, 0x42, 0x90, 0xDC, 0x23, 0x38, 0x4A, 0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80,
            0x51, 0x6A
        ]
    );
    assert_eq!(
        EDID_DISCOVERED_GUID.to_bytes(),
        [
            0xF6, 0x34, 0x0C, 0x1C, 0x80, 0xD3, 0xFA, 0x41, 0xA0, 0x49, 0x8A, 0xD0, 0x6C, 0x1A,
            0x66, 0xAA
        ]
    );
    assert_eq!(
        EDID_ACTIVE_GUID.to_bytes(),
        [
            0x56, 0x10, 0x8C, 0xBD, 0x36, 0x9F, 0xEC, 0x44, 0x92, 0xA8, 0xA6, 0x33, 0x7F, 0x81,
            0x79, 0x86
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(GRAPHICS_OUTPUT_GUID.to_bytes()),
        GRAPHICS_OUTPUT_GUID
    );
}

#[test]
fn protocol_layouts() {
    assert_eq!(
        size_of::<EfiGraphicsOutputProtocol>(),
        4 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiGraphicsOutputProtocolQueryMode>(),
        size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiGraphicsOutputProtocolSetMode>(),
        size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiGraphicsOutputProtocolBlt>(),
        size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiEdidDiscoveredProtocol>(),
        if size_of::<usize>() == 8 { 16 } else { 8 }
    );
    assert_eq!(
        size_of::<EfiEdidActiveProtocol>(),
        if size_of::<usize>() == 8 { 16 } else { 8 }
    );
    assert_eq!(size_of::<EfiPixelBitmask>(), 16);
    assert_eq!(size_of::<EfiGraphicsOutputBltPixel>(), 4);
    assert_eq!(offset_of!(EfiGraphicsOutputBltPixel, blue), 0);
    assert_eq!(offset_of!(EfiGraphicsOutputBltPixel, reserved), 3);
}

#[test]
fn mode_layouts() {
    assert_eq!(size_of::<EfiGraphicsOutputModeInformation>(), 36);
    assert_eq!(align_of::<EfiGraphicsOutputModeInformation>(), 4);
    assert_eq!(offset_of!(EfiGraphicsOutputModeInformation, version), 0);
    assert_eq!(
        offset_of!(EfiGraphicsOutputModeInformation, horizontal_resolution),
        4
    );
    assert_eq!(
        offset_of!(EfiGraphicsOutputModeInformation, vertical_resolution),
        8
    );
    assert_eq!(
        offset_of!(EfiGraphicsOutputModeInformation, pixel_format),
        12
    );
    assert_eq!(
        offset_of!(EfiGraphicsOutputModeInformation, pixel_information),
        16
    );
    assert_eq!(
        offset_of!(EfiGraphicsOutputModeInformation, pixels_per_scan_line),
        32
    );
}

#[test]
fn pixel_formats() {
    assert_eq!(PixelFormat::RGB_RESERVED.to_raw(), 0);
    assert_eq!(PixelFormat::BGR_RESERVED.to_raw(), 1);
    assert_eq!(PixelFormat::BITMASK.to_raw(), 2);
    assert_eq!(PixelFormat::BLT_ONLY.to_raw(), 3);
    assert_eq!(PixelFormat::from_raw_checked(4), None);
    assert_eq!(BltOperation::VIDEO_FILL.to_raw(), 0);
    assert_eq!(BltOperation::VIDEO_TO_VIDEO.to_raw(), 3);
    assert_eq!(BltOperation::from_raw_checked(4), None);
}

#[test]
fn size_helpers() {
    assert_eq!(framebuffer_size(80, 25), Some(8000));
    assert_eq!(framebuffer_size(0, 0), Some(0));
    assert_eq!(pixel_offset(1, 1, 80), Some(324));
    assert_eq!(blt_buffer_size(10, 10, 0), Some(400));
    assert_eq!(blt_buffer_size(10, 10, 60), Some(600));
}

#[test]
fn bounds_helpers() {
    assert!(rectangle_in_display(0, 0, 80, 25, 80, 25));
    assert!(!rectangle_in_display(0, 0, 81, 25, 80, 25));
    assert!(!rectangle_in_display(70, 0, 20, 25, 80, 25));
    assert!(validate_mode_index(0, 1));
    assert!(!validate_mode_index(1, 1));
    assert!(!validate_mode_index(0, 0));
}
