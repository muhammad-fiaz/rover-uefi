//! Integration tests for Redfish Config Handler protocol and Host Interface structures.

use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::network::redfish_config::{
    EfiRedfishConfigHandlerProtocol, REDFISH_CONFIG_HANDLER_PROTOCOL_GUID, RedfishConfigHandler,
    RedfishConfigServiceInformation,
};
use rover_uefi::protocols::network::redfish_host_interface::{
    PciInterfaceDeviceDescriptorV2, REDFISH_HOST_INTERFACE_DEVICE_TYPE_PCI_PCIE_V2,
    REDFISH_HOST_INTERFACE_DEVICE_TYPE_USB_V2, REDFISH_HOST_IP_ASSIGNMENT_DHCP,
    REDFISH_HOST_IP_ASSIGNMENT_STATIC, REDFISH_HOST_IP_FORMAT_IPV4, REDFISH_HOST_IP_FORMAT_IPV6,
    UsbInterfaceDeviceDescriptorV2,
};

static mut HANDLER_INITIALIZED: bool = false;
static mut HANDLER_STOPPED: bool = false;

unsafe extern "efiapi" fn mock_init(
    _this: *const EfiRedfishConfigHandlerProtocol,
    _service_info: *const RedfishConfigServiceInformation,
) -> EfiStatus {
    unsafe {
        HANDLER_INITIALIZED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_stop(_this: *const EfiRedfishConfigHandlerProtocol) -> EfiStatus {
    unsafe {
        HANDLER_STOPPED = true;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_redfish_config_guid_and_wrapper() {
    assert_eq!(
        REDFISH_CONFIG_HANDLER_PROTOCOL_GUID,
        EfiGuid::new(
            0xBC0F_E6BB,
            0x2CC9,
            0x463E,
            [0x90, 0x82, 0xFA, 0x11, 0x76, 0xFC, 0x67, 0xDE],
        )
    );

    let mock_proto = EfiRedfishConfigHandlerProtocol {
        init: mock_init,
        stop: mock_stop,
    };

    let handler = unsafe { RedfishConfigHandler::from_raw(&raw const mock_proto) };
    let info = RedfishConfigServiceInformation {
        rest_ex_handle: EfiHandle::NULL,
        service_version: 1,
        location: core::ptr::null(),
        uuid: core::ptr::null(),
        os: core::ptr::null(),
        os_version: core::ptr::null(),
        product: core::ptr::null(),
        product_version: core::ptr::null(),
        use_https: 1,
    };

    assert!(handler.init(&info).is_ok());
    assert!(unsafe { HANDLER_INITIALIZED });

    assert!(handler.stop().is_ok());
    assert!(unsafe { HANDLER_STOPPED });
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_redfish_host_interface_constants_and_descriptors() {
    assert_eq!(REDFISH_HOST_INTERFACE_DEVICE_TYPE_USB_V2, 0x04);
    assert_eq!(REDFISH_HOST_INTERFACE_DEVICE_TYPE_PCI_PCIE_V2, 0x05);
    assert_eq!(REDFISH_HOST_IP_ASSIGNMENT_STATIC, 0x01);
    assert_eq!(REDFISH_HOST_IP_ASSIGNMENT_DHCP, 0x02);
    assert_eq!(REDFISH_HOST_IP_FORMAT_IPV4, 0x01);
    assert_eq!(REDFISH_HOST_IP_FORMAT_IPV6, 0x02);

    let usb_desc = UsbInterfaceDeviceDescriptorV2 {
        length: core::mem::size_of::<UsbInterfaceDeviceDescriptorV2>() as u8,
        id_vendor: 0x1234,
        id_product: 0x5678,
        serial_number_str: 1,
        mac_address: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
        characteristics: 0,
        credential_bootstrapping_handle: 0,
    };
    let usb_id_vendor = usb_desc.id_vendor;
    assert_eq!(usb_id_vendor, 0x1234);

    let pci_desc = PciInterfaceDeviceDescriptorV2 {
        length: core::mem::size_of::<PciInterfaceDeviceDescriptorV2>() as u8,
        vendor_id: 0x8086,
        device_id: 0x1533,
        subsystem_vendor_id: 0x8086,
        subsystem_id: 0x0001,
        mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
        segment_group_number: 0,
        bus_number: 1,
        device_function_number: (2 << 3),
    };
    let pci_vendor_id = pci_desc.vendor_id;
    assert_eq!(pci_vendor_id, 0x8086);
}
