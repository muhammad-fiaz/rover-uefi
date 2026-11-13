//! # SMM / MM Access2 Protocol
//!
//! Controls the visibility, access rights, and locking state of SMRAM / MMRAM regions.

use crate::base::{EfiGuid, EfiStatus};

/// SMM/MM Access2 Protocol GUID: `{c2702b74-800c-4131-8746-8fb5b89ce4ac}`.
pub const SMM_ACCESS2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xc270_2b74,
    0x800c,
    0x4131,
    [0x87, 0x46, 0x8f, 0xb5, 0xb8, 0x9c, 0xe4, 0xac],
);

/// SMRAM Region State: SMRAM is open and accessible by non-SMM CPU cycles.
pub const SMRAM_OPEN: u64 = 0x0000_0001;
/// SMRAM Region State: SMRAM is closed to non-SMM CPU cycles.
pub const SMRAM_CLOSED: u64 = 0x0000_0002;
/// SMRAM Region State: SMRAM configuration is locked and cannot be modified.
pub const SMRAM_LOCKED: u64 = 0x0000_0004;
/// SMRAM Region State: SMRAM is cacheable.
pub const SMRAM_CACHEABLE: u64 = 0x0000_0008;
/// SMRAM Region State: SMRAM is allocated for use by SMM infrastructure.
pub const SMRAM_ALLOCATED: u64 = 0x0000_0010;

/// Describes a contiguous region of System Management RAM (SMRAM).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiSmramDescriptor {
    /// Physical address of SMRAM range.
    pub physical_start: u64,
    /// CPU bus address of SMRAM range.
    pub cpu_start: u64,
    /// Size of SMRAM range in bytes.
    pub physical_size: u64,
    /// Bitmask of SMRAM region attributes and state flags.
    pub region_state: u64,
}

/// Function pointer for `EFI_MM_OPEN`.
pub type EfiSmmOpen2 = unsafe extern "efiapi" fn(this: *mut EfiSmmAccess2Protocol) -> EfiStatus;

/// Function pointer for `EFI_MM_CLOSE`.
pub type EfiSmmClose2 = unsafe extern "efiapi" fn(this: *mut EfiSmmAccess2Protocol) -> EfiStatus;

/// Function pointer for `EFI_MM_LOCK`.
pub type EfiSmmLock2 = unsafe extern "efiapi" fn(this: *mut EfiSmmAccess2Protocol) -> EfiStatus;

/// Function pointer for `EFI_MM_CAPABILITIES`.
pub type EfiSmmCapabilities2 = unsafe extern "efiapi" fn(
    this: *const EfiSmmAccess2Protocol,
    smram_map_size: *mut usize,
    smram_map: *mut EfiSmramDescriptor,
) -> EfiStatus;

/// `EFI_SMM_ACCESS2_PROTOCOL` interface table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSmmAccess2Protocol {
    /// Opens the SMRAM area so that it is visible while outside SMM.
    pub open: EfiSmmOpen2,
    /// Closes the SMRAM area so that it is not visible while outside SMM.
    pub close: EfiSmmClose2,
    /// Locks the SMRAM area against future tampering.
    pub lock: EfiSmmLock2,
    /// Queries the capabilities and memory map descriptors of SMRAM.
    pub get_capabilities: EfiSmmCapabilities2,
    /// Indicates whether SMRAM is locked.
    pub lock_state: bool,
    /// Indicates whether SMRAM is open.
    pub open_state: bool,
}

/// Safe client wrapper around the SMM Access2 Protocol.
#[derive(Debug, Clone, Copy)]
pub struct SmmAccess2 {
    proto: *mut EfiSmmAccess2Protocol,
}

impl SmmAccess2 {
    /// Wraps an SMM Access2 protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiSmmAccess2Protocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiSmmAccess2Protocol) -> Self {
        Self { proto }
    }

    /// Opens SMRAM visibility to all CPU cycles.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if opening is unsupported.
    /// Returns `EfiStatus::DEVICE_ERROR` if SMRAM is locked.
    pub fn open(&self) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).open;
            func(self.proto)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Closes SMRAM visibility so it is inaccessible outside SMM.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if closing is unsupported.
    /// Returns `EfiStatus::DEVICE_ERROR` if SMRAM cannot be closed.
    pub fn close(&self) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).close;
            func(self.proto)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Locks SMRAM against further configuration changes.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if locking is unsupported.
    pub fn lock(&self) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).lock;
            func(self.proto)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns whether SMRAM is currently locked.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        unsafe { (*self.proto).lock_state }
    }

    /// Returns whether SMRAM is currently open.
    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { (*self.proto).open_state }
    }

    /// Queries the SMRAM memory descriptors from the chipset controller.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
    pub fn get_capabilities<'a>(
        &self,
        buffer: &'a mut [EfiSmramDescriptor],
    ) -> Result<&'a [EfiSmramDescriptor], EfiStatus> {
        let mut map_size = core::mem::size_of_val(buffer);
        let status = unsafe {
            let func = (*self.proto).get_capabilities;
            func(self.proto, &raw mut map_size, buffer.as_mut_ptr())
        };
        if status == EfiStatus::SUCCESS {
            let entries = map_size / core::mem::size_of::<EfiSmramDescriptor>();
            Ok(&buffer[..entries.min(buffer.len())])
        } else {
            Err(status)
        }
    }
}
