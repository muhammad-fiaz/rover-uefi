//! # Protocol interfaces
//!
//! Ownership and lifetime rules for protocol
//! interfaces.
//!
//! A protocol interface is owned by firmware
//! and accessed through the handle that
//! installs it. The pointer is valid only while
//! the handle and firmware lifecycle grant
//! access. Opening a protocol with boot
//! services records the agent and controller
//! handles. The interface must be closed with
//! the same handles before the handle is
//! uninstalled or before `ExitBootServices`
//! invalidates boot services.
//!
//! No generic trait is provided. Each protocol
//! wrapper borrows its interface for an explicit
//! lifetime so callers cannot invent a longer
//! lifetime than the handle grants. Raw
//! pointers obtained from boot services are
//! validated for null before a wrapper is
//! created. The wrapper does not extend the
//! firmware lifetime.

use crate::base::EfiHandle;
use crate::services::boot::BootServices;

/// Returns whether an interface pointer is
/// non-null.
///
/// The helper checks only the pointer value.
/// It does not prove that the pointee is a
/// valid protocol instance.
#[must_use]
pub const fn is_interface_present<T>(ptr: *const T) -> bool {
    !ptr.is_null()
}

/// Returns whether a handle is present.
#[must_use]
pub const fn is_handle_present(handle: EfiHandle) -> bool {
    !handle.is_null()
}

/// Locates a protocol interface.
///
/// The returned pointer must be checked for
/// null by the caller. The caller must open
/// and close the protocol according to its
/// `GUID` before using the interface pointer
/// beyond the locate call. Boot services must
/// remain available for the locate operation.
///
/// # Safety
///
/// The boot services view must be valid, the
/// interface pointer must be valid for writing,
/// and the registration pointer may be null.
/// The returned interface must not be used
/// after the owning handle becomes invalid.
///
/// # Errors
///
/// Returns the firmware status when the
/// protocol cannot be located.
pub unsafe fn locate_protocol<T>(
    boot: &BootServices<'_>,
    guid: &crate::base::EfiGuid,
    interface: *mut *mut T,
) -> Result<*mut T, crate::base::EfiStatus> {
    // SAFETY: The caller guarantees pointer validity for the call.
    let status = unsafe {
        boot.locate_protocol(
            core::ptr::from_ref(guid).cast_mut(),
            core::ptr::null_mut(),
            interface.cast::<*mut core::ffi::c_void>(),
        )
    };
    if status.is_success() && !interface.is_null() {
        // SAFETY: The caller guarantees the out pointer is valid for reading.
        let found = unsafe { *interface };
        if found.is_null() {
            Err(crate::base::EfiStatus::NOT_FOUND)
        } else {
            Ok(found)
        }
    } else {
        Err(status)
    }
}
