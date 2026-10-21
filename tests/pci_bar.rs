//! PCI BAR tests.

use rover_uefi::protocols::pci::bar::{parse_bar32, parse_bar64};

#[test]
fn memory_bar() {
    let mut buf = [0_u8; 64];
    buf[0x10..0x14].copy_from_slice(&0xF000_0000_u32.to_le_bytes());
    let Some(bar) = parse_bar32(&buf, 0) else {
        panic!("expected bar");
    };
    assert!(!bar.is_io);
    assert_eq!(bar.address, 0xF000_0000);
    assert!(parse_bar32(&buf, 6).is_none());
}

#[test]
fn io_bar() {
    let mut buf = [0_u8; 64];
    buf[0x10..0x14].copy_from_slice(&0x0C01_u32.to_le_bytes());
    let Some(bar) = parse_bar32(&buf, 0) else {
        panic!("expected bar");
    };
    assert!(bar.is_io);
}

#[test]
fn sixty_four_bit_bar() {
    let mut buf = [0_u8; 64];
    buf[0x10..0x14].copy_from_slice(&0x0000_000C_u32.to_le_bytes());
    buf[0x14..0x18].copy_from_slice(&0x0000_0001_u32.to_le_bytes());
    let Some(bar) = parse_bar64(&buf, 0) else {
        panic!("expected bar");
    };
    assert_eq!(bar.address, 0x1_0000_0000);
    assert!(!bar.is_io);
}
