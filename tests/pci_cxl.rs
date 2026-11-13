//! # Integration tests for CXL I/O protocol
//!
//! Verifies CXL protocol structures, GUIDs, and mock operations.

#![no_std]

extern crate alloc;

use core::ffi::c_void;
use rover_uefi::abi::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::pci::{
    CXL_IO_PROTOCOL_GUID, CxlIoProtocol, CxlIoProtocolDvsecAccess, CxlIoProtocolRegblockAccess,
    EfiCxlIoProtocol,
};

#[test]
fn test_cxl_guid() {
    assert_eq!(
        CXL_IO_PROTOCOL_GUID.to_u128_le(),
        0xf8a8_d3cd_7598_97a7_41de_bbc1_8fac_60b2
    );
}

unsafe extern "efiapi" fn mock_doe_transact(
    _this: *mut EfiCxlIoProtocol,
    _request_buffer: *const c_void,
    _request_buffer_size: UIntN,
    response_buffer: *mut c_void,
    response_buffer_size: *mut UIntN,
) -> EfiStatus {
    unsafe {
        if *response_buffer_size >= 4 {
            *response_buffer.cast::<u32>() = 0x1234_5678;
            *response_buffer_size = 4;
            EfiStatus::SUCCESS
        } else {
            EfiStatus::BUFFER_TOO_SMALL
        }
    }
}

unsafe extern "efiapi" fn mock_regblock_read(
    _this: *mut EfiCxlIoProtocol,
    _reg_block: u32,
    _offset: u64,
    buf: *mut c_void,
    size: UIntN,
) -> EfiStatus {
    for i in 0..size {
        unsafe {
            *buf.cast::<u8>().add(i) = 0xAA;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_regblock_write(
    _this: *mut EfiCxlIoProtocol,
    _reg_block: u32,
    _offset: u64,
    _buf: *const c_void,
    _size: UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_dvsec_read(
    _this: *mut EfiCxlIoProtocol,
    _cxl_dvsec_id: u32,
    _offset: u32,
    buf: *mut c_void,
    size: UIntN,
) -> EfiStatus {
    for i in 0..size {
        unsafe {
            *buf.cast::<u8>().add(i) = 0x55;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_dvsec_write(
    _this: *mut EfiCxlIoProtocol,
    _cxl_dvsec_id: u32,
    _offset: u32,
    _buf: *const c_void,
    _size: UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_cxl_operations() {
    let mut raw = EfiCxlIoProtocol {
        pci_io: core::ptr::null_mut(),
        doe_transact: mock_doe_transact,
        regblock: CxlIoProtocolRegblockAccess {
            read: mock_regblock_read,
            write: mock_regblock_write,
        },
        dvsec: CxlIoProtocolDvsecAccess {
            read: mock_dvsec_read,
            write: mock_dvsec_write,
        },
    };

    let mut cxl = CxlIoProtocol::new(&mut raw);

    let mut reg_buf = [0u8; 4];
    assert!(cxl.read_regblock(0, 0x10, &mut reg_buf).is_ok());
    assert_eq!(reg_buf, [0xAA, 0xAA, 0xAA, 0xAA]);

    assert!(cxl.write_regblock(0, 0x10, &[1, 2, 3, 4]).is_ok());

    let mut dvsec_buf = [0u8; 2];
    assert!(cxl.read_dvsec(1, 0, &mut dvsec_buf).is_ok());
    assert_eq!(dvsec_buf, [0x55, 0x55]);

    let mut resp = [0u8; 4];
    let res = cxl.doe_transact(&[0x01], &mut resp);
    assert_eq!(res, Ok(4));
    assert_eq!(u32::from_ne_bytes(resp), 0x1234_5678);
}
