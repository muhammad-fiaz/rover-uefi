//! # HTTP Utilities Protocol
//!
//! Provides services to build, parse, format, and manipulate HTTP message headers.
//! Implements the UEFI HTTP Utilities Protocol defined in UEFI 2.5+.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// HTTP Utilities Protocol GUID: `{3e35c163-4074-45dd-431e-23989dd86b32}`.
pub const EFI_HTTP_UTILITIES_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3E35_C163,
    0x4074,
    0x45DD,
    [0x43, 0x1E, 0x23, 0x98, 0x9D, 0xD8, 0x6B, 0x32],
);

/// Raw C representation of an HTTP header key-value pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiHttpHeader {
    /// Null-terminated ASCII string describing the header field name.
    pub field_name: *const u8,
    /// Null-terminated ASCII string describing the header field value.
    pub field_value: *const u8,
}

/// Function pointer for `EFI_HTTP_UTILS_BUILD`.
pub type EfiHttpUtilsBuild = unsafe extern "efiapi" fn(
    this: *mut EfiHttpUtilitiesProtocol,
    seed_message_size: UIntN,
    seed_message: *const c_void,
    delete_count: UIntN,
    delete_list: *const *const u8,
    append_count: UIntN,
    append_list: *const *const EfiHttpHeader,
    new_message_size: *mut UIntN,
    new_message: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_HTTP_UTILS_PARSE`.
pub type EfiHttpUtilsParse = unsafe extern "efiapi" fn(
    this: *mut EfiHttpUtilitiesProtocol,
    http_message: *const u8,
    http_message_size: UIntN,
    header_fields: *mut *mut EfiHttpHeader,
    field_count: *mut UIntN,
) -> EfiStatus;

/// Raw ABI structure for `EFI_HTTP_UTILITIES_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHttpUtilitiesProtocol {
    /// Builds a new HTTP header message from seed, deletion list, and append list.
    pub build: EfiHttpUtilsBuild,
    /// Parses an HTTP header byte stream into an array of key-value header pairs.
    pub parse: EfiHttpUtilsParse,
}

/// Safe client wrapper around [`EfiHttpUtilitiesProtocol`].
pub struct HttpUtilities<'a> {
    proto: &'a mut EfiHttpUtilitiesProtocol,
}

impl<'a> HttpUtilities<'a> {
    /// Creates a new `HttpUtilities` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiHttpUtilitiesProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiHttpUtilitiesProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiHttpUtilitiesProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiHttpUtilitiesProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiHttpUtilitiesProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Invokes the firmware `Build` service.
    ///
    /// # Safety
    ///
    /// The caller must ensure pointer parameters adhere to the UEFI HTTP Utilities specification.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware building fails.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn build_raw(
        &mut self,
        seed_message_size: usize,
        seed_message: *const c_void,
        delete_count: usize,
        delete_list: *const *const u8,
        append_count: usize,
        append_list: *const *const EfiHttpHeader,
        new_message_size: &mut usize,
        new_message: &mut *mut c_void,
    ) -> Result<(), EfiStatus> {
        let mut out_size = *new_message_size as UIntN;
        let mut out_msg = *new_message;
        let status = unsafe {
            (self.proto.build)(
                self.proto,
                seed_message_size as UIntN,
                seed_message,
                delete_count as UIntN,
                delete_list,
                append_count as UIntN,
                append_list,
                core::ptr::from_mut(&mut out_size),
                core::ptr::from_mut(&mut out_msg),
            )
        };
        if status == EfiStatus::SUCCESS {
            *new_message_size = out_size as usize;
            *new_message = out_msg;
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Invokes the firmware `Parse` service.
    ///
    /// # Safety
    ///
    /// `http_message` must point to a valid buffer of `http_message_size` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error status if parsing fails.
    pub unsafe fn parse_raw(
        &mut self,
        http_message: *const u8,
        http_message_size: usize,
        header_fields: &mut *mut EfiHttpHeader,
        field_count: &mut usize,
    ) -> Result<(), EfiStatus> {
        let mut count = *field_count as UIntN;
        let mut headers = *header_fields;
        let status = unsafe {
            (self.proto.parse)(
                self.proto,
                http_message,
                http_message_size as UIntN,
                core::ptr::from_mut(&mut headers),
                core::ptr::from_mut(&mut count),
            )
        };
        if status == EfiStatus::SUCCESS {
            *header_fields = headers;
            *field_count = count as usize;
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Parses a single HTTP header line into `(name, value)` slices.
///
/// Strips leading and trailing ASCII whitespace.
#[must_use]
pub fn parse_header_line(line: &[u8]) -> Option<(&str, &str)> {
    let colon_pos = line.iter().position(|&b| b == b':')?;
    let raw_name = &line[..colon_pos];
    let raw_val = &line[colon_pos + 1..];

    let name = trim_ascii_whitespace(raw_name);
    let val = trim_ascii_whitespace(raw_val);

    if name.is_empty() {
        return None;
    }

    let name_str = core::str::from_utf8(name).ok()?;
    let val_str = core::str::from_utf8(val).ok()?;
    Some((name_str, val_str))
}

/// Formats a single HTTP header line `"{name}: {value}\r\n"` into `out`.
///
/// # Errors
///
/// Returns `BUFFER_TOO_SMALL` if `out` cannot accommodate the header and CRLF.
pub fn format_header_line(name: &str, value: &str, out: &mut [u8]) -> Result<usize, EfiStatus> {
    let name_bytes = name.as_bytes();
    let val_bytes = value.as_bytes();
    let total_len = name_bytes.len() + 2 + val_bytes.len() + 2; // ": " + "\r\n"
    if out.len() < total_len {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }

    let mut cursor = 0;
    out[cursor..cursor + name_bytes.len()].copy_from_slice(name_bytes);
    cursor += name_bytes.len();

    out[cursor..cursor + 2].copy_from_slice(b": ");
    cursor += 2;

    out[cursor..cursor + val_bytes.len()].copy_from_slice(val_bytes);
    cursor += val_bytes.len();

    out[cursor..cursor + 2].copy_from_slice(b"\r\n");
    cursor += 2;

    Ok(cursor)
}

/// Parses multiple HTTP headers from a raw buffer into a slice of key-value tuples.
///
/// Returns the number of header lines parsed.
///
/// # Errors
///
/// Returns `BUFFER_TOO_SMALL` if `entries` slice is too small to hold all parsed headers.
pub fn parse_raw_headers<'a>(
    raw: &'a [u8],
    entries: &mut [(&'a str, &'a str)],
) -> Result<usize, EfiStatus> {
    let mut count = 0;
    let mut pos = 0;

    while pos < raw.len() {
        // Find next CRLF or LF
        let remaining = &raw[pos..];
        let line_end = remaining.iter().position(|&b| b == b'\n');
        let line_len = match line_end {
            Some(idx) => idx,
            None => remaining.len(),
        };

        let mut line = &remaining[..line_len];
        if let Some(stripped) = line.strip_suffix(b"\r") {
            line = stripped;
        }

        pos += line_len + usize::from(line_end.is_some());

        if line.is_empty() {
            // End of headers block
            break;
        }

        if let Some((name, val)) = parse_header_line(line) {
            if count >= entries.len() {
                return Err(EfiStatus::BUFFER_TOO_SMALL);
            }
            entries[count] = (name, val);
            count += 1;
        }
    }

    Ok(count)
}

fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let mut start = 0;
    while start < bytes.len() && (bytes[start] == b' ' || bytes[start] == b'\t') {
        start += 1;
    }
    let mut end = bytes.len();
    while end > start && (bytes[end - 1] == b' ' || bytes[end - 1] == b'\t') {
        end -= 1;
    }
    &bytes[start..end]
}
