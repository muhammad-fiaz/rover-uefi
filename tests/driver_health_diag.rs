// SPDX-License-Identifier: Apache-2.0
//! Unit and integration tests for Driver Configuration, Diagnostics, Health, and Override protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::driver::{
    DriverConfiguration, DriverConfiguration2, DriverDiagnostics, DriverDiagnostics2, DriverHealth,
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_DRIVER_CONFIGURATION_CUSTOM_DEFAULTS,
    EFI_DRIVER_CONFIGURATION_MANUFACTURING_DEFAULTS, EFI_DRIVER_CONFIGURATION_PERFORMANCE_DEFAULTS,
    EFI_DRIVER_CONFIGURATION_PROTOCOL_GUID, EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS,
    EFI_DRIVER_CONFIGURATION2_PROTOCOL_GUID, EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID,
    EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID, EFI_DRIVER_HEALTH_PROTOCOL_GUID,
    EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID, EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
    EfiDriverConfiguration2Protocol, EfiDriverConfigurationAction, EfiDriverConfigurationProtocol,
    EfiDriverDiagnosticType, EfiDriverDiagnostics2Protocol, EfiDriverDiagnosticsProtocol,
    EfiDriverHealthHiiMessage, EfiDriverHealthProtocol, EfiDriverHealthStatus,
    EfiDriverSupportedEfiVersionProtocol,
};

static LANG_RFC4646: &[u8] = b"en-US;fr-FR\0";
static LANG_ISO639: &[u8] = b"engfra\0";
static DIAG_ERROR_GUID: EfiGuid = EfiGuid::new(
    0x1234_5678,
    0xABCD,
    0x4EF0,
    [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
);
static mut DIAG_BUFFER: [u16; 8] = [0x0041, 0x0042, 0x0043, 0, 0, 0, 0, 0];

// --- Mock Driver Configuration 2 ---
unsafe extern "efiapi" fn mock_cfg2_set_options(
    _this: *const EfiDriverConfiguration2Protocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    _lang: *const u8,
    action: *mut EfiDriverConfigurationAction,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if !action.is_null() {
            unsafe {
                *action = EfiDriverConfigurationAction::RestartController;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

unsafe extern "efiapi" fn mock_cfg2_options_valid(
    _this: *const EfiDriverConfiguration2Protocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

unsafe extern "efiapi" fn mock_cfg2_force_defaults(
    _this: *const EfiDriverConfiguration2Protocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    default_type: u32,
    action: *mut EfiDriverConfigurationAction,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if !action.is_null() {
            unsafe {
                *action = if default_type == EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS {
                    EfiDriverConfigurationAction::None
                } else {
                    EfiDriverConfigurationAction::RestartPlatform
                };
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

// --- Mock Driver Configuration (Legacy) ---
unsafe extern "efiapi" fn mock_cfg_set_options(
    _this: *const EfiDriverConfigurationProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    _lang: *const u8,
    action: *mut EfiDriverConfigurationAction,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if !action.is_null() {
            unsafe {
                *action = EfiDriverConfigurationAction::StopController;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

unsafe extern "efiapi" fn mock_cfg_options_valid(
    _this: *const EfiDriverConfigurationProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

unsafe extern "efiapi" fn mock_cfg_force_defaults(
    _this: *const EfiDriverConfigurationProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    default_type: u32,
    action: *mut EfiDriverConfigurationAction,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if !action.is_null() {
            unsafe {
                *action = if default_type == EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS {
                    EfiDriverConfigurationAction::None
                } else {
                    EfiDriverConfigurationAction::RestartController
                };
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

// --- Mock Driver Diagnostics 2 ---
unsafe extern "efiapi" fn mock_diag2_run(
    _this: *const EfiDriverDiagnostics2Protocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    diag_type: EfiDriverDiagnosticType,
    _lang: *const u8,
    error_type: *mut *mut EfiGuid,
    buffer_size: *mut UIntN,
    buffer: *mut *mut u16,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if diag_type == EfiDriverDiagnosticType::Extended {
            if !error_type.is_null() {
                unsafe {
                    *error_type = core::ptr::from_ref(&DIAG_ERROR_GUID).cast_mut();
                }
            }
            if !buffer_size.is_null() {
                unsafe {
                    *buffer_size = 16;
                }
            }
            if !buffer.is_null() {
                unsafe {
                    *buffer = core::ptr::addr_of_mut!(DIAG_BUFFER).cast::<u16>();
                }
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

// --- Mock Driver Diagnostics (Legacy) ---
unsafe extern "efiapi" fn mock_diag_run(
    _this: *const EfiDriverDiagnosticsProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    diag_type: EfiDriverDiagnosticType,
    _lang: *const u8,
    error_type: *mut *mut EfiGuid,
    buffer_size: *mut UIntN,
    buffer: *mut *mut u16,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if diag_type == EfiDriverDiagnosticType::Standard {
            if !error_type.is_null() {
                unsafe {
                    *error_type = core::ptr::from_ref(&DIAG_ERROR_GUID).cast_mut();
                }
            }
            if !buffer_size.is_null() {
                unsafe {
                    *buffer_size = 8;
                }
            }
            if !buffer.is_null() {
                unsafe {
                    *buffer = core::ptr::addr_of_mut!(DIAG_BUFFER).cast::<u16>();
                }
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

// --- Mock Driver Health ---
static mut MOCK_MESSAGE_LIST: [EfiDriverHealthHiiMessage; 2] = [
    EfiDriverHealthHiiMessage {
        hii_handle: EfiHandle::from_ptr(0x5000 as *mut c_void),
        string_id: 1,
        message_code: 0x100,
    },
    EfiDriverHealthHiiMessage {
        hii_handle: EfiHandle::from_ptr(core::ptr::null_mut()),
        string_id: 0,
        message_code: 0,
    },
];

unsafe extern "efiapi" fn mock_health_get_status(
    _this: *const EfiDriverHealthProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    health_status: *mut EfiDriverHealthStatus,
    message_list: *mut *mut EfiDriverHealthHiiMessage,
    form_hii_handle: *mut EfiHandle,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if !health_status.is_null() {
            unsafe {
                *health_status = EfiDriverHealthStatus::ConfigurationRequired;
            }
        }
        if !message_list.is_null() {
            unsafe {
                *message_list =
                    core::ptr::addr_of_mut!(MOCK_MESSAGE_LIST).cast::<EfiDriverHealthHiiMessage>();
            }
        }
        if !form_hii_handle.is_null() {
            unsafe {
                *form_hii_handle = EfiHandle::from_ptr(0x8000 as *mut c_void);
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

unsafe extern "efiapi" fn mock_health_repair(
    _this: *const EfiDriverHealthProtocol,
    ctrl: EfiHandle,
    _child: EfiHandle,
    repair_notify: Option<rover_uefi::protocols::driver::EfiDriverHealthRepairNotify>,
) -> EfiStatus {
    if ctrl == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        if let Some(notify) = repair_notify {
            unsafe {
                let _ = notify(50, 100);
                let _ = notify(100, 100);
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

#[test]
fn driver_model_protocols_guids_and_enums() {
    assert!(!EFI_DRIVER_HEALTH_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_CONFIGURATION2_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_CONFIGURATION_PROTOCOL_GUID.is_zero());
    assert!(!EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID.is_zero());
    assert!(!EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID.is_zero());

    assert_eq!(EfiDriverHealthStatus::Healthy as u32, 0);
    assert_eq!(EfiDriverHealthStatus::RepairRequired as u32, 1);
    assert_eq!(EfiDriverHealthStatus::ConfigurationRequired as u32, 2);
    assert_eq!(EfiDriverHealthStatus::Failed as u32, 3);
    assert_eq!(EfiDriverHealthStatus::ReconnectRequired as u32, 4);
    assert_eq!(EfiDriverHealthStatus::RebootRequired as u32, 5);

    assert_eq!(EfiDriverDiagnosticType::Standard as u32, 0);
    assert_eq!(EfiDriverDiagnosticType::Extended as u32, 1);
    assert_eq!(EfiDriverDiagnosticType::Manufacturing as u32, 2);
    assert_eq!(EfiDriverDiagnosticType::Maximum as u32, 3);

    assert_eq!(EfiDriverConfigurationAction::None as u32, 0);
    assert_eq!(EfiDriverConfigurationAction::StopController as u32, 1);
    assert_eq!(EfiDriverConfigurationAction::RestartController as u32, 2);
    assert_eq!(EfiDriverConfigurationAction::RestartPlatform as u32, 3);
    assert_eq!(EfiDriverConfigurationAction::Maximum as u32, 4);

    assert_eq!(EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS, 0);
    assert_eq!(EFI_DRIVER_CONFIGURATION_MANUFACTURING_DEFAULTS, 1);
    assert_eq!(EFI_DRIVER_CONFIGURATION_CUSTOM_DEFAULTS, 2);
    assert_eq!(EFI_DRIVER_CONFIGURATION_PERFORMANCE_DEFAULTS, 3);

    let supported_ver = EfiDriverSupportedEfiVersionProtocol {
        length: 8,
        firmware_version: 0x0002_00A0, // UEFI 2.10
    };
    assert_eq!(supported_ver.length, 8);
    assert_eq!(supported_ver.firmware_version, 0x0002_00A0);
}

#[test]
fn driver_configuration2_client_wrapper() {
    let mock_proto = EfiDriverConfiguration2Protocol {
        set_options: mock_cfg2_set_options,
        options_valid: mock_cfg2_options_valid,
        force_defaults: mock_cfg2_force_defaults,
        supported_languages: LANG_RFC4646.as_ptr(),
    };

    let client = DriverConfiguration2::new(&mock_proto);
    assert_eq!(client.supported_languages(), Some("en-US;fr-FR"));

    let ctrl = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let invalid_ctrl = EfiHandle::from_ptr(0x2000 as *mut c_void);

    assert_eq!(client.options_valid(ctrl, None), Ok(()));
    assert_eq!(
        client.options_valid(invalid_ctrl, None),
        Err(EfiStatus::DEVICE_ERROR)
    );

    let action = client
        .set_options(ctrl, None, "en-US")
        .unwrap_or(EfiDriverConfigurationAction::None);
    assert_eq!(action, EfiDriverConfigurationAction::RestartController);

    assert_eq!(
        client.set_options(invalid_ctrl, None, "en-US"),
        Err(EfiStatus::UNSUPPORTED)
    );

    let def_action = client
        .force_defaults(ctrl, None, EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS)
        .unwrap_or(EfiDriverConfigurationAction::Maximum);
    assert_eq!(def_action, EfiDriverConfigurationAction::None);

    let perf_action = client
        .force_defaults(ctrl, None, EFI_DRIVER_CONFIGURATION_PERFORMANCE_DEFAULTS)
        .unwrap_or(EfiDriverConfigurationAction::None);
    assert_eq!(perf_action, EfiDriverConfigurationAction::RestartPlatform);
}

#[test]
fn driver_configuration_legacy_client_wrapper() {
    let mock_proto = EfiDriverConfigurationProtocol {
        set_options: mock_cfg_set_options,
        options_valid: mock_cfg_options_valid,
        force_defaults: mock_cfg_force_defaults,
        supported_languages: LANG_ISO639.as_ptr(),
    };

    let client = DriverConfiguration::new(&mock_proto);
    assert_eq!(client.supported_languages(), Some("engfra"));

    let ctrl = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let invalid_ctrl = EfiHandle::from_ptr(0x2000 as *mut c_void);

    assert_eq!(client.options_valid(ctrl, None), Ok(()));
    assert_eq!(
        client.options_valid(invalid_ctrl, None),
        Err(EfiStatus::DEVICE_ERROR)
    );

    let action = client
        .set_options(ctrl, None, "eng")
        .unwrap_or(EfiDriverConfigurationAction::None);
    assert_eq!(action, EfiDriverConfigurationAction::StopController);

    assert_eq!(
        client.set_options(invalid_ctrl, None, "eng"),
        Err(EfiStatus::UNSUPPORTED)
    );

    let def_action = client
        .force_defaults(ctrl, None, EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS)
        .unwrap_or(EfiDriverConfigurationAction::Maximum);
    assert_eq!(def_action, EfiDriverConfigurationAction::None);

    let rst_action = client
        .force_defaults(ctrl, None, EFI_DRIVER_CONFIGURATION_PERFORMANCE_DEFAULTS)
        .unwrap_or(EfiDriverConfigurationAction::None);
    assert_eq!(rst_action, EfiDriverConfigurationAction::RestartController);
}

#[test]
fn driver_diagnostics2_client_wrapper() {
    let mock_proto = EfiDriverDiagnostics2Protocol {
        run_diagnostics: mock_diag2_run,
        supported_languages: LANG_RFC4646.as_ptr(),
    };

    let client = DriverDiagnostics2::new(&mock_proto);
    assert_eq!(client.supported_languages(), Some("en-US;fr-FR"));

    let ctrl = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let (err_guid, buf_ptr, buf_len) = client
        .run_diagnostics(ctrl, None, EfiDriverDiagnosticType::Extended, "en-US")
        .unwrap_or((None, core::ptr::null_mut(), 0));

    assert_eq!(err_guid, Some(DIAG_ERROR_GUID));
    assert!(!buf_ptr.is_null());
    assert_eq!(buf_len, 16);

    let invalid_ctrl = EfiHandle::from_ptr(0x2000 as *mut c_void);
    assert_eq!(
        client.run_diagnostics(
            invalid_ctrl,
            None,
            EfiDriverDiagnosticType::Standard,
            "en-US"
        ),
        Err(EfiStatus::DEVICE_ERROR)
    );
}

#[test]
fn driver_diagnostics_legacy_client_wrapper() {
    let mock_proto = EfiDriverDiagnosticsProtocol {
        run_diagnostics: mock_diag_run,
        supported_languages: LANG_ISO639.as_ptr(),
    };

    let client = DriverDiagnostics::new(&mock_proto);
    assert_eq!(client.supported_languages(), Some("engfra"));

    let ctrl = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let (err_guid, buf_ptr, buf_len) = client
        .run_diagnostics(ctrl, None, EfiDriverDiagnosticType::Standard, "eng")
        .unwrap_or((None, core::ptr::null_mut(), 0));

    assert_eq!(err_guid, Some(DIAG_ERROR_GUID));
    assert!(!buf_ptr.is_null());
    assert_eq!(buf_len, 8);
}

#[test]
fn driver_health_client_wrapper() {
    unsafe extern "efiapi" fn test_notify(val: UIntN, limit: UIntN) -> EfiStatus {
        if val <= limit {
            EfiStatus::SUCCESS
        } else {
            EfiStatus::INVALID_PARAMETER
        }
    }

    let mock_proto = EfiDriverHealthProtocol {
        get_health_status: mock_health_get_status,
        repair: mock_health_repair,
    };

    let client = DriverHealth::new(&mock_proto);

    let ctrl = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let (status, msg_list, form_hii) = client.get_health_status(Some(ctrl), None).unwrap_or((
        EfiDriverHealthStatus::Healthy,
        core::ptr::null_mut(),
        None,
    ));

    assert_eq!(status, EfiDriverHealthStatus::ConfigurationRequired);
    assert!(!msg_list.is_null());
    assert_eq!(form_hii, Some(EfiHandle::from_ptr(0x8000 as *mut c_void)));

    assert!(client.repair(ctrl, None, Some(test_notify)).is_ok());

    let invalid_ctrl = EfiHandle::from_ptr(0x2000 as *mut c_void);
    assert_eq!(
        client.get_health_status(Some(invalid_ctrl), None),
        Err(EfiStatus::UNSUPPORTED)
    );
    assert_eq!(
        client.repair(invalid_ctrl, None, None),
        Err(EfiStatus::DEVICE_ERROR)
    );
}
