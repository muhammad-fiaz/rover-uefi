//! # Print2 and `Print2S` Protocols
//!
//! Provides standardized Unicode and ASCII formatted printing and value-to-string
//! conversions for firmware applications and drivers.
//!
//! Defined in UEFI Platform Initialization and Driver Execution Environment specifications.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Print2 Protocol GUID: `{f05976ef-83f1-4f3d-8619-f7595d41e538}`.
pub const EFI_PRINT2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xF059_76EF,
    0x83F1,
    0x4F3D,
    [0x86, 0x19, 0xF7, 0x59, 0x5D, 0x41, 0xE5, 0x38],
);

/// `Print2S` Protocol GUID: `{0cc252d2-c106-4661-b5bd-3147a4f81f92}`.
pub const EFI_PRINT2S_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0CC2_52D2,
    0xC106,
    0x4661,
    [0xB5, 0xBD, 0x31, 0x47, 0xA4, 0xF8, 0x1F, 0x92],
);

/// Format flags for value-to-string conversions.
pub mod print_flags {
    use crate::abi::types::UIntN;

    /// Prefix number with zeros up to width.
    pub const PREFIX_ZERO: UIntN = 0x0000_0001;
    /// Insert thousands separator commas every 3 digits.
    pub const COMMA_TYPE: UIntN = 0x0000_0002;
    /// Format value in hexadecimal radix.
    pub const RADIX_HEX: UIntN = 0x0000_0004;
    /// Left justify the formatted number within buffer.
    pub const LEFT_JUSTIFY: UIntN = 0x0000_0008;
    /// Maximum character width for 64-bit integer strings.
    pub const MAXIMUM_VALUE_CHARACTERS: usize = 38;
}

// ---------------------------------------------------------------------------
// Native #![no_std] Number Formatters
// ---------------------------------------------------------------------------

/// Formats a 64-bit integer into a Unicode (UTF-16) buffer according to flags.
///
/// # Errors
///
/// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
/// Returns `EfiStatus::INVALID_PARAMETER` if both hex and comma are set.
pub fn format_int64_unicode(
    buffer: &mut [u16],
    value: i64,
    flags: UIntN,
    width: usize,
) -> Result<usize, EfiStatus> {
    let mut ascii_buf = [0u8; print_flags::MAXIMUM_VALUE_CHARACTERS];
    let len = format_int64_ascii(&mut ascii_buf, value, flags, width)?;

    if buffer.len() < len + 1 {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }

    for (i, &b) in ascii_buf[..len].iter().enumerate() {
        buffer[i] = u16::from(b);
    }
    buffer[len] = 0;

    Ok(len)
}

/// Formats a 64-bit integer into an ASCII buffer according to flags.
///
/// # Errors
///
/// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
/// Returns `EfiStatus::INVALID_PARAMETER` if both hex and comma are set.
pub fn format_int64_ascii(
    buffer: &mut [u8],
    value: i64,
    flags: UIntN,
    width: usize,
) -> Result<usize, EfiStatus> {
    let is_hex = (flags & print_flags::RADIX_HEX) != 0;
    let is_comma = (flags & print_flags::COMMA_TYPE) != 0;
    let is_zero = (flags & print_flags::PREFIX_ZERO) != 0;

    if is_hex && is_comma {
        return Err(EfiStatus::INVALID_PARAMETER);
    }

    let mut temp = [0u8; 64];
    let mut temp_idx = 0;

    if is_hex {
        let mut uval = value.cast_unsigned();
        if uval == 0 {
            temp[0] = b'0';
            temp_idx = 1;
        } else {
            while uval > 0 {
                let digit = (uval & 0xF) as u8;
                let c = if digit < 10 {
                    b'0' + digit
                } else {
                    b'A' + (digit - 10)
                };
                temp[temp_idx] = c;
                temp_idx += 1;
                uval >>= 4;
            }
        }
    } else {
        let is_neg = value < 0;
        let mut uval = if is_neg {
            value.unsigned_abs()
        } else {
            value.cast_unsigned()
        };

        let mut digit_count = 0;
        if uval == 0 {
            temp[0] = b'0';
            temp_idx = 1;
        } else {
            while uval > 0 {
                if is_comma && digit_count > 0 && (digit_count % 3 == 0) {
                    temp[temp_idx] = b',';
                    temp_idx += 1;
                }
                let digit = (uval % 10) as u8;
                temp[temp_idx] = b'0' + digit;
                temp_idx += 1;
                digit_count += 1;
                uval /= 10;
            }
        }

        if is_neg {
            temp[temp_idx] = b'-';
            temp_idx += 1;
        }
    }

    // `temp` holds digits in reverse order
    let raw_len = temp_idx;
    let target_width = if width == 0 {
        raw_len
    } else {
        width.min(print_flags::MAXIMUM_VALUE_CHARACTERS)
    };

    let total_len = if is_zero && target_width > raw_len {
        target_width
    } else {
        raw_len
    };

    if buffer.len() < total_len + 1 {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }

    let pad_zeros = if is_zero && target_width > raw_len {
        target_width - raw_len
    } else {
        0
    };

    let mut out_idx = 0;

    // Check if sign character exists
    let has_minus = temp[raw_len - 1] == b'-';
    let digits_start = if has_minus {
        buffer[out_idx] = b'-';
        out_idx += 1;
        raw_len - 2
    } else {
        raw_len - 1
    };

    for _ in 0..pad_zeros {
        buffer[out_idx] = b'0';
        out_idx += 1;
    }

    for &byte in temp[..=digits_start].iter().rev() {
        buffer[out_idx] = byte;
        out_idx += 1;
    }

    buffer[out_idx] = 0;
    Ok(out_idx)
}

// ---------------------------------------------------------------------------
// Protocol ABI Tables
// ---------------------------------------------------------------------------

/// Function pointer for Unicode base list print.
pub type UnicodeBsPrint = unsafe extern "efiapi" fn(
    start_of_buffer: *mut u16,
    buffer_size: UIntN,
    format_string: *const u16,
    marker: *const c_void,
) -> UIntN;

/// Function pointer for Unicode string print.
pub type UnicodeSPrint = unsafe extern "C" fn(
    start_of_buffer: *mut u16,
    buffer_size: UIntN,
    format_string: *const u16,
    ...
) -> UIntN;

/// Function pointer for Unicode base list print with ASCII format string.
pub type UnicodeBsPrintAsciiFormat = unsafe extern "efiapi" fn(
    start_of_buffer: *mut u16,
    buffer_size: UIntN,
    format_string: *const u8,
    marker: *const c_void,
) -> UIntN;

/// Function pointer for Unicode string print with ASCII format string.
pub type UnicodeSPrintAsciiFormat = unsafe extern "C" fn(
    start_of_buffer: *mut u16,
    buffer_size: UIntN,
    format_string: *const u8,
    ...
) -> UIntN;

/// Function pointer for converting a value to Unicode string without bounds check.
pub type UnicodeValueToString =
    unsafe extern "efiapi" fn(buffer: *mut u16, flags: UIntN, value: i64, width: UIntN) -> UIntN;

/// Function pointer for converting a value to Unicode string with bounds check.
pub type UnicodeValueToStringS = unsafe extern "efiapi" fn(
    buffer: *mut u16,
    buffer_size: UIntN,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> EfiStatus;

/// Function pointer for ASCII base list print.
pub type AsciiBsPrint = unsafe extern "efiapi" fn(
    start_of_buffer: *mut u8,
    buffer_size: UIntN,
    format_string: *const u8,
    marker: *const c_void,
) -> UIntN;

/// Function pointer for ASCII string print.
pub type AsciiSPrint = unsafe extern "C" fn(
    start_of_buffer: *mut u8,
    buffer_size: UIntN,
    format_string: *const u8,
    ...
) -> UIntN;

/// Function pointer for ASCII base list print with Unicode format string.
pub type AsciiBsPrintUnicodeFormat = unsafe extern "efiapi" fn(
    start_of_buffer: *mut u8,
    buffer_size: UIntN,
    format_string: *const u16,
    marker: *const c_void,
) -> UIntN;

/// Function pointer for ASCII string print with Unicode format string.
pub type AsciiSPrintUnicodeFormat = unsafe extern "C" fn(
    start_of_buffer: *mut u8,
    buffer_size: UIntN,
    format_string: *const u16,
    ...
) -> UIntN;

/// Function pointer for converting a value to ASCII string without bounds check.
pub type AsciiValueToString =
    unsafe extern "efiapi" fn(buffer: *mut u8, flags: UIntN, value: i64, width: UIntN) -> UIntN;

/// Function pointer for converting a value to ASCII string with bounds check.
pub type AsciiValueToStringS = unsafe extern "efiapi" fn(
    buffer: *mut u8,
    buffer_size: UIntN,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> EfiStatus;

/// Low-level Print2 Protocol table (legacy unbounded string conversion).
#[derive(Debug)]
#[repr(C)]
pub struct EfiPrint2Protocol {
    /// Formats Unicode string from Unicode format and argument list.
    pub unicode_bs_print: UnicodeBsPrint,
    /// Formats Unicode string from Unicode format.
    pub unicode_s_print: UnicodeSPrint,
    /// Formats Unicode string from ASCII format and argument list.
    pub unicode_bs_print_ascii_format: UnicodeBsPrintAsciiFormat,
    /// Formats Unicode string from ASCII format.
    pub unicode_s_print_ascii_format: UnicodeSPrintAsciiFormat,
    /// Converts signed 64-bit integer to Unicode string.
    pub unicode_value_to_string: UnicodeValueToString,
    /// Formats ASCII string from ASCII format and argument list.
    pub ascii_bs_print: AsciiBsPrint,
    /// Formats ASCII string from ASCII format.
    pub ascii_s_print: AsciiSPrint,
    /// Formats ASCII string from Unicode format and argument list.
    pub ascii_bs_print_unicode_format: AsciiBsPrintUnicodeFormat,
    /// Formats ASCII string from Unicode format.
    pub ascii_s_print_unicode_format: AsciiSPrintUnicodeFormat,
    /// Converts signed 64-bit integer to ASCII string.
    pub ascii_value_to_string: AsciiValueToString,
}

/// Low-level `Print2S` Protocol table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPrint2SProtocol {
    /// Formats Unicode string from Unicode format and argument list.
    pub unicode_bs_print: UnicodeBsPrint,
    /// Formats Unicode string from Unicode format.
    pub unicode_s_print: UnicodeSPrint,
    /// Formats Unicode string from ASCII format and argument list.
    pub unicode_bs_print_ascii_format: UnicodeBsPrintAsciiFormat,
    /// Formats Unicode string from ASCII format.
    pub unicode_s_print_ascii_format: UnicodeSPrintAsciiFormat,
    /// Converts signed 64-bit integer to Unicode string safely.
    pub unicode_value_to_string_s: UnicodeValueToStringS,
    /// Formats ASCII string from ASCII format and argument list.
    pub ascii_bs_print: AsciiBsPrint,
    /// Formats ASCII string from ASCII format.
    pub ascii_s_print: AsciiSPrint,
    /// Formats ASCII string from Unicode format and argument list.
    pub ascii_bs_print_unicode_format: AsciiBsPrintUnicodeFormat,
    /// Formats ASCII string from Unicode format.
    pub ascii_s_print_unicode_format: AsciiSPrintUnicodeFormat,
    /// Converts signed 64-bit integer to ASCII string safely.
    pub ascii_value_to_string_s: AsciiValueToStringS,
}

// ---------------------------------------------------------------------------
// Safe Client Wrappers
// ---------------------------------------------------------------------------

/// Client wrapper around `EfiPrint2Protocol`.
#[derive(Clone, Copy, Debug)]
pub struct Print2 {
    proto: *const EfiPrint2Protocol,
}

impl Print2 {
    /// Wraps a raw pointer to `EfiPrint2Protocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPrint2Protocol) -> Self {
        Self { proto }
    }

    /// Converts a signed 64-bit value to a Null-terminated Unicode string.
    ///
    /// # Safety
    ///
    /// Caller must ensure `buffer` is large enough to receive the formatted string.
    pub unsafe fn value_to_unicode(
        &self,
        buffer: *mut u16,
        flags: UIntN,
        value: i64,
        width: usize,
    ) -> usize {
        unsafe { ((*self.proto).unicode_value_to_string)(buffer, flags, value, width as UIntN) }
    }

    /// Converts a signed 64-bit value to a Null-terminated ASCII string.
    ///
    /// # Safety
    ///
    /// Caller must ensure `buffer` is large enough to receive the formatted string.
    pub unsafe fn value_to_ascii(
        &self,
        buffer: *mut u8,
        flags: UIntN,
        value: i64,
        width: usize,
    ) -> usize {
        unsafe { ((*self.proto).ascii_value_to_string)(buffer, flags, value, width as UIntN) }
    }
}

/// Safe client wrapper around `EfiPrint2SProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct Print2S {
    proto: *const EfiPrint2SProtocol,
}

impl Print2S {
    /// Wraps a raw pointer to `EfiPrint2SProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPrint2SProtocol) -> Self {
        Self { proto }
    }

    /// Converts a signed 64-bit value to a Null-terminated Unicode string.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
    /// Returns `EfiStatus::INVALID_PARAMETER` if arguments are invalid.
    pub fn value_to_unicode(
        &self,
        buffer: &mut [u16],
        value: i64,
        flags: UIntN,
        width: usize,
    ) -> Result<(), EfiStatus> {
        let buffer_size: UIntN = core::mem::size_of_val(buffer);
        let status = unsafe {
            ((*self.proto).unicode_value_to_string_s)(
                buffer.as_mut_ptr(),
                buffer_size,
                flags,
                value,
                width,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Converts a signed 64-bit value to a Null-terminated ASCII string.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
    /// Returns `EfiStatus::INVALID_PARAMETER` if arguments are invalid.
    pub fn value_to_ascii(
        &self,
        buffer: &mut [u8],
        value: i64,
        flags: UIntN,
        width: usize,
    ) -> Result<(), EfiStatus> {
        let buffer_size: UIntN = core::mem::size_of_val(buffer);
        let status = unsafe {
            ((*self.proto).ascii_value_to_string_s)(
                buffer.as_mut_ptr(),
                buffer_size,
                flags,
                value,
                width,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
