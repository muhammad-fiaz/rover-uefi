//! Tests for Android boot image and vendor boot image headers.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::protocols::android::boot_img::{
    ANDROID_BOOT_MAGIC, AndroidBootImgHeader, AndroidVendorBootHeader, VENDOR_BOOT_MAGIC,
    parse_boot_image, parse_vendor_boot_image,
};

#[test]
fn test_android_boot_img_header_parsing_and_versions() {
    let hdr = AndroidBootImgHeader {
        magic: *ANDROID_BOOT_MAGIC,
        kernel_size: 10_000_000,
        kernel_addr: 0x8000_0000,
        ramdisk_size: 4_000_000,
        ramdisk_addr: 0x8400_0000,
        second_size: 0,
        second_addr: 0,
        tags_addr: 0x8000_0100,
        page_size: 4096,
        header_version: 2,
        // OS version encoded: 14.0.0 (major=14, minor=0, patch=0), patch level 2025-06
        // major = 14 << 14, minor = 0, patch = 0 => os = 14 << 14 = 0x38000
        // os_version field: (os << 11) | ((year - 2000) << 4) | month
        // year = 2025 (25), month = 6 => (25 << 4) | 6 = 406 = 0x196
        // total = (0x38000 << 11) | 0x196 = 0x1C00_0196
        os_version: (14 << 25) | (25 << 4) | 6,
        name: [0u8; 16],
        cmdline: [0u8; 512],
        id: [0u32; 8],
        extra_cmdline: [0u8; 1024],
        recovery_dtbo_size: 100_000,
        recovery_dtbo_offset: 0x1000_0000,
        header_size: core::mem::size_of::<AndroidBootImgHeader>() as u32,
        dtb_size: 200_000,
        dtb_addr: 0x8800_0000,
    };

    assert!(hdr.is_valid());
    assert!(hdr.is_valid_page_size());

    let (major, minor, patch) = hdr.unpack_os_version();
    assert_eq!((major, minor, patch), (14, 0, 0));

    let (year, month) = hdr.unpack_patch_level();
    assert_eq!((year, month), (2025, 6));

    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&raw const hdr).cast::<u8>(),
            core::mem::size_of::<AndroidBootImgHeader>(),
        )
    };

    let parsed = parse_boot_image(bytes).expect("Should parse valid boot image header");
    let ksize = parsed.kernel_size;
    assert_eq!(ksize, 10_000_000);

    let total_size = hdr.calculate_total_size();
    assert!(total_size > 14_000_000);
}

#[test]
fn test_android_vendor_boot_header() {
    let hdr = AndroidVendorBootHeader {
        magic: *VENDOR_BOOT_MAGIC,
        header_version: 4,
        page_size: 4096,
        kernel_addr: 0x8000_0000,
        ramdisk_addr: 0x8200_0000,
        vendor_ramdisk_size: 2_000_000,
        cmdline: [0u8; 2048],
        tags_addr: 0x8000_0100,
        name: [0u8; 16],
        header_size: core::mem::size_of::<AndroidVendorBootHeader>() as u32,
        dtb_size: 500_000,
        dtb_addr: 0x8600_0000,
    };

    assert!(hdr.is_valid());
    assert!(hdr.is_valid_page_size());

    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&raw const hdr).cast::<u8>(),
            core::mem::size_of::<AndroidVendorBootHeader>(),
        )
    };

    let parsed = parse_vendor_boot_image(bytes).expect("Should parse valid vendor boot header");
    let h_ver = parsed.header_version;
    let vr_size = parsed.vendor_ramdisk_size;
    assert_eq!(h_ver, 4);
    assert_eq!(vr_size, 2_000_000);

    let total = hdr.calculate_total_size();
    assert!(total > 2_500_000);
}
