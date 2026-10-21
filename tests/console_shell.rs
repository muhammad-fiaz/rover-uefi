//! Tests for UEFI Shell and Shell Parameters protocols.

use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::console::shell::{
    EFI_SHELL_PARAMETERS_PROTOCOL_GUID, EFI_SHELL_PROTOCOL_GUID, EfiShellParametersProtocol,
    EfiShellProtocol,
};

#[test]
fn test_shell_guids() {
    assert_eq!(EFI_SHELL_PROTOCOL_GUID.data1, 0x6302_D008);
    assert_eq!(EFI_SHELL_PARAMETERS_PROTOCOL_GUID.data1, 0x752F_3136);
}

unsafe extern "efiapi" fn dummy_shell_execute(
    _image_handle: *mut EfiHandle,
    _command_line: *mut u16,
    _env: *mut *mut u16,
    status: *mut EfiStatus,
) -> EfiStatus {
    if !status.is_null() {
        unsafe { *status = EfiStatus::SUCCESS };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_shell_get_env(_name: *const u16) -> *const u16 {
    core::ptr::null()
}

unsafe extern "efiapi" fn dummy_shell_set_env(
    _name: *const u16,
    _value: *const u16,
    _volatile: bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_shell_protocol() {
    let shell = EfiShellProtocol {
        execute: dummy_shell_execute,
        get_env: dummy_shell_get_env,
        set_env: dummy_shell_set_env,
    };

    let mut exit_status = EfiStatus::DEVICE_ERROR;
    let status = unsafe {
        (shell.execute)(
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &raw mut exit_status,
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(exit_status, EfiStatus::SUCCESS);

    let params = EfiShellParametersProtocol {
        argv: core::ptr::null_mut(),
        argc: 0,
        std_in: core::ptr::null_mut(),
        std_out: core::ptr::null_mut(),
        std_err: core::ptr::null_mut(),
    };
    assert_eq!(params.argc, 0);
}
