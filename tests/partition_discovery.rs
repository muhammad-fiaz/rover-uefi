//! Discovery tests.

use rover_uefi::protocols::storage::partition::discovery::detect_scheme;
use rover_uefi::protocols::storage::partition::mbr::sector::parse_mbr_sector;
use rover_uefi::protocols::storage::partition::types::PartitionScheme;

#[test]
fn scheme_detection() {
    let mut buf = [0_u8; 512];
    buf[446 + 4] = 0x83;
    buf[510] = 0x55;
    buf[511] = 0xAA;
    let Ok(sector) = parse_mbr_sector(&buf) else {
        panic!("expected sector");
    };
    assert_eq!(detect_scheme(&sector, None), PartitionScheme::Mbr);
    let mut protective = [0_u8; 512];
    protective[446 + 4] = 0xEE;
    protective[510] = 0x55;
    protective[511] = 0xAA;
    let Ok(inner) = parse_mbr_sector(&protective) else {
        panic!("expected sector");
    };
    assert_eq!(detect_scheme(&inner, None), PartitionScheme::ProtectiveMbr);
}
