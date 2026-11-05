//! # CPU I/O 2 Protocol
//!
//! Defined by the UEFI Specification Section 13.7: CPU I/O 2 Protocol.
//!
//! Provides memory-mapped I/O and I/O port space access primitives for drivers
//! and firmware services.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_CPU_IO2_PROTOCOL_GUID`.
pub const EFI_CPU_IO2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAD61_2702,
    0x56E4,
    0x47E5,
    [0xBE, 0x48, 0x82, 0x76, 0x22, 0x98, 0x92, 0x5B],
);

/// Width of the memory or I/O operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiCpuIoProtocolWidth {
    /// 8-bit unsigned integer.
    EfiCpuIoWidthUint8 = 0,
    /// 16-bit unsigned integer.
    EfiCpuIoWidthUint16 = 1,
    /// 32-bit unsigned integer.
    EfiCpuIoWidthUint32 = 2,
    /// 64-bit unsigned integer.
    EfiCpuIoWidthUint64 = 3,
    /// 8-bit FIFO transfer.
    EfiCpuIoWidthFifoUint8 = 4,
    /// 16-bit FIFO transfer.
    EfiCpuIoWidthFifoUint16 = 5,
    /// 32-bit FIFO transfer.
    EfiCpuIoWidthFifoUint32 = 6,
    /// 64-bit FIFO transfer.
    EfiCpuIoWidthFifoUint64 = 7,
    /// 8-bit Fill operation.
    EfiCpuIoWidthFillUint8 = 8,
    /// 16-bit Fill operation.
    EfiCpuIoWidthFillUint16 = 9,
    /// 32-bit Fill operation.
    EfiCpuIoWidthFillUint32 = 10,
    /// 64-bit Fill operation.
    EfiCpuIoWidthFillUint64 = 11,
    /// Maximum width enumeration count.
    EfiCpuIoWidthMaximum = 12,
}

/// Function pointer for reading and writing memory or I/O spaces.
pub type EfiCpuIo2Access = unsafe extern "efiapi" fn(
    this: *const EfiCpuIo2Protocol,
    width: EfiCpuIoProtocolWidth,
    address: u64,
    count: UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// Raw ABI structure for `EFI_CPU_IO2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiCpuIo2Protocol {
    /// Memory space access service (read and write).
    pub mem: EfiCpuIo2ProtocolAccess,
    /// I/O port space access service (read and write).
    pub io: EfiCpuIo2ProtocolAccess,
}

/// Structure containing read and write function pointers for an access domain.
#[derive(Debug)]
#[repr(C)]
pub struct EfiCpuIo2ProtocolAccess {
    /// Read transaction from memory or I/O port.
    pub read: EfiCpuIo2Access,
    /// Write transaction to memory or I/O port.
    pub write: EfiCpuIo2Access,
}

/// Safe typed wrapper around `EFI_CPU_IO2_PROTOCOL`.
pub struct CpuIo2<'a> {
    protocol: &'a EfiCpuIo2Protocol,
}

impl<'a> CpuIo2<'a> {
    /// Creates a new view borrowing the protocol instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiCpuIo2Protocol) -> Self {
        Self { protocol }
    }

    /// Reads a 32-bit value from memory space.
    ///
    /// # Errors
    /// Returns an error if the read operation fails.
    pub fn read_mem_u32(&self, address: u64) -> Result<u32, EfiStatus> {
        let mut value = 0_u32;
        // SAFETY: The protocol reference is valid and pointer targets a local u32.
        let status = unsafe {
            (self.protocol.mem.read)(
                self.protocol,
                EfiCpuIoProtocolWidth::EfiCpuIoWidthUint32,
                address,
                1,
                core::ptr::from_mut(&mut value).cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(value)
        }
    }

    /// Writes a 32-bit value to memory space.
    ///
    /// # Errors
    /// Returns an error if the write operation fails.
    pub fn write_mem_u32(&self, address: u64, value: u32) -> Result<(), EfiStatus> {
        let mut val = value;
        // SAFETY: The protocol reference is valid.
        let status = unsafe {
            (self.protocol.mem.write)(
                self.protocol,
                EfiCpuIoProtocolWidth::EfiCpuIoWidthUint32,
                address,
                1,
                core::ptr::from_mut(&mut val).cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Reads a 16-bit value from I/O port space.
    ///
    /// # Errors
    /// Returns an error if the I/O read operation fails.
    pub fn read_io_u16(&self, port: u64) -> Result<u16, EfiStatus> {
        let mut value = 0_u16;
        // SAFETY: The protocol reference is valid and pointer targets a local u16.
        let status = unsafe {
            (self.protocol.io.read)(
                self.protocol,
                EfiCpuIoProtocolWidth::EfiCpuIoWidthUint16,
                port,
                1,
                core::ptr::from_mut(&mut value).cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(value)
        }
    }

    /// Writes a 16-bit value to I/O port space.
    ///
    /// # Errors
    /// Returns an error if the I/O write operation fails.
    pub fn write_io_u16(&self, port: u64, value: u16) -> Result<(), EfiStatus> {
        let mut val = value;
        // SAFETY: The protocol reference is valid.
        let status = unsafe {
            (self.protocol.io.write)(
                self.protocol,
                EfiCpuIoProtocolWidth::EfiCpuIoWidthUint16,
                port,
                1,
                core::ptr::from_mut(&mut val).cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
