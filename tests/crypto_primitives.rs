//! Known-answer tests for the native SHA-1, AES, AES-GCM, and HKDF
//! implementations.
use std::fmt::Write as _;

use rover_uefi::protocols::security::aes::Aes;
use rover_uefi::protocols::security::gcm::{TAG_SIZE, open, seal};
use rover_uefi::protocols::security::hkdf::hkdf_sha256;
use rover_uefi::protocols::security::sha1::{digest_eq, sha1};

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

#[test]
fn sha1_fips_180_vectors() {
    assert_eq!(hex(&sha1(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(
        hex(&sha1(b"abc")),
        "a9993e364706816aba3e25717850c26c9cd0d89d"
    );
    assert_eq!(
        hex(&sha1(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
        )),
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
    );
    assert!(digest_eq(&sha1(b"abc"), &sha1(b"abc")));
    assert!(!digest_eq(&sha1(b"abc"), &sha1(b"abd")));
}

#[test]
fn sha1_streaming_matches_one_shot() {
    let mut hasher = rover_uefi::protocols::security::sha1::Sha1::new();
    hasher.update(b"abcdbcdecdefdefgefghfghighijh");
    hasher.update(b"ijkijkljklmklmnlmnomnopnopq");
    assert_eq!(
        hex(&hasher.finalize()),
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
    );
}

/// The FIPS-197 appendix C key and plaintext.
const FIPS_KEY: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

const FIPS_PLAINTEXT: [u8; 16] = [
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
];

#[test]
fn aes_fips_197_encryption() {
    let cases: [(&[u8], &str); 3] = [
        (&FIPS_KEY[..16], "69c4e0d86a7b0430d8cdb78070b4c55a"),
        (&FIPS_KEY[..24], "dda97ca4864cdfe06eaf70a0ec0d7191"),
        (&FIPS_KEY[..32], "8ea2b7ca516745bfeafc49904b496089"),
    ];
    for (key, expected) in cases {
        let Some(aes) = Aes::new(key) else {
            panic!("the key length must be accepted");
        };
        let mut output = [0_u8; 16];
        aes.encrypt_block(&FIPS_PLAINTEXT, &mut output);
        assert_eq!(hex(&output), expected);
        let mut recovered = [0_u8; 16];
        aes.decrypt_block(&output, &mut recovered);
        assert_eq!(recovered, FIPS_PLAINTEXT);
    }
}

#[test]
fn aes_rejects_invalid_key_lengths() {
    assert!(Aes::new(&[]).is_none());
    assert!(Aes::new(&[0_u8; 15]).is_none());
    assert!(Aes::new(&[0_u8; 17]).is_none());
    assert!(Aes::new(&[0_u8; 31]).is_none());
    assert!(Aes::new(&[0_u8; 33]).is_none());
    assert_eq!(Aes::new(&[0_u8; 16]).map(|aes| aes.rounds()), Some(10));
    assert_eq!(Aes::new(&[0_u8; 24]).map(|aes| aes.rounds()), Some(12));
    assert_eq!(Aes::new(&[0_u8; 32]).map(|aes| aes.rounds()), Some(14));
}

/// The NIST GCM test case 1: an all-zero key, IV, and empty message.
#[test]
fn aes_gcm_nist_case_1() {
    let key = [0_u8; 16];
    let nonce = [0_u8; 12];
    let mut ciphertext: [u8; 0] = [];
    let mut tag = [0_u8; TAG_SIZE];
    assert!(seal(&key, &nonce, &[], &[], &mut ciphertext, &mut tag));
    assert_eq!(hex(&tag), "58e2fccefa7e3061367f1d57a4e7455a");
}

/// The NIST GCM test case 2: one all-zero block of plaintext.
#[test]
fn aes_gcm_nist_case_2() {
    let key = [0_u8; 16];
    let nonce = [0_u8; 12];
    let plaintext = [0_u8; 16];
    let mut ciphertext = [0_u8; 16];
    let mut tag = [0_u8; TAG_SIZE];
    assert!(seal(
        &key,
        &nonce,
        &[],
        &plaintext,
        &mut ciphertext,
        &mut tag
    ));
    assert_eq!(hex(&ciphertext), "0388dace60b6a392f328c2b971b2fe78");
    assert_eq!(hex(&tag), "ab6e47d42cec13bdf53a67b21257bddf");

    let mut recovered = [0_u8; 16];
    assert!(open(&key, &nonce, &[], &ciphertext, &tag, &mut recovered));
    assert_eq!(recovered, plaintext);
}

#[test]
fn aes_gcm_rejects_modified_tag_and_ciphertext() {
    let key = [0x42_u8; 16];
    let nonce = [0x24_u8; 12];
    let plaintext = b"authenticated firmware payload";
    let mut ciphertext = [0_u8; 30];
    let mut tag = [0_u8; TAG_SIZE];
    assert!(seal(
        key.as_slice(),
        &nonce,
        b"aad",
        plaintext,
        &mut ciphertext,
        &mut tag
    ));

    let mut bad_tag = tag;
    bad_tag[0] ^= 0x01;
    let mut out = [0_u8; 30];
    assert!(!open(&key, &nonce, b"aad", &ciphertext, &bad_tag, &mut out));

    let mut bad_ciphertext = ciphertext;
    bad_ciphertext[3] ^= 0x80;
    assert!(!open(&key, &nonce, b"aad", &bad_ciphertext, &tag, &mut out));

    // A different additional authenticated data value must also fail.
    assert!(!open(&key, &nonce, b"aae", &ciphertext, &tag, &mut out));
}

#[test]
fn aes_gcm_rejects_malformed_parameters() {
    let nonce = [0_u8; 12];
    let mut tag = [0_u8; TAG_SIZE];
    let mut out = [0_u8; 4];
    // Invalid key length.
    assert!(!seal(&[0_u8; 7], &nonce, &[], &[], &mut [], &mut tag));
    // Empty nonce.
    assert!(!seal(&[0_u8; 16], &[], &[], &[], &mut [], &mut tag));
    // Output length mismatch.
    assert!(!seal(
        &[0_u8; 16],
        &nonce,
        &[],
        &[1, 2, 3],
        &mut out,
        &mut tag
    ));
    // Decryption with a mismatched plaintext buffer.
    assert!(!open(&[0_u8; 16], &nonce, &[], &[1, 2], &tag, &mut out));
}

/// RFC 5869 test case 1.
#[test]
fn hkdf_sha256_rfc_5869_case_1() {
    let ikm = [0x0b_u8; 22];
    let salt = [
        0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let info = [
        0xf0_u8, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9,
    ];
    let mut output = [0_u8; 42];
    assert!(hkdf_sha256(&salt, &ikm, &info, &mut output));
    assert_eq!(
        hex(&output),
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
    );
}

/// RFC 5869 test case 3 uses an empty salt and info.
#[test]
fn hkdf_sha256_rfc_5869_case_3() {
    let mut output = [0_u8; 42];
    assert!(hkdf_sha256(&[], &[0x0b_u8; 22], &[], &mut output));
    assert_eq!(
        hex(&output),
        "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8"
    );
}
