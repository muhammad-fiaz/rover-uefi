//! # Application binary interface
//!
//! This module defines the low-level types and conventions that
//! describe the `UEFI` application binary interface.
//!
//! The contents here are the foundation for the rest of the crate:
//! calling conventions, primitive types and the representation
//! of firmware function pointers. Higher-level tables, services
//! and protocols build on the definitions in this module while
//! preserving exact `ABI` compatibility with the specification.

pub mod calling_convention;
pub mod types;

pub use calling_convention::RawEfiFn;
pub use types::{
    Bool, Char8, Char16, EFI_FALSE, EFI_PAGE_MASK, EFI_PAGE_SHIFT, EFI_PAGE_SIZE, EFI_TRUE, IntN,
    UIntN,
};
