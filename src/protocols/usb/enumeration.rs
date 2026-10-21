//! # USB enumeration
//!
//! Discovery through Boot Services.
//!
//! The helper locates USB I/O handles with
//! caller-provided storage.

use crate::abi::types::UIntN;
use crate::base::{EfiHandle, EfiStatus, LocateSearchType};
use crate::protocols::usb::guid::USB_IO_GUID;
use crate::services::boot::BootServices;

/// Locates USB I/O handles.
///
/// Returns the handle count.
///
/// # Errors
///
/// Returns the firmware status when the
/// locate operation fails.
pub fn locate_usb_io(
    boot: &BootServices<'_>,
    buffer: &mut [EfiHandle],
) -> Result<UIntN, EfiStatus> {
    boot.locate_handles(LocateSearchType::BY_PROTOCOL, Some(&USB_IO_GUID), buffer)
}
