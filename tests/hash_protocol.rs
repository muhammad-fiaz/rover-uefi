//! Hash tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::HASH_GUID;
use rover_uefi::protocols::security::hash::algorithm::{
    MD5_DIGEST_SIZE, MD5_GUID, SHA1_DIGEST_SIZE, SHA1_GUID, SHA224_DIGEST_SIZE, SHA224_GUID,
    SHA256_DIGEST_SIZE, SHA256_GUID, SHA384_DIGEST_SIZE, SHA384_GUID, SHA512_DIGEST_SIZE,
    SHA512_GUID, digest_size,
};

#[test]
fn algorithm_guid_bytes() {
    assert_eq!(
        SHA256_GUID.to_bytes(),
        [
            0xDE, 0x59, 0xAA, 0x51, 0xF2, 0xFD, 0xA3, 0x4E, 0xBC, 0x63, 0x87, 0x5F, 0xB7, 0x84,
            0x2E, 0xE9
        ]
    );
    assert_eq!(
        SHA1_GUID.to_bytes(),
        [
            0x0F, 0xD8, 0xE9, 0x2A, 0xB2, 0x3F, 0x95, 0x40, 0xB7, 0xB1, 0xE9, 0x31, 0x57, 0xB9,
            0x46, 0xB6
        ]
    );
    assert_eq!(EfiGuid::from_bytes(SHA256_GUID.to_bytes()), SHA256_GUID);
}

#[test]
fn digest_sizes_match_spec() {
    assert_eq!(SHA1_DIGEST_SIZE, 20);
    assert_eq!(SHA224_DIGEST_SIZE, 28);
    assert_eq!(SHA256_DIGEST_SIZE, 32);
    assert_eq!(SHA384_DIGEST_SIZE, 48);
    assert_eq!(SHA512_DIGEST_SIZE, 64);
    assert_eq!(MD5_DIGEST_SIZE, 16);
    assert_eq!(digest_size(&SHA1_GUID), Some(20));
    assert_eq!(digest_size(&SHA224_GUID), Some(28));
    assert_eq!(digest_size(&SHA256_GUID), Some(32));
    assert_eq!(digest_size(&SHA384_GUID), Some(48));
    assert_eq!(digest_size(&SHA512_GUID), Some(64));
    assert_eq!(digest_size(&MD5_GUID), Some(16));
}

#[test]
fn unknown_algorithm_has_no_size() {
    assert_eq!(digest_size(&HASH_GUID), None);
    assert_eq!(digest_size(&EfiGuid::ZERO), None);
}

#[test]
fn algorithm_guids_are_distinct() {
    assert_ne!(SHA1_GUID, SHA256_GUID);
    assert_ne!(SHA384_GUID, SHA512_GUID);
    assert_ne!(SHA224_GUID, MD5_GUID);
    assert!(!SHA256_GUID.is_zero());
}
