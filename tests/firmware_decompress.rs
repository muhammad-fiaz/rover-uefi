// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI decompression services and protocol.

use rover_uefi::protocols::firmware::{
    EFI_DECOMPRESS_PROTOCOL_GUID, UEFI_DECOMPRESS_SCRATCH_SIZE, get_decompress_info,
};

#[test]
fn decompress_constants_and_info() {
    assert!(!EFI_DECOMPRESS_PROTOCOL_GUID.is_zero());
    assert_eq!(UEFI_DECOMPRESS_SCRATCH_SIZE, 65_536);

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
