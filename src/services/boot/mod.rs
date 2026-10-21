//! # Boot services
//!
//! Typed wrappers for the `EFI Boot Services Table`.
//!
//! The wrapper is constructed from the firmware
//! table pointer in the system table. Each method
//! invokes the corresponding raw `EFIAPI` function
//! pointer and preserves the firmware status.
//! Methods that can be implemented with owned
//! values, stack locals, or slices are safe.
//! Methods that require caller pointers,
//! callbacks, or variadic arguments are unsafe
//! and document their safety requirements.
//!
//! Boot services are valid only before a
//! successful `ExitBootServices` call. After that
//! call the table pointer must not be used and no
//! method may be invoked through the wrapper or
//! through raw table access. The exit method
//! documents this transition. Raw table access
//! remains available for advanced use, so the
//! wrapper does not prevent invalid calls by
//! itself.

use crate::tables::boot::EfiBootServices;

pub mod allocation;
pub mod counters;
pub mod events;
pub mod exit;
pub mod handles;
pub mod image;
pub mod memory;
pub mod protocol;
pub mod utilities;
pub mod watchdog;

/// A typed boot services view.
///
/// The view borrows the firmware boot services
/// table for a lifetime chosen by the caller.
/// The caller must guarantee that the table
/// reference is valid, correctly aligned, and
/// alive for the full lifetime. The table is
/// owned by firmware and is valid only before
/// `ExitBootServices`.
pub struct BootServices<'a> {
    table: &'a EfiBootServices,
}

impl<'a> BootServices<'a> {
    /// Creates a view from a table reference.
    ///
    /// The caller must ensure that the reference
    /// points to a live firmware table for the
    /// lifetime of the view.
    #[must_use]
    pub const fn new(table: &'a EfiBootServices) -> Self {
        Self { table }
    }

    /// Returns the underlying table reference.
    #[must_use]
    pub const fn table(&self) -> &'a EfiBootServices {
        self.table
    }
}

impl core::fmt::Debug for BootServices<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("BootServices")
            .field("table", &core::ptr::from_ref(self.table))
            .finish()
    }
}
