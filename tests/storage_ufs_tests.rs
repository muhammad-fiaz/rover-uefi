//! Universal Flash Storage (`UFS`) protocol integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::storage::ufs::{
    EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID, EfiUfsDeviceConfigProtocol, EfiUfsHostControllerProtocol,
    UFS_HC_PLATFORM_PROTOCOL_GUID, UFS_HOST_CONTROLLER_PROTOCOL_GUID, UfsCardRefClkFreq,
    UfsDeviceConfig, UfsHcDriverInterface, UfsHcInfo, UfsHcOperation, UfsHcPlatform,
    UfsHcPlatformCallbackPhase, UfsHcPlatformProtocol, UfsHcWidth, UfsHostController, UicCommand,
};

#[test]
fn test_ufs_guids() {
    let expected_dev_config = EfiGuid::new(
        0xB81B_FAB0,
        0x0EB3,
        0x4CF9,
        [0x84, 0x65, 0x7F, 0xA9, 0x86, 0x36, 0x16, 0x64],
    );
    assert_eq!(EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID, expected_dev_config);

    let expected_hc = EfiGuid::new(
        0xEBC0_1AF5,
        0x07A9,
        0x489E,
        [0xB7, 0xCE, 0xDC, 0x08, 0x9E, 0x45, 0x9B, 0x2F],
    );
    assert_eq!(UFS_HOST_CONTROLLER_PROTOCOL_GUID, expected_hc);

    let expected_platform = EfiGuid::new(
        0x3D18_BA13,
        0xD9B1,
        0x4DD4,
        [0xB9, 0x16, 0xD3, 0x07, 0x96, 0x53, 0x9E, 0xD8],
    );
    assert_eq!(UFS_HC_PLATFORM_PROTOCOL_GUID, expected_platform);
}

#[test]
fn test_ufs_enums_and_structures() {
    assert_eq!(UfsHcOperation::BusMasterRead as u32, 0);
    assert_eq!(UfsHcOperation::BusMasterWrite as u32, 1);
    assert_eq!(UfsHcOperation::BusMasterCommonBuffer as u32, 2);

    assert_eq!(UfsHcWidth::Uint8 as u32, 0);
    assert_eq!(UfsHcWidth::Uint16 as u32, 1);
    assert_eq!(UfsHcWidth::Uint32 as u32, 2);
    assert_eq!(UfsHcWidth::Uint64 as u32, 3);

    assert_eq!(UfsHcPlatformCallbackPhase::PreHce as u32, 0);
    assert_eq!(UfsHcPlatformCallbackPhase::PostHce as u32, 1);
    assert_eq!(UfsHcPlatformCallbackPhase::PreLinkStartup as u32, 2);
    assert_eq!(UfsHcPlatformCallbackPhase::PostLinkStartup as u32, 3);

    assert_eq!(UfsCardRefClkFreq::Freq19p2Mhz as u32, 0);
    assert_eq!(UfsCardRefClkFreq::Freq26Mhz as u32, 1);
    assert_eq!(UfsCardRefClkFreq::Freq38p4Mhz as u32, 2);
    assert_eq!(UfsCardRefClkFreq::Obsolete as u32, 3);

    let info = UfsHcInfo {
        capabilities: 0x1234_5678,
        version: 0x0003_0001,
    };
    assert_eq!(info.capabilities, 0x1234_5678);
    assert_eq!(info.version, 0x0003_0001);

    let cmd = UicCommand {
        opcode: 0x10,
        arg1: 1,
        arg2: 2,
        arg3: 3,
    };
    assert_eq!(cmd.opcode, 0x10);
    assert_eq!(cmd.arg1, 1);
    assert_eq!(cmd.arg2, 2);
    assert_eq!(cmd.arg3, 3);
}

unsafe extern "efiapi" fn mock_rw_flag(
    _this: *mut EfiUfsDeviceConfigProtocol,
    read: bool,
    _flag_id: u8,
    flag: *mut u8,
) -> EfiStatus {
    if flag.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if read {
            *flag = 1;
        } else {
            *flag = 0;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_ufs_device_config_wrapper() {
    let mut mock_proto = EfiUfsDeviceConfigProtocol {
        rw_ufs_descriptor: None,
        rw_ufs_flag: Some(mock_rw_flag),
        rw_ufs_attribute: None,
    };

    let mut dev_cfg = UfsDeviceConfig::new(&mut mock_proto);
    let Ok(val) = dev_cfg.read_flag(0x05) else {
        panic!("failed to read flag");
    };
    assert_eq!(val, 1);

    let res = dev_cfg.write_flag(0x05, 0);
    assert_eq!(res, Ok(()));
}

unsafe extern "efiapi" fn mock_get_mmio_bar(
    _this: *mut EfiUfsHostControllerProtocol,
    mmio_bar: *mut UIntN,
) -> EfiStatus {
    if mmio_bar.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *mmio_bar = 0xFED0_0000;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_flush(_this: *mut EfiUfsHostControllerProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unmap(
    _this: *mut EfiUfsHostControllerProtocol,
    _mapping: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_ufs_host_controller_wrapper() {
    let mut mock_proto = EfiUfsHostControllerProtocol {
        get_ufs_hc_mmio_bar: Some(mock_get_mmio_bar),
        allocate_buffer: None,
        free_buffer: None,
        map: None,
        unmap: Some(mock_unmap),
        flush: Some(mock_flush),
        read: None,
        write: None,
    };

    let mut hc = UfsHostController::new(&mut mock_proto);
    let Ok(bar) = hc.mmio_bar() else {
        panic!("failed to get mmio bar");
    };
    assert_eq!(bar, 0xFED0_0000);

    let res_flush = hc.flush();
    assert_eq!(res_flush, Ok(()));

    let res_unmap = unsafe { hc.unmap(core::ptr::null_mut()) };
    assert_eq!(res_unmap, Ok(()));
}

#[test]
fn test_ufs_platform_wrapper() {
    let mock_proto = UfsHcPlatformProtocol {
        version: 3,
        override_hc_info: None,
        callback: None,
        ref_clk_freq: UfsCardRefClkFreq::Freq38p4Mhz,
        skip_hce: false,
        skip_link_startup: true,
    };

    let platform = UfsHcPlatform::new(&mock_proto);
    assert_eq!(platform.version(), 3);
    assert_eq!(platform.ref_clk_freq(), UfsCardRefClkFreq::Freq38p4Mhz);

    let mut mock_hc_proto = EfiUfsHostControllerProtocol {
        get_ufs_hc_mmio_bar: None,
        allocate_buffer: None,
        free_buffer: None,
        map: None,
        unmap: None,
        flush: None,
        read: None,
        write: None,
    };

    let driver_iface = UfsHcDriverInterface {
        ufs_hc_protocol: &raw mut mock_hc_proto,
        ufs_exec_uic_command: None,
    };
    assert!(!driver_iface.ufs_hc_protocol.is_null());
}
