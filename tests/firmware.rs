//! Tests for firmware formats.

use rover_uefi::protocols::firmware::firmvol::{
    EFI_FV_UNRELIABLE_WRITE, EFI_FV2_READ_DISABLE_CAP, EFI_FVB2_READ_DISABLED_CAP,
    EFI_LBA_LIST_TERMINATOR, EfiFirmwareVolume2Protocol, EfiFirmwareVolumeBlock2Protocol,
    FIRMWARE_VOLUME_BLOCK2_PROTOCOL_GUID, FIRMWARE_VOLUME2_PROTOCOL_GUID, FV_SIGNATURE,
    parse_firmware_volume,
};
use rover_uefi::protocols::firmware::fmp::{
    EfiFirmwareImageDescriptor, EfiFirmwareManagementProtocol, FIRMWARE_MANAGEMENT_PROTOCOL_GUID,
    FMP_CAPSULE_GUID, FMP_GUID, FMP_PROTOCOL_GUID, IMAGE_ATTRIBUTE_IMAGE_UPDATABLE,
    IMAGE_UPDATABLE_VALID, PACKAGE_ATTRIBUTE_VERSION_UPDATABLE, parse_fmp_capsule_header,
};
use rover_uefi::protocols::firmware::hii::{
    HII_DATABASE_GUID, HII_FORMS_GUID, is_known_package_type, parse_package_header,
};
use rover_uefi::protocols::firmware::pecoff::{DOS_MAGIC, PE_SIGNATURE, parse_pe_image};

fn write_u16(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}
fn write_u32(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn build_pe(kind: u16, optional_size: u16) -> Vec<u8> {
    let mut buf = vec![0_u8; 1024];
    write_u16(&mut buf, 0, DOS_MAGIC);
    write_u32(&mut buf, 60, 64);
    write_u32(&mut buf, 64, PE_SIGNATURE);
    write_u16(&mut buf, 68, 0x8664);
    write_u16(&mut buf, 70, 1);
    write_u16(&mut buf, 84, optional_size);
    write_u16(&mut buf, 86, 0x22);
    write_u16(&mut buf, 88, kind);
    // Entry point at optional+16
    write_u32(&mut buf, 88 + 16, 0x1000);
    // Section alignment at +32, file alignment at +36
    write_u32(&mut buf, 88 + 32, 0x1000);
    write_u32(&mut buf, 88 + 36, 0x200);
    // SizeOfImage at +56
    write_u32(&mut buf, 88 + 56, 0x3000);
    // Subsystem at +68
    write_u16(&mut buf, 88 + 68, 10);
    let dir_count_off = if kind == 0x20B { 108 } else { 92 };
    write_u32(&mut buf, 88 + dir_count_off, 16);
    // Section header after optional
    let sec_off = 88 + optional_size as usize;
    buf[sec_off] = b'.';
    buf[sec_off + 1] = b't';
    buf[sec_off + 2] = b'e';
    buf[sec_off + 3] = b'x';
    buf[sec_off + 4] = b't';
    write_u32(&mut buf, sec_off + 8, 0x100);
    write_u32(&mut buf, sec_off + 12, 0x1000);
    write_u32(&mut buf, sec_off + 16, 0x200);
    write_u32(&mut buf, sec_off + 20, 0x200);
    write_u32(&mut buf, sec_off + 36, 0x6000_0020);
    buf
}

#[test]
fn pe32_plus_parses() {
    let buf = build_pe(0x20B, 240);
    let Some(image) = parse_pe_image(&buf) else {
        panic!("PE32+ should parse");
    };
    assert_eq!(image.dos().magic, DOS_MAGIC);
    assert_eq!(image.optional().entry_point, 0x1000);
    assert_eq!(image.sections().count(), 1);
    let Some(section) = image.sections().get(0) else {
        panic!("expected section");
    };
    assert_eq!(section.name[..4], *b".tex");
}

#[test]
fn pe32_parses() {
    let buf = build_pe(0x10B, 224);
    let Some(image) = parse_pe_image(&buf) else {
        panic!("PE32 should parse");
    };
    assert_eq!(image.coff().section_count, 1);
    assert!(image.directories().get(0).is_some());
}

#[test]
fn pe_rejections() {
    assert!(parse_pe_image(&[]).is_none());
    assert!(parse_pe_image(&[0_u8; 10]).is_none());
    let mut buf = build_pe(0x20B, 240);
    buf[0] = 0x00;
    assert!(parse_pe_image(&buf).is_none());
    let mut buf2 = build_pe(0x20B, 240);
    buf2[64] = 0x00;
    assert!(parse_pe_image(&buf2).is_none());
    let buf3 = build_pe(0xFFFF, 240);
    assert!(parse_pe_image(&buf3).is_none());
}

#[test]
fn fv_parses() {
    let mut buf = vec![0_u8; 128];
    buf[48..56].copy_from_slice(&128_u64.to_le_bytes());
    write_u32(&mut buf, 64, FV_SIGNATURE);
    write_u16(&mut buf, 68, 72);
    buf[71] = 2;
    let Some(volume) = parse_firmware_volume(&buf) else {
        panic!("FV should parse");
    };
    assert_eq!(volume.header().volume_length, 128);
    let mut bad = vec![0_u8; 128];
    bad[48..56].copy_from_slice(&128_u64.to_le_bytes());
    write_u32(&mut bad, 64, 0);
    assert!(parse_firmware_volume(&bad).is_none());
}

#[test]
fn hii_packages() {
    assert_ne!(HII_DATABASE_GUID, HII_FORMS_GUID);
    let mut buf = vec![0_u8; 32];
    write_u32(&mut buf, 0, 32);
    write_u32(&mut buf, 4, 32);
    buf[8] = 0x04;
    let Some(header) = parse_package_header(&buf) else {
        panic!("package should parse");
    };
    assert_eq!(header.package_length, 32);
    assert!(is_known_package_type(0x04));
    assert!(parse_package_header(&[0_u8; 10]).is_none());
}

#[test]
fn fmp_header() {
    assert!(!FMP_GUID.is_zero());
    assert_eq!(FMP_GUID, FIRMWARE_MANAGEMENT_PROTOCOL_GUID);
    assert_eq!(FMP_PROTOCOL_GUID, FIRMWARE_MANAGEMENT_PROTOCOL_GUID);
    assert_eq!(
        FIRMWARE_MANAGEMENT_PROTOCOL_GUID.to_bytes(),
        [
            0x67, 0x7A, 0xC7, 0x86, 0x97, 0x0B, 0x33, 0x46, 0xA1, 0x87, 0x49, 0x10, 0x4D, 0x06,
            0x85, 0xC7
        ]
    );
    assert_eq!(
        FMP_CAPSULE_GUID.to_bytes(),
        [
            0xED, 0xD5, 0xCB, 0x6D, 0x2D, 0xE8, 0x44, 0x4C, 0xBD, 0xA1, 0x71, 0x94, 0x19, 0x9A,
            0xD9, 0x2A
        ]
    );

    assert_eq!(IMAGE_ATTRIBUTE_IMAGE_UPDATABLE, 0x1);
    assert_eq!(IMAGE_UPDATABLE_VALID, 0x1);
    assert_eq!(PACKAGE_ATTRIBUTE_VERSION_UPDATABLE, 0x1);

    assert_eq!(
        core::mem::size_of::<EfiFirmwareManagementProtocol>(),
        6 * core::mem::size_of::<usize>()
    );
    assert!(core::mem::size_of::<EfiFirmwareImageDescriptor>() > 0);
    assert_eq!(
        core::mem::size_of::<EfiFirmwareVolume2Protocol>(),
        9 * core::mem::size_of::<usize>() + 8 // 8 ptrs + u32 key_size + 4 bytes padding + parent_handle
    );
    assert_eq!(
        core::mem::size_of::<EfiFirmwareVolumeBlock2Protocol>(),
        8 * core::mem::size_of::<usize>()
    );

    assert_eq!(EFI_FV2_READ_DISABLE_CAP, 0x1);
    assert_eq!(EFI_FVB2_READ_DISABLED_CAP, 0x1);
    assert_eq!(EFI_FV_UNRELIABLE_WRITE, 0x0);
    assert_eq!(EFI_LBA_LIST_TERMINATOR, 0xFFFF_FFFF_FFFF_FFFF);

    assert_eq!(
        FIRMWARE_VOLUME2_PROTOCOL_GUID.to_bytes(),
        [
            0xB6, 0x73, 0x0E, 0x22, 0xDB, 0x6B, 0x13, 0x44, 0x84, 0x05, 0xB9, 0x74, 0xB1, 0x08,
            0x61, 0x9A
        ]
    );
    assert_eq!(
        FIRMWARE_VOLUME_BLOCK2_PROTOCOL_GUID.to_bytes(),
        [
            0xA9, 0x4F, 0x64, 0x8F, 0x50, 0xE8, 0xB1, 0x4D, 0x9C, 0xE2, 0x0B, 0x44, 0x69, 0x8E,
            0x8D, 0xA4
        ]
    );

    let mut buf = vec![0_u8; 48];
    write_u32(&mut buf, 16, 48);
    write_u32(&mut buf, 20, 0);
    write_u32(&mut buf, 40, 1);
    write_u32(&mut buf, 44, 4);
    let Some(header) = parse_fmp_capsule_header(&buf) else {
        panic!("FMP header should parse");
    };
    assert_eq!(header.header_size, 48);
    assert!(parse_fmp_capsule_header(&[0_u8; 10]).is_none());
}
