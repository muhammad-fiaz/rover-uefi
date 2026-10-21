//! Tests for the native `TLS` record layer, handshake codec, key
//! schedule and record protection.

use rover_uefi::protocols::network::tls::record::TLS_RECORD_HEADER_LEN;
use rover_uefi::protocols::network::tls::{
    self, TlsError, handshake, key_schedule, protection, record,
};
use rover_uefi::protocols::security::sha256::Sha256;
use rover_uefi::protocols::security::sha512::Sha384;
use rover_uefi::protocols::security::{sha256, sha512};

fn hex(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0_u8; 32];
    assert!(bytes.len() <= output.len());
    output[..bytes.len()].copy_from_slice(bytes);
    output
}

#[test]
fn record_roundtrip_and_walk() {
    let mut buffer = [0_u8; 128];
    let Ok(first) = record::build_record(&mut buffer[..16], 22, 0x0303, &[1, 2, 3]) else {
        panic!("record did not build");
    };
    assert_eq!(first, TLS_RECORD_HEADER_LEN + 3);
    let Ok(second) =
        record::build_record(&mut buffer[first..first + 32], 23, 0x0303, &[9, 9, 9, 9])
    else {
        panic!("second record did not build");
    };
    assert_eq!(second, TLS_RECORD_HEADER_LEN + 4);

    let Ok((parsed, consumed)) = record::parse_record(&buffer[..first]) else {
        panic!("record did not parse");
    };
    assert_eq!(parsed.content_type, 22);
    assert_eq!(parsed.version, 0x0303);
    assert_eq!(parsed.payload, [1, 2, 3]);
    assert!(parsed.is_well_formed());
    assert_eq!(consumed, first);

    let Ok((parsed, consumed)) = record::parse_record(&buffer[first..first + second]) else {
        panic!("second record did not parse");
    };
    assert_eq!(parsed.content_type, 23);
    assert_eq!(parsed.payload, [9, 9, 9, 9]);
    assert_eq!(consumed, second);
}

#[test]
fn record_rejects_truncated_and_oversized() {
    let short = [23_u8, 0x03, 0x03, 0, 9, 1, 2];
    assert_eq!(
        record::parse_record(&short).err(),
        Some(TlsError::Truncated)
    );

    let mut oversized = [0_u8; 8];
    oversized[0] = 23;
    oversized[1] = 0x03;
    oversized[2] = 0x03;
    oversized[3] = 0xFF;
    oversized[4] = 0xFF;
    assert_eq!(
        record::parse_record(&oversized).err(),
        Some(TlsError::Malformed)
    );

    let mut small = [0_u8; 3];
    assert_eq!(
        record::build_record(&mut small, 23, 0x0303, &[1, 2, 3]),
        Err(TlsError::BufferTooSmall)
    );
}

#[test]
fn handshake_messages_iterate() {
    let mut buffer = [0_u8; 128];
    let Ok(first) = handshake::encode(&mut buffer[..16], handshake::HS_CLIENT_HELLO, &[1, 2])
    else {
        panic!("handshake message did not encode");
    };
    let Ok(second) = handshake::encode(
        &mut buffer[first..first + 32],
        handshake::HS_FINISHED,
        &[7; 8],
    ) else {
        panic!("second handshake message did not encode");
    };
    assert_eq!(first, 4 + 2);
    assert_eq!(second, 4 + 8);

    let Ok(messages) = tls::parse_messages(&buffer[..first + second]) else {
        panic!("handshake messages did not parse");
    };
    let collected: [u8; 2] = {
        let mut kinds = [0_u8; 2];
        let mut index = 0;
        for message in messages {
            kinds[index] = message.kind;
            index += 1;
        }
        assert_eq!(index, 2);
        kinds
    };
    assert_eq!(
        collected,
        [handshake::HS_CLIENT_HELLO, handshake::HS_FINISHED]
    );

    // A body cut short is rejected instead of being silently dropped.
    assert_eq!(
        tls::parse_messages(&buffer[..first + second - 1]).err(),
        Some(TlsError::Truncated)
    );
}

#[test]
fn extensions_roundtrip() {
    let mut buffer = [0_u8; 128];
    let Ok(first) = handshake::write_extension(
        &mut buffer[..16],
        handshake::EXT_SUPPORTED_VERSIONS,
        &[4, 3, 4, 3, 3],
    ) else {
        panic!("extension did not write");
    };
    let Ok(second) = handshake::write_extension(
        &mut buffer[first..first + 32],
        handshake::EXT_KEY_SHARE,
        &[0, 6, 0, 29, 0, 2, 0xAA, 0xBB],
    ) else {
        panic!("second extension did not write");
    };
    let Ok(extensions) = handshake::parse_extensions(&buffer[..first + second]) else {
        panic!("extensions did not parse");
    };
    let found: [u16; 2] = {
        let mut kinds = [0_u16; 2];
        let mut index = 0;
        for extension in extensions {
            kinds[index] = extension.kind;
            index += 1;
        }
        assert_eq!(index, 2);
        kinds
    };
    assert_eq!(
        found,
        [handshake::EXT_SUPPORTED_VERSIONS, handshake::EXT_KEY_SHARE]
    );

    // A truncated extension block is rejected.
    assert_eq!(
        handshake::parse_extensions(&buffer[..first + second - 1]).err(),
        Some(TlsError::Truncated)
    );
}

#[test]
fn client_hello_roundtrip() {
    let mut extension_block = [0_u8; 64];
    let Ok(versions) = handshake::write_extension(
        &mut extension_block[..16],
        handshake::EXT_SUPPORTED_VERSIONS,
        &[4, 3, 4, 3, 3],
    ) else {
        panic!("supported versions did not write");
    };
    let shares = [0, 6, 0, 29, 0, 2, 0xAA, 0xBB];
    let Ok(key_share) = handshake::write_extension(
        &mut extension_block[versions..versions + 16],
        handshake::EXT_KEY_SHARE,
        &shares,
    ) else {
        panic!("key share did not write");
    };

    let suites = [0x13, 0x01, 0x13, 0x02];
    let params = handshake::ClientHelloParams {
        legacy_version: handshake::TLS_VERSION_1_2,
        random: [0x11; 32],
        session_id: &[1, 2, 3, 4],
        cipher_suites: &suites,
        compression_methods: &[0],
        extensions: &extension_block[..versions + key_share],
    };
    let mut buffer = [0_u8; 256];
    let Ok(len) = tls::build_client_hello(&mut buffer, &params) else {
        panic!("client hello did not build");
    };
    let Ok(mut messages) = tls::parse_messages(&buffer[..len]) else {
        panic!("client hello message did not parse");
    };
    let Some(message) = messages.next() else {
        panic!("client hello missing");
    };
    assert_eq!(message.kind, handshake::HS_CLIENT_HELLO);

    let Ok(hello) = handshake::ClientHello::parse(message.body) else {
        panic!("client hello body did not parse");
    };
    assert_eq!(hello.legacy_version, handshake::TLS_VERSION_1_2);
    assert_eq!(hello.random, [0x11; 32]);
    assert_eq!(hello.session_id, [1, 2, 3, 4]);
    assert_eq!(hello.cipher_suites, suites);
    assert_eq!(hello.compression_methods, [0]);

    let Some(version_extension) = hello.find_extension(handshake::EXT_SUPPORTED_VERSIONS) else {
        panic!("supported versions extension missing");
    };
    let Ok(versions) = version_extension.supported_versions() else {
        panic!("supported versions did not decode");
    };
    assert_eq!(versions.count(), 2);
    let Ok(version_list) = version_extension.supported_versions() else {
        panic!("supported versions did not decode");
    };
    let advertised: [u16; 2] = {
        let mut list = [0_u16; 2];
        let mut index = 0;
        for version in version_list {
            list[index] = version;
            index += 1;
        }
        assert_eq!(index, 2);
        list
    };
    assert_eq!(
        advertised,
        [handshake::TLS_VERSION_1_3, handshake::TLS_VERSION_1_2]
    );

    let Some(share_extension) = hello.find_extension(handshake::EXT_KEY_SHARE) else {
        panic!("key share extension missing");
    };
    let Ok(mut shares) = share_extension.key_shares() else {
        panic!("key shares did not decode");
    };
    let Some(entry) = shares.next() else {
        panic!("key share entry missing");
    };
    assert_eq!(entry.group, handshake::GROUP_X25519);
    assert_eq!(entry.key_exchange, [0xAA, 0xBB]);
    assert!(shares.next().is_none());
}

#[test]
fn server_hello_parses() {
    let mut body = [0_u8; 64];
    let mut length = 0_usize;
    let mut push = |bytes: &[u8]| {
        body[length..length + bytes.len()].copy_from_slice(bytes);
        length += bytes.len();
    };
    push(&[0x03, 0x03]);
    push(&[0x22; 32]);
    push(&[0x00]);
    push(&[0x13, 0x01]);
    push(&[0x00]);
    push(&[0x00, 0x06]);
    push(&[0x00, 0x2B, 0x00, 0x02, 0x03, 0x04]);

    let Ok(hello) = handshake::ServerHello::parse(&body[..length]) else {
        panic!("server hello did not parse");
    };
    assert_eq!(hello.cipher_suite, handshake::CIPHER_TLS_AES_128_GCM_SHA256);
    assert_eq!(hello.compression_method, 0);
    assert_eq!(hello.session_id, b"");
    let Some(extension) = hello.find_extension(handshake::EXT_SUPPORTED_VERSIONS) else {
        panic!("supported versions extension missing");
    };
    assert_eq!(extension.data, [0x03, 0x04]);
}

#[test]
fn key_schedule_sha256_vectors() {
    let mut secret = [0_u8; 32];
    for (index, byte) in secret.iter_mut().enumerate() {
        let Ok(value) = u8::try_from(index) else {
            panic!("index does not fit");
        };
        *byte = value;
    }
    let transcript = sha256::sha256(b"messages");

    let mut key = [0_u8; 16];
    assert!(key_schedule::expand_label::<Sha256>(
        &secret,
        b"key",
        &[],
        &mut key
    ));
    assert_eq!(
        key,
        [
            0x9C, 0x97, 0x83, 0xCF, 0x77, 0xEA, 0x32, 0xD4, 0x4F, 0x36, 0x9D, 0xA4, 0x1F, 0x19,
            0xF3, 0xCC
        ]
    );

    let mut iv = [0_u8; 12];
    assert!(key_schedule::expand_label::<Sha256>(
        &secret,
        b"iv",
        &[],
        &mut iv
    ));
    assert_eq!(
        iv,
        [
            0x2F, 0x41, 0xC8, 0x46, 0xA4, 0x31, 0xA1, 0x63, 0x81, 0x4B, 0xCD, 0x71
        ]
    );

    let mut derived = [0_u8; 32];
    assert!(key_schedule::derive_secret::<Sha256>(
        &secret,
        b"c hs traffic",
        &transcript,
        &mut derived
    ));
    assert_eq!(
        derived,
        [
            0x24, 0x31, 0x6B, 0x5D, 0x36, 0xD9, 0x5F, 0xC4, 0x46, 0xDD, 0x86, 0x50, 0x49, 0x77,
            0x42, 0xE3, 0xD7, 0x0B, 0xA5, 0x97, 0x6F, 0xBE, 0x63, 0x9F, 0x02, 0x06, 0x71, 0x0E,
            0x4B, 0x36, 0x31, 0x87
        ]
    );

    let mut traffic_key = [0_u8; 32];
    assert!(key_schedule::traffic_key::<Sha256>(
        &derived,
        &mut traffic_key
    ));
    assert_eq!(
        traffic_key,
        [
            0xA8, 0x8E, 0xE8, 0x19, 0xA0, 0x1B, 0x3C, 0x82, 0x92, 0x77, 0x73, 0xCE, 0x48, 0x7B,
            0x90, 0xF1, 0x15, 0x14, 0xEC, 0x4C, 0x95, 0x1A, 0x12, 0xE1, 0x1C, 0xDB, 0x91, 0x21,
            0xD5, 0x6B, 0x1E, 0x46
        ]
    );

    let mut traffic_iv = [0_u8; 12];
    assert!(key_schedule::traffic_iv::<Sha256>(
        &derived,
        &mut traffic_iv
    ));
    assert_eq!(
        traffic_iv,
        [
            0x6A, 0x13, 0x30, 0xCF, 0xBC, 0x8E, 0x71, 0x54, 0xA7, 0xD1, 0x96, 0xFD
        ]
    );

    let mut finished = [0_u8; 32];
    assert!(key_schedule::finished_key::<Sha256>(&secret, &mut finished));
    assert_eq!(
        finished,
        [
            0x38, 0xBF, 0xB0, 0xA8, 0x34, 0xFC, 0x61, 0x26, 0x5A, 0xCC, 0x27, 0x84, 0x46, 0xDA,
            0x8B, 0x66, 0xDB, 0x08, 0x5D, 0xBF, 0x77, 0xC7, 0x52, 0x10, 0xA5, 0x3D, 0xEB, 0x87,
            0xA4, 0xCC, 0x7D, 0x0E
        ]
    );
}

#[test]
fn key_schedule_sha384_vectors() {
    let mut secret = [0_u8; 48];
    for (index, byte) in secret.iter_mut().enumerate() {
        let Ok(value) = u8::try_from(index) else {
            panic!("index does not fit");
        };
        *byte = value;
    }
    let transcript = sha512::sha384(b"messages");

    let mut derived = [0_u8; 48];
    assert!(key_schedule::derive_secret::<Sha384>(
        &secret,
        b"s hs traffic",
        &transcript,
        &mut derived
    ));
    assert_eq!(
        derived,
        [
            0xDA, 0xCD, 0xE8, 0xDF, 0xE6, 0x38, 0xE2, 0x6D, 0xC8, 0x73, 0x36, 0x01, 0x02, 0x2A,
            0x4B, 0x19, 0x10, 0x11, 0x13, 0x2D, 0xF1, 0xBA, 0xA3, 0xE8, 0x51, 0xB1, 0x8D, 0xF2,
            0x30, 0x62, 0x4B, 0xB6, 0x97, 0x33, 0x1B, 0xD7, 0x21, 0xA0, 0x0C, 0x08, 0x67, 0xC5,
            0x0A, 0x9D, 0xFA, 0x32, 0x99, 0xFA
        ]
    );

    let mut key = [0_u8; 32];
    assert!(key_schedule::traffic_key::<Sha384>(&derived, &mut key));
    assert_eq!(
        key,
        [
            0x90, 0x3E, 0x01, 0xE3, 0xF5, 0x18, 0x02, 0x39, 0xAF, 0xB1, 0x6E, 0x4D, 0x9A, 0x1A,
            0xE8, 0x78, 0x39, 0x6A, 0xB0, 0x26, 0xEA, 0xCF, 0x1F, 0xF5, 0x63, 0xCA, 0x0A, 0x2E,
            0xE4, 0xB2, 0xB0, 0xD0
        ]
    );

    let mut iv = [0_u8; 12];
    assert!(key_schedule::traffic_iv::<Sha384>(&derived, &mut iv));
    assert_eq!(
        iv,
        [
            0xFC, 0x9E, 0x25, 0x62, 0xA0, 0x29, 0x72, 0x59, 0x60, 0xA6, 0xD4, 0x4B
        ]
    );

    // An output longer than the label can encode is rejected.
    let mut oversized = [0_u8; 64];
    assert!(!key_schedule::expand_label::<Sha384>(
        &secret,
        &[0_u8; 250],
        &[],
        &mut oversized
    ));
}

#[test]
fn record_protection_roundtrip() {
    let key = hex(&[
        0xA8, 0x8E, 0xE8, 0x19, 0xA0, 0x1B, 0x3C, 0x82, 0x92, 0x77, 0x73, 0xCE, 0x48, 0x7B, 0x90,
        0xF1,
    ]);
    let iv = [
        0x6A, 0x13, 0x30, 0xCF, 0xBC, 0x8E, 0x71, 0x54, 0xA7, 0xD1, 0x96, 0xFD,
    ];
    let Some(keys) = protection::RecordKeys::new(&key[..16], &iv) else {
        panic!("record keys were rejected");
    };
    assert!(keys.is_supported());

    let plaintext = b"handshake transcript bytes";
    let mut protected = [0_u8; 128];
    let Ok(len) = protection::seal_record(
        &keys,
        0,
        record::CONTENT_TYPE_HANDSHAKE,
        plaintext,
        &mut protected,
    ) else {
        panic!("record did not seal");
    };
    assert_eq!(len, plaintext.len() + protection::TLS13_OVERHEAD);

    let mut opened = [0_u8; 128];
    let Ok((opened_len, content_type)) =
        protection::open_record(&keys, 0, &protected[..len], &mut opened)
    else {
        panic!("record did not open");
    };
    assert_eq!(content_type, record::CONTENT_TYPE_HANDSHAKE);
    assert_eq!(&opened[..opened_len], plaintext);
}

#[test]
fn record_protection_rejects_tampering_and_wrong_sequence() {
    let key = [0x11_u8; 16];
    let iv = [0x22_u8; 12];
    let Some(keys) = protection::RecordKeys::new(&key, &iv) else {
        panic!("record keys were rejected");
    };
    let mut protected = [0_u8; 64];
    let Ok(len) = protection::seal_record(
        &keys,
        7,
        record::CONTENT_TYPE_APPLICATION_DATA,
        b"secret",
        &mut protected,
    ) else {
        panic!("record did not seal");
    };

    let mut opened = [0_u8; 64];
    assert_eq!(
        protection::open_record(&keys, 8, &protected[..len], &mut opened),
        Err(TlsError::DecryptionFailed)
    );

    protected[0] ^= 0x01;
    assert_eq!(
        protection::open_record(&keys, 7, &protected[..len], &mut opened),
        Err(TlsError::DecryptionFailed)
    );

    // Short records and bad content types are rejected.
    let mut short = [0_u8; 16];
    assert_eq!(
        protection::open_record(&keys, 7, &protected[..8], &mut short),
        Err(TlsError::Truncated)
    );
    assert_eq!(
        protection::seal_record(&keys, 7, 0, b"x", &mut protected),
        Err(TlsError::Malformed)
    );
}

#[test]
fn record_protection_empty_payload_and_small_buffer() {
    let key = [0x33_u8; 32];
    let iv = [0x44_u8; 12];
    let Some(keys) = protection::RecordKeys::new(&key, &iv) else {
        panic!("record keys were rejected");
    };
    let mut protected = [0_u8; 32];
    let Ok(len) =
        protection::seal_record(&keys, 0, record::CONTENT_TYPE_ALERT, &[], &mut protected)
    else {
        panic!("empty record did not seal");
    };
    assert_eq!(len, protection::TLS13_OVERHEAD);

    let mut opened = [0_u8; 32];
    let Ok((opened_len, content_type)) =
        protection::open_record(&keys, 0, &protected[..len], &mut opened)
    else {
        panic!("empty record did not open");
    };
    assert_eq!(opened_len, 0);
    assert_eq!(content_type, record::CONTENT_TYPE_ALERT);

    let mut small = [0_u8; 8];
    assert_eq!(
        protection::seal_record(&keys, 0, record::CONTENT_TYPE_ALERT, b"payload", &mut small),
        Err(TlsError::BufferTooSmall)
    );
    assert!(protection::RecordKeys::new(&[0_u8; 8], &iv).is_none());
    assert!(protection::RecordKeys::new(&key, &[0_u8; 8]).is_none());
}
