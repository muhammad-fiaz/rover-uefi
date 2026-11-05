//! # Integration tests for I2C protocols
//!
//! Verifies I2C protocol structures, GUIDs, and mock operations.

#![no_std]

extern crate alloc;

use core::ffi::c_void;
use rover_uefi::abi::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::i2c::{
    EFI_I2C_IO_PROTOCOL_GUID, EFI_I2C_MASTER_PROTOCOL_GUID, EfiI2cControllerCapabilities,
    EfiI2cIoProtocol, EfiI2cMasterProtocol, EfiI2cOperation, EfiI2cRequestPacket, I2cIoProtocol,
    I2cMasterProtocol, i2c_flags,
};

#[test]
fn test_i2c_guids() {
    assert_eq!(
        EFI_I2C_MASTER_PROTOCOL_GUID.to_u128_le(),
        0x6274_11a8_3d31_c898_4feb_45b5_cd72_881f
    );
    assert_eq!(
        EFI_I2C_IO_PROTOCOL_GUID.to_u128_le(),
        0x8ea2_6506_a1c9_9aa2_46e5_18c4_b60a_3e6b
    );
}

unsafe extern "efiapi" fn mock_set_bus_frequency(
    _this: *const EfiI2cMasterProtocol,
    bus_clock_hertz: *mut UIntN,
) -> EfiStatus {
    unsafe {
        *bus_clock_hertz = 400_000;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_reset(_this: *const EfiI2cMasterProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_start_request(
    _this: *const EfiI2cMasterProtocol,
    _slave_address: UIntN,
    _request_packet: *mut EfiI2cRequestPacket,
    _event: *mut c_void,
    i2c_status: *mut EfiStatus,
) -> EfiStatus {
    if !i2c_status.is_null() {
        unsafe {
            *i2c_status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_i2c_master_protocol() {
    let caps = EfiI2cControllerCapabilities {
        structure_size_in_bytes: 16,
        maximum_receive_bytes: 1024,
        maximum_transmit_bytes: 1024,
        maximum_total_bytes: 2048,
    };

    let raw = EfiI2cMasterProtocol {
        set_bus_frequency: mock_set_bus_frequency,
        reset: mock_reset,
        start_request: mock_start_request,
        capabilities: &raw const caps,
    };

    let master = I2cMasterProtocol::new(&raw);
    assert_eq!(master.capabilities(), Some(caps));
    assert_eq!(master.set_bus_frequency(1_000_000), Ok(400_000));
    assert!(master.reset().is_ok());

    let mut buf = [0xAA, 0xBB];
    let mut pkt = EfiI2cRequestPacket {
        operation_count: 1,
        operations: [EfiI2cOperation {
            flags: i2c_flags::READ,
            length_in_bytes: 2,
            buffer: buf.as_mut_ptr(),
        }],
    };

    assert!(unsafe { master.start_request_sync(0x50, &mut pkt) }.is_ok());
}

unsafe extern "efiapi" fn mock_queue_request(
    _this: *const EfiI2cIoProtocol,
    _slave_address_index: UIntN,
    _event: *mut c_void,
    _request_packet: *mut EfiI2cRequestPacket,
    i2c_status: *mut EfiStatus,
) -> EfiStatus {
    if !i2c_status.is_null() {
        unsafe {
            *i2c_status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_i2c_io_protocol() {
    let raw = EfiI2cIoProtocol {
        queue_request: mock_queue_request,
        device_guid: core::ptr::null(),
        device_index: 1,
        hardware_revision: 0x10,
        capabilities: core::ptr::null(),
    };

    let io = I2cIoProtocol::new(&raw);
    assert_eq!(io.device_index(), 1);
    assert_eq!(io.hardware_revision(), 0x10);
    assert!(io.device_guid().is_none());

    let mut buf = [0x55];
    let mut pkt = EfiI2cRequestPacket {
        operation_count: 1,
        operations: [EfiI2cOperation {
            flags: 0,
            length_in_bytes: 1,
            buffer: buf.as_mut_ptr(),
        }],
    };
    assert!(unsafe { io.queue_request_sync(0, &mut pkt) }.is_ok());
}
