//! Embedded platform device and peripheral protocol integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::embedded::{
    EMBEDDED_DEVICE_PROTOCOL_GUID, EMBEDDED_EXTERNAL_DEVICE_PROTOCOL_GUID, EmbeddedDeviceProtocol,
    EmbeddedExternalDevice, EmbeddedExternalDeviceProtocol, USB_DEVICE_PROTOCOL_GUID, UsbDevice,
    UsbDeviceProtocol,
};

#[test]
fn test_embedded_device_guids() {
    let expected_dev = EfiGuid::new(
        0xBF4B_9D10,
        0x13EC,
        0x43DD,
        [0x88, 0x80, 0xE9, 0x0B, 0x71, 0x8F, 0x27, 0xDE],
    );
    assert_eq!(EMBEDDED_DEVICE_PROTOCOL_GUID, expected_dev);

    let expected_ext = EfiGuid::new(
        0x735F_8C64,
        0xD696,
        0x44D0,
        [0xBD, 0xF2, 0x44, 0x7F, 0xD0, 0x5A, 0x54, 0x06],
    );
    assert_eq!(EMBEDDED_EXTERNAL_DEVICE_PROTOCOL_GUID, expected_ext);

    let expected_usb = EfiGuid::new(
        0x021B_D2CA,
        0x51D2,
        0x11E3,
        [0x8E, 0x56, 0xB7, 0x54, 0x17, 0xC7, 0x0B, 0x44],
    );
    assert_eq!(USB_DEVICE_PROTOCOL_GUID, expected_usb);
}

#[test]
fn test_embedded_device_protocol_struct() {
    let dev = EmbeddedDeviceProtocol {
        vendor_id: 0x1234,
        device_id: 0x5678,
        revision_id: 1,
        subsystem_id: 0x9ABC,
        subsystem_vendor_id: 0xDEF0,
        class_code: [0x02, 0x00, 0x00], // Ethernet controller
        header_size: 0x10,
        base_address: 0xE000_0000,
    };

    assert_eq!(dev.vendor_id, 0x1234);
    assert_eq!(dev.device_id, 0x5678);
    assert_eq!(dev.revision_id, 1);
    assert_eq!(dev.base_address, 0xE000_0000);
}

unsafe extern "efiapi" fn mock_ext_read(
    _this: *mut EmbeddedExternalDeviceProtocol,
    register: UIntN,
    length: UIntN,
    buffer: *mut core::ffi::c_void,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer.cast::<u8>(), length) };
    #[allow(clippy::cast_possible_truncation)]
    for (i, b) in slice.iter_mut().enumerate() {
        *b = (register + i) as u8;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ext_write(
    _this: *mut EmbeddedExternalDeviceProtocol,
    _register: UIntN,
    _length: UIntN,
    buffer: *const core::ffi::c_void,
) -> EfiStatus {
    if buffer.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

#[test]
fn test_embedded_external_device_wrapper() {
    let mut mock_proto = EmbeddedExternalDeviceProtocol {
        read: Some(mock_ext_read),
        write: Some(mock_ext_write),
    };

    let Some(mut ext_dev) = (unsafe { EmbeddedExternalDevice::from_raw(&raw mut mock_proto) })
    else {
        panic!("failed to wrap external device protocol");
    };

    let mut buf = [0u8; 4];
    let res_read = ext_dev.read_registers(0x10, &mut buf);
    assert_eq!(res_read, Ok(()));
    assert_eq!(buf, [0x10, 0x11, 0x12, 0x13]);

    let res_write = ext_dev.write_registers(0x10, &[1, 2, 3]);
    assert_eq!(res_write, Ok(()));
}

unsafe extern "efiapi" fn mock_usb_send(
    endpoint_index: u8,
    size: UIntN,
    buffer: *const core::ffi::c_void,
) -> EfiStatus {
    if buffer.is_null() || endpoint_index > 15 || size == 0 {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

#[test]
fn test_usb_device_wrapper() {
    let mut mock_proto = UsbDeviceProtocol {
        start: None,
        send: Some(mock_usb_send),
    };

    let Some(mut usb) = (unsafe { UsbDevice::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap usb device protocol");
    };

    let data = [0x55, 0xAA];
    let res = usb.send(1, &data);
    assert_eq!(res, Ok(()));

    let res_err = usb.send(20, &data);
    assert_eq!(res_err, Err(EfiStatus::INVALID_PARAMETER));
}
