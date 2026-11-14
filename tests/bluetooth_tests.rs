//! # Bluetooth Protocol Tests
//!
//! Unit tests for Bluetooth Attribute and Bluetooth LE Config protocols.

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::bluetooth::attribute::{
    BluetoothAttribute, BluetoothAttributeHeader, BluetoothAttributePermission,
    BluetoothGattCharacteristicInfo, BluetoothGattPrimaryServiceInfo, BluetoothLeAddress,
    BluetoothUuid, EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL_GUID,
    EFI_BLUETOOTH_ATTRIBUTE_SERVICE_BINDING_PROTOCOL_GUID, EfiBluetoothAttributeProtocol,
    UUID_16BIT_TYPE_LEN, UUID_32BIT_TYPE_LEN, UUID_128BIT_TYPE_LEN,
};
use rover_uefi::protocols::bluetooth::guid::{
    EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID, EFI_BLUETOOTH_HC_PROTOCOL_GUID,
    EFI_BLUETOOTH_IO_PROTOCOL_GUID, EFI_BLUETOOTH_LE_CONFIG_PROTOCOL_GUID,
};
use rover_uefi::protocols::bluetooth::le_config::{
    BluetoothLeConfig, EfiBluetoothLeConfigProtocol, EfiBluetoothLeConfigScanParameter,
};

#[test]
fn test_bluetooth_guids() {
    assert_ne!(
        EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL_GUID,
        EFI_BLUETOOTH_ATTRIBUTE_SERVICE_BINDING_PROTOCOL_GUID
    );
    assert_ne!(
        EFI_BLUETOOTH_LE_CONFIG_PROTOCOL_GUID,
        EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID
    );
    assert_ne!(
        EFI_BLUETOOTH_HC_PROTOCOL_GUID,
        EFI_BLUETOOTH_IO_PROTOCOL_GUID
    );
}

#[test]
fn test_bluetooth_uuid_and_permission() {
    let uuid16 = BluetoothUuid::from_u16(0x180D);
    assert_eq!(uuid16.length, UUID_16BIT_TYPE_LEN);
    assert_eq!(&uuid16.data[0..2], &[0x0D, 0x18]);

    let uuid32 = BluetoothUuid::from_u32(0x1234_5678);
    assert_eq!(uuid32.length, UUID_32BIT_TYPE_LEN);
    assert_eq!(&uuid32.data[0..4], &[0x78, 0x56, 0x34, 0x12]);

    let raw_128 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let uuid128 = BluetoothUuid::from_u128_bytes(raw_128);
    assert_eq!(uuid128.length, UUID_128BIT_TYPE_LEN);
    assert_eq!(uuid128.data, raw_128);

    let perm = BluetoothAttributePermission(
        BluetoothAttributePermission::READABLE | BluetoothAttributePermission::WRITABLE,
    );
    assert!(perm.is_readable());
    assert!(perm.is_writable());

    let hdr = BluetoothAttributeHeader {
        r#type: uuid16,
        length: 2,
        attribute_handle: 0x0010,
        attribute_permission: perm,
    };
    assert_eq!(hdr.attribute_handle, 0x0010);

    let svc = BluetoothGattPrimaryServiceInfo {
        header: hdr,
        end_group_handle: 0x0020,
        service_uuid: uuid16,
    };
    assert_eq!(svc.end_group_handle, 0x0020);

    let char_info = BluetoothGattCharacteristicInfo {
        header: hdr,
        characteristic_properties: 0x02, // Read
        characteristic_value_handle: 0x0011,
        characteristic_uuid: uuid16,
    };
    assert_eq!(char_info.characteristic_value_handle, 0x0011);
}

static ATTRIBUTE_SEND_CALLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

unsafe extern "efiapi" fn mock_attr_send_request(
    _this: *mut EfiBluetoothAttributeProtocol,
    _data: *mut c_void,
    _data_length: rover_uefi::abi::types::UIntN,
    _callback: Option<
        rover_uefi::protocols::bluetooth::attribute::EfiBluetoothAttributeCallbackFunction,
    >,
    _context: *mut c_void,
) -> EfiStatus {
    ATTRIBUTE_SEND_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_attr_reg_notify(
    _this: *mut EfiBluetoothAttributeProtocol,
    _callback_parameter: *mut c_void,
    _callback: Option<
        rover_uefi::protocols::bluetooth::attribute::EfiBluetoothAttributeCallbackFunction,
    >,
    _context: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_attr_get_service_info(
    _this: *mut EfiBluetoothAttributeProtocol,
    _service_info_size: *mut rover_uefi::abi::types::UIntN,
    _service_info: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_attr_get_device_info(
    _this: *mut EfiBluetoothAttributeProtocol,
    _device_info_size: *mut rover_uefi::abi::types::UIntN,
    _device_info: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_bluetooth_attribute_client_wrapper() {
    let mut raw = EfiBluetoothAttributeProtocol {
        send_request: mock_attr_send_request,
        register_for_server_notification: mock_attr_reg_notify,
        get_service_info: mock_attr_get_service_info,
        get_device_info: mock_attr_get_device_info,
    };

    let mut client = BluetoothAttribute::new(&mut raw);
    assert!(!client.raw().is_null());

    let mut pdu = [0x02, 0x01, 0x00];
    let send_res = unsafe { client.send_request_raw(&mut pdu, None, core::ptr::null_mut()) };
    assert!(send_res.is_ok());
    assert!(ATTRIBUTE_SEND_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}

static LE_INIT_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static LE_DISCONNECT_CALLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

unsafe extern "efiapi" fn mock_le_init(_this: *mut EfiBluetoothLeConfigProtocol) -> EfiStatus {
    LE_INIT_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_le_scan(
    _this: *mut EfiBluetoothLeConfigProtocol,
    _re_scan: bool,
    _timeout: u32,
    _scan_parameter: *const EfiBluetoothLeConfigScanParameter,
    _callback: rover_uefi::protocols::bluetooth::le_config::EfiBluetoothLeConfigScanCallbackFunction,
    _context: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_le_connect(
    _this: *mut EfiBluetoothLeConfigProtocol,
    _auto_reconnect: bool,
    _do_bonding: bool,
    _connect_parameter: *const c_void,
    _bd_addr: *const BluetoothLeAddress,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_le_disconnect(
    _this: *mut EfiBluetoothLeConfigProtocol,
    _bd_addr: *const BluetoothLeAddress,
    _reason: u8,
) -> EfiStatus {
    LE_DISCONNECT_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_le_get_data(
    _this: *mut EfiBluetoothLeConfigProtocol,
    _data_type: u32,
    _data_size: *mut rover_uefi::abi::types::UIntN,
    _data: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_le_set_data(
    _this: *mut EfiBluetoothLeConfigProtocol,
    _data_type: u32,
    _data_size: rover_uefi::abi::types::UIntN,
    _data: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_bluetooth_le_config_client_wrapper() {
    let mut raw = EfiBluetoothLeConfigProtocol {
        init: mock_le_init,
        scan: mock_le_scan,
        connect: mock_le_connect,
        disconnect: mock_le_disconnect,
        get_data: mock_le_get_data,
        set_data: mock_le_set_data,
        get_remote_data: core::ptr::null_mut(),
        register_smp_auth_callback: core::ptr::null_mut(),
        send_smp_auth_data: core::ptr::null_mut(),
        register_smp_get_data_callback: core::ptr::null_mut(),
        register_smp_set_data_callback: core::ptr::null_mut(),
        register_link_connect_complete_callback: core::ptr::null_mut(),
    };

    let mut client = BluetoothLeConfig::new(&mut raw);
    assert!(!client.raw().is_null());

    assert!(client.init().is_ok());
    assert!(LE_INIT_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    let addr = BluetoothLeAddress {
        r#type: 0,
        address: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
    };
    assert!(client.disconnect(&addr, 0x13).is_ok());
    assert!(LE_DISCONNECT_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}
