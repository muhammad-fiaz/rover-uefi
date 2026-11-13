//! Integration and unit tests for ACPI enterprise & topology tables.
#![allow(clippy::expect_used)]

use rover_uefi::protocols::acpi::bert::{error_severity, parse_bert, parse_generic_error_status};
use rover_uefi::protocols::acpi::cedt::{cedt_iter, cedt_type, parse_cedt_header, parse_chbs};
use rover_uefi::protocols::acpi::dbg2::{dbg2_iter, parse_dbg2_header, port_subtype, port_type};
use rover_uefi::protocols::acpi::hest::{error_source_type, hest_iter, parse_hest_header};
use rover_uefi::protocols::acpi::iort::{iort_iter, iort_node_type, parse_iort_header};
use rover_uefi::protocols::acpi::nfit::{nfit_iter, nfit_type, parse_nfit_header, parse_spa_range};
use rover_uefi::protocols::acpi::pptt::{
    parse_pptt_header, parse_processor_node, pptt_iter, pptt_type, processor_flags,
};
use rover_uefi::protocols::acpi::prmt::{parse_prmt_header, prm_module_iter};
use rover_uefi::protocols::acpi::tpm2::{parse_tpm2, platform_class, start_method};
use rover_uefi::protocols::acpi::types::{
    SIG_BERT, SIG_CEDT, SIG_DBG2, SIG_HEST, SIG_IORT, SIG_NFIT, SIG_PPTT, SIG_PRMT, SIG_TPM2,
};

fn finalize_acpi_table(buf: &mut [u8], sig: [u8; 4], len: u32, rev: u8) {
    buf[0..4].copy_from_slice(&sig);
    buf[4..8].copy_from_slice(&len.to_le_bytes());
    buf[8] = rev;
    buf[9] = 0;
    buf[10..16].copy_from_slice(b"ROVER ");
    buf[16..24].copy_from_slice(b"TESTTABL");
    let mut sum = 0_u8;
    for &b in &buf[..len as usize] {
        sum = sum.wrapping_add(b);
    }
    buf[9] = (0_u8).wrapping_sub(sum);
}

#[test]
fn test_dbg2_parsing() {
    let mut buf = [0_u8; 96];
    buf[36..40].copy_from_slice(&44_u32.to_le_bytes()); // devices offset
    buf[40..44].copy_from_slice(&1_u32.to_le_bytes()); // 1 device

    // Device at offset 44 (length 52)
    buf[44] = 0; // revision
    buf[45..47].copy_from_slice(&52_u16.to_le_bytes()); // length
    buf[47] = 1; // reg count
    buf[48..50].copy_from_slice(&10_u16.to_le_bytes()); // name len
    buf[50..52].copy_from_slice(&22_u16.to_le_bytes()); // name offset
    buf[56..58].copy_from_slice(&port_type::SERIAL.to_le_bytes());
    buf[58..60].copy_from_slice(&port_subtype::SERIAL_16550.to_le_bytes());
    buf[66..76].copy_from_slice(b"\\_SB.COM1\0");

    finalize_acpi_table(&mut buf, SIG_DBG2, 96, 0);

    let parsed_hdr = parse_dbg2_header(&buf).expect("dbg2 header should parse");
    assert_eq!(parsed_hdr.devices_count, 1);
    assert_eq!(parsed_hdr.devices_offset, 44);

    let mut iter = dbg2_iter(&buf).expect("iter should create");
    let dev = iter.next().expect("first device should exist");
    assert_eq!(dev.port_type, port_type::SERIAL);
    assert_eq!(dev.port_subtype, port_subtype::SERIAL_16550);
    assert_eq!(dev.namespace_string, Some("\\_SB.COM1"));
    assert!(iter.next().is_none());
}

#[test]
fn test_tpm2_parsing() {
    let mut buf = [0_u8; 76];
    buf[36..38].copy_from_slice(&platform_class::CLIENT.to_le_bytes());
    buf[40..48].copy_from_slice(&0xFED4_0000_u64.to_le_bytes()); // control area
    buf[48..52].copy_from_slice(&start_method::CRB.to_le_bytes());
    buf[64..68].copy_from_slice(&0x10000_u32.to_le_bytes()); // log len
    buf[68..76].copy_from_slice(&0x7F00_0000_u64.to_le_bytes()); // log addr

    finalize_acpi_table(&mut buf, SIG_TPM2, 76, 4);

    let info = parse_tpm2(&buf).expect("tpm2 should parse");
    assert_eq!(info.platform_class, platform_class::CLIENT);
    assert_eq!(info.control_area_address, 0xFED4_0000);
    assert_eq!(info.start_method, start_method::CRB);
    assert_eq!(info.log_area_minimum_length, Some(0x10000));
    assert_eq!(info.log_area_start_address, Some(0x7F00_0000));
}

#[test]
fn test_prmt_parsing() {
    let mut buf = [0_u8; 96];
    buf[36..44].copy_from_slice(&0x8000_0000_u64.to_le_bytes()); // param addr
    buf[44..48].copy_from_slice(&1_u32.to_le_bytes()); // module count
    buf[48..52].copy_from_slice(&52_u32.to_le_bytes()); // module offset

    // Module at 52 (length 44)
    buf[52..54].copy_from_slice(&44_u16.to_le_bytes());
    buf[54] = 0;
    buf[56..72].copy_from_slice(&[0xAA; 16]); // guid
    buf[72..74].copy_from_slice(&1_u16.to_le_bytes()); // major
    buf[74..76].copy_from_slice(&2_u16.to_le_bytes()); // minor
    buf[76..78].copy_from_slice(&2_u16.to_le_bytes()); // handler count

    finalize_acpi_table(&mut buf, SIG_PRMT, 96, 0);

    let parsed_hdr = parse_prmt_header(&buf).expect("prmt header");
    assert_eq!(parsed_hdr.module_count, 1);
    assert_eq!(parsed_hdr.platform_param_address, 0x8000_0000);

    let mut iter = prm_module_iter(&buf).expect("module iter");
    let module = iter.next().expect("module 1");
    assert_eq!(module.module_guid, [0xAA; 16]);
    assert_eq!(module.major_version, 1);
    assert_eq!(module.minor_version, 2);
    assert_eq!(module.handler_count, 2);
    assert!(iter.next().is_none());
}

#[test]
fn test_bert_and_gesb_parsing() {
    let mut buf = [0_u8; 48];
    buf[36..40].copy_from_slice(&4096_u32.to_le_bytes()); // region len
    buf[40..48].copy_from_slice(&0x9000_0000_u64.to_le_bytes()); // region addr

    finalize_acpi_table(&mut buf, SIG_BERT, 48, 1);

    let bert = parse_bert(&buf).expect("bert should parse");
    assert_eq!(bert.region_length, 4096);
    assert_eq!(bert.region_address, 0x9000_0000);

    let mut gesb_buf = [0_u8; 20];
    gesb_buf[0..4].copy_from_slice(&1_u32.to_le_bytes()); // block status: uncorrectable
    gesb_buf[16..20].copy_from_slice(&error_severity::FATAL.to_le_bytes());

    let gesb = parse_generic_error_status(&gesb_buf).expect("gesb parse");
    assert_eq!(gesb.block_status, 1);
    assert_eq!(gesb.error_severity, error_severity::FATAL);
}

#[test]
fn test_hest_parsing() {
    let mut buf = [0_u8; 60];
    buf[36..40].copy_from_slice(&1_u32.to_le_bytes()); // 1 error source

    // Source at 40: IA32 NMI (type 2, length 20)
    buf[40..42].copy_from_slice(&error_source_type::IA32_NMI.to_le_bytes());
    buf[42..44].copy_from_slice(&7_u16.to_le_bytes()); // source id 7

    finalize_acpi_table(&mut buf, SIG_HEST, 60, 1);

    let parsed_hdr = parse_hest_header(&buf).expect("hest header");
    assert_eq!(parsed_hdr.error_source_count, 1);

    let mut iter = hest_iter(&buf).expect("hest iter");
    let entry = iter.next().expect("entry 1");
    assert_eq!(entry.source_type, error_source_type::IA32_NMI);
    assert_eq!(entry.source_id, 7);
    assert_eq!(entry.raw.len(), 20);
    assert!(iter.next().is_none());
}

#[test]
fn test_nfit_parsing() {
    let mut buf = [0_u8; 96];

    // Substructure at 40: SPA Range (type 0, length 56)
    buf[40..42].copy_from_slice(&nfit_type::SPA_RANGE.to_le_bytes());
    buf[42..44].copy_from_slice(&56_u16.to_le_bytes());
    buf[44..46].copy_from_slice(&1_u16.to_le_bytes()); // range index 1
    buf[46..48].copy_from_slice(&0_u16.to_le_bytes()); // flags
    buf[52..56].copy_from_slice(&3_u32.to_le_bytes()); // proximity domain
    buf[56..72].copy_from_slice(&[0xBB; 16]); // guid
    buf[72..80].copy_from_slice(&0x1000_0000_u64.to_le_bytes()); // spa base
    buf[80..88].copy_from_slice(&0x4000_0000_u64.to_le_bytes()); // spa length

    finalize_acpi_table(&mut buf, SIG_NFIT, 96, 1);

    let parsed_hdr = parse_nfit_header(&buf).expect("nfit header");
    assert_eq!(parsed_hdr.length, 96);

    let mut iter = nfit_iter(&buf).expect("nfit iter");
    let entry = iter.next().expect("entry 1");
    assert_eq!(entry.structure_type, nfit_type::SPA_RANGE);
    assert_eq!(entry.length, 56);

    let spa = parse_spa_range(entry.raw).expect("spa range parse");
    assert_eq!(spa.range_index, 1);
    assert_eq!(spa.proximity_domain, 3);
    assert_eq!(spa.spa_base, 0x1000_0000);
    assert_eq!(spa.spa_length, 0x4000_0000);
}

#[test]
fn test_pptt_parsing() {
    let mut buf = [0_u8; 60];

    // Processor hierarchy at 36 (type 0, length 24)
    buf[36] = pptt_type::PROCESSOR;
    buf[37] = 24;
    buf[40..44].copy_from_slice(&processor_flags::PHYSICAL_PACKAGE.to_le_bytes());
    buf[48..52].copy_from_slice(&0_u32.to_le_bytes()); // acpi proc id 0

    finalize_acpi_table(&mut buf, SIG_PPTT, 60, 1);

    let parsed_hdr = parse_pptt_header(&buf).expect("pptt header");
    assert_eq!(parsed_hdr.length, 60);

    let mut iter = pptt_iter(&buf).expect("pptt iter");
    let entry = iter.next().expect("entry 1");
    assert_eq!(entry.structure_type, pptt_type::PROCESSOR);

    let proc = parse_processor_node(entry.raw).expect("proc node parse");
    assert_eq!(
        proc.flags & processor_flags::PHYSICAL_PACKAGE,
        processor_flags::PHYSICAL_PACKAGE
    );
}

#[test]
fn test_cedt_parsing() {
    let mut buf = [0_u8; 60];

    // CHBS at 36 (type 0, length 24)
    buf[36] = cedt_type::CHBS;
    buf[38..40].copy_from_slice(&24_u16.to_le_bytes());
    buf[40..44].copy_from_slice(&5_u32.to_le_bytes()); // uid 5
    buf[44..48].copy_from_slice(&2_u32.to_le_bytes()); // cxl ver 2
    buf[52..60].copy_from_slice(&0xE000_0000_u64.to_le_bytes()); // reg base

    finalize_acpi_table(&mut buf, SIG_CEDT, 60, 1);

    let parsed_hdr = parse_cedt_header(&buf).expect("cedt header");
    assert_eq!(parsed_hdr.length, 60);

    let mut iter = cedt_iter(&buf).expect("cedt iter");
    let entry = iter.next().expect("entry 1");
    assert_eq!(entry.structure_type, cedt_type::CHBS);

    let chbs = parse_chbs(entry.raw).expect("chbs parse");
    assert_eq!(chbs.uid, 5);
    assert_eq!(chbs.cxl_version, 2);
    assert_eq!(chbs.component_reg_base, 0xE000_0000);
}

#[test]
fn test_iort_parsing() {
    let mut buf = [0_u8; 64];
    buf[36..40].copy_from_slice(&1_u32.to_le_bytes()); // 1 node
    buf[40..44].copy_from_slice(&48_u32.to_le_bytes()); // node offset 48

    // Node at 48 (type 0: ITS Group, length 16)
    buf[48] = iort_node_type::ITS_GROUP;
    buf[49..51].copy_from_slice(&16_u16.to_le_bytes());
    buf[51] = 0; // rev
    buf[52..56].copy_from_slice(&42_u32.to_le_bytes()); // id

    finalize_acpi_table(&mut buf, SIG_IORT, 64, 0);

    let parsed_hdr = parse_iort_header(&buf).expect("iort header");
    assert_eq!(parsed_hdr.node_count, 1);
    assert_eq!(parsed_hdr.node_offset, 48);

    let mut iter = iort_iter(&buf).expect("iort iter");
    let node = iter.next().expect("node 1");
    assert_eq!(node.node_type, iort_node_type::ITS_GROUP);
    assert_eq!(node.identifier, 42);
    assert!(iter.next().is_none());
}
