//! # Firmware Management Progress Protocol
//!
//! Provides configuration for visual progress bar presentation and watchdog timer
//! monitoring during firmware capsule flashing and FMP `SetImage()` operations.

use crate::base::EfiGuid;
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

/// Firmware Management Progress Protocol GUID:
/// `{1849bda2-6952-4e86-a1db-559a3c479df1}`.
pub const FIRMWARE_MANAGEMENT_PROGRESS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1849_BDA2,
    0x6952,
    0x4E86,
    [0xA1, 0xDB, 0x55, 0x9A, 0x3C, 0x47, 0x9D, 0xF1],
);

/// Initial structure revision version for `FirmwareManagementProgressProtocol`.
pub const FIRMWARE_MANAGEMENT_PROGRESS_VERSION: u32 = 0x0000_0001;

/// Firmware Management Progress Protocol interface.
///
/// Contains display styling and watchdog configuration for progress reporting
/// during firmware update operations.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirmwareManagementProgressProtocol {
    /// Structure version, typically `FIRMWARE_MANAGEMENT_PROGRESS_VERSION` (1).
    pub version: u32,
    /// Foreground color of the progress bar rendered during `SetImage()` progress calls.
    pub progress_bar_foreground_color: EfiGraphicsOutputBltPixel,
    /// Number of seconds to arm the watchdog timer on each progress notification.
    pub watchdog_seconds: usize,
}

impl Default for FirmwareManagementProgressProtocol {
    fn default() -> Self {
        Self {
            version: FIRMWARE_MANAGEMENT_PROGRESS_VERSION,
            progress_bar_foreground_color: EfiGraphicsOutputBltPixel {
                blue: 0xFF,
                green: 0xFF,
                red: 0xFF,
                reserved: 0x00,
            },
            watchdog_seconds: 0,
        }
    }
}

/// High-level client wrapper for inspecting and configuring the Firmware
/// Management Progress Protocol.
#[derive(Debug)]
pub struct FirmwareManagementProgress<'a> {
    protocol: &'a mut FirmwareManagementProgressProtocol,
}

impl<'a> FirmwareManagementProgress<'a> {
    /// Wraps a mutable reference to the raw protocol.
    pub fn new(protocol: &'a mut FirmwareManagementProgressProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the structure revision version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.protocol.version
    }

    /// Returns the current progress bar foreground color.
    #[must_use]
    pub const fn foreground_color(&self) -> EfiGraphicsOutputBltPixel {
        self.protocol.progress_bar_foreground_color
    }

    /// Sets the progress bar foreground color.
    pub fn set_foreground_color(&mut self, color: EfiGraphicsOutputBltPixel) {
        self.protocol.progress_bar_foreground_color = color;
    }

    /// Returns the watchdog timer interval in seconds.
    #[must_use]
    pub const fn watchdog_seconds(&self) -> usize {
        self.protocol.watchdog_seconds
    }

    /// Sets the watchdog timer interval in seconds.
    pub fn set_watchdog_seconds(&mut self, seconds: usize) {
        self.protocol.watchdog_seconds = seconds;
    }
}
