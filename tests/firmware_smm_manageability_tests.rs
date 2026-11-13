//! Tests for SMM firmware protocols and IPMI platform manageability.
#![allow(clippy::expect_used, clippy::unreadable_literal)]

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::firmware::mm_communication::{
    EfiMmCommunication2Protocol, MmCommunication2,
};
use rover_uefi::protocols::firmware::smm_access2::{
    EfiSmmAccess2Protocol, EfiSmramDescriptor, SMRAM_LOCKED, SMRAM_OPEN, SmmAccess2,
};
use rover_uefi::protocols::firmware::smm_control2::{EfiSmmControl2Protocol, SmmControl2};
use rover_uefi::protocols::firmware::smm_cpu::{
    EfiSmmCpuProtocol, EfiSmmSaveStateIoInfo, EfiSmmSaveStateIoType, EfiSmmSaveStateIoWidth,
    EfiSmmSaveStateRegister, SmmCpu,
};
use rover_uefi::protocols::manageability::ipmi_fru::{
    fru_checksum_ok, parse_fru_board_info, parse_fru_common_header,
};
use rover_uefi::protocols::manageability::ipmi_sdr::{
    convert_sensor_reading, parse_full_sensor, parse_sdr_header, sdr_record_type,
};
use rover_uefi::protocols::manageability::ipmi_sel::{sel_iter, sensor_type};

// Mock functions for SmmCpu
unsafe extern "efiapi" fn mock_read_save_state(
    _this: *const EfiSmmCpuProtocol,
    width: usize,
    register: EfiSmmSaveStateRegister,
    _cpu_index: usize,
    buffer: *mut core::ffi::c_void,
) -> EfiStatus {
    if register == EfiSmmSaveStateRegister::Rax && width == 8 {
        let val = 0x1234_5678_9ABC_DEF0_u64;
        unsafe { *(buffer.cast::<u64>()) = val };
        EfiStatus::SUCCESS
    } else if register == EfiSmmSaveStateRegister::Io {
        let info = EfiSmmSaveStateIoInfo {
            io_data: 0x55,
            io_port: 0x80,
            io_width: EfiSmmSaveStateIoWidth::Uint8,
            io_type: EfiSmmSaveStateIoType::Output,
        };
        unsafe { *(buffer.cast::<EfiSmmSaveStateIoInfo>()) = info };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_write_save_state(
    _this: *const EfiSmmCpuProtocol,
    _width: usize,
    register: EfiSmmSaveStateRegister,
    _cpu_index: usize,
    _buffer: *const core::ffi::c_void,
) -> EfiStatus {
    if register == EfiSmmSaveStateRegister::Rax {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

#[test]
fn test_smm_cpu_client() {
    let proto = EfiSmmCpuProtocol {
        read_save_state: mock_read_save_state,
        write_save_state: mock_write_save_state,
    };
    let client = unsafe { SmmCpu::from_raw(&raw const proto) };

    let rax: u64 = client
        .read_save_state(0, EfiSmmSaveStateRegister::Rax)
        .expect("read rax");
    assert_eq!(rax, 0x1234_5678_9ABC_DEF0);

    let io_info = client.read_io_info(0).expect("read io info");
    assert_eq!(io_info.io_port, 0x80);
    assert_eq!(io_info.io_data, 0x55);
    assert_eq!(io_info.io_width, EfiSmmSaveStateIoWidth::Uint8);

    assert!(
        client
            .write_save_state(0, EfiSmmSaveStateRegister::Rax, &0x11223344_u32)
            .is_ok()
    );
    assert!(
        client
            .write_save_state(0, EfiSmmSaveStateRegister::Rbx, &0x11223344_u32)
            .is_err()
    );
}

// Mock functions for SmmAccess2
unsafe extern "efiapi" fn mock_open(_this: *mut EfiSmmAccess2Protocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_close(_this: *mut EfiSmmAccess2Protocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_lock(_this: *mut EfiSmmAccess2Protocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_capabilities(
    _this: *const EfiSmmAccess2Protocol,
    smram_map_size: *mut usize,
    smram_map: *mut EfiSmramDescriptor,
) -> EfiStatus {
    let req_size = core::mem::size_of::<EfiSmramDescriptor>();
    if unsafe { *smram_map_size } < req_size {
        unsafe { *smram_map_size = req_size };
        return EfiStatus::BUFFER_TOO_SMALL;
    }
    unsafe {
        *smram_map = EfiSmramDescriptor {
            physical_start: 0x7E00_0000,
            cpu_start: 0x7E00_0000,
            physical_size: 0x0200_0000,
            region_state: SMRAM_OPEN | SMRAM_LOCKED,
        };
        *smram_map_size = req_size;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_smm_access2_client() {
    let mut proto = EfiSmmAccess2Protocol {
        open: mock_open,
        close: mock_close,
        lock: mock_lock,
        get_capabilities: mock_get_capabilities,
        lock_state: true,
        open_state: true,
    };
    let client = unsafe { SmmAccess2::from_raw(&raw mut proto) };
    assert!(client.is_locked());
    assert!(client.is_open());
    assert!(client.open().is_ok());
    assert!(client.close().is_ok());
    assert!(client.lock().is_ok());

    let mut desc_buf = [EfiSmramDescriptor {
        physical_start: 0,
        cpu_start: 0,
        physical_size: 0,
        region_state: 0,
    }; 2];
    let descs = client
        .get_capabilities(&mut desc_buf)
        .expect("capabilities");
    assert_eq!(descs.len(), 1);
    assert_eq!(descs[0].physical_start, 0x7E00_0000);
}

// Mock functions for SmmControl2
unsafe extern "efiapi" fn mock_trigger(
    _this: *const EfiSmmControl2Protocol,
    _cmd_port: *mut u8,
    _data_port: *mut u8,
    _periodic: bool,
    _interval: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clear(
    _this: *const EfiSmmControl2Protocol,
    _periodic: bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_smm_control2_client() {
    let proto = EfiSmmControl2Protocol {
        trigger: mock_trigger,
        clear: mock_clear,
        minimum_trigger_period: 100,
    };
    let client = unsafe { SmmControl2::from_raw(&raw const proto) };
    assert_eq!(client.minimum_trigger_period(), 100);
    assert!(client.trigger(None, None, false, 0).is_ok());
    assert!(client.clear(false).is_ok());
}

// Mock function for MmCommunication2
unsafe extern "efiapi" fn mock_communicate2(
    _this: *const EfiMmCommunication2Protocol,
    _phys: *mut core::ffi::c_void,
    _virt: *mut core::ffi::c_void,
    _size: *mut usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_mm_communication2_client() {
    let proto = EfiMmCommunication2Protocol {
        communicate: mock_communicate2,
    };
    let client = unsafe { MmCommunication2::from_raw(&raw const proto) };
    let mut buf1 = [0_u8; 16];
    let mut buf2 = [0_u8; 16];
    assert!(client.communicate(&mut buf1, &mut buf2).is_ok());
}

#[test]
fn test_ipmi_fru_parsing() {
    let mut fru = [0_u8; 64];
    // Common header at offset 0 (8 bytes)
    fru[0] = 0x01; // format version 1
    fru[1] = 0; // internal use
    fru[2] = 0; // chassis
    fru[3] = 1; // board info at 1 * 8 = 8
    fru[4] = 3; // product info at 3 * 8 = 24
    fru[5] = 0; // multirecord
    fru[6] = 0; // pad
    let mut sum = 0_u8;
    for &b in &fru[0..7] {
        sum = sum.wrapping_add(b);
    }
    fru[7] = (0_u8).wrapping_sub(sum); // zero checksum

    assert!(fru_checksum_ok(&fru[0..8]));
    let hdr = parse_fru_common_header(&fru).expect("fru common header");
    assert_eq!(hdr.format_version, 1);
    assert_eq!(hdr.board_info_offset, 8);
    assert_eq!(hdr.product_info_offset, 24);

    // Board info at 8 (length 16 = 2 * 8)
    fru[8] = 0x01; // version
    fru[9] = 2; // length = 2 * 8 = 16 bytes
    fru[10] = 0; // English
    fru[11..14].copy_from_slice(&[0x10, 0x20, 0x30]); // mfg date
    // Manufacturer string: 0xC0 | 5 = 5 bytes ascii "ROVER"
    fru[14] = 0xC0 | 5;
    fru[15..20].copy_from_slice(b"ROVER");
    fru[20] = 0xC1; // end of strings sentinel
    let mut b_sum = 0_u8;
    for &b in &fru[8..23] {
        b_sum = b_sum.wrapping_add(b);
    }
    fru[23] = (0_u8).wrapping_sub(b_sum); // zero checksum

    let board = parse_fru_board_info(&fru, 8).expect("board info");
    assert_eq!(board.manufacturer, Some("ROVER"));
    assert_eq!(board.mfg_date_minutes, 0x0030_2010);
}

#[test]
fn test_ipmi_sel_parsing() {
    let mut sel = [0_u8; 32];
    // Entry 1
    sel[0..2].copy_from_slice(&1_u16.to_le_bytes()); // record id 1
    sel[2] = 0x02; // system event record
    sel[3..7].copy_from_slice(&1_700_000_000_u32.to_le_bytes()); // timestamp
    sel[7..9].copy_from_slice(&0x0020_u16.to_le_bytes()); // BIOS generator
    sel[9] = 0x04; // IPMI 2.0
    sel[10] = sensor_type::TEMPERATURE;
    sel[11] = 1; // sensor 1
    sel[12] = 0x01; // assertion, threshold event
    sel[13] = 0x55; // upper non-critical
    sel[14] = 0xAA;
    sel[15] = 0x00;

    // Entry 2
    sel[16..18].copy_from_slice(&2_u16.to_le_bytes()); // record id 2
    sel[18] = 0x02;
    sel[26] = sensor_type::PROCESSOR;
    sel[27] = 0;
    sel[28] = 0x81; // deassertion

    let mut iter = sel_iter(&sel);
    let r1 = iter.next().expect("rec 1");
    assert_eq!(r1.record_id, 1);
    assert_eq!(r1.sensor_type, sensor_type::TEMPERATURE);
    assert!(!r1.is_deassertion);
    assert_eq!(r1.event_data1, 0x55);

    let r2 = iter.next().expect("rec 2");
    assert_eq!(r2.record_id, 2);
    assert_eq!(r2.sensor_type, sensor_type::PROCESSOR);
    assert!(r2.is_deassertion);
    assert!(iter.next().is_none());
}

#[test]
fn test_ipmi_sdr_parsing_and_conversion() {
    let mut sdr = [0_u8; 64];
    sdr[0..2].copy_from_slice(&10_u16.to_le_bytes()); // id 10
    sdr[2] = 0x51; // v1.5
    sdr[3] = sdr_record_type::FULL_SENSOR;
    sdr[4] = 50; // length
    sdr[7] = 3; // sensor num
    sdr[12] = sensor_type::TEMPERATURE;
    sdr[21] = 1; // degrees C
    sdr[24] = 10; // M = 10
    sdr[25] = 0;
    sdr[27] = 0; // B = 0
    sdr[28] = 0;
    sdr[29] = 0; // exp = 0
    sdr[47] = 0x04; // string length 4
    sdr[48..52].copy_from_slice(b"CPU0");

    let hdr = parse_sdr_header(&sdr).expect("sdr hdr");
    assert_eq!(hdr.record_id, 10);
    assert_eq!(hdr.record_type, sdr_record_type::FULL_SENSOR);

    let full = parse_full_sensor(&sdr).expect("full sensor");
    assert_eq!(full.sensor_number, 3);
    assert_eq!(full.sensor_type, sensor_type::TEMPERATURE);
    assert_eq!(full.id_string, Some("CPU0"));

    // Conversion: raw = 5, M = 10, B = 0, exp = 0 -> (10 * 5) = 50.0 degrees C
    let reading = convert_sensor_reading(5, full.m, full.b, full.r_exp, full.b_exp);
    assert!((reading - 50.0).abs() < 1e-4);
}
