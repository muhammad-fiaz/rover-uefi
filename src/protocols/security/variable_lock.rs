//! # Variable Lock Protocol
//!
//! Provides services to mark UEFI variables as read-only prior to boot services exit.
//!
//! Once locked, variables cannot be modified or deleted by standard runtime callers.

use crate::base::{EfiGuid, EfiStatus};

/// Variable Lock Protocol GUID: `{cd3d0a05-9e24-437c-a891-1ee053db7638}`.
pub const VARIABLE_LOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCD3D_0A05,
    0x9E24,
    0x437C,
    [0xA8, 0x91, 0x1E, 0xE0, 0x53, 0xDB, 0x76, 0x38],
);

/// Function pointer for `RequestToLock`.
pub type EfiVariableLockRequestToLock = unsafe extern "efiapi" fn(
    this: *const EfiVariableLockProtocol,
    variable_name: *const u16,
    vendor_guid: *const EfiGuid,
) -> EfiStatus;

/// Low-level Variable Lock Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiVariableLockProtocol {
    /// Marks a variable to become read-only upon leaving DXE phase.
    pub request_to_lock: EfiVariableLockRequestToLock,
}

/// Safe client wrapper around the Variable Lock Protocol.
#[derive(Debug, Clone, Copy)]
pub struct VariableLock {
    proto: *const EfiVariableLockProtocol,
}

impl VariableLock {
    /// Wraps a Variable Lock protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiVariableLockProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiVariableLockProtocol) -> Self {
        Self { proto }
    }

    /// Requests that a variable be marked read-only.
    ///
    /// `variable_name` must be a null-terminated UTF-16 string slice.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if name is empty or invalid.
    /// Returns `EfiStatus::ACCESS_DENIED` if ready-to-boot has already signaled.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory is exhausted.
    pub fn request_to_lock(
        &self,
        variable_name: &[u16],
        vendor_guid: &EfiGuid,
    ) -> Result<(), EfiStatus> {
        if variable_name.is_empty() || variable_name.last() != Some(&0) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }

        let status = unsafe {
            let func = (*self.proto).request_to_lock;
            func(
                self.proto,
                variable_name.as_ptr(),
                core::ptr::from_ref(vendor_guid),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
