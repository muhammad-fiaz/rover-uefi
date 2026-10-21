//! Known-answer tests for SHA-224.

use rover_uefi::protocols::security::sha224::{Sha224, digest_eq_224, sha224};

#[test]
fn sha224_empty_kat() {
    let digest = sha224(b"");
    let expected: [u8; 28] = [
        0xd1, 0x4a, 0x02, 0x8c, 0x2a, 0x3a, 0x2b, 0xc9, 0x47, 0x61, 0x02, 0xbb, 0x28, 0x82, 0x34,
        0xc4, 0x15, 0xa2, 0xb0, 0x1f, 0x82, 0x8e, 0xa6, 0x2a, 0xc5, 0xb3, 0xe4, 0x2f,
    ];
    assert!(digest_eq_224(&digest, &expected));
}

#[test]
fn sha224_abc_kat() {
    let digest = sha224(b"abc");
    let expected: [u8; 28] = [
        0x23, 0x09, 0x7d, 0x22, 0x34, 0x05, 0xd8, 0x22, 0x86, 0x42, 0xa4, 0x77, 0xbd, 0xa2, 0x55,
        0xb3, 0x2a, 0xad, 0xbc, 0xe4, 0xbd, 0xa0, 0xb3, 0xf7, 0xe3, 0x6c, 0x9d, 0xa7,
    ];
    assert!(digest_eq_224(&digest, &expected));
}

#[test]
fn sha224_streaming_and_comparison() {
    let mut hasher = Sha224::new();
    hasher.update(binding_ma(b""));
    hasher.update(binding_ma(b"ab"));
    hasher.update(binding_ma(b"c"));
    assert!(digest_eq_224(&hasher.finalize(), &sha224(b"abc")));
    assert!(!digest_eq_224(&sha224(b"abc"), &sha224(b"abd")));
}

fn binding_ma(bytes: &[u8]) -> &[u8] {
    bytes
}
