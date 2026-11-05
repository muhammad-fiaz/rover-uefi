// SPDX-License-Identifier: Apache-2.0
//! Unit tests for FFS section parsing.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::firmware::sections::{
    EFI_SECTION_DXE_DEPEX, EFI_SECTION_PE32, EFI_SECTION_RAW, EFI_STANDARD_COMPRESSION,
    FfsSectionIter, parse_compression_header, parse_guid_defined_header, parse_section_header,
};

#[test]
fn section_header_parsing() {
    // 4-byte standard section header: size = 16 (0x000010), type = EFI_SECTION_PE32 (0x10)
    let buf = [0x10, 0x00, 0x00, EFI_SECTION_PE32, 0xAA, 0xBB, 0xCC, 0xDD];
    let header = parse_section_header(&buf);
    let Some(h) = header else {
        panic!("header parse failed");
    };
    assert_eq!(h.total_size, 16);
    assert_eq!(h.header_size, 4);
    assert_eq!(h.section_type, EFI_SECTION_PE32);

    // Extended section header: size_24 = 0xFFFFFF, ext_size = 24
    let ext_buf = [0xFF, 0xFF, 0xFF, EFI_SECTION_RAW, 0x18, 0x00, 0x00, 0x00];
    let ext_header = parse_section_header(&ext_buf);
    let Some(eh) = ext_header else {
        panic!("ext header parse failed");
    };
    assert_eq!(eh.total_size, 24);
    assert_eq!(eh.header_size, 8);
    assert_eq!(eh.section_type, EFI_SECTION_RAW);
}

#[test]
fn section_iteration_and_payload() {
    // Two sections: First is 8 bytes total (4 header + 4 payload), second is 12 bytes total
    let data = [
        0x08,
        0x00,
        0x00,
        EFI_SECTION_PE32,
        0x11,
        0x22,
        0x33,
        0x44,
        0x0C,
        0x00,
        0x00,
        EFI_SECTION_DXE_DEPEX,
        0x55,
        0x66,
        0x77,
        0x88,
        0x99,
        0xAA,
        0xBB,
        0xCC,
    ];

    let mut iter = FfsSectionIter::new(&data);
    let Some(s1) = iter.next() else {
        panic!("s1 failed");
    };
    assert_eq!(s1.header.section_type, EFI_SECTION_PE32);
    assert_eq!(s1.payload, &[0x11, 0x22, 0x33, 0x44]);

    let Some(s2) = iter.next() else {
        panic!("s2 failed");
    };
    assert_eq!(s2.header.section_type, EFI_SECTION_DXE_DEPEX);
    assert_eq!(s2.payload.len(), 8);

    assert!(iter.next().is_none());
}

#[test]
fn compression_header_parsing() {
    let payload = [0x00, 0x10, 0x00, 0x00, EFI_STANDARD_COMPRESSION, 0xDE, 0xAD];
    let parsed = parse_compression_header(&payload);
    let Some((h, data)) = parsed else {
        panic!("compression parse failed");
    };
    assert_eq!(h.uncompressed_length, 0x1000);
    assert_eq!(h.compression_type, EFI_STANDARD_COMPRESSION);
    assert_eq!(data, &[0xDE, 0xAD]);
}

#[test]
fn guid_defined_header_parsing() {
    let guid = EfiGuid::new(0x1234_5678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let mut payload = Vec::new();
    payload.extend_from_slice(&guid.to_bytes());
    payload.extend_from_slice(&24_u16.to_le_bytes()); // Data offset = 24
    payload.extend_from_slice(&1_u16.to_le_bytes()); // Attributes = 1
    payload.extend_from_slice(b"INNER_PAYLOAD_HERE");

    let parsed = parse_guid_defined_header(&payload);
    let Some((h, inner)) = parsed else {
        panic!("guid defined parse failed");
    };
    assert_eq!(h.section_definition_guid, guid);
    assert_eq!(h.data_offset, 24);
    assert_eq!(h.attributes, 1);
    assert_eq!(inner, b"INNER_PAYLOAD_HERE");
}
