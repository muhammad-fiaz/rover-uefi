// SPDX-License-Identifier: Apache-2.0
//! Unit tests for CPU I/O 2 Protocol.

use core::ffi::c_void;
use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::core::{
    CpuIo2, EFI_CPU_IO2_PROTOCOL_GUID, EfiCpuIo2Protocol, EfiCpuIo2ProtocolAccess,
    EfiCpuIoProtocolWidth,
};

static TEST_MEM_VALUE: AtomicU32 = AtomicU32::new(0x1234_5678);
static TEST_IO_VALUE: AtomicU16 = AtomicU16::new(0xA5A5);

unsafe extern "efiapi" fn mock_mem_read(
    _this: *const EfiCpuIo2Protocol,
    _width: EfiCpuIoProtocolWidth,
    _address: u64,
    _count: UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    unsafe {
        *buffer.cast::<u32>() = TEST_MEM_VALUE.load(Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mem_write(
    _this: *const EfiCpuIo2Protocol,
    _width: EfiCpuIoProtocolWidth,
    _address: u64,
    _count: UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    unsafe {
        TEST_MEM_VALUE.store(*buffer.cast::<u32>(), Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_io_read(
    _this: *const EfiCpuIo2Protocol,
    _width: EfiCpuIoProtocolWidth,
    _address: u64,
    _count: UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    unsafe {
        *buffer.cast::<u16>() = TEST_IO_VALUE.load(Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_io_write(
    _this: *const EfiCpuIo2Protocol,
    _width: EfiCpuIoProtocolWidth,
    _address: u64,
    _count: UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    unsafe {
        TEST_IO_VALUE.store(*buffer.cast::<u16>(), Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

#[test]
fn cpu_io2_protocol_and_wrapper() {
    assert!(!EFI_CPU_IO2_PROTOCOL_GUID.is_zero());

    let proto = EfiCpuIo2Protocol {
        mem: EfiCpuIo2ProtocolAccess {
            read: mock_mem_read,
            write: mock_mem_write,
        },
        io: EfiCpuIo2ProtocolAccess {
            read: mock_io_read,
            write: mock_io_write,
        },
    };

    let wrapper = CpuIo2::new(&proto);
    assert_eq!(wrapper.read_mem_u32(0x1000), Ok(0x1234_5678));
    assert_eq!(wrapper.write_mem_u32(0x1000, 0x8765_4321), Ok(()));
    assert_eq!(wrapper.read_mem_u32(0x1000), Ok(0x8765_4321));

    assert_eq!(wrapper.read_io_u16(0x80), Ok(0xA5A5));
    assert_eq!(wrapper.write_io_u16(0x80, 0x5A5A), Ok(()));
    assert_eq!(wrapper.read_io_u16(0x80), Ok(0x5A5A));
}
