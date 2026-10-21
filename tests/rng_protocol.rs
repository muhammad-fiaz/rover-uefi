//! RNG tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::rng::algorithm::{
    ALGORITHM_ARM_RNDR, ALGORITHM_RAW, ALGORITHM_SP800_90_CTR_256, ALGORITHM_SP800_90_HASH_256,
    ALGORITHM_SP800_90_HMAC_256, ALGORITHM_X9_31_3DES, ALGORITHM_X9_31_AES, is_defined_algorithm,
};
use rover_uefi::protocols::security::rng::protocol::EfiRngProtocol;
use rover_uefi::protocols::security::validation::{checked_total_size, validate_output_length};

#[test]
fn algorithm_guid_bytes() {
    assert_eq!(
        ALGORITHM_RAW.to_bytes(),
        [
            0xD7, 0x76, 0x31, 0xE4, 0xE8, 0xB6, 0x27, 0x48, 0xB7, 0x84, 0x7F, 0xFD, 0xC4, 0xB6,
            0x85, 0x61
        ]
    );
    assert_eq!(
        ALGORITHM_SP800_90_HASH_256.to_bytes(),
        [
            0xCB, 0x67, 0xAF, 0xA7, 0x3B, 0x60, 0x42, 0x4D, 0xBA, 0x21, 0x70, 0xBF, 0xB6, 0x29,
            0x3F, 0x96
        ]
    );
    assert_eq!(EfiGuid::from_bytes(ALGORITHM_RAW.to_bytes()), ALGORITHM_RAW);
}

#[test]
fn all_defined_algorithms_recognized() {
    assert!(is_defined_algorithm(&ALGORITHM_RAW));
    assert!(is_defined_algorithm(&ALGORITHM_SP800_90_HASH_256));
    assert!(is_defined_algorithm(&ALGORITHM_SP800_90_HMAC_256));
    assert!(is_defined_algorithm(&ALGORITHM_SP800_90_CTR_256));
    assert!(is_defined_algorithm(&ALGORITHM_X9_31_3DES));
    assert!(is_defined_algorithm(&ALGORITHM_X9_31_AES));
    assert!(is_defined_algorithm(&ALGORITHM_ARM_RNDR));
    assert!(!is_defined_algorithm(&EfiGuid::ZERO));
}

#[test]
fn unknown_algorithm_preserved_not_substituted() {
    let unknown = EfiGuid::new(1, 2, 3, [4; 8]);
    assert!(!is_defined_algorithm(&unknown));
    assert_ne!(unknown, ALGORITHM_RAW);
}

#[test]
fn output_length_validation() {
    assert!(validate_output_length(1));
    assert!(validate_output_length(4096));
    assert!(!validate_output_length(0));
    assert_eq!(checked_total_size(10, 5), Some(15));
    assert_eq!(checked_total_size(usize::MAX, 1), None);
}

#[test]
fn protocol_layout_distinct_from_hash() {
    assert_eq!(
        core::mem::size_of::<EfiRngProtocol>(),
        2 * core::mem::size_of::<usize>()
    );
    assert!(validate_output_length(32));
}
