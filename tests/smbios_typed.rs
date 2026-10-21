//! SMBIOS typed record tests.

use rover_uefi::protocols::smbios::parse_bios_info as reexported_bios;
use rover_uefi::protocols::smbios::typed::{
    decode_cache_size_kb, decode_memory_device_size_kb, parse_baseboard_info, parse_bios_info,
    parse_cache_info, parse_enclosure_info, parse_memory_array_mapped, parse_memory_device,
    parse_memory_device_mapped, parse_physical_memory_array, parse_port_connector,
    parse_processor_info, parse_system_info, parse_system_slots, u16_le, u32_le, u64_le,
};

#[test]
fn le_readers() {
    let buf = [0x34_u8, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xFF];
    assert_eq!(u16_le(&buf, 0), Some(0x1234));
    assert_eq!(u32_le(&buf, 0), Some(0x5678_1234));
    assert_eq!(u64_le(&buf, 0), Some(0xFFF0_9ABC_5678_1234));
    assert_eq!(u16_le(&buf, 7), None);
    assert_eq!(u32_le(&buf, 6), None);
    assert_eq!(u64_le(&buf, 1), None);
    assert_eq!(u16_le(&[], 0), None);
    assert_eq!(u32_le(&[0_u8; 3], 0), None);
    assert_eq!(u64_le(&[0_u8; 7], 0), None);
}

#[test]
fn cache_size_decode_both_units() {
    assert_eq!(decode_cache_size_kb(0x0100), 256);
    assert_eq!(decode_cache_size_kb(0x8008), 8 * 1024);
    assert_eq!(decode_cache_size_kb(0x0000), 0);
    assert_eq!(decode_memory_device_size_kb(0x8008), Some(8));
    assert_eq!(decode_memory_device_size_kb(0x0008), Some(8 * 1024));
    assert_eq!(decode_memory_device_size_kb(0x7FFF), None);
    assert_eq!(decode_memory_device_size_kb(0xFFFF), Some(0));
}

#[test]
fn bios_valid_truncated_malformed() {
    let mut buf = [0_u8; 23];
    buf[0] = 0;
    buf[1] = 23;
    buf[2..4].copy_from_slice(&0x0001_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06..0x08].copy_from_slice(&0xE000_u16.to_le_bytes());
    buf[0x08] = 3;
    buf[0x09] = 0x42;
    buf[0x0A..0x12].copy_from_slice(&0x1122_3344_5566_7788_u64.to_le_bytes());
    buf[0x12] = 0xAA;
    buf[0x13] = 0xBB;
    buf[0x14] = 3;
    buf[0x15] = 5;
    buf[0x16] = 7;
    let Some(info) = parse_bios_info(&buf) else {
        panic!("expected bios info");
    };
    assert_eq!(info.vendor_idx, 1);
    assert_eq!(info.version_idx, 2);
    assert_eq!(info.starting_address, 0xE000);
    assert_eq!(info.release_date_idx, 3);
    assert_eq!(info.rom_size, 0x42);
    assert_eq!(info.characteristics, 0x1122_3344_5566_7788);
    assert_eq!(info.characteristic_extension_1, 0xAA);
    assert_eq!(info.characteristic_extension_2, 0xBB);
    assert_eq!(info.major_version, 3);
    assert_eq!(info.minor_version, 5);
    assert_eq!(info.embedded_controller_revision, 7);
    assert_eq!(reexported_bios(&buf), Some(info));
    assert!(parse_bios_info(&[]).is_none());
    assert!(parse_bios_info(&[0_u8; 22]).is_none());
    assert!(parse_bios_info(&buf[..22]).is_none());
}

#[test]
fn system_valid_truncated() {
    let mut buf = [0_u8; 27];
    buf[0] = 1;
    buf[1] = 27;
    buf[2..4].copy_from_slice(&0x0002_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06] = 3;
    buf[0x07] = 4;
    let mut uuid = [0_u8; 16];
    let mut idx = 0_usize;
    while idx < 16 {
        uuid[idx] = u8::try_from(idx).unwrap_or(0).wrapping_add(0x10);
        idx += 1;
    }
    buf[0x08..0x18].copy_from_slice(&uuid);
    buf[0x18] = 6;
    buf[0x19] = 5;
    buf[0x1A] = 6;
    let Some(info) = parse_system_info(&buf) else {
        panic!("expected system info");
    };
    assert_eq!(info.manufacturer_idx, 1);
    assert_eq!(info.product_name_idx, 2);
    assert_eq!(info.version_idx, 3);
    assert_eq!(info.serial_number_idx, 4);
    assert_eq!(info.uuid, uuid);
    assert_eq!(info.wakeup_type, 6);
    assert_eq!(info.sku_number_idx, 5);
    assert_eq!(info.family_idx, 6);
    assert!(parse_system_info(&[]).is_none());
    assert!(parse_system_info(&[0_u8; 26]).is_none());
}

#[test]
fn baseboard_valid_truncated() {
    let mut buf = [0_u8; 15];
    buf[0] = 2;
    buf[1] = 15;
    buf[2..4].copy_from_slice(&0x0003_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06] = 3;
    buf[0x07] = 4;
    buf[0x08] = 5;
    buf[0x09] = 0x0F;
    buf[0x0A] = 6;
    buf[0x0B..0x0D].copy_from_slice(&0x0044_u16.to_le_bytes());
    buf[0x0D] = 10;
    buf[0x0E] = 2;
    let Some(info) = parse_baseboard_info(&buf) else {
        panic!("expected baseboard info");
    };
    assert_eq!(info.manufacturer_idx, 1);
    assert_eq!(info.product_idx, 2);
    assert_eq!(info.version_idx, 3);
    assert_eq!(info.serial_number_idx, 4);
    assert_eq!(info.asset_tag_idx, 5);
    assert_eq!(info.feature_flags, 0x0F);
    assert_eq!(info.location_in_chassis_idx, 6);
    assert_eq!(info.chassis_handle, 0x0044);
    assert_eq!(info.board_type, 10);
    assert_eq!(info.object_handle_count, 2);
    assert!(parse_baseboard_info(&[]).is_none());
    assert!(parse_baseboard_info(&[0_u8; 14]).is_none());
}

#[test]
fn enclosure_valid_with_oem() {
    let mut buf = [0_u8; 16];
    buf[0] = 3;
    buf[1] = 16;
    buf[2..4].copy_from_slice(&0x0001_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06] = 3;
    buf[0x07] = 4;
    buf[0x08] = 5;
    buf[0x09] = 1;
    buf[0x0A] = 2;
    buf[0x0B] = 3;
    buf[0x0C] = 4;
    buf[0x0D] = 0;
    buf[0x0E..0x10].copy_from_slice(&7_u16.to_le_bytes());
    let Some(info) = parse_enclosure_info(&buf) else {
        panic!("expected enclosure info");
    };
    assert_eq!(info.manufacturer_idx, 1);
    assert_eq!(info.enclosure_type, 2);
    assert_eq!(info.version_idx, 3);
    assert_eq!(info.serial_number_idx, 4);
    assert_eq!(info.asset_tag_idx, 5);
    assert_eq!(info.bootup_state, 1);
    assert_eq!(info.power_supply_state, 2);
    assert_eq!(info.thermal_state, 3);
    assert_eq!(info.security_status, 4);
    assert_eq!(info.height, Some(7));
    let mut oem_buf = [0_u8; 20];
    oem_buf[0] = 3;
    oem_buf[1] = 20;
    oem_buf[2..4].copy_from_slice(&0x0001_u16.to_le_bytes());
    oem_buf[0x04] = 1;
    oem_buf[0x05] = 2;
    oem_buf[0x06] = 3;
    oem_buf[0x07] = 4;
    oem_buf[0x08] = 5;
    oem_buf[0x09] = 1;
    oem_buf[0x0A] = 2;
    oem_buf[0x0B] = 3;
    oem_buf[0x0C] = 4;
    oem_buf[0x0D] = 1;
    oem_buf[0x0E..0x12].copy_from_slice(&0xDEAD_BEEF_u32.to_le_bytes());
    oem_buf[0x12..0x14].copy_from_slice(&9_u16.to_le_bytes());
    let Some(oem_info) = parse_enclosure_info(&oem_buf) else {
        panic!("expected enclosure info with oem");
    };
    assert_eq!(oem_info.oem_defined_count, 1);
    assert_eq!(oem_info.height, Some(9));
    assert!(parse_enclosure_info(&[]).is_none());
    assert!(parse_enclosure_info(&[0_u8; 13]).is_none());
    let mut malformed = [0_u8; 14];
    malformed[0] = 3;
    malformed[1] = 14;
    malformed[0x0D] = 255;
    assert!(parse_enclosure_info(&malformed).is_none());
}

#[test]
fn processor_valid_version_aware() {
    let mut buf = [0_u8; 52];
    buf[0] = 4;
    buf[1] = 52;
    buf[2..4].copy_from_slice(&0x0004_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 3;
    buf[0x06] = 0x42;
    buf[0x07..0x0F].copy_from_slice(&0x1122_3344_5566_7788_u64.to_le_bytes());
    buf[0x0F] = 2;
    buf[0x10] = 0xFF;
    buf[0x11..0x13].copy_from_slice(&100_u16.to_le_bytes());
    buf[0x13..0x15].copy_from_slice(&3400_u16.to_le_bytes());
    buf[0x15..0x17].copy_from_slice(&2600_u16.to_le_bytes());
    buf[0x17] = 0x01;
    buf[0x18] = 0x02;
    buf[0x19..0x1B].copy_from_slice(&0x0010_u16.to_le_bytes());
    buf[0x1B..0x1D].copy_from_slice(&0x0011_u16.to_le_bytes());
    buf[0x1D..0x1F].copy_from_slice(&0x0012_u16.to_le_bytes());
    buf[0x1F] = 3;
    buf[0x20] = 4;
    buf[0x21] = 5;
    buf[0x22] = 8;
    buf[0x23] = 6;
    buf[0x24] = 16;
    buf[0x25] = 12;
    buf[0x26..0x28].copy_from_slice(&0x0102_u16.to_le_bytes());
    buf[0x28..0x2A].copy_from_slice(&0x0304_u16.to_le_bytes());
    buf[0x2A..0x2C].copy_from_slice(&8_u16.to_le_bytes());
    buf[0x2C..0x2E].copy_from_slice(&6_u16.to_le_bytes());
    buf[0x2E..0x30].copy_from_slice(&16_u16.to_le_bytes());
    buf[0x30..0x32].copy_from_slice(&12_u16.to_le_bytes());
    buf[0x32..0x34].copy_from_slice(&0x0001_u16.to_le_bytes());
    let Some(info) = parse_processor_info(&buf) else {
        panic!("expected processor info");
    };
    assert_eq!(info.socket_designation_idx, 1);
    assert_eq!(info.processor_type, 3);
    assert_eq!(info.processor_family, 0x42);
    assert_eq!(info.processor_id, 0x1122_3344_5566_7788);
    assert_eq!(info.version_idx, 2);
    assert_eq!(info.max_speed_mhz, 3400);
    assert_eq!(info.current_speed_mhz, 2600);
    assert_eq!(info.external_clock_mhz, 100);
    assert_eq!(info.status, 1);
    assert_eq!(info.core_count_max, Some(8));
    assert_eq!(info.core_enabled, Some(6));
    assert_eq!(info.thread_count_max, Some(16));
    assert_eq!(info.thread_enabled, Some(12));
    assert_eq!(info.characteristics, Some(0x0102));
    assert_eq!(info.instruction_set, Some(0x0001));
    let mut base = [0_u8; 31];
    base[0] = 4;
    base[1] = 31;
    base[2..4].copy_from_slice(&0x0004_u16.to_le_bytes());
    base[0x04] = 1;
    base[0x05] = 3;
    base[0x06] = 0x42;
    base[0x07..0x0F].copy_from_slice(&0x1122_3344_5566_7788_u64.to_le_bytes());
    base[0x0F] = 2;
    base[0x10] = 0xFF;
    base[0x11..0x13].copy_from_slice(&100_u16.to_le_bytes());
    base[0x13..0x15].copy_from_slice(&3400_u16.to_le_bytes());
    base[0x15..0x17].copy_from_slice(&2600_u16.to_le_bytes());
    base[0x17] = 1;
    base[0x18] = 2;
    base[0x19..0x1B].copy_from_slice(&0x0010_u16.to_le_bytes());
    base[0x1B..0x1D].copy_from_slice(&0x0011_u16.to_le_bytes());
    base[0x1D..0x1F].copy_from_slice(&0x0012_u16.to_le_bytes());
    let Some(base_info) = parse_processor_info(&base) else {
        panic!("expected base processor info");
    };
    assert_eq!(base_info.serial_number_idx, None);
    assert_eq!(base_info.core_count_max, None);
    assert_eq!(base_info.thread_count_max, None);
    assert_eq!(base_info.characteristics, None);
    assert!(parse_processor_info(&[]).is_none());
    assert!(parse_processor_info(&[0_u8; 30]).is_none());
    assert!(parse_processor_info(&buf[..30]).is_none());
}

#[test]
fn cache_valid_kb_and_mb() {
    let mut buf = [0_u8; 17];
    buf[0] = 7;
    buf[1] = 17;
    buf[2..4].copy_from_slice(&0x0007_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05..0x07].copy_from_slice(&0x0123_u16.to_le_bytes());
    buf[0x07..0x09].copy_from_slice(&0x0100_u16.to_le_bytes());
    buf[0x09..0x0B].copy_from_slice(&0x8008_u16.to_le_bytes());
    buf[0x0B] = 0x0F;
    buf[0x0C] = 0x08;
    buf[0x0D] = 3;
    buf[0x0E] = 2;
    buf[0x0F] = 1;
    buf[0x10] = 4;
    let Some(info) = parse_cache_info(&buf) else {
        panic!("expected cache info");
    };
    assert_eq!(info.socket_designation_idx, 1);
    assert_eq!(info.cache_configuration, 0x0123);
    assert_eq!(info.max_size_raw, 0x0100);
    assert_eq!(info.current_size_raw, 0x8008);
    assert_eq!(info.max_size_kb, 256);
    assert_eq!(info.current_size_kb, 8192);
    assert_eq!(info.supported_sram_type, 0x0F);
    assert_eq!(info.current_sram_type, 8);
    assert_eq!(info.speed, 3);
    assert_eq!(info.error_correction_type, 2);
    assert_eq!(info.system_cache_type, 1);
    assert_eq!(info.associativity, 4);
    assert!(parse_cache_info(&[]).is_none());
    assert!(parse_cache_info(&[0_u8; 16]).is_none());
}

#[test]
fn port_connector_valid_truncated() {
    let mut buf = [0_u8; 8];
    buf[0] = 8;
    buf[1] = 8;
    buf[2..4].copy_from_slice(&0x0008_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06] = 3;
    buf[0x07] = 4;
    let Some(info) = parse_port_connector(&buf) else {
        panic!("expected port connector");
    };
    assert_eq!(info.internal_designator_idx, 1);
    assert_eq!(info.external_designator_idx, 2);
    assert_eq!(info.port_type, 3);
    assert_eq!(info.connector_type, 4);
    assert!(parse_port_connector(&[]).is_none());
    assert!(parse_port_connector(&[0_u8; 7]).is_none());
}

#[test]
fn system_slots_valid_truncated() {
    let mut buf = [0_u8; 13];
    buf[0] = 9;
    buf[1] = 13;
    buf[2..4].copy_from_slice(&0x0009_u16.to_le_bytes());
    buf[0x04] = 1;
    buf[0x05] = 2;
    buf[0x06] = 3;
    buf[0x07] = 4;
    buf[0x08] = 5;
    buf[0x09..0x0B].copy_from_slice(&0x0102_u16.to_le_bytes());
    buf[0x0B] = 6;
    buf[0x0C] = 7;
    let Some(info) = parse_system_slots(&buf) else {
        panic!("expected system slots");
    };
    assert_eq!(info.designation_idx, 1);
    assert_eq!(info.slot_type, 2);
    assert_eq!(info.data_bus_width, 3);
    assert_eq!(info.current_usage, 4);
    assert_eq!(info.length, 5);
    assert_eq!(info.id, 0x0102);
    assert_eq!(info.characteristics_1, 6);
    assert_eq!(info.characteristics_2, 7);
    assert!(parse_system_slots(&[]).is_none());
    assert!(parse_system_slots(&[0_u8; 12]).is_none());
}

#[test]
fn physical_memory_array_valid_truncated() {
    let mut buf = [0_u8; 12];
    buf[0] = 16;
    buf[1] = 12;
    buf[2..4].copy_from_slice(&0x0010_u16.to_le_bytes());
    buf[0x04] = 3;
    buf[0x05] = 1;
    buf[0x06..0x0A].copy_from_slice(&0x00FF_FFFF_u32.to_le_bytes());
    buf[0x0A..0x0C].copy_from_slice(&8_u16.to_le_bytes());
    let Some(info) = parse_physical_memory_array(&buf) else {
        panic!("expected physical memory array");
    };
    assert_eq!(info.use_kind, 3);
    assert_eq!(info.error_correction_type, 1);
    assert_eq!(info.maximum_capacity, 0x00FF_FFFF);
    assert_eq!(info.maximum_number_of_devices, 8);
    assert!(parse_physical_memory_array(&[]).is_none());
    assert!(parse_physical_memory_array(&[0_u8; 11]).is_none());
}

#[test]
fn memory_device_valid_version_aware() {
    let mut buf = [0_u8; 39];
    buf[0] = 17;
    buf[1] = 39;
    buf[2..4].copy_from_slice(&0x0011_u16.to_le_bytes());
    buf[0x04..0x06].copy_from_slice(&0x0010_u16.to_le_bytes());
    buf[0x06..0x08].copy_from_slice(&0xFFFF_u16.to_le_bytes());
    buf[0x08..0x0A].copy_from_slice(&64_u16.to_le_bytes());
    buf[0x0A..0x0C].copy_from_slice(&64_u16.to_le_bytes());
    buf[0x0C..0x0E].copy_from_slice(&0x8008_u16.to_le_bytes());
    buf[0x0E] = 2;
    buf[0x0F] = 1;
    buf[0x10] = 2;
    buf[0x11] = 0x1A;
    buf[0x12..0x14].copy_from_slice(&0x0002_u16.to_le_bytes());
    buf[0x14..0x16].copy_from_slice(&3200_u16.to_le_bytes());
    buf[0x16] = 1;
    buf[0x17] = 2;
    buf[0x18] = 3;
    buf[0x19] = 4;
    buf[0x1A] = 5;
    buf[0x1B..0x1F].copy_from_slice(&8192_u32.to_le_bytes());
    buf[0x1F..0x21].copy_from_slice(&3200_u16.to_le_bytes());
    buf[0x21..0x23].copy_from_slice(&1200_u16.to_le_bytes());
    buf[0x23..0x25].copy_from_slice(&1350_u16.to_le_bytes());
    buf[0x25..0x27].copy_from_slice(&1300_u16.to_le_bytes());
    let Some(info) = parse_memory_device(&buf) else {
        panic!("expected memory device");
    };
    assert_eq!(info.physical_memory_array_handle, 0x0010);
    assert_eq!(info.total_width, 64);
    assert_eq!(info.data_width, 64);
    assert_eq!(info.size_raw, 0x8008);
    assert_eq!(info.size_kb, Some(8));
    assert_eq!(info.form_factor, 2);
    assert_eq!(info.device_locator_idx, 1);
    assert_eq!(info.memory_type, 0x1A);
    assert_eq!(info.speed_mtps, 3200);
    assert_eq!(info.extended_size, Some(8192));
    assert_eq!(info.configured_clock_speed_mtps, Some(3200));
    assert_eq!(info.minimum_voltage_mv, Some(1200));
    assert_eq!(info.maximum_voltage_mv, Some(1350));
    assert_eq!(info.configured_voltage_mv, Some(1300));
    let base = [&buf[..27], &buf[..27]][0];
    let Some(base_info) = parse_memory_device(base) else {
        panic!("expected base memory device");
    };
    assert_eq!(base_info.extended_size, None);
    assert_eq!(base_info.configured_voltage_mv, None);
    assert!(parse_memory_device(&[]).is_none());
    assert!(parse_memory_device(&[0_u8; 26]).is_none());
}

#[test]
fn memory_array_mapped_valid_truncated() {
    let mut buf = [0_u8; 15];
    buf[0] = 19;
    buf[1] = 15;
    buf[2..4].copy_from_slice(&0x0013_u16.to_le_bytes());
    buf[0x04..0x08].copy_from_slice(&0x0010_0000_u32.to_le_bytes());
    buf[0x08..0x0C].copy_from_slice(&0x0020_0000_u32.to_le_bytes());
    buf[0x0C..0x0E].copy_from_slice(&0x0010_u16.to_le_bytes());
    buf[0x0E] = 1;
    let Some(info) = parse_memory_array_mapped(&buf) else {
        panic!("expected memory array mapped");
    };
    assert_eq!(info.starting_address, 0x0010_0000);
    assert_eq!(info.ending_address, 0x0020_0000);
    assert_eq!(info.physical_memory_array_handle, 0x0010);
    assert_eq!(info.partition_width, 1);
    assert!(parse_memory_array_mapped(&[]).is_none());
    assert!(parse_memory_array_mapped(&[0_u8; 14]).is_none());
}

#[test]
fn memory_device_mapped_valid_truncated() {
    let mut buf = [0_u8; 18];
    buf[0] = 20;
    buf[1] = 18;
    buf[2..4].copy_from_slice(&0x0014_u16.to_le_bytes());
    buf[0x04..0x06].copy_from_slice(&0x0013_u16.to_le_bytes());
    buf[0x06..0x0A].copy_from_slice(&0x0010_0000_u32.to_le_bytes());
    buf[0x0A..0x0E].copy_from_slice(&0x0018_0000_u32.to_le_bytes());
    buf[0x0E..0x10].copy_from_slice(&1_u16.to_le_bytes());
    buf[0x10] = 0;
    buf[0x11] = 1;
    let Some(info) = parse_memory_device_mapped(&buf) else {
        panic!("expected memory device mapped");
    };
    assert_eq!(info.memory_array_mapped_address_handle, 0x0013);
    assert_eq!(info.starting_address, 0x0010_0000);
    assert_eq!(info.ending_address, 0x0018_0000);
    assert_eq!(info.partition_row_position, 1);
    assert_eq!(info.interleave_position, 0);
    assert_eq!(info.maximum_interleave, 1);
    assert!(parse_memory_device_mapped(&[]).is_none());
    assert!(parse_memory_device_mapped(&[0_u8; 17]).is_none());
}
