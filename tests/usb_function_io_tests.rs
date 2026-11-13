//! Unit and integration tests for the USB Function I/O Protocol.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use core::ffi::c_void;
use core::sync::atomic::{AtomicU8, Ordering};
use rover_uefi::abi::types::{Bool, EFI_FALSE, UIntN};
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::usb::function_io::{
    EfiUsbFnIoProtocol, USBFN_IO_PROTOCOL_GUID, USBFN_IO_PROTOCOL_REVISION, UsbDeviceInfo,
    UsbFnBusSpeed, UsbFnDeviceInfoId, UsbFnEndpointDirection, UsbFnEndpointType, UsbFnIo,
    UsbFnMessage, UsbFnMessagePayload, UsbFnPolicyType, UsbFnPortType,
};

static TEST_STALL_STATE: AtomicU8 = AtomicU8::new(EFI_FALSE);
static TEST_ABORTED_EP: AtomicU8 = AtomicU8::new(0xFF);

unsafe extern "efiapi" fn mock_detect_port(
    _this: *mut EfiUsbFnIoProtocol,
    port_type: *mut UsbFnPortType,
) -> EfiStatus {
    unsafe {
        *port_type = UsbFnPortType::ChargingDownstreamPort;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_configure_enable_endpoints(
    _this: *mut EfiUsbFnIoProtocol,
    _info: *mut UsbDeviceInfo,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_device_info(
    _this: *mut EfiUsbFnIoProtocol,
    _id: UsbFnDeviceInfoId,
    _size: *mut UIntN,
    _buf: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_vendor_id_product_id(
    _this: *mut EfiUsbFnIoProtocol,
    vid: *mut u16,
    pid: *mut u16,
) -> EfiStatus {
    unsafe {
        *vid = 0x18D1; // Google VID
        *pid = 0x4EE0; // Fastboot PID
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_endpoint_max_packet_size(
    _this: *mut EfiUsbFnIoProtocol,
    endpoint_type: UsbFnEndpointType,
    bus_speed: UsbFnBusSpeed,
    max_packet_size: *mut u16,
) -> EfiStatus {
    unsafe {
        *max_packet_size = match (endpoint_type, bus_speed) {
            (UsbFnEndpointType::Bulk, UsbFnBusSpeed::High) => 512,
            (UsbFnEndpointType::Bulk, UsbFnBusSpeed::Super) => 1024,
            _ => 64,
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_abort_transfer(
    _this: *mut EfiUsbFnIoProtocol,
    endpoint_index: u8,
    _direction: UsbFnEndpointDirection,
) -> EfiStatus {
    TEST_ABORTED_EP.store(endpoint_index, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_endpoint_stall_state(
    _this: *mut EfiUsbFnIoProtocol,
    _endpoint_index: u8,
    _direction: UsbFnEndpointDirection,
    state: *mut Bool,
) -> EfiStatus {
    unsafe {
        *state = TEST_STALL_STATE.load(Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_endpoint_stall_state(
    _this: *mut EfiUsbFnIoProtocol,
    _endpoint_index: u8,
    _direction: UsbFnEndpointDirection,
    state: *mut Bool,
) -> EfiStatus {
    unsafe {
        TEST_STALL_STATE.store(*state, Ordering::SeqCst);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_event_handler(
    _this: *mut EfiUsbFnIoProtocol,
    message: *mut UsbFnMessage,
    _payload_size: *mut UIntN,
    _payload: *mut UsbFnMessagePayload,
) -> EfiStatus {
    unsafe {
        *message = UsbFnMessage::BusEventReset;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transfer(
    _this: *mut EfiUsbFnIoProtocol,
    _endpoint_index: u8,
    _direction: UsbFnEndpointDirection,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    unsafe {
        let slice = core::slice::from_raw_parts_mut(buffer.cast::<u8>(), *buffer_size);
        if !slice.is_empty() {
            slice[0] = 0x42;
        }
        *buffer_size = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_max_transfer_size(
    _this: *mut EfiUsbFnIoProtocol,
    max_transfer_size: *mut UIntN,
) -> EfiStatus {
    unsafe {
        *max_transfer_size = 16 * 1024 * 1024; // 16 MiB
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_allocate_transfer_buffer(
    _this: *mut EfiUsbFnIoProtocol,
    _size: UIntN,
    _buf: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_free_transfer_buffer(
    _this: *mut EfiUsbFnIoProtocol,
    _buf: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_start_controller(_this: *mut EfiUsbFnIoProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_stop_controller(_this: *mut EfiUsbFnIoProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_endpoint_policy(
    _this: *mut EfiUsbFnIoProtocol,
    _ep: u8,
    _dir: UsbFnEndpointDirection,
    _pol: UsbFnPolicyType,
    _size: UIntN,
    _buf: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_endpoint_policy(
    _this: *mut EfiUsbFnIoProtocol,
    _ep: u8,
    _dir: UsbFnEndpointDirection,
    _pol: UsbFnPolicyType,
    _size: *mut UIntN,
    _buf: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_usb_function_io_protocol_guid_and_revision() {
    assert_eq!(UsbFnIo::GUID, USBFN_IO_PROTOCOL_GUID);
    assert_eq!(USBFN_IO_PROTOCOL_REVISION, 0x0001_0001);
}

#[test]
fn test_usb_function_io_operations() {
    let mut raw_protocol = EfiUsbFnIoProtocol {
        revision: USBFN_IO_PROTOCOL_REVISION,
        detect_port: mock_detect_port,
        configure_enable_endpoints: mock_configure_enable_endpoints,
        get_endpoint_max_packet_size: mock_get_endpoint_max_packet_size,
        get_device_info: mock_get_device_info,
        get_vendor_id_product_id: mock_get_vendor_id_product_id,
        abort_transfer: mock_abort_transfer,
        get_endpoint_stall_state: mock_get_endpoint_stall_state,
        set_endpoint_stall_state: mock_set_endpoint_stall_state,
        event_handler: mock_event_handler,
        transfer: mock_transfer,
        get_max_transfer_size: mock_get_max_transfer_size,
        allocate_transfer_buffer: mock_allocate_transfer_buffer,
        free_transfer_buffer: mock_free_transfer_buffer,
        start_controller: mock_start_controller,
        stop_controller: mock_stop_controller,
        set_endpoint_policy: mock_set_endpoint_policy,
        get_endpoint_policy: mock_get_endpoint_policy,
    };

    let mut client = UsbFnIo::new(&mut raw_protocol);

    // Test port detection
    let port = client.detect_port().expect("port detection");
    assert_eq!(port, UsbFnPortType::ChargingDownstreamPort);

    // Test VID/PID
    let (vid, pid) = client.get_vendor_id_product_id().expect("vid pid");
    assert_eq!(vid, 0x18D1);
    assert_eq!(pid, 0x4EE0);

    // Test max packet size
    let bulk_high = client
        .get_endpoint_max_packet_size(UsbFnEndpointType::Bulk, UsbFnBusSpeed::High)
        .expect("max packet size");
    assert_eq!(bulk_high, 512);

    let bulk_super = client
        .get_endpoint_max_packet_size(UsbFnEndpointType::Bulk, UsbFnBusSpeed::Super)
        .expect("max packet size");
    assert_eq!(bulk_super, 1024);

    // Test stall state
    assert!(
        !client
            .get_endpoint_stall_state(1, UsbFnEndpointDirection::HostIn)
            .unwrap()
    );
    client
        .set_endpoint_stall_state(1, UsbFnEndpointDirection::HostIn, true)
        .unwrap();
    assert!(
        client
            .get_endpoint_stall_state(1, UsbFnEndpointDirection::HostIn)
            .unwrap()
    );
    client
        .set_endpoint_stall_state(1, UsbFnEndpointDirection::HostIn, false)
        .unwrap();
    assert!(
        !client
            .get_endpoint_stall_state(1, UsbFnEndpointDirection::HostIn)
            .unwrap()
    );

    // Test abort transfer
    client
        .abort_transfer(3, UsbFnEndpointDirection::HostOut)
        .unwrap();
    assert_eq!(TEST_ABORTED_EP.load(Ordering::SeqCst), 3);

    // Test event handler
    let mut payload: UsbFnMessagePayload = UsbFnMessagePayload {
        ubs: UsbFnBusSpeed::High,
    };
    let msg = client.event_handler(&mut payload).unwrap();
    assert_eq!(msg, UsbFnMessage::BusEventReset);

    // Test transfer
    let mut buf = [0u8; 8];
    let transferred = client
        .transfer(1, UsbFnEndpointDirection::HostOut, &mut buf)
        .unwrap();
    assert_eq!(transferred, 1);
    assert_eq!(buf[0], 0x42);

    // Test max transfer size
    let max_size = client.get_max_transfer_size().unwrap();
    assert_eq!(max_size, 16 * 1024 * 1024);

    // Test controller start / stop
    assert!(client.start_controller().is_ok());
    assert!(client.stop_controller().is_ok());
}
