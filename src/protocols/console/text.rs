//! # Text helpers
//!
//! UTF-16 string validation for text protocols.
//!
//! Firmware text functions require
//! null-terminated UTF-16 strings. The helpers
//! check termination and compute byte sizes
//! with checked arithmetic. No allocation
//! occurs.

use crate::abi::types::Char16;
use crate::abi::types::UIntN;

/// Returns whether the string is non-empty
/// and null-terminated.
#[must_use]
pub fn is_null_terminated(text: &[Char16]) -> bool {
    !text.is_empty() && text.last().is_some_and(|last| *last == 0)
}

/// Returns the byte size of a UTF-16 string.
///
/// Returns `None` on multiplication overflow.
#[must_use]
pub fn utf16_bytes(text: &[Char16]) -> Option<UIntN> {
    text.len().checked_mul(core::mem::size_of::<Char16>())
}

/// Returns whether the string contains only
/// printable or control characters allowed by
/// the text protocols.
///
/// The check is limited to termination and
/// does not validate glyph availability, which
/// firmware reports through `TestString` and
/// output warnings.
#[must_use]
pub fn is_printable_terminated(text: &[Char16]) -> bool {
    if !is_null_terminated(text) {
        return false;
    }
    text.iter().all(|unit| {
        *unit == 0 || (0x0009..=0x000D).contains(unit) || (0x0020..=0xFFFD).contains(unit)
    })
}
