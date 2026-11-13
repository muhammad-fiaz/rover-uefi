//! Integration tests for UEFI Shell Interface and parameters protocols.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::similar_names,
    clippy::cast_lossless
)]

use core::ffi::c_void;
use rover_uefi::base::EfiHandle;
use rover_uefi::protocols::console::shell::*;

#[test]
fn test_shell_guids_and_constants() {
    assert_eq!(
        SHELL_INTERFACE_PROTOCOL_GUID.to_string(),
        "47c7b223-c42a-11d2-8e57-00a0c969723b"
    );
    assert_eq!(
        SHELL_ENVIRONMENT_PROTOCOL_GUID.to_string(),
        "47c7b221-c42a-11d2-8e57-00a0c969723b"
    );
    assert_eq!(
        SHELL_ENVIRONMENT2_EXTENSION_GUID.to_string(),
        "d2c18636-40e5-4eb5-a31b-36695fd42c87"
    );
    assert_eq!(
        SHELL_ALIAS_VARIABLE_GUID.to_string(),
        "0053d9d6-2659-4599-a266-7936f8e7e376"
    );
    assert_eq!(
        SHELL_VARIABLE_GUID.to_string(),
        "158def5a-f656-419c-b027-7a3192c079d2"
    );
    assert_eq!(
        SHELL_MAP_GUID.to_string(),
        "51271e13-7de3-43af-a024-600f533080ce"
    );

    assert_eq!(ARG_NO_ATTRIB, 0);
    assert_eq!(ARG_IS_QUOTED, 1);
    assert_eq!(ARG_PARTIALLY_QUOTED, 2);
    assert_eq!(ARG_FIRST_HALF_QUOTED, 4);
    assert_eq!(ARG_FIRST_CHAR_IS_ESC, 8);
}

#[test]
fn test_shell_interface_client_wrapper() {
    let mut arg0: [u16; 5] = [b'f' as u16, b's' as u16, b'0' as u16, b':' as u16, 0];
    let mut arg1: [u16; 3] = [b'l' as u16, b's' as u16, 0];
    let mut argv: [*mut u16; 2] = [arg0.as_mut_ptr(), arg1.as_mut_ptr()];

    let mut arg_infos = [
        EfiShellArgInfo {
            attributes: ARG_NO_ATTRIB,
        },
        EfiShellArgInfo {
            attributes: ARG_IS_QUOTED,
        },
    ];

    let raw_iface = EfiShellInterface {
        image_handle: EfiHandle::from_ptr(0x1000 as *mut c_void),
        info: core::ptr::null_mut(),
        argv: argv.as_mut_ptr(),
        argc: 2,
        redir_argv: core::ptr::null_mut(),
        redir_argc: 0,
        std_in: core::ptr::null_mut(),
        std_out: core::ptr::null_mut(),
        std_err: core::ptr::null_mut(),
        arg_info: arg_infos.as_mut_ptr(),
        echo_on: true,
    };

    let wrapper = ShellInterface::new(&raw_iface);
    assert_eq!(wrapper.argc(), 2);
    assert_eq!(wrapper.redir_argc(), 0);
    assert!(wrapper.is_echo_on());

    assert_eq!(wrapper.arg(0), Some(arg0.as_ptr()));
    assert_eq!(wrapper.arg(1), Some(arg1.as_ptr()));
    assert_eq!(wrapper.arg(2), None);

    assert_eq!(wrapper.arg_attributes(0), Some(ARG_NO_ATTRIB));
    assert_eq!(wrapper.arg_attributes(1), Some(ARG_IS_QUOTED));
    assert_eq!(wrapper.arg_attributes(2), None);
}
