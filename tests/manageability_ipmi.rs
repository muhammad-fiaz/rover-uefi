//! Tests for Manageability IPMI protocol.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::manageability::ipmi::{
    EfiIpmiProtocol, IPMI_PROTOCOL_GUID, SMM_IPMI_PROTOCOL_GUID, app_cmd, completion_code, kcs,
    net_fn,
};

#[test]
fn test_ipmi_guids() {
    assert_eq!(IPMI_PROTOCOL_GUID.data1, 0xDBC6_381F);
    assert_eq!(SMM_IPMI_PROTOCOL_GUID.data1, 0x5169_AF60);
}

#[test]
fn test_ipmi_constants() {
    assert_eq!(net_fn::CHASSIS, 0x00);
    assert_eq!(net_fn::BRIDGE, 0x02);
    assert_eq!(net_fn::SENSOR_EVENT, 0x04);
    assert_eq!(net_fn::APP, 0x06);
    assert_eq!(net_fn::STORAGE, 0x0A);

    assert_eq!(app_cmd::GET_DEVICE_ID, 0x01);
    assert_eq!(app_cmd::GET_SELF_TEST_RESULTS, 0x04);
    assert_eq!(app_cmd::RESET_WATCHDOG_TIMER, 0x22);

    assert_eq!(completion_code::SUCCESS, 0x00);
    assert_eq!(completion_code::NODE_BUSY, 0xC0);
    assert_eq!(completion_code::INVALID_COMMAND, 0xC1);
    assert_eq!(completion_code::UNSPECIFIED_ERROR, 0xFF);

    assert_eq!(kcs::STATUS_OBF, 1);
    assert_eq!(kcs::STATUS_IBF, 2);
    assert_eq!(kcs::CMD_GET_STATUS, 0x60);
}

unsafe extern "efiapi" fn dummy_ipmi_submit(
    _this: *mut EfiIpmiProtocol,
    net_fn: u8,
    command: u8,
    _req_data: *const u8,
    _req_size: u32,
    resp_data: *mut u8,
    resp_size: *mut u32,
) -> EfiStatus {
    if net_fn == net_fn::APP
        && command == app_cmd::GET_DEVICE_ID
        && !resp_data.is_null()
        && !resp_size.is_null()
        && unsafe { *resp_size } >= 2
    {
        unsafe {
            *resp_data = completion_code::SUCCESS;
            *resp_data.add(1) = 0x20; // Device ID
            *resp_size = 2;
        }
        return EfiStatus::SUCCESS;
    }
    EfiStatus::UNSUPPORTED
}

#[test]
fn test_ipmi_protocol_dispatch() {
    let mut ipmi = EfiIpmiProtocol {
        submit_command: dummy_ipmi_submit,
    };

    let mut response_buf = [0u8; 16];
    let mut response_size = 16u32;

    let status = unsafe {
        (ipmi.submit_command)(
            &raw mut ipmi,
            net_fn::APP,
            app_cmd::GET_DEVICE_ID,
            core::ptr::null(),
            0,
            response_buf.as_mut_ptr(),
            &raw mut response_size,
        )
    };

    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(response_size, 2);
    assert_eq!(response_buf[0], completion_code::SUCCESS);
    assert_eq!(response_buf[1], 0x20);
}
