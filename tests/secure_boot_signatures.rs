//! Signature tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::secure_boot::signature::{
    EFI_CERT_SHA1_GUID, EFI_CERT_SHA256_GUID, EFI_CERT_X509_GUID, EfiSignatureList,
    expected_signature_size, signature_data_iter, signature_list_iter,
};
use rover_uefi::protocols::security::secure_boot::validation::{
    signature_count, validate_signature_list_size, validate_signature_size,
};

fn build_list(signature_type: EfiGuid, signature_size: u32, entries: &[u8]) -> Vec<u8> {
    let total = 28_u32 + u32::try_from(entries.len()).unwrap_or(u32::MAX);
    let mut buf = Vec::new();
    buf.extend_from_slice(&signature_type.to_bytes());
    buf.extend_from_slice(&total.to_le_bytes());
    buf.extend_from_slice(&0_u32.to_le_bytes());
    buf.extend_from_slice(&signature_size.to_le_bytes());
    buf.extend_from_slice(entries);
    buf
}

#[test]
fn signature_list_layout() {
    assert_eq!(size_of::<EfiSignatureList>(), 28);
    assert_eq!(offset_of!(EfiSignatureList, signature_type), 0);
    assert_eq!(offset_of!(EfiSignatureList, signature_list_size), 16);
    assert_eq!(offset_of!(EfiSignatureList, signature_header_size), 20);
    assert_eq!(offset_of!(EfiSignatureList, signature_size), 24);
}

#[test]
fn cert_guid_bytes() {
    assert_eq!(
        EFI_CERT_SHA256_GUID.to_bytes(),
        [
            0x26, 0x16, 0xC4, 0xC1, 0x4C, 0x50, 0x92, 0x40, 0xAC, 0xA9, 0x41, 0xF9, 0x36, 0x93,
            0x43, 0x28
        ]
    );
    assert_eq!(
        EFI_CERT_X509_GUID.to_bytes(),
        [
            0xA1, 0x59, 0xC0, 0xA5, 0xE4, 0x94, 0xA7, 0x4A, 0x87, 0xB5, 0xAB, 0x15, 0x5C, 0x2B,
            0xF0, 0x72
        ]
    );
    assert_eq!(
        EFI_CERT_SHA1_GUID.to_bytes(),
        [
            0x12, 0xA5, 0x6C, 0x82, 0x10, 0xCF, 0xC9, 0x4A, 0xB1, 0x87, 0xBE, 0x01, 0x49, 0x66,
            0x31, 0xBD
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(EFI_CERT_X509_GUID.to_bytes()),
        EFI_CERT_X509_GUID
    );
}

#[test]
fn parses_single_list() {
    let owner = EfiGuid::new(1, 2, 3, [4; 8]);
    let mut entry = Vec::new();
    entry.extend_from_slice(&owner.to_bytes());
    entry.extend_from_slice(&[0x99_u8; 32]);
    let buf = build_list(EFI_CERT_SHA256_GUID, 48, &entry);
    let mut iter = signature_list_iter(&buf);
    let Some(info) = iter.next() else {
        panic!("expected list");
    };
    assert_eq!(info.signature_type, EFI_CERT_SHA256_GUID);
    assert_eq!(info.signature_list_size, 28 + 48);
    assert_eq!(info.signatures.len(), 48);
    assert!(iter.next().is_none());
    let Some(mut iterator) = signature_data_iter(info.signatures, info.signature_size) else {
        panic!("expected data iterator");
    };
    let Some((parsed_owner, data)) = iterator.next() else {
        panic!("expected entry");
    };
    assert_eq!(parsed_owner, EfiGuid::new(1, 2, 3, [4; 8]));
    assert_eq!(data.len(), 32);
    assert_eq!(data[0], 0x99);
    assert!(iterator.next().is_none());
}

#[test]
fn parses_two_lists_with_unknown_type() {
    let entry_a = [0x01_u8; 20];
    let buf_a = build_list(EFI_CERT_SHA1_GUID, 20, &entry_a);
    let unknown = EfiGuid::new(9, 9, 9, [9; 8]);
    let entry_b = [0x02_u8; 10];
    let buf_b = build_list(unknown, 10, &entry_b);
    let mut combined = buf_a;
    combined.extend_from_slice(&buf_b);
    let mut iter = signature_list_iter(&combined);
    let Some(first) = iter.next() else {
        panic!("expected first");
    };
    assert_eq!(first.signature_type, EFI_CERT_SHA1_GUID);
    let Some(second) = iter.next() else {
        panic!("expected second");
    };
    assert_eq!(second.signature_type, unknown);
    assert_eq!(second.signatures, entry_b);
    assert!(iter.next().is_none());
}

#[test]
fn rejects_malformed_lists() {
    assert!(signature_list_iter(&[]).next().is_none());
    assert!(signature_list_iter(&[0_u8; 10]).next().is_none());
    let buf = build_list(EFI_CERT_SHA256_GUID, 16, &[0_u8; 4]);
    let mut truncated = buf.clone();
    truncated.truncate(20);
    assert!(signature_list_iter(&truncated).next().is_none());
    let mut small = buf;
    small[16..20].copy_from_slice(&10_u32.to_le_bytes());
    assert!(signature_list_iter(&small).next().is_none());
    assert!(signature_data_iter(&[0_u8; 4], 8).is_none());
    assert!(signature_data_iter(&[0_u8; 4], 0).is_none());
}

#[test]
fn validation_and_expected_sizes() {
    assert!(validate_signature_list_size(28));
    assert!(!validate_signature_list_size(27));
    assert!(validate_signature_size(16));
    assert!(!validate_signature_size(15));
    assert_eq!(signature_count(28 + 96, 0, 48), Some(2));
    assert_eq!(signature_count(10, 0, 48), None);
    assert_eq!(expected_signature_size(&EFI_CERT_SHA256_GUID), Some(48));
    assert_eq!(expected_signature_size(&EFI_CERT_SHA1_GUID), Some(36));
    assert_eq!(expected_signature_size(&EfiGuid::ZERO), None);
    assert_eq!(expected_signature_size(&EFI_CERT_X509_GUID), None);
}
