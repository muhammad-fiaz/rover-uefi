//! # Management Mode (MM) and SMM Base protocol
//!
//! Defined in PI Specification Volume 4: Management Mode Core Interface (PI 1.5+)
//! and previously PI 1.2 for SMM Base2.
//!
//! Utilized by MM/SMM drivers to locate the MM infrastructure services and
//! determine whether the driver is being executed inside MMRAM or outside MMRAM.

use core::ffi::c_void;

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiStatus};

/// The MM Base protocol `GUID`.
pub const MM_BASE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xF4CC_BFB7,
    0xF6E0,
    0x47FD,
    [0x9D, 0xD4, 0x10, 0xA8, 0xF1, 0x50, 0xC1, 0x91],
);

/// The SMM Base2 protocol `GUID` (identical to [`MM_BASE_PROTOCOL_GUID`]).
pub const SMM_BASE2_PROTOCOL_GUID: EfiGuid = MM_BASE_PROTOCOL_GUID;

/// Checks whether the driver is currently executing inside MMRAM.
pub type EfiMmInsideOut =
    unsafe extern "efiapi" fn(*const EfiMmBaseProtocol, *mut Bool) -> EfiStatus;

/// Retrieves the pointer to the Management Mode Service Table (MMST).
pub type EfiMmGetMmstLocation =
    unsafe extern "efiapi" fn(*const EfiMmBaseProtocol, *mut *mut c_void) -> EfiStatus;

/// The raw MM Base protocol interface (`EFI_MM_BASE_PROTOCOL` / `EFI_SMM_BASE2_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiMmBaseProtocol {
    /// Service to indicate whether the driver is currently executing in MMRAM.
    pub in_mm: EfiMmInsideOut,
    /// Returns the location of the Management Mode System Table (MMST).
    pub get_mmst_location: EfiMmGetMmstLocation,
}

/// Type alias for `EFI_SMM_BASE2_PROTOCOL`.
pub type EfiSmmBase2Protocol = EfiMmBaseProtocol;

/// A safe wrapper for [`EfiMmBaseProtocol`].
pub struct MmBaseProtocol<'a> {
    protocol: &'a EfiMmBaseProtocol,
}

impl<'a> MmBaseProtocol<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiMmBaseProtocol) -> Self {
        Self { protocol }
    }

    /// Returns a reference to the raw protocol structure.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiMmBaseProtocol {
        self.protocol
    }

    /// Queries whether execution is currently inside MMRAM.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying protocol call fails.
    pub fn in_mm(&self) -> Result<bool, EfiStatus> {
        let mut in_mmram: Bool = 0;
        let status = unsafe { (self.protocol.in_mm)(self.protocol, &raw mut in_mmram) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(in_mmram != 0)
        }
    }

    /// Returns the pointer to the Management Mode System Table (MMST).
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying protocol call fails (e.g., if called
    /// outside MM/SMM).
    pub fn get_mmst_location(&self) -> Result<*mut c_void, EfiStatus> {
        let mut mmst: *mut c_void = core::ptr::null_mut();
        let status = unsafe { (self.protocol.get_mmst_location)(self.protocol, &raw mut mmst) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(mmst)
        }
    }
}

impl core::fmt::Debug for MmBaseProtocol<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MmBaseProtocol")
            .field("protocol", &self.protocol)
            .finish()
    }
}
