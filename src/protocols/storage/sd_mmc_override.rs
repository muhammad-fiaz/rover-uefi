//! # SD/`MMC` Override Protocol and Client API
//!
//! Provides platform hook overrides required to support non-standard `SDHCI`
//! implementations, mobile and embedded `SoCs`, and custom board timings
//! for `SD` and `eMMC` storage devices.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// SD/`MMC` Override Protocol GUID.
pub const SD_MMC_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEAF9_E3C1,
    0xC9CD,
    0x46DB,
    [0xA5, 0xE5, 0x5A, 0x12, 0x4C, 0x83, 0x23, 0x23],
);

/// SD/`MMC` Override Protocol Version.
pub const SD_MMC_OVERRIDE_PROTOCOL_VERSION: usize = 0x3;

/// Ignore bus width override flag.
pub const SD_MMC_BUS_WIDTH_IGNORE: u8 = u8::MAX;
/// Ignore clock frequency override flag.
pub const SD_MMC_CLOCK_FREQ_IGNORE: u32 = u32::MAX;
/// Ignore driver strength override flag.
pub const SD_MMC_DRIVER_STRENGTH_IGNORE: u8 = u8::MAX;

/// `SD` card driver strength types.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdDriverStrengthType {
    /// Type B (nominal driver impedance 50 ohm).
    TypeB = 0,
    /// Type A (nominal driver impedance 33 ohm).
    TypeA = 1,
    /// Type C (nominal driver impedance 66 ohm).
    TypeC = 2,
    /// Type D (nominal driver impedance 100 ohm).
    TypeD = 3,
    /// Ignore driver strength setting.
    Ignore = SD_MMC_DRIVER_STRENGTH_IGNORE,
}

/// `eMMC` driver strength types.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmmcDriverStrengthType {
    /// Type 0 (nominal driver impedance 50 ohm).
    Type0 = 0,
    /// Type 1 (nominal driver impedance 33 ohm).
    Type1 = 1,
    /// Type 2 (nominal driver impedance 66 ohm).
    Type2 = 2,
    /// Type 3 (nominal driver impedance 100 ohm).
    Type3 = 3,
    /// Type 4 (nominal driver impedance 40 ohm).
    Type4 = 4,
    /// Ignore driver strength setting.
    Ignore = SD_MMC_DRIVER_STRENGTH_IGNORE,
}

/// SD or `eMMC` Driver Strength Setting.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SdMmcDriverStrength {
    /// Raw driver strength value.
    pub raw: u8,
}

/// Operating parameters for SD/`MMC` bus.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SdMmcOperatingParameters {
    /// Target width of the bus (1, 4, or 8 bits, or ignore).
    pub bus_width: u8,
    /// Target clock frequency in MHz (or ignore).
    pub clock_freq: u32,
    /// Target driver strength.
    pub driver_strength: SdMmcDriverStrength,
}

/// SD/`MMC` Bus Speed Modes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdMmcBusMode {
    /// SD Default Speed (up to 25 MHz).
    SdDs = 0,
    /// SD High Speed (up to 50 MHz).
    SdHs = 1,
    /// SD `UHS-I` SDR12 (up to 25 MHz).
    UhsSdr12 = 2,
    /// SD `UHS-I` SDR25 (up to 50 MHz).
    UhsSdr25 = 3,
    /// SD `UHS-I` SDR50 (up to 100 MHz).
    UhsSdr50 = 4,
    /// SD `UHS-I` DDR50 (up to 50 MHz DDR).
    UhsDdr50 = 5,
    /// SD `UHS-I` SDR104 (up to 208 MHz).
    UhsSdr104 = 6,
    /// `eMMC` Legacy speed.
    MmcLegacy = 7,
    /// `eMMC` High Speed SDR (up to 52 MHz).
    MmcHsSdr = 8,
    /// `eMMC` High Speed DDR (up to 52 MHz DDR).
    MmcHsDdr = 9,
    /// `eMMC` HS200 (up to 200 MHz).
    MmcHs200 = 10,
    /// `eMMC` HS400 (up to 200 MHz DDR).
    MmcHs400 = 11,
}

/// Phase notification types during controller initialization and clock switching.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdMmcPhaseType {
    /// Prior to controller reset.
    ResetPre = 0,
    /// After controller reset.
    ResetPost = 1,
    /// Prior to host initialization.
    InitHostPre = 2,
    /// After host initialization.
    InitHostPost = 3,
    /// Prior to `UHS` 1.8V signaling voltage switch.
    UhsSignaling = 4,
    /// After switching clock frequency.
    SwitchClockFreqPost = 5,
    /// Query operating parameters for given bus mode.
    GetOperatingParam = 6,
}

/// Override function for `SDHCI` capability bits.
pub type SdMmcCapability = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    slot: u8,
    sd_mmc_hc_slot_capability: *mut c_void,
    base_clk_freq: *mut u32,
) -> EfiStatus;

/// Override notification function for `SDHCI` controller operations.
pub type SdMmcNotifyPhase = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    slot: u8,
    phase_type: SdMmcPhaseType,
    phase_data: *mut c_void,
) -> EfiStatus;

/// SD/`MMC` Override Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SdMmcOverrideProtocol {
    /// Protocol version (`SD_MMC_OVERRIDE_PROTOCOL_VERSION`).
    pub version: usize,
    /// Callback to override `SDHCI` slot capabilities.
    pub capability: Option<SdMmcCapability>,
    /// Callback to invoke platform phase hooks.
    pub notify_phase: Option<SdMmcNotifyPhase>,
}

/// Safe client-side wrapper for SD/`MMC` override operations.
#[derive(Debug)]
pub struct SdMmcOverride<'a> {
    protocol: &'a SdMmcOverrideProtocol,
}

impl<'a> SdMmcOverride<'a> {
    /// Create a new safe view of the SD/`MMC` override protocol.
    #[must_use]
    pub fn new(protocol: &'a SdMmcOverrideProtocol) -> Self {
        Self { protocol }
    }

    /// Return the protocol version.
    #[must_use]
    pub const fn version(&self) -> usize {
        self.protocol.version
    }

    /// Notify platform of an SD/`MMC` initialization phase transition.
    ///
    /// # Safety
    /// Caller must ensure `data` points to the expected payload structure for `phase` or is null.
    ///
    /// # Errors
    /// Returns an error if the platform phase transition hook fails.
    pub unsafe fn notify_phase(
        &self,
        controller: EfiHandle,
        slot: u8,
        phase: SdMmcPhaseType,
        data: *mut c_void,
    ) -> Result<(), EfiStatus> {
        if let Some(notify_fn) = self.protocol.notify_phase {
            let status = unsafe { notify_fn(controller, slot, phase, data) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Ok(())
        }
    }
}
