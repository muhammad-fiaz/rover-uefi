//! # Services
//!
//! Typed access to firmware services.
//!
//! Boot services are available only before a
//! successful `ExitBootServices` call. Runtime
//! services remain available afterwards under the
//! virtual memory map. This module provides the
//! boot and runtime services implementations.
//!
//! The wrappers in this crate call the raw
//! `EFIAPI` function pointers defined in the tables
//! module. Safe wrappers hide pointers with stack
//! locals or slices and preserve the firmware
//! status. Unsafe wrappers expose the exact binary
//! interface for operations that require caller
//! pointers, callbacks, or variadic arguments.

pub mod boot;
pub mod runtime;
