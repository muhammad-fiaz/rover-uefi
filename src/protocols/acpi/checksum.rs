//! # ACPI checksum
//!
//! Legacy ACPI checksum validation.
//!
//! The sum of all bytes modulo 256 must be
//! zero.

/// Returns whether the buffer checksum is
/// valid.
#[must_use]
pub fn acpi_checksum_ok(data: &[u8]) -> bool {
    data.iter().fold(0_u8, |acc, byte| acc.wrapping_add(*byte)) == 0
}
