//! Network validation tests.

use rover_uefi::protocols::network::validation::validate_packet_length;

#[test]
fn length_checks() {
    assert!(validate_packet_length(1));
    assert!(validate_packet_length(64));
    assert!(validate_packet_length(1500));
    assert!(!validate_packet_length(0));
    assert!(validate_packet_length(usize::MAX));
}
