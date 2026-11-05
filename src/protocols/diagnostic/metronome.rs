//! # Metronome architectural protocol
//!
//! Defined in PI Specification Volume 2: DXE.
//!
//! Provides access to a known time source in the platform to the DXE core
//! for calibrated delays.

use crate::base::{EfiGuid, EfiStatus};

/// The Metronome architectural protocol `GUID`.
pub const METRONOME_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB2,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// The `WaitForTick` function pointer type.
pub type EfiMetronomeWaitForTick =
    unsafe extern "efiapi" fn(*mut EfiMetronomeArchProtocol, u32) -> EfiStatus;

/// The Metronome architectural protocol interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMetronomeArchProtocol {
    /// Waits for the specified number of ticks from the platform tick source.
    pub wait_for_tick: EfiMetronomeWaitForTick,
    /// The period of one platform tick in 100ns units.
    pub tick_period: u32,
}

/// A safe wrapper for [`EfiMetronomeArchProtocol`].
pub struct MetronomeArch<'a> {
    protocol: &'a EfiMetronomeArchProtocol,
}

impl<'a> MetronomeArch<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiMetronomeArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns a reference to the raw protocol structure.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiMetronomeArchProtocol {
        self.protocol
    }

    /// Returns the tick period in 100ns units.
    #[must_use]
    pub const fn tick_period(&self) -> u32 {
        self.protocol.tick_period
    }

    /// Waits for `tick_number` ticks.
    ///
    /// # Errors
    ///
    /// Returns [`EfiStatus::TIMEOUT`] if a timeout occurs waiting for ticks,
    /// or another firmware error status.
    pub fn wait_for_tick(&self, tick_number: u32) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status = unsafe { (self.protocol.wait_for_tick)(ptr, tick_number) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Debug for MetronomeArch<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MetronomeArch")
            .field("tick_period", &self.protocol.tick_period)
            .finish()
    }
}
