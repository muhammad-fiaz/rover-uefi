//! # Open and close
//!
//! Helpers for opening and closing protocol
//! interfaces.
//!
//! Opening records the agent and controller
//! handles. The interface must be closed with
//! the same handles. The helpers take raw
//! pointers because open semantics depend on
//! caller handles and attributes that Rust
//! cannot verify.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::services::boot::BootServices;

/// Opens a protocol.
///
/// # Safety
///
/// The protocol and interface pointers must be
/// valid, the handles must be valid, and the
/// attributes must use the defined
/// `OPEN_PROTOCOL` values. The interface
/// obtained must be closed with `close` using
/// the same agent and controller handles.
#[must_use]
pub unsafe fn open<T>(
    boot: &BootServices<'_>,
    handle: EfiHandle,
    guid: &EfiGuid,
    interface: *mut *mut T,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: u32,
) -> EfiStatus {
    // SAFETY: The caller guarantees pointer, handle, and attribute validity.
    unsafe {
        boot.open_protocol(
            handle,
            core::ptr::from_ref(guid).cast_mut(),
            interface.cast::<*mut core::ffi::c_void>(),
            agent_handle,
            controller_handle,
            attributes,
        )
    }
}

/// Closes a protocol.
///
/// # Safety
///
/// The protocol pointer must be valid, and
/// the handles must match the corresponding
/// open call.
#[must_use]
pub unsafe fn close(
    boot: &BootServices<'_>,
    handle: EfiHandle,
    guid: &EfiGuid,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
) -> EfiStatus {
    // SAFETY: The caller guarantees pointer and handle validity.
    unsafe {
        boot.close_protocol(
            handle,
            core::ptr::from_ref(guid).cast_mut(),
            agent_handle,
            controller_handle,
        )
    }
}
