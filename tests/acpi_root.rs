//! ACPI root tests.

use rover_uefi::protocols::acpi::root::{rsdt_iter, validate_root_table, xsdt_iter};

#[test]
fn root_validation() {
    let mut buf = [0_u8; 44];
    buf[0..4].copy_from_slice(b"RSDT");
    buf[4..8].copy_from_slice(&44_u32.to_le_bytes());
    buf[36..40].copy_from_slice(&0x1000_u32.to_le_bytes());
    buf[40..44].copy_from_slice(&0x2000_u32.to_le_bytes());
    let sum = buf.iter().map(|b| u32::from(*b)).sum::<u32>();
    buf[9] = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    assert!(validate_root_table(&buf, false));
    let Some(mut iter) = rsdt_iter(&buf) else {
        panic!("expected iterator");
    };
    assert_eq!(iter.next(), Some(0x1000));
    assert_eq!(iter.next(), Some(0x2000));
    assert!(iter.next().is_none());
    assert!(xsdt_iter(&buf).is_none());
}
