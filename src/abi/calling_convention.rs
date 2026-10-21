//! # Calling convention
//!
//! The `UEFI` specification requires the `EFIAPI` calling convention
//! for all firmware services, protocol functions and image entry
//! points.
//!
//! In Rust this maps to `extern "efiapi"`. The convention matches the
//! platform `ABI` for every supported architecture: the Microsoft
//! x64 convention on `x86_64`, the relevant variant on `x86`, and
//! the platform convention on `AArch64` and `RISC-V` 64-bit.
//!
//! Use the types in this module whenever an `ABI` function pointer
//! must be named. For a specific firmware function, prefer the
//! typed function pointer or wrapper provided by the relevant
//! table or protocol module, so the parameter and return types
//! encode the required signature.

/// A raw function pointer using the `UEFI` calling convention.
///
/// The pointer carries no argument or return type information.
/// A concrete firmware function requires a precise signature;
/// define it with `extern "efiapi"` and the exact parameter types
/// for that function.
///
/// Calling through this pointer is unsafe because the caller
/// must guarantee that the pointer refers to a live firmware or
/// image function with the matching signature, calling
/// convention and calling requirements.
pub type RawEfiFn = unsafe extern "efiapi" fn();
