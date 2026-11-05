//! # Reset Notification Protocol
//!
//! UEFI 2.7 Reset Notification Protocol (`EFI_RESET_NOTIFICATION_PROTOCOL`).
//! Provides services to register for a notification prior to system reset.

use crate::base::{EfiGuid, EfiStatus};
use crate::tables::EfiResetSystem;

/// Reset Notification Protocol GUID: `9da34ae0-eaf9-4bbf-8ec3-fd60226c44be`
pub const RESET_NOTIFICATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9da3_4ae0,
    0xeaf9,
    0x4bbf,
    [0x8e, 0xc3, 0xfd, 0x60, 0x22, 0x6c, 0x44, 0xbe],
);

/// Function pointer type for `RegisterResetNotify`.
pub type EfiRegisterResetNotify = unsafe extern "efiapi" fn(
    this: *mut EfiResetNotificationProtocol,
    reset_function: EfiResetSystem,
) -> EfiStatus;

/// Function pointer type for `UnregisterResetNotify`.
pub type EfiUnregisterResetNotify = unsafe extern "efiapi" fn(
    this: *mut EfiResetNotificationProtocol,
    reset_function: EfiResetSystem,
) -> EfiStatus;

/// Raw UEFI Reset Notification Protocol interface (`EFI_RESET_NOTIFICATION_PROTOCOL`).
#[repr(C)]
pub struct EfiResetNotificationProtocol {
    /// Registers a reset callback function.
    pub register_reset_notify: EfiRegisterResetNotify,
    /// Unregisters a reset callback function.
    pub unregister_reset_notify: EfiUnregisterResetNotify,
}

/// Safe wrapper around `EfiResetNotificationProtocol`.
pub struct ResetNotificationProtocol<'a> {
    raw: &'a mut EfiResetNotificationProtocol,
}

impl ResetNotificationProtocol<'_> {
    /// Wraps a raw `EfiResetNotificationProtocol` instance.
    ///
    /// # Safety
    /// The caller must ensure that `raw` points to a valid, live protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut EfiResetNotificationProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // SAFETY: Caller guarantees `raw` points to a valid, live protocol instance.
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Registers a notification function to be called before platform reset.
    pub fn register(&mut self, reset_function: EfiResetSystem) -> EfiStatus {
        unsafe { (self.raw.register_reset_notify)(core::ptr::from_mut(self.raw), reset_function) }
    }

    /// Unregisters a notification function previously registered.
    pub fn unregister(&mut self, reset_function: EfiResetSystem) -> EfiStatus {
        unsafe { (self.raw.unregister_reset_notify)(core::ptr::from_mut(self.raw), reset_function) }
    }
}
