//! Integration tests for SD/MMC Override Protocol.
#![allow(clippy::expect_used)]

use core::ffi::c_void;
use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::driver::sd_mmc_override::{
    EmmcDriverStrengthType, SD_MMC_BUS_WIDTH_IGNORE, SD_MMC_CLOCK_FREQ_IGNORE,
    SD_MMC_DRIVER_STRENGTH_IGNORE, SD_MMC_OVERRIDE_PROTOCOL_GUID, SD_MMC_OVERRIDE_PROTOCOL_VERSION,
    SdDriverStrengthType, SdMmcBusMode, SdMmcOperatingParameters, SdMmcOverride,
    SdMmcOverrideProtocol, SdMmcPhaseType,
};

static mut NOTIFIED_PHASE: Option<SdMmcPhaseType> = None;

unsafe extern "efiapi" fn mock_sd_mmc_capability(
    _controller: EfiHandle,
    _slot: u8,
    _capability_struct: *mut c_void,
    base_clk_freq: *mut u32,
) -> EfiStatus {
    if !base_clk_freq.is_null() {
        unsafe {
            *base_clk_freq = 200;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sd_mmc_notify_phase(
    _controller: EfiHandle,
    _slot: u8,
    phase_type: SdMmcPhaseType,
    _phase_data: *mut c_void,
) -> EfiStatus {
    unsafe {
        NOTIFIED_PHASE = Some(phase_type);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_sd_mmc_override_constants_and_types() {
    assert_eq!(
        SD_MMC_OVERRIDE_PROTOCOL_GUID.to_string(),
        "eaf9e3c1-c9cd-46db-a5e5-5a124c832323"
    );
    assert_eq!(SD_MMC_OVERRIDE_PROTOCOL_VERSION, 3);
    assert_eq!(SD_MMC_BUS_WIDTH_IGNORE, 0xFF);
    assert_eq!(SD_MMC_CLOCK_FREQ_IGNORE, 0xFFFF_FFFF);
    assert_eq!(SD_MMC_DRIVER_STRENGTH_IGNORE, 0xFF);

    assert_eq!(SdDriverStrengthType::TypeB as u8, 0);
    assert_eq!(SdDriverStrengthType::Ignore as u8, 0xFF);
    assert_eq!(EmmcDriverStrengthType::Type0 as u8, 0);
    assert_eq!(EmmcDriverStrengthType::Ignore as u8, 0xFF);

    assert_eq!(SdMmcBusMode::SdDs as u32, 0);
    assert_eq!(SdMmcBusMode::UhsSdr104 as u32, 6);
    assert_eq!(SdMmcBusMode::MmcHs400 as u32, 11);

    let default_params = SdMmcOperatingParameters::default();
    assert_eq!(default_params.bus_width, SD_MMC_BUS_WIDTH_IGNORE);
    assert_eq!(default_params.clock_freq, SD_MMC_CLOCK_FREQ_IGNORE);
    assert_eq!(
        default_params.driver_strength,
        SD_MMC_DRIVER_STRENGTH_IGNORE
    );
}

#[test]
fn test_sd_mmc_override_wrapper() {
    let mut raw_proto = SdMmcOverrideProtocol {
        version: SD_MMC_OVERRIDE_PROTOCOL_VERSION,
        capability: mock_sd_mmc_capability,
        notify_phase: mock_sd_mmc_notify_phase,
    };

    let mut client =
        unsafe { SdMmcOverride::from_raw(&raw mut raw_proto) }.expect("valid sd mmc override");

    assert_eq!(client.version(), 3);

    let mut clk = 0u32;
    let cap_res =
        unsafe { client.override_capability(EfiHandle::NULL, 0, core::ptr::null_mut(), &mut clk) };
    assert!(cap_res.is_ok());
    assert_eq!(clk, 200);

    let notify_res = unsafe {
        client.notify_phase(
            EfiHandle::NULL,
            0,
            SdMmcPhaseType::UhsSignaling,
            core::ptr::null_mut(),
        )
    };
    assert!(notify_res.is_ok());
    assert_eq!(
        unsafe { NOTIFIED_PHASE },
        Some(SdMmcPhaseType::UhsSignaling)
    );
}
