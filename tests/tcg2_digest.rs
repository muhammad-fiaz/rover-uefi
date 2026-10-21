//! TCG digest tests.

use rover_uefi::protocols::security::tcg2::pcr::{
    TPM_ALG_SHA1, TPM_ALG_SHA256, TPM_ALG_SHA384, TPM_ALG_SHA512, TPM_ALG_SM3_256,
    digest_size_for_algorithm, is_known_algorithm, validate_pcr_index,
};

#[test]
fn pcr_indexes() {
    assert!(validate_pcr_index(0));
    assert!(validate_pcr_index(23));
    assert!(!validate_pcr_index(24));
    assert!(!validate_pcr_index(u32::MAX));
}

#[test]
fn digest_sizes_by_algorithm() {
    assert_eq!(TPM_ALG_SHA1, 0x0004);
    assert_eq!(TPM_ALG_SHA256, 0x000B);
    assert_eq!(TPM_ALG_SHA384, 0x000C);
    assert_eq!(TPM_ALG_SHA512, 0x000D);
    assert_eq!(TPM_ALG_SM3_256, 0x0012);
    assert_eq!(digest_size_for_algorithm(TPM_ALG_SHA1), Some(20));
    assert_eq!(digest_size_for_algorithm(TPM_ALG_SHA256), Some(32));
    assert_eq!(digest_size_for_algorithm(TPM_ALG_SHA384), Some(48));
    assert_eq!(digest_size_for_algorithm(TPM_ALG_SHA512), Some(64));
    assert_eq!(digest_size_for_algorithm(TPM_ALG_SM3_256), Some(32));
}

#[test]
fn unknown_algorithms() {
    assert_eq!(digest_size_for_algorithm(0x9999), None);
    assert_eq!(digest_size_for_algorithm(0), None);
    assert!(!is_known_algorithm(0xFFFF));
    assert!(is_known_algorithm(TPM_ALG_SHA256));
}
