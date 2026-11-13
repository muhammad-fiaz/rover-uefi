//! Tests for REST EX and Redfish Discover protocols.

#![allow(clippy::similar_names, clippy::expect_used, clippy::unwrap_used)]

use core::ffi::c_void;
use rover_uefi::base::{EfiHandle, EfiStatus, EfiTime};
use rover_uefi::protocols::network::guid::{
    REDFISH_DISCOVER_PROTOCOL_GUID, REST_EX_PROTOCOL_GUID, REST_EX_SERVICE_BINDING_PROTOCOL_GUID,
    REST_PROTOCOL_GUID,
};
use rover_uefi::protocols::network::redfish_discover::{
    EFI_REDFISH_DISCOVER_HOST_INTERFACE, EFI_REDFISH_DISCOVER_SSDP,
    EfiRedfishDiscoverNetworkInterface, EfiRedfishDiscoverProtocol, EfiRedfishDiscoveredList,
    EfiRedfishDiscoveredToken, REDFISH_DISCOVER_TOKEN_SIGNATURE, RedfishDiscover,
};
use rover_uefi::protocols::network::rest_ex::{
    EfiHttpMessage, EfiRestExProtocol, EfiRestExServiceInfoV10, EfiRestExServiceType,
    EfiRestExToken, EfiRestProtocol, Rest, RestEx,
};

#[test]
fn test_rest_ex_guids() {
    assert_eq!(REST_EX_SERVICE_BINDING_PROTOCOL_GUID.data1, 0x456B_BE01);
    assert_eq!(REST_EX_PROTOCOL_GUID.data1, 0x5564_8B91);
    assert_eq!(REST_PROTOCOL_GUID.data1, 0x0DB4_8A36);
    assert_eq!(REDFISH_DISCOVER_PROTOCOL_GUID.data1, 0x5DB1_2509);
    assert_eq!(
        REST_PROTOCOL_GUID.to_string(),
        "0db48a36-4e54-ea9c-9b09-1ea5be3a660b"
    );
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

unsafe extern "efiapi" fn mock_rest_send_receive(
    _this: *mut EfiRestProtocol,
    _req: *mut EfiHttpMessage,
    _resp: *mut EfiHttpMessage,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_rest_get_time(
    _this: *mut EfiRestProtocol,
    time: *mut EfiTime,
) -> EfiStatus {
    if !time.is_null() {
        unsafe {
            (*time).year = 2026;
            (*time).month = 11;
            (*time).day = 13;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_rest_and_rest_ex_wrappers() {
    let mut raw_rest = EfiRestProtocol {
        send_receive: mock_rest_send_receive,
        get_service_time: mock_rest_get_time,
    };

    let mut rest = Rest::new(&mut raw_rest);
    let mut req = EfiHttpMessage {
        request: core::ptr::null_mut(),
        response: core::ptr::null_mut(),
        header_count: 0,
        headers: core::ptr::null_mut(),
        body_length: 0,
        body: core::ptr::null_mut(),
    };
    let mut resp = req;

    assert!(rest.send_receive(&mut req, &mut resp).is_ok());
    let time = rest.service_time().expect("service_time should succeed");
    assert_eq!(time.year, 2026);
    assert_eq!(time.month, 11);

    let mut raw_rest_ex = EfiRestExProtocol {
        send_receive: dummy_send_receive,
        get_service_time: dummy_get_service_time,
        get_service: dummy_get_service,
        get_mode_data: dummy_get_mode_data,
        configure: dummy_configure,
        async_send_receive: dummy_async_send_receive,
        event_service: dummy_event_service,
    };
    let mut rest_ex = RestEx::new(&mut raw_rest_ex);
    assert!(rest_ex.send_receive(&mut req, &mut resp).is_ok());
    assert!(rest_ex.configure(core::ptr::null_mut()).is_ok());
    assert!(rest_ex.service_info().is_ok());
}

#[test]
fn test_redfish_discover_wrapper() {
    let mut proto = EfiRedfishDiscoverProtocol {
        get_network_interface_list: dummy_get_network_list,
        acquire_redfish_service: dummy_acquire,
        abort_acquire_redfish_service: dummy_abort,
        release_redfish_service: dummy_release,
    };

    let mut discover = RedfishDiscover::new(&mut proto);
    let (ifaces, count) = discover
        .get_network_interfaces(EfiHandle::NULL)
        .expect("get network interfaces failed");
    assert_eq!(count, 0);
    assert!(ifaces.is_null());

    let mut target = EfiRedfishDiscoverNetworkInterface {
        mac_address: rover_uefi::protocols::network::types::MacAddress([0; 6]),
        is_ipv6: false,
        subnet_id: rover_uefi::protocols::network::redfish_discover::EfiIpAddress { addr: [0; 4] },
        subnet_prefix_length: 24,
        vlan_id: 0,
    };
    let mut token = EfiRedfishDiscoveredToken {
        signature: REDFISH_DISCOVER_TOKEN_SIGNATURE,
        discover_list: EfiRedfishDiscoveredList {
            number_of_service_found: 0,
            redfish_instances: core::ptr::null_mut(),
        },
        event: rover_uefi::base::EfiEvent::NULL,
        timeout: 0,
    };

    assert!(
        discover
            .acquire_service(EfiHandle::NULL, &mut target, 0, &mut token)
            .is_ok()
    );
    assert!(discover.abort_acquire(&mut target).is_ok());
    assert!(discover.release_service(&mut token.discover_list).is_ok());
}
