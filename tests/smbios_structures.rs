//! SMBIOS structure tests.

use rover_uefi::protocols::smbios::iterator::smbios_iter;
use rover_uefi::protocols::smbios::strings::string_at;

#[test]
fn structure_iteration() {
    let mut buf = [0_u8; 12];
    buf[0] = 1;
    buf[1] = 4;
    buf[2..4].copy_from_slice(&0x0001_u16.to_le_bytes());
    buf[4] = b'A';
    buf[5] = 0;
    buf[6] = b'B';
    buf[7] = 0;
    buf[8] = 0;
    buf[9] = 0;
    buf[10] = 127;
    buf[11] = 4;
    let mut iter = smbios_iter(&buf);
    let Some(first) = iter.next() else {
        panic!("expected structure");
    };
    assert_eq!(first.structure_type(), 1);
    assert_eq!(string_at(first.strings(), 1), Some(&b"A"[..]));
    assert_eq!(string_at(first.strings(), 0), None);
}
