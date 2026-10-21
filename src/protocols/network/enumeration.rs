//! # Network enumeration
//!
//! Protocol discovery helpers.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus, LocateSearchType};
use crate::services::boot::BootServices;

/// Locates handles for a protocol.
///
/// Returns the handle count.
///
/// # Errors
///
/// Returns the firmware status when the
/// locate operation fails.
pub fn locate_handles(
    boot: &BootServices<'_>,
    guid: &EfiGuid,
    buffer: &mut [EfiHandle],
) -> Result<UIntN, EfiStatus> {
    boot.locate_handles(LocateSearchType::BY_PROTOCOL, Some(guid), buffer)
}
