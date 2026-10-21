//! PCI protocol tests.

use core::mem::size_of;

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::pci::guid::{PCI_IO_GUID, PCI_ROOT_BRIDGE_IO_GUID};
use rover_uefi::protocols::pci::types::PciWidth;

#[test]
fn guid_values() {
    assert_eq!(
        PCI_ROOT_BRIDGE_IO_GUID.to_bytes(),
        [
            0xBB, 0x7E, 0x70, 0x2F, 0x1A, 0x4A, 0xD4, 0x11, 0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        PCI_IO_GUID.to_bytes(),
        [
            0x00, 0xB2, 0xF5, 0x4C, 0xB8, 0x68, 0xA5, 0x4C, 0x9E, 0xEC, 0xB2, 0x3E, 0x3F, 0x50,
            0x02, 0x9A
        ]
    );
    assert_eq!(EfiGuid::from_bytes(PCI_IO_GUID.to_bytes()), PCI_IO_GUID);
}

#[test]
fn width_values() {
    assert_eq!(PciWidth::UINT8.to_raw(), 0);
    assert_eq!(PciWidth::UINT64.byte_width(), Some(8));
    assert_eq!(PciWidth::FIFO_UINT8.byte_width(), Some(1));
    assert_eq!(PciWidth::MAX.byte_width(), None);
    assert_eq!(size_of::<PciWidth>(), 4);
}
