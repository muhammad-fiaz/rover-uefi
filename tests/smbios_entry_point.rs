//! SMBIOS entry point tests.

use rover_uefi::protocols::smbios::entry_point::{ANCHOR_2X, parse_2x};

#[test]
fn entry_point_2x() {
    let mut buf = [0_u8; 31];
    buf[0..4].copy_from_slice(&ANCHOR_2X);
    buf[6] = 31;
    buf[7] = 2;
    buf[22..24].copy_from_slice(&100_u16.to_le_bytes());
    let sum = buf.iter().map(|b| u32::from(*b)).sum::<u32>();
    buf[4] = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    let Some(entry) = parse_2x(&buf) else {
        panic!("expected entry point");
    };
    assert_eq!(entry.major_version, 2);
}

#[test]
fn entry_point_invalid() {
    assert!(parse_2x(&[0_u8; 10]).is_none());
}
