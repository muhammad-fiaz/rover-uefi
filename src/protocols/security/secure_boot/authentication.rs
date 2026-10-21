//! # Authenticated variables
//!
//! Authentication structures for authenticated variable writes.
//!
//! The first descriptor contains a monotonic count and a
//! certificate envelope. The second descriptor contains a
//! timestamp and a certificate envelope. Parsing checks the
//! win-certificate length and revision fields; a type
//! identifier alone is never treated as proof of validity.

use crate::base::{EfiGuid, EfiTime};

/// The certificate type for EFI GUID certificates.
pub const WIN_CERT_TYPE_EFI_GUID: u16 = 0x0EF1;

/// The certificate type for PKCS signed data.
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;

/// The certificate type for the EFI PKCS1.15 envelope.
pub const WIN_CERT_TYPE_EFI_PKCS115: u16 = 0x0EF0;

/// The current win-certificate revision.
pub const WIN_CERT_REVISION: u16 = 0x0200;

/// The RSA-2048 with SHA-256 certificate type `GUID`.
pub const EFI_CERT_TYPE_RSA2048_SHA256_GUID: EfiGuid = EfiGuid::new(
    0xA771_7414,
    0xC616,
    0x4977,
    [0x94, 0x20, 0x84, 0x47, 0x12, 0xA7, 0x35, 0xBF],
);

/// A win-certificate header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct WinCertificate {
    /// The full certificate length in bytes.
    pub length: u32,
    /// The revision.
    pub revision: u16,
    /// The certificate type.
    pub certificate_type: u16,
}

/// The authenticated variable descriptor with monotonic count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiVariableAuthentication {
    /// The monotonic count for replay protection.
    pub monotonic_count: u64,
    /// The certificate envelope.
    pub auth_info: WinCertificate,
    /// The certificate type `GUID`.
    pub cert_type: EfiGuid,
}

/// The time-based authenticated variable descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiVariableAuthentication2 {
    /// The timestamp in universal time.
    pub timestamp: EfiTime,
    /// The certificate envelope.
    pub auth_info: WinCertificate,
    /// The certificate type `GUID`.
    pub cert_type: EfiGuid,
}

/// Parses a win-certificate header from bytes.
///
/// Returns `None` when the slice is shorter than the header,
/// the length is smaller than the header or exceeds the slice,
/// or the revision is not the current value.
#[must_use]
pub fn parse_win_certificate(bytes: &[u8]) -> Option<WinCertificate> {
    if bytes.len() < 8 {
        return None;
    }
    let mut length_raw = [0_u8; 4];
    length_raw.copy_from_slice(bytes.get(0..4)?);
    let length = u32::from_le_bytes(length_raw);
    let length_usize = usize::try_from(length).ok()?;
    if length_usize < 8 || length_usize > bytes.len() {
        return None;
    }
    let mut revision_raw = [0_u8; 2];
    revision_raw.copy_from_slice(bytes.get(4..6)?);
    let revision = u16::from_le_bytes(revision_raw);
    if revision != WIN_CERT_REVISION {
        return None;
    }
    let mut type_raw = [0_u8; 2];
    type_raw.copy_from_slice(bytes.get(6..8)?);
    let certificate_type = u16::from_le_bytes(type_raw);
    Some(WinCertificate {
        length,
        revision,
        certificate_type,
    })
}

/// Parses the time-based descriptor header.
///
/// Returns the timestamp bytes and the win-certificate when
/// both fit within the slice.
#[must_use]
pub fn parse_authentication_2(bytes: &[u8]) -> Option<EfiVariableAuthentication2> {
    if bytes.len() < 40 {
        return None;
    }
    let mut time_bytes = [0_u8; 16];
    time_bytes.copy_from_slice(bytes.get(0..16)?);
    let mut year_raw = [0_u8; 2];
    year_raw.copy_from_slice(time_bytes.get(0..2)?);
    let timestamp = EfiTime {
        year: u16::from_le_bytes(year_raw),
        month: time_bytes[2],
        day: time_bytes[3],
        hour: time_bytes[4],
        minute: time_bytes[5],
        second: time_bytes[6],
        pad1: time_bytes[7],
        nanosecond: u32::from_le_bytes([
            time_bytes[8],
            time_bytes[9],
            time_bytes[10],
            time_bytes[11],
        ]),
        timezone: i16::from_le_bytes([time_bytes[12], time_bytes[13]]),
        daylight: time_bytes[14],
        pad2: time_bytes[15],
    };
    let cert = parse_win_certificate(bytes.get(16..)?)?;
    if cert.certificate_type != WIN_CERT_TYPE_EFI_GUID {
        return None;
    }
    let mut guid_bytes = [0_u8; 16];
    guid_bytes.copy_from_slice(bytes.get(24..40)?);
    Some(EfiVariableAuthentication2 {
        timestamp,
        auth_info: cert,
        cert_type: EfiGuid::from_bytes(guid_bytes),
    })
}

/// Returns the full descriptor size for the monotonic form.
#[must_use]
pub const fn authentication_size() -> usize {
    8 + 8 + 16
}

/// Returns the full descriptor size for the time-based form.
#[must_use]
pub const fn authentication_2_size() -> usize {
    16 + 8 + 16
}
