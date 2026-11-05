//! # Erase Block Protocol
//!
//! Defined by the UEFI Specification Section 13.11: Erase Block Protocol.
//!
//! Provides fast block-level erase and TRIM capabilities for flash and SSD devices.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};

/// The `EFI_ERASE_BLOCK_PROTOCOL_GUID`.
pub const EFI_ERASE_BLOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9595_0386,
    0x4D2C,
    0x47E2,
    [0x9E, 0x48, 0xBA, 0x48, 0xD0, 0xAB, 0x80, 0x4E],
);

/// Token for asynchronous block erase operations.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiEraseBlockToken {
    /// Event signaled when the erase operation finishes.
    pub event: EfiEvent,
    /// Transaction return status code.
    pub transaction_status: EfiStatus,
}

/// Function pointer for `EFI_ERASE_BLOCK_PROTOCOL.EraseBlocks`.
pub type EfiEraseBlocks = unsafe extern "efiapi" fn(
    this: *const EfiEraseBlockProtocol,
    media_id: u32,
    lba: u64,
    token: *mut EfiEraseBlockToken,
    size: UIntN,
) -> EfiStatus;

/// The raw ABI structure for `EFI_ERASE_BLOCK_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEraseBlockProtocol {
    /// Protocol revision.
    pub revision: u64,
    /// Number of blocks that must be erased simultaneously.
    pub erase_length_granularity: u32,
    /// Erases specified block range.
    pub erase_blocks: EfiEraseBlocks,
}
