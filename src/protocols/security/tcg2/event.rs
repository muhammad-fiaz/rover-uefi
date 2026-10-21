//! # TCG events
//!
//! Event header definitions for measurement submission.

/// The TCG2 event header version.
pub const EVENT_HEADER_VERSION: u16 = 1;

/// The TCG2 event header.
///
/// The structure is packed in the specification with size 12.
/// Rust field order matches the wire order; parsing of wire
/// bytes uses explicit byte offsets in the event log module
/// so unaligned input is never dereferenced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTcg2EventHeader {
    /// The header size.
    pub header_size: u32,
    /// The header version.
    pub header_version: u16,
    /// The PCR index.
    pub pcr_index: u32,
    /// The event type.
    pub event_type: u32,
}

/// The TCG2 event template header.
///
/// The total size field covers the whole event including the
/// size component, the header and the event data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTcg2Event {
    /// The total event size.
    pub size: u32,
    /// The event header.
    pub header: EfiTcg2EventHeader,
}

/// Standard event type for pre-boot certification.
pub const EV_PREBOOT_CERT: u32 = 0x0000_0000;

/// Standard event type for post code.
pub const EV_POST_CODE: u32 = 0x0000_0001;

/// Standard event type for no action.
pub const EV_NO_ACTION: u32 = 0x0000_0003;

/// Standard event type for separator.
pub const EV_SEPARATOR: u32 = 0x0000_0004;

/// Standard event type for action.
pub const EV_ACTION: u32 = 0x0000_0005;

/// Base of the firmware-specific event types.
pub const EV_EFI_EVENT_BASE: u32 = 0x8000_0000;

/// Returns whether an event type is in the firmware range.
#[must_use]
pub fn is_efi_event_type(event_type: u32) -> bool {
    event_type >= EV_EFI_EVENT_BASE
}
