//! IDE Controller Initialization protocol integration tests.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::storage::ide_controller::{
    EFI_ATAPI_DEVICE_IDENTIFY_DATA, EFI_IDE_CONTROLLER_INIT_PROTOCOL_GUID, EFI_SATA_AUTO_SPEED,
    EFI_SATA_GEN1_SPEED, EFI_SATA_GEN2_SPEED, EFI_SATA_GEN3_SPEED, EfiAtaCollectiveMode,
    EfiAtaExtTransferProtocol, EfiAtaExtendedMode, EfiAtaMode, EfiIdeControllerEnumPhase,
    EfiIdeControllerInitProtocol, EfiIdeIdentifyData, IdeControllerInit,
};

#[test]
fn test_ide_controller_guid_and_constants() {
    assert_eq!(
        EFI_IDE_CONTROLLER_INIT_PROTOCOL_GUID,
        EfiGuid::new(
            0xA1E3_7052,
            0x80D9,
            0x4E65,
            [0xA3, 0x17, 0x3E, 0x9A, 0x55, 0xC4, 0x3E, 0xC9]
        )
    );
    assert_eq!(EFI_SATA_AUTO_SPEED, 0);
    assert_eq!(EFI_SATA_GEN1_SPEED, 1);
    assert_eq!(EFI_SATA_GEN2_SPEED, 2);
    assert_eq!(EFI_SATA_GEN3_SPEED, 3);
    assert_eq!(EFI_ATAPI_DEVICE_IDENTIFY_DATA, 0x8000);
}

#[test]
fn test_ide_controller_enum_phase_values() {
    assert_eq!(
        EfiIdeControllerEnumPhase::BeforeChannelEnumeration as u32,
        0
    );
    assert_eq!(EfiIdeControllerEnumPhase::AfterChannelEnumeration as u32, 1);
    assert_eq!(EfiIdeControllerEnumPhase::BeforeChannelReset as u32, 2);
    assert_eq!(EfiIdeControllerEnumPhase::AfterChannelReset as u32, 3);
    assert_eq!(
        EfiIdeControllerEnumPhase::BusBeforeDevicePresenceDetection as u32,
        4
    );
    assert_eq!(
        EfiIdeControllerEnumPhase::BusAfterDevicePresenceDetection as u32,
        5
    );
    assert_eq!(EfiIdeControllerEnumPhase::ResetMode as u32, 6);
    assert_eq!(EfiIdeControllerEnumPhase::BusPhaseMaximum as u32, 7);
}

#[test]
fn test_ide_modes_default() {
    let collective = EfiAtaCollectiveMode::default();
    assert!(!collective.pio_mode.valid);
    assert_eq!(collective.pio_mode.mode, 0);
    assert_eq!(collective.ext_mode_count, 0);
    assert_eq!(
        collective.ext_mode[0].transfer_protocol,
        EfiAtaExtTransferProtocol::Sata
    );
    assert_eq!(collective.ext_mode[0].mode, EFI_SATA_AUTO_SPEED);
}

unsafe extern "efiapi" fn mock_get_channel_info(
    _this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    enabled: *mut bool,
    max_devices: *mut u8,
) -> EfiStatus {
    if enabled.is_null() || max_devices.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if channel == 0 {
        unsafe {
            *enabled = true;
            *max_devices = 2;
        }
        EfiStatus::SUCCESS
    } else {
        unsafe {
            *enabled = false;
            *max_devices = 0;
        }
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_notify_phase(
    _this: *const EfiIdeControllerInitProtocol,
    _phase: EfiIdeControllerEnumPhase,
    channel: u8,
) -> EfiStatus {
    if channel > 1 {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_submit_data(
    _this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    _device: u8,
    identify_data: *const EfiIdeIdentifyData,
) -> EfiStatus {
    if channel > 1 {
        return EfiStatus::INVALID_PARAMETER;
    }
    if identify_data.is_null() {
        EfiStatus::NOT_FOUND
    } else {
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_disqualify_mode(
    _this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    _device: u8,
    bad_modes: *const EfiAtaCollectiveMode,
) -> EfiStatus {
    if channel > 1 || bad_modes.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

static mut CALC_MODE_STORAGE: EfiAtaCollectiveMode = EfiAtaCollectiveMode {
    pio_mode: EfiAtaMode {
        valid: true,
        mode: 4,
    },
    single_word_dma_mode: EfiAtaMode {
        valid: false,
        mode: 0,
    },
    multi_word_dma_mode: EfiAtaMode {
        valid: true,
        mode: 2,
    },
    udma_mode: EfiAtaMode {
        valid: true,
        mode: 6,
    },
    ext_mode_count: 1,
    ext_mode: [EfiAtaExtendedMode {
        transfer_protocol: EfiAtaExtTransferProtocol::Sata,
        mode: EFI_SATA_GEN3_SPEED,
    }],
};

unsafe extern "efiapi" fn mock_calculate_mode(
    _this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    _device: u8,
    supported_modes: *mut *mut EfiAtaCollectiveMode,
) -> EfiStatus {
    if channel > 1 || supported_modes.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *supported_modes = core::ptr::addr_of_mut!(CALC_MODE_STORAGE);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_timing(
    _this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    _device: u8,
    modes: *const EfiAtaCollectiveMode,
) -> EfiStatus {
    if channel > 1 || modes.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

#[test]
fn test_ide_controller_wrapper() {
    let raw_protocol = EfiIdeControllerInitProtocol {
        get_channel_info: mock_get_channel_info,
        notify_phase: mock_notify_phase,
        submit_data: mock_submit_data,
        disqualify_mode: mock_disqualify_mode,
        calculate_mode: mock_calculate_mode,
        set_timing: mock_set_timing,
        enum_all: false,
        channel_count: 2,
    };

    let client = IdeControllerInit::new(&raw_protocol);
    assert_eq!(client.channel_count(), 2);
    assert!(!client.enum_all());

    // Channel 0 info
    let (enabled, max_dev) = client.channel_info(0).unwrap_or_default();
    assert!(enabled);
    assert_eq!(max_dev, 2);

    // Channel 1 info fails
    assert_eq!(client.channel_info(1), Err(EfiStatus::NOT_FOUND));

    // Notify phase
    assert_eq!(
        client.notify_phase(EfiIdeControllerEnumPhase::BeforeChannelEnumeration, 0),
        Ok(())
    );
    assert_eq!(
        client.notify_phase(EfiIdeControllerEnumPhase::BeforeChannelEnumeration, 5),
        Err(EfiStatus::INVALID_PARAMETER)
    );

    // Submit data
    let ident = EfiIdeIdentifyData { config: 0x0040 };
    assert_eq!(client.submit_data(0, 0, Some(&ident)), Ok(()));
    assert_eq!(client.submit_data(0, 1, None), Err(EfiStatus::NOT_FOUND));

    // Disqualify mode
    let bad_modes = EfiAtaCollectiveMode::default();
    assert_eq!(client.disqualify_mode(0, 0, &bad_modes), Ok(()));

    // Calculate mode
    let calc = match unsafe { client.calculate_mode(0, 0) } {
        Ok(ptr) => ptr,
        Err(_) => core::ptr::null_mut(),
    };
    assert!(!calc.is_null());
    let calc_ref = unsafe { &*calc };
    assert!(calc_ref.pio_mode.valid);
    assert_eq!(calc_ref.pio_mode.mode, 4);
    assert_eq!(calc_ref.udma_mode.mode, 6);
    assert_eq!(calc_ref.ext_mode[0].mode, EFI_SATA_GEN3_SPEED);

    // Set timing
    assert_eq!(client.set_timing(0, 0, calc_ref), Ok(()));
}

#[test]
fn test_ide_from_raw() {
    assert!(unsafe { IdeControllerInit::from_raw(core::ptr::null()) }.is_none());
}
