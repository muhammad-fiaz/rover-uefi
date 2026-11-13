//! Tests for Android Fastboot and GBL Fastboot protocols.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::manual_c_str_literals)]

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::android::fastboot::{
    GBL_EFI_FASTBOOT_PROTOCOL_GUID, GBL_EFI_FASTBOOT_TRANSPORT_PROTOCOL_GUID,
    GblEfiFastbootProtocol, GblEfiFastbootTransportProtocol, GblFastboot, GblFastbootTransport,
};

unsafe extern "efiapi" fn mock_get_var(
    _this: *mut GblEfiFastbootProtocol,
    _num_args: usize,
    _args: *const *const u8,
    buffer_size: *mut usize,
    buffer: *mut u8,
) -> EfiStatus {
    let val = b"unlocked\0";
    unsafe {
        if *buffer_size < val.len() {
            *buffer_size = val.len();
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        core::ptr::copy_nonoverlapping(val.as_ptr(), buffer, val.len());
        *buffer_size = val.len();
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transport_start(
    _this: *mut GblEfiFastbootTransportProtocol,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transport_stop(
    _this: *mut GblEfiFastbootTransportProtocol,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transport_send(
    _this: *mut GblEfiFastbootTransportProtocol,
    buffer_size: *mut usize,
    _buffer: *const c_void,
) -> EfiStatus {
    unsafe {
        *buffer_size = 12;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transport_flush(
    _this: *mut GblEfiFastbootTransportProtocol,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_gbl_fastboot_client() {
    let mut raw = GblEfiFastbootProtocol {
        revision: 0x0001_0000,
        serial_number: [0u8; 32],
        get_var: Some(mock_get_var),
        get_var_all: None,
        get_staged: None,
        command_exec: None,
        get_partition_type: None,
    };
    raw.serial_number[..9].copy_from_slice(b"ROVER0001");

    let mut client = GblFastboot::new(&mut raw);
    assert_eq!(client.revision(), 0x0001_0000);
    assert_eq!(client.serial_number(), b"ROVER0001");

    let mut buf = [0u8; 32];
    let len = client
        .get_var(&[b"is-userspace"], &mut buf)
        .expect("get_var should succeed");
    assert_eq!(&buf[..len], b"unlocked\0");
    assert_eq!(
        GBL_EFI_FASTBOOT_PROTOCOL_GUID.to_string(),
        "c67e48a0-5eb8-4127-be89-df2ed93d8a9a"
    );
}

#[test]
fn test_gbl_fastboot_transport_client() {
    let mut raw = GblEfiFastbootTransportProtocol {
        revision: 0x0001_0000,
        description: b"USB-OTG\0".as_ptr(),
        start: Some(mock_transport_start),
        stop: Some(mock_transport_stop),
        receive: None,
        send: Some(mock_transport_send),
        flush: Some(mock_transport_flush),
    };

    let mut transport = GblFastbootTransport::new(&mut raw);
    assert!(transport.start().is_ok());
    let sent = transport.send(b"OKAY0.4").expect("send should succeed");
    assert_eq!(sent, 12);
    assert!(transport.flush().is_ok());
    assert!(transport.stop().is_ok());
    assert_eq!(
        GBL_EFI_FASTBOOT_TRANSPORT_PROTOCOL_GUID.to_string(),
        "edade92c-5c48-440d-849c-e2a0c7e55143"
    );
}
