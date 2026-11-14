//! # Memory Accept Protocol and Client API
//!
//! Provides the interface for memory services to accept memory in Confidential
//! Computing environments (such as Intel `TDX` and AMD `SEV-SNP`) where unaccepted
//! host pages require explicit cryptographic validation before access.
//! Also provides the OVMF SEV Memory Acceptance Protocol allowing OS loaders
//! to disable unconditional page acceptance upon `ExitBootServices`.

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// Memory Accept Protocol GUID: `{38c74800-5590-4db4-a0f3-675d9b8e8026}`.
pub const MEMORY_ACCEPT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x38C7_4800,
    0x5590,
    0x4DB4,
    [0xA0, 0xF3, 0x67, 0x5D, 0x9B, 0x8E, 0x80, 0x26],
);

/// OVMF SEV Memory Acceptance Protocol GUID: `{c5a010fe-38a7-4531-8a4a-0500d2fd1649}`.
pub const OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xC5A0_10FE,
    0x38A7,
    0x4531,
    [0x8A, 0x4A, 0x05, 0x00, 0xD2, 0xFD, 0x16, 0x49],
);

/// Function pointer to accept physical memory.
pub type AcceptMemory = unsafe extern "efiapi" fn(
    this: *const MemoryAcceptProtocol,
    start_address: EfiPhysicalAddress,
    size: usize,
) -> EfiStatus;

/// Memory Accept Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryAcceptProtocol {
    /// Accept designated memory region.
    pub accept_memory: Option<AcceptMemory>,
}

/// Safe client wrapper for memory acceptance operations.
#[derive(Debug)]
pub struct MemoryAccept<'a> {
    protocol: &'a MemoryAcceptProtocol,
}

impl<'a> MemoryAccept<'a> {
    /// Create a new safe view of the memory accept protocol.
    #[must_use]
    pub fn new(protocol: &'a MemoryAcceptProtocol) -> Self {
        Self { protocol }
    }

    /// Cryptographically accept a physical memory range.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn accept(&self, start: EfiPhysicalAddress, size: usize) -> Result<(), EfiStatus> {
        if let Some(accept_fn) = self.protocol.accept_memory {
            let status = unsafe { accept_fn(self.protocol, start, size) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}

/// Function pointer to indicate OS loader support for unaccepted memory.
pub type OvmfSevAllowUnacceptedMemory =
    unsafe extern "efiapi" fn(this: *const OvmfSevMemoryAcceptanceProtocol) -> EfiStatus;

/// OVMF SEV Memory Acceptance Protocol ABI structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OvmfSevMemoryAcceptanceProtocol {
    /// Allows the OS loader to indicate to firmware that `ExitBootServices`
    /// should not unconditionally accept all memory pages.
    pub allow_unaccepted_memory: Option<OvmfSevAllowUnacceptedMemory>,
}

/// Safe client wrapper for OVMF SEV Memory Acceptance operations.
#[derive(Debug)]
pub struct SevMemoryAcceptance<'a> {
    protocol: &'a OvmfSevMemoryAcceptanceProtocol,
}

impl<'a> SevMemoryAcceptance<'a> {
    /// Creates a new safe view around the protocol.
    #[must_use]
    pub fn new(protocol: &'a OvmfSevMemoryAcceptanceProtocol) -> Self {
        Self { protocol }
    }

    /// Indicates to firmware that the OS supports unaccepted memory.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the firmware error code.
    pub fn allow_unaccepted_memory(&self) -> Result<(), EfiStatus> {
        if let Some(allow_fn) = self.protocol.allow_unaccepted_memory {
            let status = unsafe { allow_fn(self.protocol) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
