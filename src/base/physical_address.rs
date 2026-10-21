//! # Physical addresses
//!
//! The 64-bit physical address type used by `UEFI`
//! memory services.
//!
//! A physical address identifies a byte in system
//! memory without translation. The value `0` denotes
//! the null address. Conversions to and from raw
//! pointers and integers are explicit so the caller
//! remains responsible for validating that the address
//! names accessible memory.

/// A 64-bit physical address.
///
/// The wrapper stores the address as `u64` so firmware
/// values outside the host pointer range are preserved
/// on 32-bit targets. The address has no ownership
/// semantics; it is only a number.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EfiPhysicalAddress(u64);

impl EfiPhysicalAddress {
    /// The null physical address.
    pub const NULL: Self = Self(0);

    /// Creates an address from a raw value.
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw address value.
    #[must_use]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    /// Returns whether the address is the null address.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Converts the address to a raw pointer.
    ///
    /// The conversion does not validate the address.
    /// Dereferencing the resulting pointer requires a
    /// separate guarantee that the address names
    /// accessible, correctly aligned memory for the
    /// intended access.
    #[must_use]
    pub fn to_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }

    /// Creates an address from a raw pointer.
    ///
    /// The pointer does not need to be dereferencable
    /// to form the address value. Any later use of
    /// the address must satisfy the same requirements
    /// as a direct pointer use.
    #[must_use]
    pub fn from_ptr(ptr: *const u8) -> Self {
        Self(ptr as u64)
    }
}

impl core::fmt::Debug for EfiPhysicalAddress {
    /// Formats the address as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("EfiPhysicalAddress")
            .field(&self.0)
            .finish()
    }
}
