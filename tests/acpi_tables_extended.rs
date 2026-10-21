//! Extended ACPI table tests.

use rover_uefi::protocols::acpi::bgrt::parse_bgrt;
use rover_uefi::protocols::acpi::dmar::{dmar_iter, parse_dmar_header};
use rover_uefi::protocols::acpi::dsdt::parse_dsdt;
use rover_uefi::protocols::acpi::facs::parse_facs;
use rover_uefi::protocols::acpi::gtdt::parse_gtdt;
use rover_uefi::protocols::acpi::hpet::parse_hpet;
use rover_uefi::protocols::acpi::slit::{parse_slit, slit_iter};
use rover_uefi::protocols::acpi::spcr::parse_spcr;
use rover_uefi::protocols::acpi::srat::{parse_srat_header, srat_iter};
use rover_uefi::protocols::acpi::ssdt::parse_ssdt;

fn fix_checksum_header(buf: &mut [u8]) {
    buf[9] = 0;
    let sum: u32 = buf.iter().map(|b| u32::from(*b)).sum();
    let checksum = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    buf[9] = checksum;
}

fn fix_checksum_facs(buf: &mut [u8]) {
    buf[33] = 0;
    let sum: u32 = buf.iter().map(|b| u32::from(*b)).sum();
    let checksum = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    buf[33] = checksum;
}

fn build_table(signature: [u8; 4], body: &[u8]) -> Vec<u8> {
    let total_len = 36 + body.len();
    let mut buf = vec![0_u8; total_len];
    buf[0..4].copy_from_slice(&signature);
    buf[4..8].copy_from_slice(&u32::try_from(total_len).unwrap_or(0).to_le_bytes());
    buf[8] = 2;
    if !body.is_empty() {
        buf[36..].copy_from_slice(body);
    }
    fix_checksum_header(&mut buf);
    buf
}

fn build_facs(
    hw_sig: u32,
    fw_vector: u32,
    lock: u32,
    flags: u32,
    x_vector: u64,
    version: u8,
    ospm: u32,
) -> Vec<u8> {
    let mut buf = vec![0_u8; 40];
    buf[0..4].copy_from_slice(b"FACS");
    buf[4..8].copy_from_slice(&40_u32.to_le_bytes());
    buf[8..12].copy_from_slice(&hw_sig.to_le_bytes());
    buf[12..16].copy_from_slice(&fw_vector.to_le_bytes());
    buf[16..20].copy_from_slice(&lock.to_le_bytes());
    buf[20..24].copy_from_slice(&flags.to_le_bytes());
    buf[24..32].copy_from_slice(&x_vector.to_le_bytes());
    buf[32] = version;
    buf[36..40].copy_from_slice(&ospm.to_le_bytes());
    fix_checksum_facs(&mut buf);
    buf
}

#[test]
fn facs_valid_parse() {
    let buf = build_facs(
        0x1122_3344,
        0x5566_7788,
        0x99AA_BBCC,
        0xDDEE_FF00,
        0x0102_0304_0506_0708,
        2,
        0xAABB_CCDD,
    );
    let Some(info) = parse_facs(&buf) else {
        panic!("expected facs");
    };
    assert_eq!(info.hardware_signature, 0x1122_3344);
    assert_eq!(info.firmware_waking_vector, 0x5566_7788);
    assert_eq!(info.global_lock, 0x99AA_BBCC);
    assert_eq!(info.flags, 0xDDEE_FF00);
    assert_eq!(info.x_firmware_waking_vector, 0x0102_0304_0506_0708);
    assert_eq!(info.version, 2);
    assert_eq!(info.ospm_flags, 0xAABB_CCDD);
}

#[test]
fn facs_invalid_signature() {
    let mut buf = build_facs(1, 2, 3, 4, 5, 1, 6);
    buf[0..4].copy_from_slice(b"FACX");
    fix_checksum_facs(&mut buf);
    assert!(parse_facs(&buf).is_none());
}

#[test]
fn facs_truncated() {
    assert!(parse_facs(&[]).is_none());
    assert!(parse_facs(&[0_u8; 10]).is_none());
    let buf = build_facs(1, 2, 3, 4, 5, 1, 6);
    assert!(parse_facs(&buf[..20]).is_none());
}

#[test]
fn facs_bad_checksum() {
    let mut buf = build_facs(1, 2, 3, 4, 5, 1, 6);
    buf[12] = buf[12].wrapping_add(1);
    assert!(parse_facs(&buf).is_none());
}

#[test]
fn facs_length_too_small() {
    let mut buf = build_facs(1, 2, 3, 4, 5, 1, 6);
    let long = vec![0_u8; 80];
    assert!(parse_facs(&long).is_none());
    buf[4..8].copy_from_slice(&20_u32.to_le_bytes());
    buf[33] = 0;
    let sum: u32 = buf.iter().map(|b| u32::from(*b)).sum();
    buf[33] = u8::try_from((256 - (sum % 256)) % 256).unwrap_or(0);
    assert!(parse_facs(&buf).is_none());
}

#[test]
fn dsdt_valid_parse() {
    let body = [0x10_u8, 0x41, 0x41, 0x99];
    let buf = build_table(*b"DSDT", &body);
    let Some(view) = parse_dsdt(&buf) else {
        panic!("expected dsdt");
    };
    assert_eq!(view.revision, 2);
    assert_eq!(view.aml_bytes, &body[..]);
}

#[test]
fn dsdt_invalid_signature() {
    let body = [0x10_u8, 0x41];
    let mut buf = build_table(*b"DSDT", &body);
    buf[0..4].copy_from_slice(b"DSDE");
    fix_checksum_header(&mut buf);
    assert!(parse_dsdt(&buf).is_none());
}

#[test]
fn dsdt_truncated() {
    assert!(parse_dsdt(&[]).is_none());
    assert!(parse_dsdt(&[0_u8; 10]).is_none());
    let buf = build_table(*b"DSDT", &[1_u8, 2, 3]);
    assert!(parse_dsdt(&buf[..20]).is_none());
}

#[test]
fn dsdt_bad_checksum() {
    let buf = build_table(*b"DSDT", &[1_u8, 2, 3]);
    let mut bad = buf.clone();
    bad[36] = bad[36].wrapping_add(1);
    assert!(parse_dsdt(&bad).is_none());
}

#[test]
fn dsdt_empty_body() {
    let buf = build_table(*b"DSDT", &[]);
    let Some(view) = parse_dsdt(&buf) else {
        panic!("expected empty dsdt");
    };
    assert_eq!(view.aml_bytes, []);
}

#[test]
fn ssdt_valid_parse() {
    let body = [0xAA_u8, 0xBB, 0xCC];
    let buf = build_table(*b"SSDT", &body);
    let Some(view) = parse_ssdt(&buf) else {
        panic!("expected ssdt");
    };
    assert_eq!(view.revision, 2);
    assert_eq!(view.aml_bytes, &body[..]);
}

#[test]
fn ssdt_invalid_signature() {
    let mut buf = build_table(*b"SSDT", &[1_u8]);
    buf[0..4].copy_from_slice(b"DSDT");
    fix_checksum_header(&mut buf);
    assert!(parse_ssdt(&buf).is_none());
}

#[test]
fn ssdt_truncated_and_checksum() {
    assert!(parse_ssdt(&[]).is_none());
    assert!(parse_ssdt(&[0_u8; 5]).is_none());
    let mut buf = build_table(*b"SSDT", &[1_u8, 2]);
    buf[37] = buf[37].wrapping_add(2);
    assert!(parse_ssdt(&buf).is_none());
}

#[test]
fn hpet_valid_parse() {
    let mut body = vec![0_u8; 18];
    body[0] = 0;
    body[1] = 64;
    body[2] = 0;
    body[3] = 1;
    body[4..12].copy_from_slice(&0xFED0_0000_0000_0001_u64.to_le_bytes());
    body[12..14].copy_from_slice(&0x1234_u16.to_le_bytes());
    body[14..16].copy_from_slice(&0x5678_u16.to_le_bytes());
    body[16..18].copy_from_slice(&0x0009_u16.to_le_bytes());
    let buf = build_table(*b"HPET", &body);
    let Some(info) = parse_hpet(&buf) else {
        panic!("expected hpet");
    };
    assert_eq!(info.base_address, 0xFED0_0000_0000_0001);
    assert_eq!(info.event_timer_block_id, 0x1234);
    assert_eq!(info.main_counter_min, 0x5678);
    assert_eq!(info.sequence, 0x0009);
}

#[test]
fn hpet_invalid_cases() {
    let mut body = vec![0_u8; 18];
    body[4..12].copy_from_slice(&0x1000_u64.to_le_bytes());
    let mut buf = build_table(*b"HPET", &body);
    buf[0..4].copy_from_slice(b"HPFX");
    fix_checksum_header(&mut buf);
    assert!(parse_hpet(&buf).is_none());
    assert!(parse_hpet(&[]).is_none());
    let buf2 = build_table(*b"HPET", &body);
    let mut bad = buf2.clone();
    bad[40] = bad[40].wrapping_add(1);
    assert!(parse_hpet(&bad).is_none());
    let short = build_table(*b"HPET", &[0_u8; 4]);
    assert!(parse_hpet(&short).is_none());
}

#[test]
fn srat_valid_header_and_iter() {
    let mut body = vec![0_u8; 4];
    body[0..4].copy_from_slice(&0xDEAD_BEEF_u32.to_le_bytes());
    body.extend_from_slice(&[0_u8, 4, 0xAA, 0xBB, 1, 5, 0xCC, 0xDD, 0xEE]);
    let buf = build_table(*b"SRAT", &body);
    let Some(header) = parse_srat_header(&buf) else {
        panic!("expected srat header");
    };
    assert_eq!(header.reserved, 0xDEAD_BEEF);
    let Some(mut iter) = srat_iter(&buf) else {
        panic!("expected srat iter");
    };
    let Some(e1) = iter.next() else {
        panic!("expected entry 1");
    };
    assert_eq!(e1.entry_type(), 0);
    assert_eq!(e1.bytes().len(), 4);
    let Some(e2) = iter.next() else {
        panic!("expected entry 2");
    };
    assert_eq!(e2.entry_type(), 1);
    assert_eq!(e2.bytes().len(), 5);
    assert!(iter.next().is_none());
}

#[test]
fn srat_invalid_and_malformed() {
    let mut body = vec![0_u8; 4];
    body.extend_from_slice(&[0_u8, 1, 0xFF]);
    let buf = build_table(*b"SRAT", &body);
    let Some(mut iter) = srat_iter(&buf) else {
        panic!("expected srat iter");
    };
    assert!(iter.next().is_none());
    let mut buf2 = buf.clone();
    buf2[0..4].copy_from_slice(b"SRAX");
    fix_checksum_header(&mut buf2);
    assert!(parse_srat_header(&buf2).is_none());
    assert!(srat_iter(&buf2).is_none());
    assert!(parse_srat_header(&[]).is_none());
    assert!(srat_iter(&[0_u8; 10]).is_none());
    let bad_len = build_table(*b"SRAT", &[0_u8, 1, 0xFF]);
    assert!(srat_iter(&bad_len).is_none());
}

#[test]
fn slit_valid_parse_and_iter() {
    let mut body = vec![0_u8; 8];
    body[0..8].copy_from_slice(&2_u64.to_le_bytes());
    body.extend_from_slice(&[10_u8, 20, 30, 40]);
    let buf = build_table(*b"SLIT", &body);
    let Some(info) = parse_slit(&buf) else {
        panic!("expected slit");
    };
    assert_eq!(info.num_localities, 2);
    let Some(mut iter) = slit_iter(&buf) else {
        panic!("expected slit iter");
    };
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), Some(30));
    assert_eq!(iter.next(), Some(40));
    assert_eq!(iter.next(), None);
}

#[test]
fn slit_invalid_and_truncated() {
    let mut body = vec![0_u8; 8];
    body[0..8].copy_from_slice(&3_u64.to_le_bytes());
    body.extend_from_slice(&[1_u8, 2]);
    let buf = build_table(*b"SLIT", &body);
    assert!(parse_slit(&buf).is_none());
    assert!(slit_iter(&buf).is_none());
    assert!(parse_slit(&[]).is_none());
    let mut buf2 = build_table(*b"SLIT", &[0_u8; 8]);
    buf2[0..4].copy_from_slice(b"SLIX");
    fix_checksum_header(&mut buf2);
    assert!(parse_slit(&buf2).is_none());
    let mut buf3 = build_table(*b"SLIT", &[0_u8; 8]);
    buf3[36] = buf3[36].wrapping_add(1);
    assert!(parse_slit(&buf3).is_none());
}

#[test]
fn dmar_valid_parse_and_iter() {
    let mut body = vec![0_u8; 4];
    body[0] = 48;
    body[1] = 1;
    body.extend_from_slice(&[0, 6, 0, 0xAA, 0xBB, 0xCC, 1, 4, 0, 0xDD]);
    let buf = build_table(*b"DMAR", &body);
    let Some(header) = parse_dmar_header(&buf) else {
        panic!("expected dmar header");
    };
    assert_eq!(header.width, 48);
    assert_eq!(header.flags, 1);
    let Some(mut iter) = dmar_iter(&buf) else {
        panic!("expected dmar iter");
    };
    let Some(e1) = iter.next() else {
        panic!("expected dmar entry1");
    };
    assert_eq!(e1.entry_type(), 0);
    assert_eq!(e1.bytes().len(), 6);
    let Some(e2) = iter.next() else {
        panic!("expected dmar entry2");
    };
    assert_eq!(e2.entry_type(), 1);
    assert_eq!(e2.bytes().len(), 4);
    assert!(iter.next().is_none());
}

#[test]
fn dmar_invalid_cases() {
    let body = [48_u8, 0, 0, 0, 0, 2, 0, 0xAA];
    let buf = build_table(*b"DMAR", &body);
    let Some(mut iter) = dmar_iter(&buf) else {
        panic!("expected iter");
    };
    assert!(iter.next().is_none());
    let mut buf2 = buf.clone();
    buf2[0..4].copy_from_slice(b"DMAD");
    fix_checksum_header(&mut buf2);
    assert!(parse_dmar_header(&buf2).is_none());
    assert!(parse_dmar_header(&[]).is_none());
}

#[test]
fn gtdt_valid_parse() {
    let mut body = vec![0_u8; 26];
    body[0..4].copy_from_slice(&1_u32.to_le_bytes());
    body[4..8].copy_from_slice(&2_u32.to_le_bytes());
    body[8..16].copy_from_slice(&0xF800_0000_u64.to_le_bytes());
    body[16..24].copy_from_slice(&0xF801_0000_u64.to_le_bytes());
    body[24] = 7;
    body[25] = 3;
    let buf = build_table(*b"GTDT", &body);
    let Some(info) = parse_gtdt(&buf) else {
        panic!("expected gtdt");
    };
    assert_eq!(info.counter_block_count, 1);
    assert_eq!(info.compare_timer_count, 2);
    assert_eq!(info.phys_base, 0xF800_0000);
    assert_eq!(info.secure_base, 0xF801_0000);
    assert_eq!(info.ppi, 7);
    assert_eq!(info.flags, 3);
}

#[test]
fn gtdt_invalid_cases() {
    let buf = build_table(*b"GTDT", &[0_u8; 26]);
    let mut bad_sig = buf.clone();
    bad_sig[0..4].copy_from_slice(b"GTDX");
    fix_checksum_header(&mut bad_sig);
    assert!(parse_gtdt(&bad_sig).is_none());
    assert!(parse_gtdt(&[]).is_none());
    let mut bad_sum = buf.clone();
    bad_sum[36] = bad_sum[36].wrapping_add(1);
    assert!(parse_gtdt(&bad_sum).is_none());
}

#[test]
fn spcr_valid_parse() {
    let mut body = vec![0_u8; 22];
    body[0] = 0;
    body[1] = 2;
    body[3] = 5;
    body[4] = 9;
    body[9] = 1;
    body[10..18].copy_from_slice(&0x3F8_u64.to_le_bytes());
    body[18..20].copy_from_slice(&0x0BAD_u16.to_le_bytes());
    body[20] = 1;
    body[21] = 2;
    let buf = build_table(*b"SPCR", &body);
    let Some(info) = parse_spcr(&buf) else {
        panic!("expected spcr");
    };
    assert_eq!(info.interface_type, 0);
    assert_eq!(info.spec_revision, 2);
    assert_eq!(info.interrupt_type, 5);
    assert_eq!(info.interrupt_number, 9);
    assert_eq!(info.port_type, 1);
    assert_eq!(info.port_address, 0x3F8);
    assert_eq!(info.baud_rate, 0x0BAD);
    assert_eq!(info.parity, 1);
    assert_eq!(info.stop_bits, 2);
}

#[test]
fn spcr_invalid_cases() {
    let buf = build_table(*b"SPCR", &[0_u8; 22]);
    let mut bad = buf.clone();
    bad[0..4].copy_from_slice(b"SPCX");
    fix_checksum_header(&mut bad);
    assert!(parse_spcr(&bad).is_none());
    assert!(parse_spcr(&[]).is_none());
    let mut bad2 = buf.clone();
    bad2[36] = bad2[36].wrapping_add(1);
    assert!(parse_spcr(&bad2).is_none());
}

#[test]
fn bgrt_valid_parse() {
    let mut body = vec![0_u8; 16];
    body[0..2].copy_from_slice(&1_u16.to_le_bytes());
    body[2] = 1;
    body[3] = 1;
    body[4..12].copy_from_slice(&0x1234_5678_u64.to_le_bytes());
    body[12..16].copy_from_slice(&0x42_u32.to_le_bytes());
    let buf = build_table(*b"BGRT", &body);
    let Some(info) = parse_bgrt(&buf) else {
        panic!("expected bgrt");
    };
    assert_eq!(info.version, 1);
    assert_eq!(info.status, 1);
    assert_eq!(info.image_type, 1);
    assert_eq!(info.image_address, 0x1234_5678);
    assert_eq!(info.image_offset, 0x42);
}

#[test]
fn bgrt_invalid_cases() {
    let buf = build_table(*b"BGRT", &[0_u8; 16]);
    let mut bad = buf.clone();
    bad[0..4].copy_from_slice(b"BGXX");
    fix_checksum_header(&mut bad);
    assert!(parse_bgrt(&bad).is_none());
    assert!(parse_bgrt(&[]).is_none());
    let mut bad2 = buf.clone();
    bad2[36] = bad2[36].wrapping_add(1);
    assert!(parse_bgrt(&bad2).is_none());
}

#[test]
fn malformed_global_inputs() {
    let garbage = [0xFF_u8; 64];
    assert!(parse_facs(&garbage).is_none());
    assert!(parse_dsdt(&garbage).is_none());
    assert!(parse_ssdt(&garbage).is_none());
    assert!(parse_hpet(&garbage).is_none());
    assert!(parse_srat_header(&garbage).is_none());
    assert!(parse_slit(&garbage).is_none());
    assert!(parse_dmar_header(&garbage).is_none());
    assert!(parse_gtdt(&garbage).is_none());
    assert!(parse_spcr(&garbage).is_none());
    assert!(parse_bgrt(&garbage).is_none());
}
