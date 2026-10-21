//! Tests for REST EX and Redfish Discover protocols.

use core::ffi::c_void;
use rover_uefi::base::{EfiHandle, EfiStatus, EfiTime};
use rover_uefi::protocols::network::guid::{
    REDFISH_DISCOVER_PROTOCOL_GUID, REST_EX_PROTOCOL_GUID, REST_EX_SERVICE_BINDING_PROTOCOL_GUID,
};
use rover_uefi::protocols::network::redfish_discover::{
    EFI_REDFISH_DISCOVER_HOST_INTERFACE, EFI_REDFISH_DISCOVER_SSDP,
    EfiRedfishDiscoverNetworkInterface, EfiRedfishDiscoverProtocol, EfiRedfishDiscoveredList,
    EfiRedfishDiscoveredToken, REDFISH_DISCOVER_TOKEN_SIGNATURE,
};
use rover_uefi::protocols::network::rest_ex::{
    EfiHttpMessage, EfiRestExProtocol, EfiRestExServiceInfoV10, EfiRestExServiceType,
    EfiRestExToken,
};

#[test]
fn test_rest_ex_guids() {
    assert_eq!(REST_EX_SERVICE_BINDING_PROTOCOL_GUID.data1, 0x456B_BE01);
    assert_eq!(REST_EX_PROTOCOL_GUID.data1, 0x5564_8B91);
    assert_eq!(REDFISH_DISCOVER_PROTOCOL_GUID.data1, 0x5DB1_2509);
}

unsafe extern "efiapi" fn dummy_send_receive(
    _this: *mut EfiRestExProtocol,
    _req: *mut EfiHttpMessage,
    _resp: *mut EfiHttpMessage,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_get_service_time(
    _this: *mut EfiRestExProtocol,
    _time: *mut EfiTime,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn dummy_get_service(
    _this: *mut EfiRestExProtocol,
    _info: *mut *mut EfiRestExServiceInfoV10,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_get_mode_data(
    _this: *mut EfiRestExProtocol,
    _data: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::NOT_READY
}

unsafe extern "efiapi" fn dummy_configure(
    _this: *mut EfiRestExProtocol,
    _data: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_async_send_receive(
    _this: *mut EfiRestExProtocol,
    _req: *mut EfiHttpMessage,
    _token: *mut EfiRestExToken,
    _timeout: *mut usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_event_service(
    _this: *mut EfiRestExProtocol,
    _req: *mut EfiHttpMessage,
    _token: *mut EfiRestExToken,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

#[test]
fn test_rest_ex_protocol_instantiation() {
    let mut protocol = EfiRestExProtocol {
        send_receive: dummy_send_receive,
        get_service_time: dummy_get_service_time,
        get_service: dummy_get_service,
        get_mode_data: dummy_get_mode_data,
        configure: dummy_configure,
        async_send_receive: dummy_async_send_receive,
        event_service: dummy_event_service,
    };

    let mut req = EfiHttpMessage {
        request: core::ptr::null_mut(),
        response: core::ptr::null_mut(),
        header_count: 0,
        headers: core::ptr::null_mut(),
        body_length: 0,
        body: core::ptr::null_mut(),
    };
    let mut resp = req;

    let status = unsafe { (protocol.send_receive)(&raw mut protocol, &raw mut req, &raw mut resp) };
    assert_eq!(status, EfiStatus::SUCCESS);

    let status = unsafe { (protocol.configure)(&raw mut protocol, core::ptr::null_mut()) };
    assert_eq!(status, EfiStatus::SUCCESS);

    assert_eq!(EfiRestExServiceType::Redfish as u32, 2);
    assert_eq!(EfiRestExServiceType::VendorSpecific as u32, 0xFF);
}

unsafe extern "efiapi" fn dummy_get_network_list(
    _this: *mut EfiRedfishDiscoverProtocol,
    _image_handle: EfiHandle,
    count: *mut usize,
    _interfaces: *mut *mut EfiRedfishDiscoverNetworkInterface,
) -> EfiStatus {
    if !count.is_null() {
        unsafe { *count = 0 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_acquire(
    _this: *mut EfiRedfishDiscoverProtocol,
    _image: EfiHandle,
    _target: *mut EfiRedfishDiscoverNetworkInterface,
    _flags: u32,
    _token: *mut EfiRedfishDiscoveredToken,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_abort(
    _this: *mut EfiRedfishDiscoverProtocol,
    _target: *mut EfiRedfishDiscoverNetworkInterface,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_release(
    _this: *mut EfiRedfishDiscoverProtocol,
    _list: *mut EfiRedfishDiscoveredList,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_redfish_discover_protocol() {
    let mut proto = EfiRedfishDiscoverProtocol {
        get_network_interface_list: dummy_get_network_list,
        acquire_redfish_service: dummy_acquire,
        abort_acquire_redfish_service: dummy_abort,
        release_redfish_service: dummy_release,
    };

    let mut count = 10usize;
    let status = unsafe {
        (proto.get_network_interface_list)(
            &raw mut proto,
            EfiHandle::NULL,
            &raw mut count,
            core::ptr::null_mut(),
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(count, 0);

    assert_eq!(REDFISH_DISCOVER_TOKEN_SIGNATURE, 0x5354_4652);
    assert_eq!(EFI_REDFISH_DISCOVER_HOST_INTERFACE, 0x0000_0001);
    assert_eq!(EFI_REDFISH_DISCOVER_SSDP, 0x0000_0002);
}
