//! Integration tests for UEFI EDID protocols and VESA EDID binary parsing.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::graphics::edid::{
    EFI_EDID_ACTIVE_PROTOCOL_GUID, EFI_EDID_DISCOVERED_PROTOCOL_GUID,
    EFI_EDID_OVERRIDE_DONT_OVERRIDE, EFI_EDID_OVERRIDE_ENABLE_HOT_PLUG,
    EFI_EDID_OVERRIDE_PROTOCOL_GUID, EfiEdidActiveProtocol, EfiEdidDiscoveredProtocol,
    EfiEdidOverrideProtocol, parse_edid_block,
};
use rover_uefi::protocols::graphics::guid::{
    EDID_ACTIVE_GUID, EDID_DISCOVERED_GUID, EDID_OVERRIDE_GUID,
};

fn build_sample_edid() -> [u8; 128] {
    let mut block = [0u8; 128];
    // 8-byte header
    block[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);

    // Manufacturer ID: "DEL" (Dell) -> 'D'=4, 'E'=5, 'L'=12
    // mfg_val = (4 << 10) | (5 << 5) | 12 = 0x10AC
    block[8] = 0x10;
    block[9] = 0xAC;

    // Product code: 0x2001
    block[10] = 0x01;
    block[11] = 0x20;

    // Serial number: 0x12345678
    block[12] = 0x78;
    block[13] = 0x56;
    block[14] = 0x34;
    block[15] = 0x12;

    // Week 25, Year 2024 (1990 + 34)
    block[16] = 25;
    block[17] = 34;

    // Version 1, Revision 4
    block[18] = 1;
    block[19] = 4;

    // Detailed Timing Descriptor at offset 54 (0x36)
    // Pixel clock: 14850 * 10kHz = 148.5 MHz -> 0x3A02
    block[54] = 0x02;
    block[55] = 0x3A;

    // H active: 1920 (0x780) -> low 8 bits 0x80, high nibble 0x70 in byte 58
    block[56] = 0x80;
    // H blanking low 8 bits
    block[57] = 0x10;
    // High nibbles (H active high 4 bits: 0x07, H blank high 4 bits: 0x00) -> 0x70
    block[58] = 0x70;

    // V active: 1080 (0x438) -> low 8 bits 0x38, high nibble 0x40 in byte 61
    block[59] = 0x38;
    // V blanking low 8 bits
    block[60] = 0x20;
    // High nibbles (V active high 4 bits: 0x04, V blank high 4 bits: 0x00) -> 0x40
    block[61] = 0x40;

    // Calculate checksum so sum mod 256 == 0
    let sum: u8 = block[0..127]
        .iter()
        .fold(0u8, |acc, &b| acc.wrapping_add(b));
    block[127] = (0u8).wrapping_sub(sum);

    block
}

#[test]
fn test_edid_guid_constants() {
    assert_eq!(
        EFI_EDID_DISCOVERED_PROTOCOL_GUID,
        EfiGuid::new(
            0x1C0C_34F6,
            0xD380,
            0x41FA,
            [0xA0, 0x49, 0x8A, 0xD0, 0x6C, 0x1A, 0x66, 0xAA]
        )
    );
    assert_eq!(EFI_EDID_DISCOVERED_PROTOCOL_GUID, EDID_DISCOVERED_GUID);

    assert_eq!(
        EFI_EDID_ACTIVE_PROTOCOL_GUID,
        EfiGuid::new(
            0xBD8C_1056,
            0x9F36,
            0x44EC,
            [0x92, 0xA8, 0xA6, 0x33, 0x7F, 0x81, 0x79, 0x86]
        )
    );
    assert_eq!(EFI_EDID_ACTIVE_PROTOCOL_GUID, EDID_ACTIVE_GUID);

    assert_eq!(
        EFI_EDID_OVERRIDE_PROTOCOL_GUID,
        EfiGuid::new(
            0x48EC_B431,
            0xFB72,
            0x45C0,
            [0xA9, 0x22, 0xF4, 0x58, 0xFE, 0x04, 0x0B, 0xD5]
        )
    );
    assert_eq!(EFI_EDID_OVERRIDE_PROTOCOL_GUID, EDID_OVERRIDE_GUID);
    assert_eq!(EFI_EDID_OVERRIDE_DONT_OVERRIDE, 0x01);
    assert_eq!(EFI_EDID_OVERRIDE_ENABLE_HOT_PLUG, 0x02);
}

#[test]
fn test_edid_parsing_valid() {
    let block = build_sample_edid();
    let maybe_info = parse_edid_block(&block);
    assert!(maybe_info.is_some());
    if let Some(info) = maybe_info {
        assert_eq!(&info.manufacturer_id, b"DEL");
        assert_eq!(info.product_code, 0x2001);
        assert_eq!(info.serial_number, 0x1234_5678);
        assert_eq!(info.manufacture_week, 25);
        assert_eq!(info.manufacture_year, 2024);
        assert_eq!(info.version, 1);
        assert_eq!(info.revision, 4);
        assert_eq!(info.preferred_width, 1920);
        assert_eq!(info.preferred_height, 1080);
        assert_eq!(info.preferred_pixel_clock_10khz, 0x3A02);
    }
}

#[test]
fn test_edid_parsing_invalid_checksum() {
    let mut block = build_sample_edid();
    block[127] = block[127].wrapping_add(1); // Corrupt checksum
    assert!(parse_edid_block(&block).is_none());
}

#[test]
fn test_edid_parsing_invalid_header() {
    let mut block = build_sample_edid();
    block[0] = 0xAA; // Corrupt header
    assert!(parse_edid_block(&block).is_none());
}

#[test]
fn test_edid_parsing_too_short() {
    let short_buf = [0u8; 100];
    assert!(parse_edid_block(&short_buf).is_none());
}

#[test]
fn test_edid_protocol_abi_layouts() {
    assert_eq!(
        core::mem::size_of::<EfiEdidDiscoveredProtocol>(),
        if core::mem::size_of::<usize>() == 8 {
            16
        } else {
            8
        }
    );
    assert_eq!(
        core::mem::size_of::<EfiEdidActiveProtocol>(),
        if core::mem::size_of::<usize>() == 8 {
            16
        } else {
            8
        }
    );
    assert_eq!(
        core::mem::size_of::<EfiEdidOverrideProtocol>(),
        core::mem::size_of::<usize>()
    );
}

unsafe extern "efiapi" fn dummy_get_edid(
    _this: *mut EfiEdidOverrideProtocol,
    _child: *mut EfiHandle,
    attributes: *mut u32,
    edid_size: *mut UIntN,
    edid: *mut *mut u8,
) -> EfiStatus {
    if !attributes.is_null() {
        unsafe { *attributes = EFI_EDID_OVERRIDE_DONT_OVERRIDE };
    }
    if !edid_size.is_null() {
        unsafe { *edid_size = 0 };
    }
    if !edid.is_null() {
        unsafe { *edid = core::ptr::null_mut() };
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_edid_override_protocol_invocation() {
    let mut proto = EfiEdidOverrideProtocol {
        get_edid: dummy_get_edid,
    };
    let mut attr = 0u32;
    let mut size: UIntN = 100;
    let mut edid_ptr: *mut u8 = 0x1234 as *mut u8;

    let status = unsafe {
        (proto.get_edid)(
            core::ptr::addr_of_mut!(proto),
            core::ptr::null_mut(),
            core::ptr::addr_of_mut!(attr),
            core::ptr::addr_of_mut!(size),
            core::ptr::addr_of_mut!(edid_ptr),
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(attr, EFI_EDID_OVERRIDE_DONT_OVERRIDE);
    assert_eq!(size, 0);
    assert!(edid_ptr.is_null());
}
