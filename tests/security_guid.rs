//! Security GUID tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::{
    HASH_GUID, HASH_SERVICE_BINDING_GUID, HASH2_GUID, RNG_GUID, SECURITY_ARCH_GUID,
    SECURITY2_ARCH_GUID, TCG2_FINAL_EVENTS_TABLE_GUID, TCG2_GUID,
};
use rover_uefi::protocols::security::types::{GLOBAL_VARIABLE_GUID, IMAGE_SECURITY_DATABASE_GUID};

#[test]
fn rng_guid_bytes() {
    assert_eq!(
        RNG_GUID.to_bytes(),
        [
            0xA5, 0xBC, 0x52, 0x31, 0xDE, 0xEA, 0x3D, 0x43, 0x86, 0x2E, 0xC0, 0x1C, 0xDC, 0x29,
            0x1F, 0x44
        ]
    );
    assert_eq!(EfiGuid::from_bytes(RNG_GUID.to_bytes()), RNG_GUID);
}

#[test]
fn hash_guid_bytes() {
    assert_eq!(
        HASH_GUID.to_bytes(),
        [
            0x32, 0x49, 0x18, 0xC5, 0xA5, 0xDB, 0xDB, 0x46, 0xA5, 0xBA, 0xCC, 0x0B, 0xDA, 0x9C,
            0x14, 0x35
        ]
    );
    assert_eq!(
        HASH2_GUID.to_bytes(),
        [
            0x34, 0xD7, 0xB1, 0x55, 0xE1, 0xC5, 0xDB, 0x49, 0x96, 0x47, 0xB1, 0x6A, 0xFB, 0x0E,
            0x30, 0x5B
        ]
    );
    assert_eq!(EfiGuid::from_bytes(HASH2_GUID.to_bytes()), HASH2_GUID);
}

#[test]
fn security_guid_bytes() {
    assert_eq!(
        SECURITY_ARCH_GUID.to_bytes(),
        [
            0xE3, 0x23, 0x64, 0xA4, 0x17, 0x46, 0xF1, 0x49, 0xB9, 0xFF, 0xD1, 0xBF, 0xA9, 0x11,
            0x58, 0x39
        ]
    );
    assert_eq!(
        SECURITY2_ARCH_GUID.to_bytes(),
        [
            0x58, 0x2F, 0xAB, 0x94, 0x38, 0x14, 0xF1, 0x4E, 0x91, 0x52, 0x18, 0x94, 0x1A, 0x3A,
            0x0E, 0x68
        ]
    );
    assert_ne!(SECURITY_ARCH_GUID, SECURITY2_ARCH_GUID);
}

#[test]
fn tcg2_guid_bytes() {
    assert_eq!(
        TCG2_GUID.to_bytes(),
        [
            0x6C, 0x76, 0x7F, 0x60, 0x55, 0x74, 0xBE, 0x42, 0x93, 0x0B, 0xE4, 0xD7, 0x6D, 0xB2,
            0x72, 0x0F
        ]
    );
    assert_eq!(EfiGuid::from_bytes(TCG2_GUID.to_bytes()), TCG2_GUID);
    assert!(!TCG2_FINAL_EVENTS_TABLE_GUID.is_zero());
    assert_ne!(TCG2_FINAL_EVENTS_TABLE_GUID, TCG2_GUID);
}

#[test]
fn variable_guid_bytes() {
    assert_eq!(
        GLOBAL_VARIABLE_GUID.to_bytes(),
        [
            0x61, 0xDF, 0xE4, 0x8B, 0xCA, 0x93, 0xD2, 0x11, 0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03,
            0x2B, 0x8C
        ]
    );
    assert_eq!(
        IMAGE_SECURITY_DATABASE_GUID.to_bytes(),
        [
            0xCB, 0xB2, 0x19, 0xD7, 0x3A, 0x3D, 0x96, 0x45, 0xA3, 0xBC, 0xDA, 0xD0, 0x0E, 0x67,
            0x65, 0x6F
        ]
    );
    assert_ne!(GLOBAL_VARIABLE_GUID, IMAGE_SECURITY_DATABASE_GUID);
}

#[test]
fn service_binding_guids_distinct() {
    assert!(!HASH_SERVICE_BINDING_GUID.is_zero());
    assert_ne!(HASH_SERVICE_BINDING_GUID, HASH_GUID);
    assert_ne!(HASH2_GUID, RNG_GUID);
}
