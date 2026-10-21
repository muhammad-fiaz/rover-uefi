//! Authenticated variable tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::secure_boot::authentication::{
    EFI_CERT_TYPE_RSA2048_SHA256_GUID, EfiVariableAuthentication, EfiVariableAuthentication2,
    WIN_CERT_REVISION, WIN_CERT_TYPE_EFI_GUID, WIN_CERT_TYPE_PKCS_SIGNED_DATA,
    authentication_2_size, authentication_size, parse_authentication_2, parse_win_certificate,
};

fn build_cert(certificate_type: u16, total: u32) -> Vec<u8> {
    let mut buf = vec![0_u8; total as usize];
    buf[0..4].copy_from_slice(&total.to_le_bytes());
    buf[4..6].copy_from_slice(&WIN_CERT_REVISION.to_le_bytes());
    buf[6..8].copy_from_slice(&certificate_type.to_le_bytes());
    if buf.len() >= 24 {
        buf[8..24].copy_from_slice(&EFI_CERT_TYPE_RSA2048_SHA256_GUID.to_bytes());
    }
    buf
}

#[test]
fn descriptor_layouts() {
    assert_eq!(size_of::<EfiVariableAuthentication>(), 32);
    assert_eq!(offset_of!(EfiVariableAuthentication, monotonic_count), 0);
    assert_eq!(offset_of!(EfiVariableAuthentication, auth_info), 8);
    assert_eq!(offset_of!(EfiVariableAuthentication, cert_type), 16);
    assert_eq!(size_of::<EfiVariableAuthentication2>(), 40);
    assert_eq!(authentication_size(), 32);
    assert_eq!(authentication_2_size(), 40);
}

#[test]
fn parses_win_certificate() {
    let buf = build_cert(WIN_CERT_TYPE_EFI_GUID, 40);
    let Some(cert) = parse_win_certificate(&buf) else {
        panic!("expected certificate");
    };
    assert_eq!(cert.length, 40);
    assert_eq!(cert.revision, WIN_CERT_REVISION);
    assert_eq!(cert.certificate_type, WIN_CERT_TYPE_EFI_GUID);
}

#[test]
fn rejects_malformed_certificates() {
    assert!(parse_win_certificate(&[]).is_none());
    assert!(parse_win_certificate(&[0_u8; 4]).is_none());
    let mut bad_rev = build_cert(WIN_CERT_TYPE_EFI_GUID, 16);
    bad_rev[4] = 0;
    bad_rev[5] = 0;
    assert!(parse_win_certificate(&bad_rev).is_none());
    let mut small = build_cert(WIN_CERT_TYPE_EFI_GUID, 8);
    small[0..4].copy_from_slice(&4_u32.to_le_bytes());
    assert!(parse_win_certificate(&small).is_none());
    let mut large = build_cert(WIN_CERT_TYPE_EFI_GUID, 16);
    large[0..4].copy_from_slice(&1000_u32.to_le_bytes());
    assert!(parse_win_certificate(&large).is_none());
    let pkcs = build_cert(WIN_CERT_TYPE_PKCS_SIGNED_DATA, 16);
    let Some(cert) = parse_win_certificate(&pkcs) else {
        panic!("expected pkcs cert");
    };
    assert_eq!(cert.certificate_type, WIN_CERT_TYPE_PKCS_SIGNED_DATA);
}

#[test]
fn parses_time_based_descriptor() {
    let mut buf = vec![0_u8; 40];
    buf[0..2].copy_from_slice(&2026_u16.to_le_bytes());
    buf[2] = 11;
    buf[3] = 7;
    buf[16..20].copy_from_slice(&24_u32.to_le_bytes());
    buf[20..22].copy_from_slice(&WIN_CERT_REVISION.to_le_bytes());
    buf[22..24].copy_from_slice(&WIN_CERT_TYPE_EFI_GUID.to_le_bytes());
    buf[24..40].copy_from_slice(&EFI_CERT_TYPE_RSA2048_SHA256_GUID.to_bytes());
    let Some(parsed) = parse_authentication_2(&buf) else {
        panic!("expected descriptor");
    };
    assert_eq!(parsed.timestamp.year, 2026);
    assert_eq!(parsed.timestamp.month, 11);
    assert_eq!(parsed.cert_type, EFI_CERT_TYPE_RSA2048_SHA256_GUID);
    assert_eq!(parsed.auth_info.length, 24);
}

#[test]
fn rejects_truncated_time_based_descriptor() {
    assert!(parse_authentication_2(&[]).is_none());
    assert!(parse_authentication_2(&[0_u8; 20]).is_none());
    assert_eq!(
        EFI_CERT_TYPE_RSA2048_SHA256_GUID.to_bytes(),
        [
            0x14, 0x74, 0x71, 0xA7, 0x16, 0xC6, 0x77, 0x49, 0x94, 0x20, 0x84, 0x47, 0x12, 0xA7,
            0x35, 0xBF
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(EFI_CERT_TYPE_RSA2048_SHA256_GUID.to_bytes()),
        EFI_CERT_TYPE_RSA2048_SHA256_GUID
    );
}
