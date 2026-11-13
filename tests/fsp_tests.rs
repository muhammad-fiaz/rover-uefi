//! Integration tests for Intel Firmware Support Package (FSP) parser and structures.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rover_uefi::protocols::firmware::fsp::*;

#[test]
fn test_fsp_constants_and_status() {
    assert_eq!(FSP_INFO_HEADER_OFFSET, 0x94);
    assert_eq!(FSP_INFO_HEADER_SIGNATURE, 0x4850_5346);

    assert_eq!(FSP_COMPONENT_T, 1);
    assert_eq!(FSP_COMPONENT_M, 2);
    assert_eq!(FSP_COMPONENT_S, 3);
    assert_eq!(FSP_COMPONENT_I, 4);
    assert_eq!(FSP_COMPONENT_O, 8);

    let cold_status = encode_fsp_reset_request(FSP_STATUS_RESET_REQUIRED_COLD);
    assert!(cold_status.is_error());

    let warm_status = encode_fsp_reset_request(FSP_STATUS_RESET_REQUIRED_WARM);
    assert!(warm_status.is_error());
}

#[test]
fn test_fsp_image_parsing() {
    let mut raw_image = [0_u8; 512];
    let offset = FSP_INFO_HEADER_OFFSET;

    // Signature ('FSPH')
    raw_image[offset..offset + 4].copy_from_slice(&FSP_INFO_HEADER_SIGNATURE.to_le_bytes());
    // HeaderLength (76)
    raw_image[offset + 4..offset + 8].copy_from_slice(&76_u32.to_le_bytes());
    // SpecVersion (0x24)
    raw_image[offset + 10] = 0x24;
    // HeaderRevision (7)
    raw_image[offset + 11] = 7;
    // ImageRevision (0x01020304)
    raw_image[offset + 12..offset + 16].copy_from_slice(&0x0102_0304_u32.to_le_bytes());
    // ImageId
    raw_image[offset + 16..offset + 24].copy_from_slice(b"TESTFSP1");
    // ImageSize (512)
    raw_image[offset + 24..offset + 28].copy_from_slice(&512_u32.to_le_bytes());
    // ImageBase (0xFFF0_0000)
    raw_image[offset + 28..offset + 32].copy_from_slice(&0xFFF0_0000_u32.to_le_bytes());
    // ImageAttribute (Graphics + 64bit + Dispatch)
    let attr = IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT
        | IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT
        | IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT;
    raw_image[offset + 32..offset + 34].copy_from_slice(&attr.to_le_bytes());
    // ComponentAttribute (FSP-S = 3 in bits 15:12 => 0x3000)
    raw_image[offset + 34..offset + 36].copy_from_slice(&0x3000_u16.to_le_bytes());

    let fsp_img = FspImage::parse(&raw_image).expect("valid FSP image header should parse");
    assert_eq!(fsp_img.signature(), FSP_INFO_HEADER_SIGNATURE);
    assert_eq!(fsp_img.spec_version(), 0x24);
    assert_eq!(fsp_img.header_revision(), 7);
    assert_eq!(&fsp_img.image_id(), b"TESTFSP1");
    assert_eq!(fsp_img.component_type(), FSP_COMPONENT_S);
    assert!(fsp_img.has_graphics_support());
    assert!(fsp_img.has_dispatch_mode());
    assert!(fsp_img.is_64bit_mode());
}

#[test]
fn test_fsp_error_cases() {
    let short_buf = [0_u8; 100];
    assert_eq!(
        FspImage::parse(&short_buf).unwrap_err(),
        FspError::BufferTooSmall
    );

    let mut bad_sig_buf = [0_u8; 256];
    bad_sig_buf[FSP_INFO_HEADER_OFFSET..FSP_INFO_HEADER_OFFSET + 4].copy_from_slice(b"ABCD");
    assert_eq!(
        FspImage::parse(&bad_sig_buf).unwrap_err(),
        FspError::InvalidSignature
    );
}
