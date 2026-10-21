//! # Runtime services
//!
//! Typed wrappers for the `EFI Runtime Services Table`.
//!
//! The wrapper borrows the firmware runtime table.
//! Each method calls the raw `EFIAPI` function
//! pointer and preserves the firmware status.
//! Methods that can be implemented with stack
//! locals or slices are safe. Methods that
//! require caller pointers or caller-controlled
//! maps are unsafe.
//!
//! Runtime services may be called before
//! `ExitBootServices`. Services that remain
//! available afterwards require the virtual
//! address map. `SetVirtualAddressMap`
//! performs the physical-to-virtual transition
//! and must be called exactly once by the
//! operating system loader. Other services
//! may have additional restrictions after the
//! transition, described on each method. Raw
//! table access remains available, so the
//! wrapper does not prevent invalid calls by
//! itself.

use crate::tables::runtime::EfiRuntimeServices;

pub mod capsules;
pub mod counters;
pub mod reset;
pub mod time;
pub mod variables;
pub mod virtual_memory;

/// A typed runtime services view.
///
/// The view borrows the firmware runtime table
/// for a lifetime chosen by the caller. The
/// caller must guarantee that the reference is
/// valid, correctly aligned, and alive for the
/// full lifetime.
pub struct RuntimeServices<'a> {
    table: &'a EfiRuntimeServices,
}

impl<'a> RuntimeServices<'a> {
    /// Creates a view from a table reference.
    ///
    /// The caller must ensure that the reference
    /// points to a live firmware table for the
    /// lifetime of the view.
    #[must_use]
    pub const fn new(table: &'a EfiRuntimeServices) -> Self {
        Self { table }
    }

    /// Returns the underlying table reference.
    #[must_use]
    pub const fn table(&self) -> &'a EfiRuntimeServices {
        self.table
    }
}

impl core::fmt::Debug for RuntimeServices<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RuntimeServices")
            .field("table", &core::ptr::from_ref(self.table))
            .finish()
    }
}
