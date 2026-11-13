//! # Realm Aperture Management Protocol
//!
//! Provides the Arm Confidential Compute Architecture (CCA) Realm Aperture
//! Management Protocol interface. Enables virtualized drivers and guest firmware
//! executing within an Arm Realm to dynamically open and close shared host memory
//! apertures across guest-host boundaries.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr
)]

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// Realm Aperture Management Protocol GUID.
///
/// `{585c00be-cf7c-4db8-8aa2-490d67f5f6e6}`
pub const REALM_APERTURE_MANAGEMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x585C_00BE,
    0xCF7C,
    0x4DB8,
    [0x8A, 0xA2, 0x49, 0x0D, 0x67, 0xF5, 0xF6, 0xE6],
);

/// Realm Aperture Management Protocol Revision 1.0.
pub const REALM_APERTURE_MANAGEMENT_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Raw function pointer for opening a shared memory aperture with the host.
pub type RealmApertureOpen = unsafe extern "efiapi" fn(
    memory: u64,
    pages: usize,
    aperture_reference: *mut EfiHandle,
) -> EfiStatus;

/// Raw function pointer for closing a previously opened memory aperture.
pub type RealmApertureClose =
    unsafe extern "efiapi" fn(aperture_reference: EfiHandle, memory_map_locked: bool) -> EfiStatus;

/// Realm Aperture Management Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RealmApertureManagementProtocol {
    /// Protocol revision (e.g. `0x00010000`).
    pub revision: u64,
    /// Shares realm physical pages with the host.
    pub open_aperture: RealmApertureOpen,
    /// Closes the shared aperture and restores private realm access.
    pub close_aperture: RealmApertureClose,
}

/// High-level safe client wrapper around [`RealmApertureManagementProtocol`].
pub struct RealmApertureManagement<'a> {
    proto: &'a RealmApertureManagementProtocol,
}

impl<'a> RealmApertureManagement<'a> {
    /// Wraps a raw [`RealmApertureManagementProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a RealmApertureManagementProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Opens a shared aperture over `pages` starting at physical address `memory`.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if parameters are invalid, allocation fails, or access is denied.
    pub fn open_aperture(&self, memory: u64, pages: usize) -> Result<EfiHandle, EfiStatus> {
        let mut aperture = EfiHandle::NULL;
        let status = unsafe {
            (self.proto.open_aperture)(memory, pages, core::ptr::from_mut(&mut aperture))
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(aperture)
        }
    }

    /// Closes an opened aperture reference, restoring private realm memory protection.
    ///
    /// If `memory_map_locked` is true (e.g. during `ExitBootServices`), modifications
    /// to the UEFI memory map are forbidden.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on invalid aperture handle or memory state error.
    pub fn close_aperture(
        &self,
        aperture: EfiHandle,
        memory_map_locked: bool,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.close_aperture)(aperture, memory_map_locked) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
