//! Integration tests for Universal Payload (UPL) structures and parser.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rover_uefi::protocols::firmware::universal_payload::*;

#[test]
fn test_universal_payload_guids_and_constants() {
    assert_eq!(UNIVERSAL_PAYLOAD_IDENTIFIER, 0x4844_4C50);
    assert_eq!(UNIVERSAL_PAYLOAD_INFO_SEC_NAME, ".upld_info");
    assert_eq!(UNIVERSAL_PAYLOAD_EXTRA_SEC_NAME_PREFIX, ".upld.");

    assert_eq!(
        UNIVERSAL_PAYLOAD_BASE_GUID.to_string(),
        "03d4c61d-2713-4ec5-a1cc-883be9dc18e5"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_GUID.to_string(),
        "aa7e190d-be21-4409-8e67-a2cd0f61e170"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_PCI_ROOT_BRIDGE_INFO_GUID.to_string(),
        "ec4ebacb-2638-416e-be80-e5fa4b511901"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_ACPI_TABLE_GUID.to_string(),
        "9f9a9506-5597-4515-bab6-8bcde784ba87"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_SMBIOS_TABLE_GUID.to_string(),
        "590a0d26-06e5-4d20-8a82-59ea1b34982d"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_SMBIOS3_TABLE_GUID.to_string(),
        "92b7896c-3362-46ce-99b3-4f5e3c34eb42"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_EXTRA_DATA_GUID.to_string(),
        "15a5baf6-1c91-467d-9dfb-319d178d4bb4"
    );
    assert_eq!(
        UNIVERSAL_PAYLOAD_DEVICE_TREE_GUID.to_string(),
        "6784b889-b13c-4c3b-ae4b-0f0a2e320ea3"
    );
}

#[test]
fn test_universal_payload_parse_info_header() {
    let mut buffer = [0_u8; 64];
    // Identifier ('PLDH')
    buffer[0..4].copy_from_slice(&UNIVERSAL_PAYLOAD_IDENTIFIER.to_le_bytes());
    // HeaderLength (56)
    buffer[4..8].copy_from_slice(&56_u32.to_le_bytes());
    // SpecRevision (0x0075 for v0.75)
    buffer[8..10].copy_from_slice(&0x0075_u16.to_le_bytes());
    // Revision (1)
    buffer[12..16].copy_from_slice(&1_u32.to_le_bytes());
    // Attribute (0)
    buffer[16..20].copy_from_slice(&0_u32.to_le_bytes());
    // Capability (1)
    buffer[20..24].copy_from_slice(&1_u32.to_le_bytes());
    // Producer ID
    buffer[24..40].copy_from_slice(b"ROVER-BOOTLOADER");
    // Image ID
    buffer[40..56].copy_from_slice(b"ROVER-UEFI-IMAGE");

    let info = parse_info_header(&buffer).expect("valid info header should parse");
    let id = info.identifier;
    let hdr_len = info.header_length;
    let spec = info.spec_revision;
    let rev = info.revision;
    let cap = info.capability;
    let prod = info.producer_id;
    let img = info.image_id;
    assert_eq!(id, UNIVERSAL_PAYLOAD_IDENTIFIER);
    assert_eq!(hdr_len, 56);
    assert_eq!(spec, 0x0075);
    assert_eq!(rev, 1);
    assert_eq!(cap, 1);
    assert_eq!(&prod, b"ROVER-BOOTLOADER");
    assert_eq!(&img, b"ROVER-UEFI-IMAGE");
}

#[test]
fn test_universal_payload_parse_serial_port_info() {
    let mut buffer = [0_u8; 32];
    // Revision (1)
    buffer[0] = UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_REVISION;
    // Reserved
    buffer[1] = 0;
    // Length (18)
    buffer[2..4].copy_from_slice(&18_u16.to_le_bytes());
    // UseMmio (true)
    buffer[4] = 1;
    // RegisterStride (4)
    buffer[5] = 4;
    // BaudRate (115200)
    buffer[6..10].copy_from_slice(&115_200_u32.to_le_bytes());
    // RegisterBase (0xFED0_0000)
    buffer[10..18].copy_from_slice(&0xFED0_0000_u64.to_le_bytes());

    let serial = parse_serial_port_info(&buffer).expect("valid serial port info should parse");
    let s_rev = serial.header.revision;
    let s_mmio = serial.use_mmio;
    let s_stride = serial.register_stride;
    let s_baud = serial.baud_rate;
    let s_base = serial.register_base.to_u64();
    assert_eq!(s_rev, 1);
    assert!(s_mmio);
    assert_eq!(s_stride, 4);
    assert_eq!(s_baud, 115_200);
    assert_eq!(s_base, 0xFED0_0000);
}

#[test]
fn test_universal_payload_parse_acpi_and_smbios() {
    let mut acpi_buf = [0_u8; 16];
    acpi_buf[0] = UNIVERSAL_PAYLOAD_ACPI_TABLE_REVISION;
    acpi_buf[2..4].copy_from_slice(&12_u16.to_le_bytes());
    acpi_buf[4..12].copy_from_slice(&0x7FFE_0000_u64.to_le_bytes());

    let acpi = parse_acpi_table(&acpi_buf).expect("valid ACPI table should parse");
    let rsdp = acpi.rsdp.to_u64();
    assert_eq!(rsdp, 0x7FFE_0000);

    let mut smbios_buf = [0_u8; 16];
    smbios_buf[0] = UNIVERSAL_PAYLOAD_SMBIOS_TABLE_REVISION;
    smbios_buf[2..4].copy_from_slice(&12_u16.to_le_bytes());
    smbios_buf[4..12].copy_from_slice(&0x7FFD_0000_u64.to_le_bytes());

    let smbios = parse_smbios_table(&smbios_buf).expect("valid SMBIOS table should parse");
    let smb = smbios.smbios_entry_point.to_u64();
    assert_eq!(smb, 0x7FFD_0000);
}
