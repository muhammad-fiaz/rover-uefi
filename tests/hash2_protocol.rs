//! Hash2 tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::HASH2_GUID;
use rover_uefi::protocols::security::hash::algorithm::{SHA256_GUID, digest_size};
use rover_uefi::protocols::security::hash::hash2::EfiHash2Protocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        HASH2_GUID.to_bytes(),
        [
            0x34, 0xD7, 0xB1, 0x55, 0xE1, 0xC5, 0xDB, 0x49, 0x96, 0x47, 0xB1, 0x6A, 0xFB, 0x0E,
            0x30, 0x5B
        ]
    );
    assert_eq!(EfiGuid::from_bytes(HASH2_GUID.to_bytes()), HASH2_GUID);
    assert!(!HASH2_GUID.is_zero());
}

#[test]
fn protocol_has_five_entries() {
    assert_eq!(
        size_of::<EfiHash2Protocol>(),
        5 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiHash2Protocol, get_hash_size), 0);
    assert_eq!(
        offset_of!(EfiHash2Protocol, hash_final),
        4 * core::mem::size_of::<usize>()
    );
}

#[test]
fn digest_size_via_shared_table() {
    assert_eq!(digest_size(&SHA256_GUID), Some(32));
    assert_eq!(digest_size(&HASH2_GUID), None);
}
