//! Known-answer tests for SHA-1, HMAC, HKDF, AES and AES-GCM.

use rover_uefi::protocols::security::aes::Aes;
use rover_uefi::protocols::security::gcm::{open, seal};
use rover_uefi::protocols::security::hkdf::{expand, extract, hkdf_sha256};
use rover_uefi::protocols::security::hmac::{
    HmacSha256, hmac_sha1, hmac_sha256, hmac_sha384, hmac_sha512,
};
use rover_uefi::protocols::security::rsa::{parse_rsa_public_key, verify_sha256};
use rover_uefi::protocols::security::sha1::{digest_eq, sha1};

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let clean: String = hex.chars().filter(char::is_ascii_hexdigit).collect();
    clean
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let Ok(text) = std::str::from_utf8(pair) else {
                panic!("invalid hex literal")
            };
            let Ok(value) = u8::from_str_radix(text, 16) else {
                panic!("invalid hex digit")
            };
            value
        })
        .collect()
}

fn hex_to_array<const N: usize>(hex: &str) -> [u8; N] {
    let bytes = hex_to_bytes(hex);
    assert_eq!(bytes.len(), N, "unexpected length for {hex}");
    let mut array = [0_u8; N];
    array.copy_from_slice(&bytes);
    array
}

#[test]
fn sha1_known_answers() {
    let empty = [
        0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55, 0xbf, 0xef, 0x95, 0x60, 0x18,
        0x90, 0xaf, 0xd8, 0x07, 0x09,
    ];
    assert!(digest_eq(&sha1(b""), &empty));
    let abc = [
        0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25, 0x71, 0x78, 0x50, 0xc2,
        0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
    ];
    assert!(digest_eq(&sha1(b"abc"), &abc));
    let long = [
        0x84, 0x98, 0x3e, 0x44, 0x1c, 0x3b, 0xd2, 0x6e, 0xba, 0xae, 0x4a, 0xa1, 0xf9, 0x51, 0x29,
        0xe5, 0xe5, 0x46, 0x70, 0xf1,
    ];
    assert!(digest_eq(
        &sha1(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        &long
    ));
    let million = vec![b'a'; 1_000_000];
    let digest = sha1(&million);
    let expected = [
        0x34, 0xaa, 0x97, 0x3c, 0xd4, 0xc4, 0xda, 0xa4, 0xf6, 0x1e, 0xeb, 0x2b, 0xdb, 0xad, 0x27,
        0x31, 0x65, 0x34, 0x01, 0x6f,
    ];
    assert_eq!(digest, expected);
}

#[test]
fn sha1_streaming_matches_single_shot() {
    let message = b"streaming SHA-1 input split across arbitrary boundaries";
    let mut hasher = rover_uefi::protocols::security::sha1::Sha1::new();
    for chunk in message.chunks(7) {
        hasher.update(chunk);
    }
    assert_eq!(hasher.finalize(), sha1(message));
}

#[test]
fn hmac_sha1_rfc2202() {
    assert_eq!(
        hex_to_array::<20>("b617318655057264e28bc0b6fb378c8ef146be00"),
        hmac_sha1(&[0x0b; 20], b"Hi There")
    );
    assert_eq!(
        hex_to_array::<20>("effcdf6ae5eb2fa2d27416d5f184df9c259a7c79"),
        hmac_sha1(b"Jefe", b"what do ya want for nothing?")
    );
    assert_eq!(
        hex_to_array::<20>("125d7342b9ac11cd91a39af48aa17b4f63f175d3"),
        hmac_sha1(&[0xaa; 20], &[0xdd; 50])
    );
    assert_eq!(
        hex_to_array::<20>("aa4ae5e15272d00e95705637ce8a3b55ed402112"),
        hmac_sha1(
            &[0xaa; 80],
            b"Test Using Larger Than Block-Size Key - Hash Key First"
        )
    );
}

#[test]
fn hmac_sha256_rfc4231() {
    assert_eq!(
        hex_to_array::<32>("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
        hmac_sha256(&[0x0b; 20], b"Hi There")
    );
    assert_eq!(
        hex_to_array::<32>("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
        hmac_sha256(b"Jefe", b"what do ya want for nothing?")
    );
    assert_eq!(
        hex_to_array::<32>("60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"),
        hmac_sha256(
            &[0xaa; 131],
            b"Test Using Larger Than Block-Size Key - Hash Key First"
        )
    );
}

#[test]
fn hmac_sha384_and_sha512_rfc4231() {
    assert_eq!(
        hex_to_array::<48>(
            "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59cfaea9ea9076ede7f4af152e8b2fa9cb6"
        ),
        hmac_sha384(&[0x0b; 20], b"Hi There")
    );
    assert_eq!(
        hex_to_array::<64>(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        ),
        hmac_sha512(&[0x0b; 20], b"Hi There")
    );
}

#[test]
fn hmac_streaming_matches_single_shot() {
    let mut state = HmacSha256::new(b"key material");
    state.update(b"part one ");
    state.update(b"part two");
    let mut streamed = [0_u8; 32];
    assert!(state.finalize_into(&mut streamed));
    assert_eq!(streamed, hmac_sha256(b"key material", b"part one part two"));
    let mut short = [0_u8; 16];
    assert!(!HmacSha256::new(b"key").finalize_into(&mut short));
}

#[test]
fn hkdf_rfc5869_test_case_one() {
    let ikm = [0x0b; 22];
    let salt = hex_to_bytes("000102030405060708090a0b0c");
    let info = hex_to_bytes("f0f1f2f3f4f5f6f7f8f9");
    let mut prk = [0_u8; 32];
    assert!(extract::<rover_uefi::protocols::security::sha256::Sha256>(
        &salt, &ikm, &mut prk
    ));
    assert_eq!(
        prk,
        hex_to_array::<32>("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5")
    );
    let mut okm = [0_u8; 42];
    assert!(expand::<rover_uefi::protocols::security::sha256::Sha256>(
        &prk, &info, &mut okm
    ));
    assert_eq!(
        okm.to_vec(),
        hex_to_bytes(
            "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
        )
    );
}

#[test]
fn hkdf_rfc5869_test_case_three() {
    let mut okm = [0_u8; 42];
    assert!(hkdf_sha256(&[], &[0x0b; 22], &[], &mut okm));
    assert_eq!(
        okm.to_vec(),
        hex_to_bytes(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8"
        )
    );
}

#[test]
fn hkdf_rejects_oversized_output() {
    let mut output = vec![0_u8; 255 * 32 + 1];
    assert!(!hkdf_sha256(b"salt", b"ikm", b"info", &mut output));
}

#[test]
fn aes_fips197_block_vectors() {
    let plaintext = hex_to_array::<16>("00112233445566778899aabbccddeeff");
    let mut output = [0_u8; 16];

    let aes128 = Aes::new_128(&hex_to_array::<16>("000102030405060708090a0b0c0d0e0f"));
    aes128.encrypt_block(&plaintext, &mut output);
    assert_eq!(
        output,
        hex_to_array::<16>("69c4e0d86a7b0430d8cdb78070b4c55a")
    );
    let ciphertext = output;
    aes128.decrypt_block(&ciphertext, &mut output);
    assert_eq!(output, plaintext);

    let aes192 = Aes::new_192(&hex_to_array::<24>(
        "000102030405060708090a0b0c0d0e0f1011121314151617",
    ));
    aes192.encrypt_block(&plaintext, &mut output);
    assert_eq!(
        output,
        hex_to_array::<16>("dda97ca4864cdfe06eaf70a0ec0d7191")
    );
    let ciphertext = output;
    aes192.decrypt_block(&ciphertext, &mut output);
    assert_eq!(output, plaintext);

    let aes256 = Aes::new_256(&hex_to_array::<32>(
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    ));
    aes256.encrypt_block(&plaintext, &mut output);
    assert_eq!(
        output,
        hex_to_array::<16>("8ea2b7ca516745bfeafc49904b496089")
    );
    let ciphertext = output;
    aes256.decrypt_block(&ciphertext, &mut output);
    assert_eq!(output, plaintext);

    let from_slice = Aes::new(&hex_to_bytes("000102030405060708090a0b0c0d0e0f"));
    assert_eq!(from_slice, Some(aes128));
    assert_eq!(Aes::new(&[0_u8; 15]), None);
}

#[test]
fn aes_gcm_nist_empty_plaintext() {
    let mut tag = [0_u8; 16];
    let mut ciphertext = [];
    assert!(seal(
        &[0_u8; 16],
        &[0_u8; 12],
        &[],
        &[],
        &mut ciphertext,
        &mut tag
    ));
    assert_eq!(tag, hex_to_array::<16>("58e2fccefa7e3061367f1d57a4e7455a"));
}

#[test]
fn aes_gcm_nist_one_block() {
    let key = [0_u8; 16];
    let nonce = [0_u8; 12];
    let plaintext = [0_u8; 16];
    let mut ciphertext = [0_u8; 16];
    let mut tag = [0_u8; 16];
    assert!(seal(
        &key,
        &nonce,
        &[],
        &plaintext,
        &mut ciphertext,
        &mut tag
    ));
    assert_eq!(
        ciphertext,
        hex_to_array::<16>("0388dace60b6a392f328c2b971b2fe78")
    );
    assert_eq!(tag, hex_to_array::<16>("ab6e47d42cec13bdf53a67b21257bddf"));
    let mut recovered = [0_u8; 16];
    assert!(open(&key, &nonce, &[], &ciphertext, &tag, &mut recovered));
    assert_eq!(recovered, plaintext);
}

#[test]
fn aes_gcm_nist_with_aad() {
    let key = hex_to_array::<16>("feffe9928665731c6d6a8f9467308308");
    let nonce = hex_to_array::<12>("cafebabefacedbaddecaf888");
    let plaintext = hex_to_bytes(
        "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39",
    );
    let aad = hex_to_bytes("feedfacedeadbeeffeedfacedeadbeefabaddad2");
    let expected_ciphertext = hex_to_bytes(
        "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091",
    );
    let mut ciphertext = vec![0_u8; plaintext.len()];
    let mut tag = [0_u8; 16];
    assert!(seal(
        &key,
        &nonce,
        &aad,
        &plaintext,
        &mut ciphertext,
        &mut tag
    ));
    assert_eq!(ciphertext, expected_ciphertext);
    assert_eq!(tag, hex_to_array::<16>("5bc94fbc3221a5db94fae95ae7121a47"));
    let mut recovered = vec![0_u8; plaintext.len()];
    assert!(open(&key, &nonce, &aad, &ciphertext, &tag, &mut recovered));
    assert_eq!(recovered, plaintext);
}

#[test]
fn aes_gcm_rejects_tampering() {
    let key = [7_u8; 32];
    let nonce = [9_u8; 12];
    let plaintext = b"authenticated data";
    let mut ciphertext = [0_u8; 18];
    let mut tag = [0_u8; 16];
    assert!(seal(
        &key,
        &nonce,
        b"aad",
        plaintext,
        &mut ciphertext,
        &mut tag
    ));
    tag[0] ^= 1;
    let mut recovered = [0_u8; 18];
    assert!(!open(
        &key,
        &nonce,
        b"aad",
        &ciphertext,
        &tag,
        &mut recovered
    ));
    assert_eq!(recovered, [0_u8; 18]);
}

const RSA_PUBLIC_DER: &str = "30820122300D06092A864886F70D01010105000382010F003082010A0282010100D8CA8590CBFE1BA8930F8AD9E5C1308BE66C11FC210462FB3668B91D34B5D3CD12C6311279039F984FFBBC6ECD27FBB7F830D316E6D3B4773E656444D6D85D7983C1858F086924EEB1B33115DD00FF16C5241CB4085FBE1FE2C6767171DDC91928EC6A502C453C24A2DA6BEC22AB35BEB0A9AB14730F68CE97E613B26A3313AB078D757A362D8A29D71872264A1E9D791C08CD80A02AB78C1AD7CC76687C3831EC2CBA0C04318A4C540BE6C85FCC96559B859EDD1B420678811F6EF2C2F7983C2085F6D44FCE388813364869FFF11F5BFAEBE41930BF36D4FC0C5C86B412E82135D912FA05C3542DBD0C1A6AD8F28C7C4EB22AADEFB4E0AB1CF3FBB87B08A2730203010001";
const RSA_SIGNATURE: &str = "D5119EB9B6B79AE9069A50F37EFB068966B46D2FF345AB192EAC71568989D08D863205ACAAF929045DE8DA84AA856966A798626A07CCE927802FCCECCDB41E5F1D70F7F84F77F23F878B16BE8C46EF684EA59D59C9388195E637664CFCD2B728D4072420E4D3DDA62A3B66BC27B54C166466F0A349A14567733F70688ABBD7D9F8F78D0B072932543FCDD97C99212C8CC37FD618AF88854E277938D409AD3DB5C62FA6E119E6D1C3A86BDBD8FB5C638786D33760E7DCAC705B03674F26441D446B2F08FF17A23B7616752D1EC4E2620AE1CF5A03E61580988074A583B65EE73463B96E1CFEACC88BF83D869DA1162856CF7C639B428506681A62137D6E358C96";

#[test]
fn rsa_pkcs1_v15_sha256_verifies_openssl_vector() {
    let der = hex_to_bytes(RSA_PUBLIC_DER);
    let Ok(key) = parse_rsa_public_key(&der) else {
        panic!("public key did not parse")
    };
    assert_eq!(key.modulus.len(), 256);
    assert_eq!(key.exponent, [0x01, 0x00, 0x01]);
    let signature = hex_to_bytes(RSA_SIGNATURE);
    assert!(verify_sha256(
        &key,
        b"Rover RSA PKCS#1 v1.5 known-answer vector",
        &signature
    ));
    assert!(!verify_sha256(&key, b"tampered message", &signature));
}

#[test]
fn rsa_rejects_invalid_signature_length_and_padding() {
    let der = hex_to_bytes(RSA_PUBLIC_DER);
    let Ok(key) = parse_rsa_public_key(&der) else {
        panic!("public key did not parse")
    };
    let signature = hex_to_bytes(RSA_SIGNATURE);
    assert!(!verify_sha256(&key, b"message", &signature[..255]));
    let mut altered = signature.clone();
    altered[0] ^= 1;
    assert!(!verify_sha256(
        &key,
        b"Rover RSA PKCS#1 v1.5 known-answer vector",
        &altered
    ));
}

#[test]
fn rsa_rejects_unsupported_and_truncated_keys() {
    assert!(parse_rsa_public_key(&[]).is_err());
    assert!(parse_rsa_public_key(&[0x30, 0x00]).is_err());
    let ec_key = [
        0x30, 0x13, 0x30, 0x07, 0x06, 0x05, 0x2b, 0x65, 0x6e, 0x03, 0x04, 0x02, 0x01, 0x03, 0x08,
        0x00, 0x03, 0x04, 0x00,
    ];
    assert!(parse_rsa_public_key(&ec_key).is_err());
}
