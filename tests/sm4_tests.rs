//! Integration tests for SM4 128-bit block cipher algorithm.

use rover_uefi::protocols::security::sm4::{SM4_BLOCK_SIZE, SM4_KEY_SIZE, SM4_NUM_ROUNDS, Sm4};

#[test]
fn test_sm4_constants() {
    assert_eq!(SM4_KEY_SIZE, 16);
    assert_eq!(SM4_BLOCK_SIZE, 16);
    assert_eq!(SM4_NUM_ROUNDS, 32);
}

#[test]
fn test_sm4_standard_test_vector() {
    let key: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let plaintext: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let expected_ciphertext: [u8; 16] = [
        0x68, 0x1E, 0xDF, 0x34, 0xD2, 0x06, 0x96, 0x5E, 0x86, 0xB3, 0xE9, 0x4F, 0x53, 0x6E, 0x42,
        0x46,
    ];

    let cipher = Sm4::new(&key);

    let mut ciphertext = [0u8; 16];
    cipher.encrypt_block(&plaintext, &mut ciphertext);
    assert_eq!(ciphertext, expected_ciphertext);

    let mut decrypted = [0u8; 16];
    cipher.decrypt_block(&ciphertext, &mut decrypted);
    assert_eq!(decrypted, plaintext);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_sm4_cbc_mode_round_trip() {
    let key: [u8; 16] = [0xA5; 16];
    let iv: [u8; 16] = [0x5A; 16];

    let mut data = [0u8; 48];
    for (i, b) in data.iter_mut().enumerate() {
        *b = (i * 7 + 13) as u8;
    }
    let original = data;

    let cipher = Sm4::new(&key);
    cipher.cbc_encrypt(&iv, &mut data);
    assert_ne!(data, original);

    cipher.cbc_decrypt(&iv, &mut data);
    assert_eq!(data, original);
}
