//! # LockBox Protocol and SMM LockBox Communication
//!
//! Provides services to preserve confidential firmware buffers across ACPI S3
//! sleep and resume transitions, with optional in-place restoration and S3-only access gating.

#![allow(clippy::doc_markdown)]

use crate::base::{EfiGuid, EfiStatus};

/// LockBox Protocol GUID: `{bd445d79-b7ad-4f04-9ad8-29bd2040eb3c}`.
///
/// Published to indicate that the LockBox service is active and ready to accept calls.
pub const LOCK_BOX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBD44_5D79,
    0xB7AD,
    0x4F04,
    [0x9A, 0xD8, 0x29, 0xBD, 0x20, 0x40, 0xEB, 0x3C],
);

/// SMM LockBox Communication GUID: `{2a3cfebd-27e8-4d0a-8b79-d688c2a3e1c0}`.
pub const SMM_LOCK_BOX_COMMUNICATION_GUID: EfiGuid = EfiGuid::new(
    0x2A3C_FEBD,
    0x27E8,
    0x4D0A,
    [0x8B, 0x79, 0xD6, 0x88, 0xC2, 0xA3, 0xE1, 0xC0],
);

/// Allows restoring this LockBox buffer to its original physical address via `restore_all_in_place`.
pub const LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE: u64 = 1 << 0;

/// Limits restoration of this LockBox buffer strictly to the ACPI S3 resume phase.
pub const LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY: u64 = 1 << 1;

/// Command ID: Save confidential buffer into LockBox.
pub const SMM_LOCK_BOX_COMMAND_SAVE: u32 = 0x0000_0001;

/// Command ID: Update existing LockBox buffer at specified offset.
pub const SMM_LOCK_BOX_COMMAND_UPDATE: u32 = 0x0000_0002;

/// Command ID: Restore confidential buffer from LockBox.
pub const SMM_LOCK_BOX_COMMAND_RESTORE: u32 = 0x0000_0003;

/// Command ID: Set flags/attributes on an existing LockBox entry.
pub const SMM_LOCK_BOX_COMMAND_SET_ATTRIBUTES: u32 = 0x0000_0004;

/// Command ID: Restore all LockBox entries configured with `RESTORE_IN_PLACE`.
pub const SMM_LOCK_BOX_COMMAND_RESTORE_ALL_IN_PLACE: u32 = 0x0000_0005;

/// Header for SMM LockBox communication payloads.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterHeader {
    /// Command identifier (`SMM_LOCK_BOX_COMMAND_*`).
    pub command: u32,
    /// Size of the payload following this header in bytes.
    pub data_length: u32,
    /// Status code returned by the SMM handler.
    pub return_status: u64,
}

/// SMM LockBox parameter payload for saving confidential information.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterSave {
    /// Command header.
    pub header: SmmLockBoxParameterHeader,
    /// Identifier GUID for the confidential entry.
    pub guid: EfiGuid,
    /// Physical address of the confidential source buffer.
    pub buffer: u64,
    /// Length of the source buffer in bytes.
    pub length: u64,
}

/// SMM LockBox parameter payload for updating an existing entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterUpdate {
    /// Command header.
    pub header: SmmLockBoxParameterHeader,
    /// Identifier GUID for the entry to update.
    pub guid: EfiGuid,
    /// Byte offset within the target LockBox entry.
    pub offset: u64,
    /// Physical address of the replacement data buffer.
    pub buffer: u64,
    /// Length of the replacement data in bytes.
    pub length: u64,
}

/// SMM LockBox parameter payload for restoring confidential information.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterRestore {
    /// Command header.
    pub header: SmmLockBoxParameterHeader,
    /// Identifier GUID for the entry to restore.
    pub guid: EfiGuid,
    /// Destination physical address (0 indicates restoring to original physical address).
    pub buffer: u64,
    /// Maximum destination buffer size on input; actual restored size on output.
    pub length: u64,
}

/// SMM LockBox parameter payload for configuring entry attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterSetAttributes {
    /// Command header.
    pub header: SmmLockBoxParameterHeader,
    /// Identifier GUID for the entry.
    pub guid: EfiGuid,
    /// New bitmask attributes (`LOCK_BOX_ATTRIBUTE_*`).
    pub attributes: u64,
}

/// SMM LockBox parameter payload for restoring all in-place entries.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmLockBoxParameterRestoreAllInPlace {
    /// Command header.
    pub header: SmmLockBoxParameterHeader,
}

/// Represents an individual LockBox entry descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LockBoxDescriptor {
    /// Unique GUID identifying the preserved buffer.
    pub guid: EfiGuid,
    /// Physical base address.
    pub buffer_address: u64,
    /// Buffer size in bytes.
    pub size: usize,
    /// Protection and restoration attributes.
    pub attributes: u64,
}

impl LockBoxDescriptor {
    /// Creates a new descriptor.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if `size` is zero or if incompatible restoration
    /// flags (`RESTORE_IN_PLACE` and `RESTORE_IN_S3_ONLY`) are simultaneously specified.
    pub const fn new(
        guid: EfiGuid,
        buffer_address: u64,
        size: usize,
        attributes: u64,
    ) -> Result<Self, EfiStatus> {
        if size == 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        if (attributes & LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE != 0)
            && (attributes & LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY != 0)
        {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        Ok(Self {
            guid,
            buffer_address,
            size,
            attributes,
        })
    }

    /// Returns true if this entry must only be restored during ACPI S3 resume.
    #[must_use]
    pub const fn is_s3_only(&self) -> bool {
        (self.attributes & LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY) != 0
    }

    /// Returns true if this entry can be restored directly in place.
    #[must_use]
    pub const fn is_restore_in_place(&self) -> bool {
        (self.attributes & LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE) != 0
    }
}
