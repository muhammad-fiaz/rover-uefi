//! # SMBIOS Protocol
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 2:
//! Driver Execution Environment Core Interface.
//!
//! Provides the primary runtime service to add, modify, enumerate, and remove
//! SMBIOS table structures.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::smbios::header::SmbiosHeader;
use core::ffi::c_void;

/// The `EFI_SMBIOS_PROTOCOL_GUID`.
pub const EFI_SMBIOS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0358_3FF6,
    0x0B5E,
    0x436E,
    [0xA0, 0xE1, 0x44, 0x32, 0xB8, 0x29, 0x58, 0x95],
);

/// Function pointer for `EFI_SMBIOS_ADD`.
pub type EfiSmbiosAdd = unsafe extern "efiapi" fn(
    this: *const EfiSmbiosProtocol,
    producer_handle: *const c_void,
    smbios_handle: *mut u16,
    record: *const SmbiosHeader,
) -> EfiStatus;

/// Function pointer for `EFI_SMBIOS_UPDATE_STRING`.
pub type EfiSmbiosUpdateString = unsafe extern "efiapi" fn(
    this: *const EfiSmbiosProtocol,
    smbios_handle: *mut u16,
    string_number: *mut UIntN,
    string: *const u8,
) -> EfiStatus;

/// Function pointer for `EFI_SMBIOS_REMOVE`.
pub type EfiSmbiosRemove =
    unsafe extern "efiapi" fn(this: *const EfiSmbiosProtocol, smbios_handle: u16) -> EfiStatus;

/// Function pointer for `EFI_SMBIOS_GET_NEXT`.
pub type EfiSmbiosGetNext = unsafe extern "efiapi" fn(
    this: *const EfiSmbiosProtocol,
    smbios_handle: *mut u16,
    record_type: *mut u8,
    record: *mut *mut SmbiosHeader,
    producer_handle: *mut *mut c_void,
) -> EfiStatus;

/// The raw ABI structure for `EFI_SMBIOS_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSmbiosProtocol {
    /// Adds a new SMBIOS structure record.
    pub add: EfiSmbiosAdd,
    /// Appends or updates a null-terminated string in an existing record.
    pub update_string: EfiSmbiosUpdateString,
    /// Removes an existing SMBIOS record by handle.
    pub remove: EfiSmbiosRemove,
    /// Enumerates next matching SMBIOS record.
    pub get_next: EfiSmbiosGetNext,
    /// Major SMBIOS specification version supported.
    pub major_version: u8,
    /// Minor SMBIOS specification version supported.
    pub minor_version: u8,
}

/// Safe wrapper around `EFI_SMBIOS_PROTOCOL`.
pub struct SmbiosProtocol<'a> {
    protocol: &'a EfiSmbiosProtocol,
}

impl<'a> SmbiosProtocol<'a> {
    /// Creates a new view borrowing the protocol instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiSmbiosProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the SMBIOS version supported by this firmware protocol as `(major, minor)`.
    #[must_use]
    pub const fn version(&self) -> (u8, u8) {
        (self.protocol.major_version, self.protocol.minor_version)
    }

    /// Removes an SMBIOS record by handle.
    ///
    /// # Errors
    /// Returns an error if removal fails or handle is not found.
    pub fn remove_record(&self, handle: u16) -> Result<(), EfiStatus> {
        // SAFETY: Protocol reference is valid.
        let status = unsafe { (self.protocol.remove)(self.protocol, handle) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
