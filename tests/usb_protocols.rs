//! USB protocol tests.

use core::mem::size_of;

use rover_uefi::protocols::usb::types::{UsbDeviceRequest, UsbDirection, UsbSpeed};

#[test]
fn direction_values() {
    assert_eq!(UsbDirection::IN.to_raw(), 0);
    assert_eq!(UsbDirection::OUT.to_raw(), 1);
    assert_eq!(UsbDirection::NO_DATA.to_raw(), 2);
    assert_eq!(UsbDirection::from_raw(0), UsbDirection::IN);
    assert_eq!(UsbDirection::from_raw(5).to_raw(), 5);
}

#[test]
fn speed_values() {
    assert_eq!(UsbSpeed::FULL.to_raw(), 0);
    assert_eq!(UsbSpeed::LOW.to_raw(), 1);
    assert_eq!(UsbSpeed::HIGH.to_raw(), 2);
    assert_eq!(UsbSpeed::SUPER.to_raw(), 3);
    assert_eq!(UsbSpeed::from_raw(2), UsbSpeed::HIGH);
}

#[test]
fn device_request_layout() {
    assert_eq!(size_of::<UsbDeviceRequest>(), 8);
    let request = UsbDeviceRequest {
        request_type: 0x80,
        request: 6,
        value: 0x0100,
        index: 0,
        length: 18,
    };
    assert_eq!(request.length, 18);
    assert_eq!(request.value, 0x0100);
}
