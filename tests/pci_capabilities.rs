//! PCI capability tests.

use rover_uefi::protocols::pci::capabilities::{MAX_CAPABILITIES, capability_at, capability_iter};

#[test]
fn capability_read() {
    let mut buf = [0_u8; 256];
    buf[0x40] = 0x01;
    buf[0x41] = 0x50;
    buf[0x50] = 0x11;
    buf[0x51] = 0x00;
    let Some(cap) = capability_at(&buf, 0x40) else {
        panic!("expected capability");
    };
    assert_eq!(cap.id, 0x01);
    let Some(mut iter) = capability_iter(&buf, 0x40) else {
        panic!("expected iterator");
    };
    let first = iter.next();
    assert_eq!(first.map(|cap| cap.id), Some(0x01));
    let second = iter.next();
    assert_eq!(second.map(|cap| cap.id), Some(0x11));
    assert!(iter.next().is_none());
    assert!(capability_iter(&buf, 0).is_none());
    assert_eq!(MAX_CAPABILITIES, 32);
}

#[test]
fn cycle_detection() {
    let mut buf = [0_u8; 256];
    buf[0x40] = 0x01;
    buf[0x41] = 0x40;
    let Some(mut iter) = capability_iter(&buf, 0x40) else {
        panic!("expected iterator");
    };
    let mut count = 0_usize;
    for item in iter.by_ref() {
        let _ = item;
        count += 1;
        if count > 40 {
            break;
        }
    }
    assert!(count <= MAX_CAPABILITIES);
}
