//! # Events
//!
//! The opaque event type used by `UEFI` timer and
//! notification services.
//!
//! An event is a firmware-owned token. The value is
//! never dereferenced by the application; it is passed
//! to boot services to signal, wait on, check, or close
//! the event. The null value represents no event.
//!
//! The wrapper stores a raw pointer so the null value
//! and firmware-provided values are preserved without
//! implying that the pointee is accessible.

use core::ffi::c_void;

/// An opaque `UEFI` event.
///
/// The event names a timer, notification or wait
/// object owned by firmware. Events are created and
/// destroyed through boot services. The null event
/// represents no object.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EfiEvent(*mut c_void);

impl EfiEvent {
    /// The null event.
    pub const NULL: Self = Self(core::ptr::null_mut());

    /// Creates an event from a raw pointer.
    ///
    /// The pointer may be null or any value supplied
    /// by firmware. The event does not take ownership
    /// of the pointee.
    #[must_use]
    pub const fn from_ptr(ptr: *mut c_void) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer value.
    #[must_use]
    pub const fn to_ptr(self) -> *mut c_void {
        self.0
    }

    /// Returns whether the event is null.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
}

impl PartialEq for EfiEvent {
    /// Compares two events by raw pointer value.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for EfiEvent {}

impl core::fmt::Debug for EfiEvent {
    /// Formats the event as its raw pointer value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("EfiEvent").field(&self.0).finish()
    }
}
