//! SMBIOS record tests.

use rover_uefi::protocols::smbios::records::{
    TYPE_BIOS_INFORMATION, TYPE_END_OF_TABLE, TYPE_PROCESSOR_INFORMATION, is_end_of_table,
};
use rover_uefi::protocols::smbios::strings::string_at;
use rover_uefi::protocols::smbios::structure::parse_header;
use rover_uefi::protocols::smbios::validation::validate_formatted_length;

#[test]
fn record_types() {
    assert_eq!(TYPE_BIOS_INFORMATION, 0);
    assert_eq!(TYPE_PROCESSOR_INFORMATION, 4);
    assert_eq!(TYPE_END_OF_TABLE, 127);
    assert!(is_end_of_table(TYPE_END_OF_TABLE));
    assert!(!is_end_of_table(0));
    assert!(!is_end_of_table(126));
}

#[test]
fn header_parses_and_rejects_short() {
    let mut buf = [0_u8; 4];
    buf[0] = 1;
    buf[1] = 4;
    buf[2..4].copy_from_slice(&0x0102_u16.to_le_bytes());
    let Some(header) = parse_header(&buf) else {
        panic!("expected header");
    };
    assert_eq!(header.structure_type, 1);
    assert_eq!(header.length, 4);
    assert_eq!(header.handle, 0x0102);
    assert!(parse_header(&[0_u8; 3]).is_none());
    assert!(parse_header(&[]).is_none());
}

#[test]
fn strings_and_length_validation() {
    let strings: [u8; 8] = *b"A\0B\0\0\0\0\0";
    assert_eq!(string_at(&strings, 1), Some(&b"A"[..]));
    assert_eq!(string_at(&strings, 2), Some(&b"B"[..]));
    assert_eq!(string_at(&strings, 0), None);
    assert_eq!(string_at(&strings, 3), Some(&b""[..]));
    assert_eq!(string_at(&[], 1), None);
    assert!(validate_formatted_length(3));
    assert!(validate_formatted_length(24));
    assert!(!validate_formatted_length(2));
    assert!(!validate_formatted_length(0));
}
