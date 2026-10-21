//! # Time
//!
//! The `EFI_TIME` structure used by runtime services
//! for wall-clock time and wakeup timers.
//!
//! The structure mirrors the specification field order
//! and layout exactly: year, calendar fields, a pad
//! byte, nanoseconds, timezone offset, daylight flags
//! and a second pad byte. The total size is 16 bytes.
//!
//! The raw fields are exposed so firmware data can be
//! represented without transformation. Values are not
//! validated on construction; runtime services define
//! which ranges are meaningful and which combinations
//! are rejected.
//!
//! The module also defines the time capabilities
//! structure used by the get time runtime service.

use crate::abi::types::Bool;

/// The adjustment for daylight saving time.
///
/// When this bit is set, the clock is adjusted for
/// daylight saving time.
pub const EFI_TIME_ADJUST_DAYLIGHT: u8 = 0x01;

/// The in-daylight flag.
///
/// When this bit is set, the time is currently in
/// daylight saving time.
pub const EFI_TIME_IN_DAYLIGHT: u8 = 0x02;

/// The unspecified timezone value.
///
/// Firmware uses this value when the timezone is
/// unknown. The raw field value is `0x07FF`.
pub const EFI_UNSPECIFIED_TIMEZONE: i16 = 0x07FF;

/// The calendar time representation.
///
/// The struct is `#[repr(C)]` so field order, size
/// and alignment match the firmware definition.
/// All padding bytes are explicit fields so the
/// layout does not depend on compiler-inserted
/// padding.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTime {
    /// The year, with valid range `1900` to `9999`.
    pub year: u16,
    /// The month, with valid range `1` to `12`.
    pub month: u8,
    /// The day of the month, with valid range `1` to `31`.
    pub day: u8,
    /// The hour, with valid range `0` to `23`.
    pub hour: u8,
    /// The minute, with valid range `0` to `59`.
    pub minute: u8,
    /// The second, with valid range `0` to `59`.
    pub second: u8,
    /// Reserved padding byte that must be zero.
    pub pad1: u8,
    /// The nanosecond count, with valid range
    /// `0` to `999_999_999`.
    pub nanosecond: u32,
    /// The timezone offset in minutes from UTC.
    ///
    /// The range is `-1440` to `1440`, with
    /// [`EFI_UNSPECIFIED_TIMEZONE`] meaning unknown.
    pub timezone: i16,
    /// The daylight flags, using
    /// [`EFI_TIME_ADJUST_DAYLIGHT`] and
    /// [`EFI_TIME_IN_DAYLIGHT`].
    pub daylight: u8,
    /// Reserved padding byte that must be zero.
    pub pad2: u8,
}

impl core::fmt::Debug for EfiTime {
    /// Formats the time as its fields.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("EfiTime")
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("minute", &self.minute)
            .field("second", &self.second)
            .field("pad1", &self.pad1)
            .field("nanosecond", &self.nanosecond)
            .field("timezone", &self.timezone)
            .field("daylight", &self.daylight)
            .field("pad2", &self.pad2)
            .finish()
    }
}

/// Real-time clock capabilities.
///
/// The struct is `#[repr(C)]` so field order, size
/// and alignment match the firmware definition:
/// a resolution, an accuracy, a zeroing flag, and
/// padding to 12 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct EfiTimeCapabilities {
    /// The resolution in counts per second.
    pub resolution: u32,
    /// The accuracy in parts per million.
    pub accuracy: u32,
    /// Whether time setting clears sub-resolution
    /// state.
    pub sets_to_zero: Bool,
}
