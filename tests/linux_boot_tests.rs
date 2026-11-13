//! Tests for Linux x86 Boot Protocol and bzImage structures.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::arch::x86::linux_boot::{
    BOOT_SIGNATURE, BootParams, E820_RAM, E820_RESERVED, E820Entry, EfiInfo, FLAG_LOADED_HIGH,
    SETUP_HEADER_MAGIC, SETUP_HEADER_OFFSET, SetupHeader, XLF_EFI_HANDOVER_64, XLF_KERNEL_64,
    parse_setup_header,
};

#[test]
fn test_setup_header_validation_and_parsing() {
    let mut buffer = [0u8; 1024];

    // Initially all zeros: invalid
    assert!(parse_setup_header(&buffer).is_none());

    // Place header at offset 0x1F1
    let hdr_ptr = unsafe {
        buffer
            .as_mut_ptr()
            .add(SETUP_HEADER_OFFSET)
            .cast::<SetupHeader>()
    };
    unsafe {
        (*hdr_ptr).signature = BOOT_SIGNATURE;
        (*hdr_ptr).header = SETUP_HEADER_MAGIC;
        (*hdr_ptr).version = 0x020E;
        (*hdr_ptr).load_flags = FLAG_LOADED_HIGH;
        (*hdr_ptr).xloadflags = XLF_KERNEL_64 | XLF_EFI_HANDOVER_64;
        (*hdr_ptr).handover_offset = 0x1000;
        (*hdr_ptr).relocatable_kernel = 1;
    }

    let parsed = parse_setup_header(&buffer).expect("Should parse valid setup header");
    assert!(parsed.is_valid());
    assert!(parsed.is_loaded_high());
    assert!(parsed.is_relocatable());
    assert!(parsed.supports_64bit_kernel());
    assert!(parsed.supports_efi_handover_64());
    assert_eq!(parsed.efi_handover_offset_64(), Some(0x1200));
}

#[test]
fn test_e820_and_efi_info() {
    let entry_ram = E820Entry::new(0x1000_0000, 0x2000_0000, E820_RAM);
    assert!(entry_ram.is_ram());

    let entry_res = E820Entry::new(0x3000_0000, 0x1000, E820_RESERVED);
    assert!(!entry_res.is_ram());

    let efi = EfiInfo::new_64(0x1_0000_0000, 0x2_0000_0000, 4096, 48, 1);
    let sig = efi.efi_loader_signature;
    let systab = efi.efi_systab;
    let systab_hi = efi.efi_systab_hi;
    let memmap = efi.efi_memmap;
    let memmap_hi = efi.efi_memmap_hi;
    let memmap_size = efi.efi_memmap_size;
    assert_eq!(sig, u32::from_le_bytes(*b"EL64"));
    assert_eq!(systab, 0);
    assert_eq!(systab_hi, 1);
    assert_eq!(memmap, 0);
    assert_eq!(memmap_hi, 2);
    assert_eq!(memmap_size, 4096);
}

#[test]
fn test_boot_params_size() {
    // Standard Linux x86 zero-page is exactly 4096 bytes (1 page)
    assert_eq!(core::mem::size_of::<BootParams>(), 4096);
}
