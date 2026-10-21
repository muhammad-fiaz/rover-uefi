//! Tests for Device Security protocol.

use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::security::device_security::{
    DEVICE_IDENTIFIER_REVISION, DEVICE_IDENTIFIER_TYPE_PCI_GUID, DEVICE_IDENTIFIER_TYPE_USB_GUID,
    DEVICE_SECURITY_PROTOCOL_GUID, DEVICE_SECURITY_PROTOCOL_REVISION, EfiDeviceIdentifier,
    EfiDeviceSecurityProtocol,
};

#[test]
fn test_device_security_guids_and_constants() {
    assert_eq!(DEVICE_SECURITY_PROTOCOL_GUID.data1, 0x5D6B_38C8);
    assert_eq!(DEVICE_IDENTIFIER_TYPE_PCI_GUID.data1, 0x2509_B2F1);
    assert_eq!(DEVICE_IDENTIFIER_TYPE_USB_GUID.data1, 0x7394_F350);
    assert_eq!(DEVICE_SECURITY_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(DEVICE_IDENTIFIER_REVISION, 0x0001_0000);
}

unsafe extern "efiapi" fn dummy_device_authenticate(
    _this: *mut EfiDeviceSecurityProtocol,
    _device_id: *mut EfiDeviceIdentifier,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_device_security_protocol() {
    let mut dev_sec = EfiDeviceSecurityProtocol {
        revision: DEVICE_SECURITY_PROTOCOL_REVISION,
        device_authenticate: dummy_device_authenticate,
    };

    let mut dev_id = EfiDeviceIdentifier {
        version: DEVICE_IDENTIFIER_REVISION,
        device_type: DEVICE_IDENTIFIER_TYPE_PCI_GUID,
        device_handle: EfiHandle::NULL,
    };

    let status = unsafe { (dev_sec.device_authenticate)(&raw mut dev_sec, &raw mut dev_id) };
    assert_eq!(status, EfiStatus::SUCCESS);
}
