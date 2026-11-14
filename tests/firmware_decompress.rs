// SPDX-License-Identifier: Apache-2.0
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::large_stack_arrays,
    clippy::cast_possible_truncation
)]
//! Unit tests for UEFI and Tiano compression and decompression services and protocols.

use rover_uefi::protocols::firmware::{
    BROTLI_CUSTOM_DECOMPRESS_GUID, EFI_DECOMPRESS_PROTOCOL_GUID, EFI_PEI_DECOMPRESS_PPI_GUID,
    LZMA_CUSTOM_DECOMPRESS_GUID, LZMAF86_CUSTOM_DECOMPRESS_GUID, TIANO_CUSTOM_DECOMPRESS_GUID,
    TIANO_DECOMPRESS_SCRATCH_SIZE, UEFI_DECOMPRESS_SCRATCH_SIZE, compress_tiano, compress_uefi,
    decompress_tiano, decompress_uefi, get_decompress_info,
};

#[test]
fn decompress_constants_and_guids() {
    assert!(!EFI_DECOMPRESS_PROTOCOL_GUID.is_zero());
    assert!(!TIANO_CUSTOM_DECOMPRESS_GUID.is_zero());
    assert!(!LZMA_CUSTOM_DECOMPRESS_GUID.is_zero());
    assert!(!LZMAF86_CUSTOM_DECOMPRESS_GUID.is_zero());
    assert!(!BROTLI_CUSTOM_DECOMPRESS_GUID.is_zero());
    assert!(!EFI_PEI_DECOMPRESS_PPI_GUID.is_zero());

    assert_eq!(UEFI_DECOMPRESS_SCRATCH_SIZE, 65_536);
    assert_eq!(TIANO_DECOMPRESS_SCRATCH_SIZE, 65_536);
}

#[test]
fn decompress_header_parsing_and_bounds() {
    // Corrupted short stream
    let short_data = [0x01, 0x02, 0x03];
    assert!(get_decompress_info(&short_data).is_err());

    // Valid header: CompSize = 4, OrigSize = 10, followed by 4 bytes
    let valid_data = [
        0x04, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0xAA, 0xBB, 0xCC, 0xDD,
    ];
    let Ok(info) = get_decompress_info(&valid_data) else {
        panic!("decompress info failed");
    };
    assert_eq!(info.compressed_size, 4);
    assert_eq!(info.uncompressed_size, 10);
    assert_eq!(info.scratch_size, 65_536);
}

#[test]
fn uefi_compress_and_decompress_roundtrip_empty() {
    let mut comp_buf = [0_u8; 32];
    let comp_len = compress_uefi(&[], &mut comp_buf).expect("compress empty failed");
    assert_eq!(comp_len, 8);

    let info = get_decompress_info(&comp_buf[..comp_len]).expect("get info failed");
    assert_eq!(info.uncompressed_size, 0);

    let mut dest = [0_u8; 16];
    let mut scratch = [0_u8; UEFI_DECOMPRESS_SCRATCH_SIZE];
    decompress_uefi(&comp_buf[..comp_len], &mut dest, &mut scratch)
        .expect("decompress empty failed");
}

#[test]
fn uefi_compress_and_decompress_roundtrip_text() {
    let original =
        b"The UEFI Specification defines extensible firmware interfaces for modern computers!";
    let mut comp_buf = [0_u8; 1024];
    let comp_len = compress_uefi(original, &mut comp_buf).expect("compress failed");
    assert!(comp_len > 8);

    let info = get_decompress_info(&comp_buf[..comp_len]).expect("get info failed");
    assert_eq!(info.uncompressed_size as usize, original.len());

    let mut decompressed = [0_u8; 256];
    let mut scratch = [0_u8; UEFI_DECOMPRESS_SCRATCH_SIZE];
    decompress_uefi(
        &comp_buf[..comp_len],
        &mut decompressed[..original.len()],
        &mut scratch,
    )
    .expect("decompress failed");

    assert_eq!(&decompressed[..original.len()], original);
}

#[test]
fn tiano_compress_and_decompress_roundtrip_text() {
    let original =
        b"The custom compression algorithm utilizes 5-bit position fields and 64KB window.";
    let mut comp_buf = [0_u8; 1024];
    let comp_len = compress_tiano(original, &mut comp_buf).expect("compress tiano failed");
    assert!(comp_len > 8);

    let info = get_decompress_info(&comp_buf[..comp_len]).expect("get info failed");
    assert_eq!(info.uncompressed_size as usize, original.len());

    let mut decompressed = [0_u8; 256];
    let mut scratch = [0_u8; TIANO_DECOMPRESS_SCRATCH_SIZE];
    decompress_tiano(
        &comp_buf[..comp_len],
        &mut decompressed[..original.len()],
        &mut scratch,
    )
    .expect("decompress tiano failed");

    assert_eq!(&decompressed[..original.len()], original);
}

#[test]
fn uefi_compress_and_decompress_repeated_pattern() {
    // Highly compressible repeated pattern
    let mut original = [0_u8; 512];
    for (i, byte) in original.iter_mut().enumerate() {
        *byte = (i % 8) as u8;
    }

    let mut comp_buf = [0_u8; 1024];
    let comp_len = compress_uefi(&original, &mut comp_buf).expect("compress repeated failed");
    assert!(comp_len < original.len()); // Verified compression achieved

    let mut decompressed = [0_u8; 512];
    let mut scratch = [0_u8; UEFI_DECOMPRESS_SCRATCH_SIZE];
    decompress_uefi(&comp_buf[..comp_len], &mut decompressed, &mut scratch)
        .expect("decompress repeated failed");

    assert_eq!(&decompressed[..], &original[..]);
}
