//! ACPI header tests.

use rover_uefi::protocols::acpi::header::{parse_header, signature_matches};

#[test]
fn header_valid() {
    let mut buf = [0_u8; 36];
    buf[0..4].copy_from_slice(b"FACP");
    buf[4..8].copy_from_slice(&36_u32.to_le_bytes());
    buf[8] = 6;
    let sum = buf.iter().map(|b| u32::from(*b)).sum::<u32>();
    buf[9] = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    let Some(header) = parse_header(&buf) else {
        panic!("expected header");
    };
    assert!(signature_matches(&header, *b"FACP"));
    assert_eq!(header.length, 36);
}

#[test]
fn header_invalid() {
    assert!(parse_header(&[0_u8; 10]).is_none());
}
