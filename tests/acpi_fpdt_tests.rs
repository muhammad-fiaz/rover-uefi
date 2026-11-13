//! Integration tests for ACPI Firmware Performance Data Table (FPDT).

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::acpi::fpdt::*;
use rover_uefi::protocols::acpi::types::SIG_FPDT;

fn compute_acpi_checksum(buffer: &mut [u8]) {
    buffer[9] = 0;
    let sum: u8 = buffer.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    buffer[9] = 0u8.wrapping_sub(sum);
}

#[test]
fn test_fpdt_root_table_parsing() {
    let mut table = vec![0u8; 68];
    // ACPI Header
    table[0..4].copy_from_slice(&SIG_FPDT);
    let len = 68u32;
    table[4..8].copy_from_slice(&len.to_le_bytes());
    table[8] = 1; // Revision
    table[10..16].copy_from_slice(b"ROVER ");
    table[16..24].copy_from_slice(b"FIRMWARE");
    table[24..28].copy_from_slice(&1u32.to_le_bytes());
    table[28..32].copy_from_slice(b"ROVR");
    table[32..36].copy_from_slice(&1u32.to_le_bytes());

    // Record 0: Basic Boot Performance Table Pointer (16 bytes)
    table[36..38].copy_from_slice(&FPDT_RECORD_TYPE_FIRMWARE_BASIC_BOOT_POINTER.to_le_bytes());
    table[38] = 16; // Length
    table[39] = 1; // Revision
    // Bytes 40..44 reserved
    let fbpt_ptr = 0x0000_0000_7F00_0000u64;
    table[44..52].copy_from_slice(&fbpt_ptr.to_le_bytes());

    // Record 1: S3 Performance Table Pointer (16 bytes)
    table[52..54].copy_from_slice(&FPDT_RECORD_TYPE_S3_PERFORMANCE_TABLE_POINTER.to_le_bytes());
    table[54] = 16; // Length
    table[55] = 1; // Revision
    // Bytes 56..60 reserved
    let s3pt_ptr = 0x0000_0000_7F10_0000u64;
    table[60..68].copy_from_slice(&s3pt_ptr.to_le_bytes());

    compute_acpi_checksum(&mut table);

    let Some(info) = parse_fpdt_root(&table) else {
        panic!("failed to parse valid FPDT root table");
    };

    assert_eq!(info.boot_performance_table_address, Some(fbpt_ptr));
    assert_eq!(info.s3_performance_table_address, Some(s3pt_ptr));
}

#[test]
fn test_fbpt_records_parsing() {
    let mut fbpt = Vec::new();
    // FBPT Header: "FBPT", length
    fbpt.extend_from_slice(&SIG_FBPT);
    fbpt.extend_from_slice(&0u32.to_le_bytes()); // placeholder length

    // Record 1: Basic Boot Record (48 bytes)
    fbpt.extend_from_slice(&FPDT_RUNTIME_RECORD_TYPE_FIRMWARE_BASIC_BOOT.to_le_bytes());
    fbpt.push(48); // length
    fbpt.push(2); // revision
    fbpt.extend_from_slice(&[0u8; 4]); // reserved
    fbpt.extend_from_slice(&100_000_000u64.to_le_bytes()); // reset_end
    fbpt.extend_from_slice(&250_000_000u64.to_le_bytes()); // os_loader_load
    fbpt.extend_from_slice(&300_000_000u64.to_le_bytes()); // os_loader_start
    fbpt.extend_from_slice(&400_000_000u64.to_le_bytes()); // ebs_entry
    fbpt.extend_from_slice(&450_000_000u64.to_le_bytes()); // ebs_exit

    // Record 2: Extended GUID Event Record (34 bytes)
    fbpt.extend_from_slice(&FPDT_RECORD_TYPE_GUID_EVENT.to_le_bytes());
    fbpt.push(34); // length
    fbpt.push(1); // revision
    fbpt.extend_from_slice(&0x0010u16.to_le_bytes()); // progress_id
    fbpt.extend_from_slice(&0u32.to_le_bytes()); // apic_id
    fbpt.extend_from_slice(&120_000_000u64.to_le_bytes()); // timestamp
    let test_guid = EfiGuid::new(
        0x1122_3344,
        0x5566,
        0x7788,
        [0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00],
    );
    fbpt.extend_from_slice(&test_guid.to_bytes());

    // Record 3: Extended String Event Record (26 bytes)
    fbpt.extend_from_slice(&FPDT_RECORD_TYPE_DYNAMIC_STRING_EVENT.to_le_bytes());
    fbpt.push(26); // length
    fbpt.push(1); // revision
    fbpt.extend_from_slice(&0x0020u16.to_le_bytes()); // progress_id
    fbpt.extend_from_slice(&0u32.to_le_bytes()); // apic_id
    fbpt.extend_from_slice(&130_000_000u64.to_le_bytes()); // timestamp
    fbpt.extend_from_slice(b"BDS:Boot"); // 8 bytes description

    let total_len = u32::try_from(fbpt.len()).unwrap_or(0);
    fbpt[4..8].copy_from_slice(&total_len.to_le_bytes());

    let Some(iter) = FpdtRecordIter::new(&fbpt) else {
        panic!("failed to create FBPT record iterator");
    };

    let records: Vec<FpdtParsedRecord> = iter.collect();
    assert_eq!(records.len(), 3);

    match &records[0] {
        FpdtParsedRecord::BasicBoot(bb) => {
            assert_eq!(bb.reset_end, 100_000_000);
            assert_eq!(bb.os_loader_load_image_start, 250_000_000);
            assert_eq!(bb.os_loader_start_image_start, 300_000_000);
            assert_eq!(bb.exit_boot_services_entry, 400_000_000);
            assert_eq!(bb.exit_boot_services_exit, 450_000_000);
        }
        other => panic!("expected BasicBoot, got {other:?}"),
    }

    match &records[1] {
        FpdtParsedRecord::GuidEvent(ge) => {
            assert_eq!(ge.progress_id, 0x0010);
            assert_eq!(ge.timestamp, 120_000_000);
            assert_eq!(ge.guid, test_guid);
        }
        other => panic!("expected GuidEvent, got {other:?}"),
    }

    match &records[2] {
        FpdtParsedRecord::StringEvent(se) => {
            assert_eq!(se.progress_id, 0x0020);
            assert_eq!(se.timestamp, 130_000_000);
            assert_eq!(&se.description[..se.description_length], b"BDS:Boot");
        }
        other => panic!("expected StringEvent, got {other:?}"),
    }
}

#[test]
fn test_s3pt_records_parsing() {
    let mut s3pt = Vec::new();
    s3pt.extend_from_slice(&SIG_S3PT);
    s3pt.extend_from_slice(&0u32.to_le_bytes()); // placeholder length

    // Record 1: S3 Resume Record (24 bytes)
    s3pt.extend_from_slice(&FPDT_RUNTIME_RECORD_TYPE_S3_RESUME.to_le_bytes());
    s3pt.push(24); // length
    s3pt.push(1); // revision
    s3pt.extend_from_slice(&5u32.to_le_bytes()); // resume_count
    s3pt.extend_from_slice(&15_000_000u64.to_le_bytes()); // full_resume
    s3pt.extend_from_slice(&14_500_000u64.to_le_bytes()); // average_resume

    // Record 2: S3 Suspend Record (20 bytes)
    s3pt.extend_from_slice(&FPDT_RUNTIME_RECORD_TYPE_S3_SUSPEND.to_le_bytes());
    s3pt.push(20); // length
    s3pt.push(1); // revision
    s3pt.extend_from_slice(&8_000_000u64.to_le_bytes()); // suspend_start
    s3pt.extend_from_slice(&12_000_000u64.to_le_bytes()); // suspend_end

    let total_len = u32::try_from(s3pt.len()).unwrap_or(0);
    s3pt[4..8].copy_from_slice(&total_len.to_le_bytes());

    let Some(iter) = FpdtRecordIter::new(&s3pt) else {
        panic!("failed to create S3PT record iterator");
    };

    let records: Vec<FpdtParsedRecord> = iter.collect();
    assert_eq!(records.len(), 2);

    match &records[0] {
        FpdtParsedRecord::S3Resume(r) => {
            assert_eq!(r.resume_count, 5);
            assert_eq!(r.full_resume, 15_000_000);
            assert_eq!(r.average_resume, 14_500_000);
        }
        other => panic!("expected S3Resume, got {other:?}"),
    }

    match &records[1] {
        FpdtParsedRecord::S3Suspend(s) => {
            assert_eq!(s.suspend_start, 8_000_000);
            assert_eq!(s.suspend_end, 12_000_000);
        }
        other => panic!("expected S3Suspend, got {other:?}"),
    }
}

#[test]
fn test_fpdt_malformed_rejections() {
    assert_eq!(parse_fpdt_root(&[]), None);
    assert_eq!(parse_fpdt_root(&[0u8; 35]), None);

    let bad_sig = [0u8; 36];
    assert_eq!(parse_fpdt_root(&bad_sig), None);

    assert!(FpdtRecordIter::new(&[]).is_none());
    assert!(FpdtRecordIter::new(b"INVALID_").is_none());
}
