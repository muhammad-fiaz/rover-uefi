//! Security validation tests.

use rover_uefi::protocols::security::secure_boot::validation::{
    signature_count, validate_signature_list_size, validate_signature_size,
};
use rover_uefi::protocols::security::validation::{
    checked_total_size, validate_digest_size, validate_output_length,
};

#[test]
fn digest_sizes() {
    assert!(validate_digest_size(32, 32));
    assert!(validate_digest_size(20, 20));
    assert!(!validate_digest_size(20, 32));
    assert!(!validate_digest_size(32, 20));
    assert!(!validate_digest_size(0, 0));
    assert!(!validate_digest_size(1, 0));
}

#[test]
fn output_and_checked_sizes() {
    assert!(validate_output_length(1));
    assert!(!validate_output_length(0));
    assert_eq!(checked_total_size(0, 0), Some(0));
    assert_eq!(checked_total_size(100, 50), Some(150));
    assert_eq!(checked_total_size(usize::MAX - 1, 10), None);
}

#[test]
fn signature_validation() {
    assert!(validate_signature_list_size(28));
    assert!(validate_signature_list_size(1000));
    assert!(!validate_signature_list_size(0));
    assert!(!validate_signature_list_size(27));
    assert!(validate_signature_size(16));
    assert!(!validate_signature_size(0));
    assert_eq!(signature_count(124, 0, 48), Some(2));
    assert_eq!(signature_count(28, 0, 16), Some(0));
    assert_eq!(signature_count(27, 0, 16), None);
}
