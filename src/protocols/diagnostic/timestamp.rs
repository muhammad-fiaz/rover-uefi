//! # Timestamp Protocol
//!
//! UEFI 2.4 Timestamp Protocol (`EFI_TIMESTAMP_PROTOCOL`).
//! Provides a platform-independent interface for retrieving a high-resolution timestamp counter.

use crate::base::{EfiGuid, EfiStatus};

/// Timestamp Protocol GUID: `afbfde41-2e6e-4262-ba65-62b9236e5495`
pub const TIMESTAMP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xafbf_de41,
    0x2e6e,
    0x4262,
    [0xba, 0x65, 0x62, 0xb9, 0x23, 0x6e, 0x54, 0x95],
);

/// Timestamp counter properties (frequency in Hz and rollover limit).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct EfiTimestampProperties {
    /// Frequency of the timestamp counter in Hz (ticks per second).
    pub frequency: u64,
    /// Value that the counter ends with immediately before rolling over (e.g. `0xFFFF_FFFF_FFFF_FFFF`).
    pub end_value: u64,
}

/// Function pointer type for `GetTimestamp`.
pub type EfiTimestampGet = unsafe extern "efiapi" fn() -> u64;

/// Function pointer type for `GetProperties`.
pub type EfiTimestampGetProperties =
    unsafe extern "efiapi" fn(properties: *mut EfiTimestampProperties) -> EfiStatus;

/// Raw UEFI Timestamp Protocol interface (`EFI_TIMESTAMP_PROTOCOL`).
#[repr(C)]
pub struct EfiTimestampProtocol {
    /// Retrieves current free-running timestamp counter value.
    pub get_timestamp: EfiTimestampGet,
    /// Retrieves properties (frequency and rollover end value).
    pub get_properties: EfiTimestampGetProperties,
}

/// Safe wrapper around `EfiTimestampProtocol`.
pub struct TimestampProtocol<'a> {
    raw: &'a mut EfiTimestampProtocol,
}

impl TimestampProtocol<'_> {
    /// Wraps a raw `EfiTimestampProtocol` instance.
    ///
    /// # Safety
    /// The caller must ensure that `raw` points to a valid, live protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut EfiTimestampProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // SAFETY: Caller guarantees `raw` points to a valid, live protocol instance.
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Reads the current high-resolution timestamp counter value.
    #[must_use]
    pub fn get_timestamp(&self) -> u64 {
        unsafe { (self.raw.get_timestamp)() }
    }

    /// Obtains timestamp properties (frequency in Hz and rollover end value).
    ///
    /// # Errors
    /// Returns `EfiStatus::INVALID_PARAMETER` if properties cannot be written,
    /// or `EfiStatus::DEVICE_ERROR` on hardware timer subsystem failure.
    pub fn get_properties(&mut self) -> Result<EfiTimestampProperties, EfiStatus> {
        let mut props = EfiTimestampProperties::default();
        let status = unsafe { (self.raw.get_properties)(&raw mut props) };
        if status == EfiStatus::SUCCESS {
            Ok(props)
        } else {
            Err(status)
        }
    }
}
