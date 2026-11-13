//! # CXL I/O Protocol
//!
//! Compute Express Link (CXL) I/O protocol definitions for CXL device interaction.

use crate::abi::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::pci::access::EfiPciIoProtocol;
use core::ffi::c_void;

/// The CXL I/O protocol GUID.
pub const CXL_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8FAC_60B2,
    0xBBC1,
    0x41DE,
    [0xA7, 0x97, 0x98, 0x75, 0xCD, 0xD3, 0xA8, 0xF8],
);

/// Regblock accessors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CxlIoProtocolRegblockAccess {
    /// Read CXL regblock.
    pub read: unsafe extern "efiapi" fn(
        this: *mut EfiCxlIoProtocol,
        reg_block: u32,
        offset: u64,
        buf: *mut c_void,
        size: UIntN,
    ) -> EfiStatus,
    /// Write CXL regblock.
    pub write: unsafe extern "efiapi" fn(
        this: *mut EfiCxlIoProtocol,
        reg_block: u32,
        offset: u64,
        buf: *const c_void,
        size: UIntN,
    ) -> EfiStatus,
}

/// DVSEC accessors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CxlIoProtocolDvsecAccess {
    /// Read CXL DVSEC.
    pub read: unsafe extern "efiapi" fn(
        this: *mut EfiCxlIoProtocol,
        cxl_dvsec_id: u32,
        offset: u32,
        buf: *mut c_void,
        size: UIntN,
    ) -> EfiStatus,
    /// Write CXL DVSEC.
    pub write: unsafe extern "efiapi" fn(
        this: *mut EfiCxlIoProtocol,
        cxl_dvsec_id: u32,
        offset: u32,
        buf: *const c_void,
        size: UIntN,
    ) -> EfiStatus,
}

/// Raw CXL I/O protocol interface.
#[repr(C)]
pub struct EfiCxlIoProtocol {
    /// Pointer to associated `EFI_PCI_IO_PROTOCOL`.
    pub pci_io: *mut EfiPciIoProtocol,
    /// Data object exchange (DOE) request-response transaction.
    pub doe_transact: unsafe extern "efiapi" fn(
        this: *mut EfiCxlIoProtocol,
        request_buffer: *const c_void,
        request_buffer_size: UIntN,
        response_buffer: *mut c_void,
        response_buffer_size: *mut UIntN,
    ) -> EfiStatus,
    /// Regblock register access.
    pub regblock: CxlIoProtocolRegblockAccess,
    /// DVSEC register access.
    pub dvsec: CxlIoProtocolDvsecAccess,
}

/// Safe wrapper around `EfiCxlIoProtocol`.
pub struct CxlIoProtocol<'a> {
    raw: &'a mut EfiCxlIoProtocol,
}

impl<'a> CxlIoProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = CXL_IO_PROTOCOL_GUID;

    /// Creates a safe wrapper from a mutable raw protocol pointer.
    #[must_use]
    pub fn new(raw: &'a mut EfiCxlIoProtocol) -> Self {
        Self { raw }
    }

    /// Read from a CXL regblock into `buffer`.
    ///
    /// # Errors
    /// Returns an error if reading the regblock registers fails.
    pub fn read_regblock(
        &mut self,
        reg_block: u32,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.regblock.read)(
                self.raw,
                reg_block,
                offset,
                buffer.as_mut_ptr().cast::<c_void>(),
                buffer.len(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Write from `buffer` to a CXL regblock.
    ///
    /// # Errors
    /// Returns an error if writing to the regblock registers fails.
    pub fn write_regblock(
        &mut self,
        reg_block: u32,
        offset: u64,
        buffer: &[u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.regblock.write)(
                self.raw,
                reg_block,
                offset,
                buffer.as_ptr().cast::<c_void>(),
                buffer.len(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Read from a CXL DVSEC into `buffer`.
    ///
    /// # Errors
    /// Returns an error if reading DVSEC registers fails.
    pub fn read_dvsec(
        &mut self,
        cxl_dvsec_id: u32,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.dvsec.read)(
                self.raw,
                cxl_dvsec_id,
                offset,
                buffer.as_mut_ptr().cast::<c_void>(),
                buffer.len(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Write from `buffer` to a CXL DVSEC.
    ///
    /// # Errors
    /// Returns an error if writing to DVSEC registers fails.
    pub fn write_dvsec(
        &mut self,
        cxl_dvsec_id: u32,
        offset: u32,
        buffer: &[u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.dvsec.write)(
                self.raw,
                cxl_dvsec_id,
                offset,
                buffer.as_ptr().cast::<c_void>(),
                buffer.len(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Performs a Data Object Exchange (DOE) transaction.
    ///
    /// # Errors
    /// Returns an error if the transaction fails or the device does not support DOE.
    pub fn doe_transact(
        &mut self,
        request: &[u8],
        response: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut resp_len = response.len();
        let req_ptr = if request.is_empty() {
            core::ptr::null()
        } else {
            request.as_ptr().cast::<c_void>()
        };
        let status = unsafe {
            (self.raw.doe_transact)(
                self.raw,
                req_ptr,
                request.len(),
                response.as_mut_ptr().cast::<c_void>(),
                &raw mut resp_len,
            )
        };
        if status.is_success() {
            Ok(resp_len)
        } else {
            Err(status)
        }
    }
}
