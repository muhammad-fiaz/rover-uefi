//! Tests for X.509 DER parsing.

use rover_uefi::protocols::security::x509::{
    DerError, decode_oid, oid_eq, parse_certificate, parse_tlv, parse_validity_time,
};

#[test]
fn tlv_short_form() {
    let Ok(tlv) = parse_tlv(&[0x30, 0x03, 0x02, 0x01, 0x05]) else {
        panic!("expected tlv");
    };
    assert_eq!(tlv.tag(), 0x30);
    assert_eq!(tlv.value(), &[0x02, 0x01, 0x05]);
}

#[test]
fn tlv_long_form_and_rejections() {
    let mut buf = vec![0x30, 0x81, 0x80];
    buf.extend_from_slice(&[0_u8; 128]);
    let Ok(tlv) = parse_tlv(&buf) else {
        panic!("expected long tlv");
    };
    assert_eq!(tlv.value().len(), 128);
    assert_eq!(parse_tlv(&[0x30, 0x80]), Err(DerError::InvalidTag));
    assert_eq!(
        parse_tlv(&[0x30, 0x81, 0x7F]),
        Err(DerError::NonMinimalLength)
    );
    assert_eq!(parse_tlv(&[0x30]), Err(DerError::Truncated));
    assert_eq!(parse_tlv(&[]), Err(DerError::Truncated));
}

#[test]
fn oid_decoding() {
    let Ok(oid) = decode_oid(&[0x2A, 0x03]) else {
        panic!("expected oid");
    };
    assert_eq!(oid.len(), 3);
    assert_eq!(oid.get(0), Some(1));
    assert_eq!(oid.get(1), Some(2));
    assert_eq!(oid.get(2), Some(3));
    assert!(oid_eq(&oid, &[1, 2, 3]));
    assert!(!oid_eq(&oid, &[1, 2, 4]));
    assert_eq!(decode_oid(&[]), Err(DerError::InvalidOid));
    // Non-minimal leading group 0x80 is rejected.
    assert_eq!(decode_oid(&[0x2A, 0x80, 0x01]), Err(DerError::InvalidOid));
    // Truncated arc (continuation without final byte).
    assert_eq!(decode_oid(&[0x2A, 0x81]), Err(DerError::InvalidOid));
}

#[test]
fn validity_times() {
    assert_eq!(parse_validity_time(b"240101000000Z"), Ok(1_704_067_200));
    // UTCTime year 00 maps to 2000.
    assert_eq!(parse_validity_time(b"000101000000Z"), Ok(946_684_800));
    assert!(parse_validity_time(b"241301000000Z").is_err());
    assert!(parse_validity_time(b"240101000000").is_err());
    assert!(parse_validity_time(b"241231235959+0000").is_err());
}

#[test]
fn certificate_parse() {
    // Minimal certificate with OID 1.2.3 (content 2A 03).
    let mut tbs_inner = Vec::new();
    tbs_inner.extend_from_slice(&[0xA0, 0x03, 0x02, 0x01, 0x02]);
    tbs_inner.extend_from_slice(&[0x02, 0x03, 0x00, 0x12, 0x34]);
    tbs_inner.extend_from_slice(&[0x30, 0x04, 0x06, 0x02, 0x2A, 0x03]);
    tbs_inner.extend_from_slice(&[0x30, 0x00]);
    let mut validity = Vec::new();
    validity.extend_from_slice(&[0x17, 0x0D]);
    validity.extend_from_slice(b"240101000000Z");
    validity.extend_from_slice(&[0x17, 0x0D]);
    validity.extend_from_slice(b"250101000000Z");
    let validity_len = u8::try_from(validity.len()).unwrap_or(0);
    let mut validity_seq = vec![0x30, validity_len];
    validity_seq.extend_from_slice(&validity);
    tbs_inner.extend_from_slice(&validity_seq);
    tbs_inner.extend_from_slice(&[0x30, 0x00]);
    let tbs_len = u8::try_from(tbs_inner.len()).unwrap_or(0);
    let mut tbs = vec![0x30, tbs_len];
    tbs.extend_from_slice(&tbs_inner);
    let alg = [0x30, 0x04, 0x06, 0x02, 0x2A, 0x03];
    let sig = [0x03, 0x02, 0x00, 0xFF];
    let mut outer_content = Vec::new();
    outer_content.extend_from_slice(&tbs);
    outer_content.extend_from_slice(&alg);
    outer_content.extend_from_slice(&sig);
    let outer_len = u8::try_from(outer_content.len()).unwrap_or(0);
    let mut outer = vec![0x30, outer_len];
    outer.extend_from_slice(&outer_content);
    let Ok(cert) = parse_certificate(&outer) else {
        panic!("expected certificate");
    };
    assert_eq!(cert.version, Some(2));
    assert_eq!(cert.serial, &[0x00, 0x12, 0x34]);
    assert!(oid_eq(&cert.signature_algorithm, &[1, 2, 3]));
    assert_eq!(cert.signature, &[0xFF]);
    assert_eq!(cert.not_before, 1_704_067_200);
}

#[test]
fn certificate_rejections() {
    assert!(parse_certificate(&[]).is_err());
    assert!(parse_certificate(&[0x30, 0x00]).is_err());
    assert!(parse_certificate(&[0x31, 0x00]).is_err());
}
