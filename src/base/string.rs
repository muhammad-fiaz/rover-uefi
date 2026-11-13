//! # ASCII and Unicode (UCS-2 / UTF-16) String Utilities
//!
//! Provides `#![no_std]` string length, comparison, case insensitivity, number conversion,
//! and transcoding routines between ASCII (`[u8]`) and Unicode (`[u16]`).

use core::cmp::Ordering;

/// Returns the character length of a null-terminated ASCII string (excluding null terminator).
#[must_use]
pub fn ascii_str_len(s: &[u8]) -> usize {
    let mut len = 0;
    while len < s.len() && s[len] != 0 {
        len += 1;
    }
    len
}

/// Returns the byte size of an ASCII string including the null terminator.
#[must_use]
pub fn ascii_str_size(s: &[u8]) -> usize {
    let len = ascii_str_len(s);
    if len < s.len() && s[len] == 0 {
        len + 1
    } else {
        len
    }
}

/// Converts an ASCII character to uppercase.
#[must_use]
pub const fn ascii_to_upper(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        c - (b'a' - b'A')
    } else {
        c
    }
}

/// Converts an ASCII character to lowercase.
#[must_use]
pub const fn ascii_to_lower(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        c + (b'a' - b'A')
    } else {
        c
    }
}

/// Performs a lexicographical comparison between two ASCII byte slices.
#[must_use]
pub fn ascii_str_cmp(s1: &[u8], s2: &[u8]) -> Ordering {
    let len1 = ascii_str_len(s1);
    let len2 = ascii_str_len(s2);
    let min_len = if len1 < len2 { len1 } else { len2 };

    let mut i = 0;
    while i < min_len {
        let b1 = s1[i];
        let b2 = s2[i];
        if b1 != b2 {
            return b1.cmp(&b2);
        }
        i += 1;
    }

    len1.cmp(&len2)
}

/// Performs a case-insensitive lexicographical comparison between two ASCII byte slices.
#[must_use]
pub fn ascii_str_case_cmp(s1: &[u8], s2: &[u8]) -> Ordering {
    let len1 = ascii_str_len(s1);
    let len2 = ascii_str_len(s2);
    let min_len = if len1 < len2 { len1 } else { len2 };

    let mut i = 0;
    while i < min_len {
        let b1 = ascii_to_lower(s1[i]);
        let b2 = ascii_to_lower(s2[i]);
        if b1 != b2 {
            return b1.cmp(&b2);
        }
        i += 1;
    }

    len1.cmp(&len2)
}

/// Parses an unsigned decimal integer from an ASCII string.
#[must_use]
pub fn ascii_str_to_u64(s: &[u8]) -> Option<u64> {
    let len = ascii_str_len(s);
    if len == 0 {
        return None;
    }

    let mut result: u64 = 0;
    let mut i = 0;
    while i < len {
        let c = s[i];
        if !c.is_ascii_digit() {
            return None;
        }
        let digit = u64::from(c - b'0');
        result = result.checked_mul(10)?.checked_add(digit)?;
        i += 1;
    }
    Some(result)
}

/// Parses an unsigned hexadecimal integer from an ASCII string (optional `0x` prefix).
#[must_use]
pub fn ascii_hex_to_u64(s: &[u8]) -> Option<u64> {
    let mut slice = s;
    if slice.len() >= 2 && slice[0] == b'0' && (slice[1] == b'x' || slice[1] == b'X') {
        slice = &slice[2..];
    }

    let len = ascii_str_len(slice);
    if len == 0 {
        return None;
    }

    let mut result: u64 = 0;
    let mut i = 0;
    while i < len {
        let c = slice[i];
        let digit = match c {
            b'0'..=b'9' => u64::from(c - b'0'),
            b'a'..=b'f' => u64::from(c - b'a' + 10),
            b'A'..=b'F' => u64::from(c - b'A' + 10),
            _ => return None,
        };
        result = result.checked_mul(16)?.checked_add(digit)?;
        i += 1;
    }
    Some(result)
}

/// Returns the character length of a null-terminated UTF-16 string (excluding null terminator).
#[must_use]
pub fn utf16_str_len(s: &[u16]) -> usize {
    let mut len = 0;
    while len < s.len() && s[len] != 0 {
        len += 1;
    }
    len
}

/// Returns the byte size of a UTF-16 string including the null terminator.
#[must_use]
pub fn utf16_str_size(s: &[u16]) -> usize {
    let len = utf16_str_len(s);
    let chars = if len < s.len() && s[len] == 0 {
        len + 1
    } else {
        len
    };
    chars.saturating_mul(2)
}

/// Converts a UTF-16 character to uppercase (basic multilingual plane ASCII range).
#[must_use]
pub const fn utf16_to_upper(c: u16) -> u16 {
    if c >= 0x0061 && c <= 0x007A {
        c - 0x0020
    } else {
        c
    }
}

/// Converts a UTF-16 character to lowercase (basic multilingual plane ASCII range).
#[must_use]
pub const fn utf16_to_lower(c: u16) -> u16 {
    if c >= 0x0041 && c <= 0x005A {
        c + 0x0020
    } else {
        c
    }
}

/// Performs a lexicographical comparison between two UTF-16 slices.
#[must_use]
pub fn utf16_str_cmp(s1: &[u16], s2: &[u16]) -> Ordering {
    let len1 = utf16_str_len(s1);
    let len2 = utf16_str_len(s2);
    let min_len = if len1 < len2 { len1 } else { len2 };

    let mut i = 0;
    while i < min_len {
        let c1 = s1[i];
        let c2 = s2[i];
        if c1 != c2 {
            return c1.cmp(&c2);
        }
        i += 1;
    }

    len1.cmp(&len2)
}

/// Performs a case-insensitive comparison between two UTF-16 slices.
#[must_use]
pub fn utf16_str_case_cmp(s1: &[u16], s2: &[u16]) -> Ordering {
    let len1 = utf16_str_len(s1);
    let len2 = utf16_str_len(s2);
    let min_len = if len1 < len2 { len1 } else { len2 };

    let mut i = 0;
    while i < min_len {
        let c1 = utf16_to_lower(s1[i]);
        let c2 = utf16_to_lower(s2[i]);
        if c1 != c2 {
            return c1.cmp(&c2);
        }
        i += 1;
    }

    len1.cmp(&len2)
}

/// Parses an unsigned decimal integer from a UTF-16 string.
#[must_use]
pub fn utf16_str_to_u64(s: &[u16]) -> Option<u64> {
    let len = utf16_str_len(s);
    if len == 0 {
        return None;
    }

    let mut result: u64 = 0;
    let mut i = 0;
    while i < len {
        let c = s[i];
        if !(0x0030..=0x0039).contains(&c) {
            return None;
        }
        let digit = u64::from(c - 0x0030);
        result = result.checked_mul(10)?.checked_add(digit)?;
        i += 1;
    }
    Some(result)
}

/// Parses an unsigned hexadecimal integer from a UTF-16 string (optional `0x` prefix).
#[must_use]
pub fn utf16_hex_to_u64(s: &[u16]) -> Option<u64> {
    let mut slice = s;
    if slice.len() >= 2 && slice[0] == 0x0030 && (slice[1] == 0x0078 || slice[1] == 0x0058) {
        slice = &slice[2..];
    }

    let len = utf16_str_len(slice);
    if len == 0 {
        return None;
    }

    let mut result: u64 = 0;
    let mut i = 0;
    while i < len {
        let c = slice[i];
        let digit = match c {
            0x0030..=0x0039 => u64::from(c - 0x0030),
            0x0061..=0x0066 => u64::from(c - 0x0061 + 10),
            0x0041..=0x0046 => u64::from(c - 0x0041 + 10),
            _ => return None,
        };
        result = result.checked_mul(16)?.checked_add(digit)?;
        i += 1;
    }
    Some(result)
}

/// Copies characters from an ASCII buffer into a UTF-16 buffer with zero extension.
///
/// Returns the number of characters written, including the null terminator if present.
pub fn ascii_to_utf16(src: &[u8], dest: &mut [u16]) -> usize {
    let count = if src.len() < dest.len() {
        src.len()
    } else {
        dest.len()
    };
    let mut i = 0;
    while i < count {
        dest[i] = u16::from(src[i]);
        if src[i] == 0 {
            return i + 1;
        }
        i += 1;
    }
    count
}

/// Copies characters from a UTF-16 buffer into an ASCII buffer with truncation.
///
/// Non-ASCII characters (>= 0x80) are converted to `?` (0x3F).
/// Returns the number of characters written, including the null terminator if present.
#[allow(clippy::cast_possible_truncation)]
pub fn utf16_to_ascii(src: &[u16], dest: &mut [u8]) -> usize {
    let count = if src.len() < dest.len() {
        src.len()
    } else {
        dest.len()
    };
    let mut i = 0;
    while i < count {
        let ch = src[i];
        if ch == 0 {
            dest[i] = 0;
            return i + 1;
        }
        dest[i] = if ch < 0x80 { ch as u8 } else { b'?' };
        i += 1;
    }
    count
}
