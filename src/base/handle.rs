//! # Handles
//!
//! The opaque handle type used to identify images and
//! devices in `UEFI`.
//!
//! A handle is a firmware-owned token. The value is never
//! dereferenced by the application; it is passed to boot
//! services and protocol functions to identify the object
//! it names. The null value represents no handle.
//!
//! The wrapper stores a raw pointer so the null value
//! and firmware-provided values are preserved without
//! implying that the pointee is accessible.

use core::ffi::c_void;

/// An opaque `UEFI` handle.
///
/// The handle names an image or a device. Handles are
/// created by firmware and remain valid according to
/// the image and protocol lifetime rules. The null
/// handle represents no object.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EfiHandle(*mut c_void);

impl EfiHandle {
    /// The null handle.
    pub const NULL: Self = Self(core::ptr::null_mut());

    /// Creates a handle from a raw pointer.
    ///
    /// The pointer may be null or any value supplied
    /// by firmware. The handle does not take ownership
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

    /// Returns whether the handle is null.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
}

impl PartialEq for EfiHandle {
    /// Compares two handles by raw pointer value.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for EfiHandle {}

impl core::fmt::Debug for EfiHandle {
    /// Formats the handle as its raw pointer value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("EfiHandle").field(&self.0).finish()
    }
}
