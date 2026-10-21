//! PCI enumeration tests.

use rover_uefi::base::EfiHandle;
use rover_uefi::protocols::pci::class::{is_display, is_mass_storage, is_network};
use rover_uefi::protocols::pci::device::PciLocation;
use rover_uefi::protocols::pci::enumeration::make_device;
use rover_uefi::protocols::pci::types::{decode_bdf, pci_address};
use rover_uefi::protocols::pci::validation::{validate_bar_index, validate_location};

#[test]
fn address_encoding() {
    let addr = pci_address(1, 2, 3, 0x10);
    assert_eq!(decode_bdf(addr), (1, 2, 3));
}

#[test]
fn class_helpers() {
    assert!(is_mass_storage(0x01));
    assert!(is_network(0x02, 0x00));
    assert!(is_display(0x03));
    assert!(!is_mass_storage(0x02));
}

#[test]
fn location_helpers() {
    assert!(validate_bar_index(5));
    assert!(!validate_bar_index(6));
    assert!(validate_location(0, 0, 1, 0));
    assert!(!validate_location(0, 0, 32, 0));
    let device = make_device(
        EfiHandle::NULL,
        PciLocation {
            segment: 0,
            bus: 0,
            device: 1,
            function: 0,
        },
    );
    assert!(device.handle.is_null());
}
