//! # Android Verified Boot Protocol and Client API
//!
//! Provides board-specific cryptographic validation, lock states, tamper detection,
//! boot color signaling (Green, Yellow, Orange, Red), and roll-back protection index
//! management for Android Verified Boot (`AVB`).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// `GBL` EFI `AVB` Protocol GUID.
pub const GBL_EFI_AVB_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6BC6_6B9A,
    0xD5C9,
    0x4C02,
    [0x9D, 0xA9, 0x50, 0xAF, 0x19, 0x8D, 0x91, 0x2C],
);

/// `GBL` EFI `AVB` Protocol Revision (1.0).
pub const GBL_EFI_AVB_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// `AVB` Device Status bitmask flags.
pub mod device_status {
    /// Bootloader is unlocked.
    pub const UNLOCKED: u64 = 1 << 0;
    /// `dm-verity` cryptographic verification has failed.
    pub const DM_VERITY_FAILED: u64 = 1 << 1;
    /// Critical partitions are unlocked.
    pub const UNLOCKED_CRITICAL: u64 = 1 << 2;
    /// Device is unlockable by user action.
    pub const UNLOCKABLE: u64 = 1 << 3;
}

/// `AVB` Boot Color state bitmask flags.
pub mod boot_color {
    /// Red boot state: unverified image or broken chain of trust.
    pub const RED: u64 = 1 << 0;
    /// Orange boot state: unlocked device bootloader.
    pub const ORANGE: u64 = 1 << 1;
    /// Yellow boot state: verified with custom self-signed root key.
    pub const YELLOW: u64 = 1 << 2;
    /// Green boot state: full factory chain of trust verified.
    pub const GREEN: u64 = 1 << 3;
    /// Red `EIO` boot state: verification failed with I/O error.
    pub const RED_EIO: u64 = 1 << 4;
}

/// `AVB` Partition bitmask flags.
pub mod partition_flag {
    /// Cryptographic verification required.
    pub const VERIFY: u64 = 1 << 0;
    /// Verify partition if present on storage.
    pub const VERIFY_IF_EXISTS: u64 = 1 << 1;
    /// Flashing requires critical partition unlock.
    pub const FLASH_CRITICAL: u64 = 1 << 2;
    /// Wipe during factory data reset (`FDR`).
    pub const FDR: u64 = 1 << 3;
}

/// Key validation status.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiAvbKeyValidationStatus {
    /// Public key is invalid or untrusted.
    Invalid = 0,
    /// Public key matches user-enrolled custom certificate.
    ValidCustomKey = 1,
    /// Public key matches factory embedded root of trust.
    Valid = 2,
}

/// Lock type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiAvbLockType {
    /// General device bootloader lock.
    Device = 0,
    /// Critical partitions lock.
    Critical = 1,
}

/// Lock state.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiAvbLockState {
    /// Partition or device is unlocked.
    Unlocked = 0,
    /// Partition or device is locked.
    Locked = 1,
}

/// Function pointer to validate an `AVB` public key.
pub type GblEfiAvbValidatePublicKey = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvbProtocol,
    public_key_len: usize,
    public_key: *const u8,
    public_key_metadata_len: usize,
    public_key_metadata: *const u8,
    status: *mut GblEfiAvbKeyValidationStatus,
) -> EfiStatus;

/// Function pointer to read a rollback index.
pub type GblEfiAvbReadRollbackIndex = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvbProtocol,
    rollback_index_location: usize,
    rollback_index: *mut u64,
) -> EfiStatus;

/// Function pointer to write a rollback index.
pub type GblEfiAvbWriteRollbackIndex = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvbProtocol,
    rollback_index_location: usize,
    rollback_index: u64,
) -> EfiStatus;

/// Function pointer to read device lock state.
pub type GblEfiAvbReadLockState = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvbProtocol,
    lock_type: GblEfiAvbLockType,
    lock_state: *mut GblEfiAvbLockState,
) -> EfiStatus;

/// Function pointer to write device lock state.
pub type GblEfiAvbWriteLockState = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvbProtocol,
    lock_type: GblEfiAvbLockType,
    lock_state: GblEfiAvbLockState,
) -> EfiStatus;

/// Google Bootloader `AVB` Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiAvbProtocol {
    /// Revision number of the protocol.
    pub revision: u64,
    /// Validate public key against root of trust.
    pub validate_public_key: Option<GblEfiAvbValidatePublicKey>,
    /// Read rollback protection index.
    pub read_rollback_index: Option<GblEfiAvbReadRollbackIndex>,
    /// Write rollback protection index.
    pub write_rollback_index: Option<GblEfiAvbWriteRollbackIndex>,
    /// Read partition or device lock state.
    pub read_lock_state: Option<GblEfiAvbReadLockState>,
    /// Write partition or device lock state.
    pub write_lock_state: Option<GblEfiAvbWriteLockState>,
    /// Custom reserved pointers for platform-specific security extensions.
    pub reserved: [*const c_void; 4],
}

/// Safe client wrapper for Android Verified Boot operations.
#[derive(Debug)]
pub struct Avb<'a> {
    protocol: &'a mut GblEfiAvbProtocol,
}

impl<'a> Avb<'a> {
    /// Create a new safe view of the `AVB` protocol.
    #[must_use]
    pub fn new(protocol: &'a mut GblEfiAvbProtocol) -> Self {
        Self { protocol }
    }

    /// Read the device lock state.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn read_lock_state(
        &mut self,
        lock_type: GblEfiAvbLockType,
    ) -> Result<GblEfiAvbLockState, EfiStatus> {
        if let Some(read_fn) = self.protocol.read_lock_state {
            let mut state = GblEfiAvbLockState::Unlocked;
            let status = unsafe { read_fn(self.protocol, lock_type, &raw mut state) };
            if status == EfiStatus::SUCCESS {
                Ok(state)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Read the rollback index at the specified location.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn read_rollback_index(&mut self, location: usize) -> Result<u64, EfiStatus> {
        if let Some(read_fn) = self.protocol.read_rollback_index {
            let mut index = 0u64;
            let status = unsafe { read_fn(self.protocol, location, &raw mut index) };
            if status == EfiStatus::SUCCESS {
                Ok(index)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
