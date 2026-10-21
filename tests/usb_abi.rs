//! USB ABI tests.

use core::mem::size_of;

use rover_uefi::protocols::usb::host_controller::usb2::UsbPortStatus;
use rover_uefi::protocols::usb::usb_io::protocol::EfiUsbIoProtocol;

#[test]
fn protocol_sizes() {
    assert_eq!(
        size_of::<EfiUsbIoProtocol>(),
        13 * core::mem::size_of::<usize>()
    );
    assert_eq!(size_of::<UsbPortStatus>(), 4);
}
