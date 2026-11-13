//! # Memory Accept Protocol and Client API
//!
//! Provides the interface for memory services to accept memory in Confidential
//! Computing environments (such as Intel `TDX` and AMD `SEV-SNP`) where unaccepted
//! host pages require explicit cryptographic validation before access.

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// Memory Accept Protocol GUID.
pub const MEMORY_ACCEPT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x38C7_4800,
    0x5590,
    0x4DB4,
    [0xA0, 0xF3, 0x67, 0x5D, 0x9B, 0x8E, 0x80, 0x26],
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
