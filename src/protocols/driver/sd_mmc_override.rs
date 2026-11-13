//! # SD/MMC Host Controller Override Protocol
//!
//! Provides platform capability hooks and phase notification overrides for non-standard
//! SDHCI, eMMC, and SD card host controller hardware implementations.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// SD/MMC Override Protocol GUID: `{eaf9e3c1-c9cd-46db-a5e5-5a124c832323}`.
pub const SD_MMC_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEAF9_E3C1,
    0xC9CD,
    0x46DB,
    [0xA5, 0xE5, 0x5A, 0x12, 0x4C, 0x83, 0x23, 0x23],
);

/// Current revision of the SD/MMC Override Protocol interface.
pub const SD_MMC_OVERRIDE_PROTOCOL_VERSION: usize = 0x3;

/// Magic value instructing driver to ignore explicit bus width constraint.
pub const SD_MMC_BUS_WIDTH_IGNORE: u8 = u8::MAX;
/// Magic value instructing driver to ignore explicit clock frequency constraint.
pub const SD_MMC_CLOCK_FREQ_IGNORE: u32 = u32::MAX;
/// Magic value instructing driver to ignore explicit driver strength constraint.
pub const SD_MMC_DRIVER_STRENGTH_IGNORE: u8 = u8::MAX;

/// SD card driver strength type.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdDriverStrengthType {
    /// Type B driver impedance (nominal 50 ohm).
    TypeB = 0,
    /// Type A driver impedance (33 ohm, high drive).
    TypeA = 1,
    /// Type C driver impedance (66 ohm, lower drive).
    TypeC = 2,
    /// Type D driver impedance (100 ohm, weak drive).
    TypeD = 3,
    /// Do not override driver strength.
    Ignore = SD_MMC_DRIVER_STRENGTH_IGNORE,
}

/// eMMC driver strength type.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmmcDriverStrengthType {
    /// Type 0 driver impedance (nominal 50 ohm).
    Type0 = 0,
    /// Type 1 driver impedance (33 ohm).
    Type1 = 1,
    /// Type 2 driver impedance (66 ohm).
    Type2 = 2,
    /// Type 3 driver impedance (100 ohm).
    Type3 = 3,
    /// Type 4 driver impedance (40 ohm).
    Type4 = 4,
    /// Do not override driver strength.
    Ignore = SD_MMC_DRIVER_STRENGTH_IGNORE,
}

/// Target operating parameters for an SD/MMC bus channel.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SdMmcOperatingParameters {
    /// Target width of the data bus (1, 4, or 8 bits, or `SD_MMC_BUS_WIDTH_IGNORE`).
    pub bus_width: u8,
    /// Target clock frequency in MHz, or `SD_MMC_CLOCK_FREQ_IGNORE`.
    pub clock_freq: u32,
    /// Target driver strength setting (raw byte value), or `SD_MMC_DRIVER_STRENGTH_IGNORE`.
    pub driver_strength: u8,
}

impl Default for SdMmcOperatingParameters {
    fn default() -> Self {
        Self {
            bus_width: SD_MMC_BUS_WIDTH_IGNORE,
            clock_freq: SD_MMC_CLOCK_FREQ_IGNORE,
            driver_strength: SD_MMC_DRIVER_STRENGTH_IGNORE,
        }
    }
}

/// SD/MMC bus operating mode and signaling timing.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdMmcBusMode {
    /// SD Default Speed (up to 25 MHz, 3.3V).
    SdDs = 0,
    /// SD High Speed (up to 50 MHz, 3.3V).
    SdHs = 1,
    /// UHS-I SDR12 (up to 25 MHz, 1.8V).
    UhsSdr12 = 2,
    /// UHS-I SDR25 (up to 50 MHz, 1.8V).
    UhsSdr25 = 3,
    /// UHS-I SDR50 (up to 100 MHz, 1.8V).
    UhsSdr50 = 4,
    /// UHS-I DDR50 (up to 50 MHz DDR, 1.8V).
    UhsDdr50 = 5,
    /// UHS-I SDR104 (up to 208 MHz, 1.8V).
    UhsSdr104 = 6,
    /// eMMC Legacy timing (up to 26 MHz).
    MmcLegacy = 7,
    /// eMMC High Speed SDR (up to 52 MHz).
    MmcHsSdr = 8,
    /// eMMC High Speed DDR (up to 52 MHz DDR).
    MmcHsDdr = 9,
    /// eMMC HS200 (up to 200 MHz SDR, 1.8V/1.2V).
    MmcHs200 = 10,
    /// eMMC HS400 (up to 200 MHz DDR, 1.8V/1.2V).
    MmcHs400 = 11,
}

/// Phase notification types for SD/MMC host controller operations.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdMmcPhaseType {
    /// Invoked immediately prior to host controller soft reset.
    ResetPre = 0,
    /// Invoked immediately after host controller soft reset.
    ResetPost = 1,
    /// Invoked immediately prior to host controller initialization.
    InitHostPre = 2,
    /// Invoked immediately after host controller initialization.
    InitHostPost = 3,
    /// Invoked to configure UHS-I voltage signaling (e.g. 1.8V switch).
    UhsSignaling = 4,
    /// Invoked after switching clock frequency.
    SwitchClockFreqPost = 5,
    /// Invoked to query platform-specified operating parameters.
    GetOperatingParam = 6,
}

/// Function pointer type for overriding SDHCI host controller capabilities.
pub type SdMmcCapabilityFn = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    slot: u8,
    sd_mmc_hc_slot_capability: *mut c_void,
    base_clk_freq: *mut u32,
) -> EfiStatus;

/// Function pointer type for notifying platform hook of controller phases.
pub type SdMmcNotifyPhaseFn = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    slot: u8,
    phase_type: SdMmcPhaseType,
    phase_data: *mut c_void,
) -> EfiStatus;

/// Raw SD/MMC Override Protocol interface table.
#[repr(C)]
pub struct SdMmcOverrideProtocol {
    /// Protocol implementation version.
    pub version: usize,
    /// Capability override hook.
    pub capability: SdMmcCapabilityFn,
    /// Phase notification hook.
    pub notify_phase: SdMmcNotifyPhaseFn,
}

/// Safe client wrapper around [`SdMmcOverrideProtocol`].
pub struct SdMmcOverride<'a> {
    raw: &'a mut SdMmcOverrideProtocol,
}

impl SdMmcOverride<'_> {
    /// Wraps a raw [`SdMmcOverrideProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut SdMmcOverrideProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Returns the protocol version.
    #[must_use]
    pub fn version(&self) -> usize {
        self.raw.version
    }

    /// Invokes the capability override hook for a slot on the specified controller.
    ///
    /// # Safety
    ///
    /// `capability_struct` must point to valid memory for the SDHCI slot capability register.
    ///
    /// # Errors
    ///
    /// Returns the status code reported by the underlying platform capability override routine.
    pub unsafe fn override_capability(
        &mut self,
        controller: EfiHandle,
        slot: u8,
        capability_struct: *mut c_void,
        base_clk_freq: &mut u32,
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.raw.capability)(controller, slot, capability_struct, base_clk_freq) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Invokes the phase notification hook for a slot on the specified controller.
    ///
    /// # Safety
    ///
    /// If `phase_data` is required for `phase_type`, it must point to an appropriate data structure.
    ///
    /// # Errors
    ///
    /// Returns the status code reported by the underlying platform phase notification hook.
    pub unsafe fn notify_phase(
        &mut self,
        controller: EfiHandle,
        slot: u8,
        phase_type: SdMmcPhaseType,
        phase_data: *mut c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.notify_phase)(controller, slot, phase_type, phase_data) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
