//! RSDP tests.

use rover_uefi::protocols::acpi::checksum::acpi_checksum_ok;
use rover_uefi::protocols::acpi::rsdp::{RSDP_SIGNATURE, parse_rsdp};

fn build_rsdp1() -> [u8; 20] {
    let mut buf = [0_u8; 20];
    buf[0..8].copy_from_slice(&RSDP_SIGNATURE);
    buf[9..15].copy_from_slice(b"OEMID ");
    buf[15] = 1;
    buf[16..20].copy_from_slice(&0x1234_0000_u32.to_le_bytes());
    let sum = buf.iter().map(|b| u32::from(*b)).sum::<u32>();
    buf[8] = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    assert!(acpi_checksum_ok(&buf));
    buf
}

#[test]
fn rsdp1_valid() {
    let buf = build_rsdp1();
    let Some(info) = parse_rsdp(&buf) else {
        panic!("expected rsdp");
    };
    assert_eq!(info.revision, 1);
    assert_eq!(info.rsdt_address, 0x1234_0000);
}

#[test]
fn rsdp_invalid() {
    let mut buf = build_rsdp1();
    buf[0] = b'X';
    assert!(parse_rsdp(&buf).is_none());
    let truncated = [0_u8; 10];
    assert!(parse_rsdp(&truncated).is_none());
}
