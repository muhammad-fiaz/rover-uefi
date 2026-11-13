//! Tests for PCI hardware class codes, device categorization, and accelerator detection.

use rover_uefi::protocols::pci::class::*;

#[test]
fn test_pci_base_classes_values() {
    assert_eq!(BASE_CLASS_OLD, 0x00);
    assert_eq!(BASE_CLASS_MASS_STORAGE, 0x01);
    assert_eq!(BASE_CLASS_NETWORK, 0x02);
    assert_eq!(BASE_CLASS_DISPLAY, 0x03);
    assert_eq!(BASE_CLASS_MULTIMEDIA, 0x04);
    assert_eq!(BASE_CLASS_MEMORY, 0x05);
    assert_eq!(BASE_CLASS_BRIDGE, 0x06);
    assert_eq!(BASE_CLASS_COMMUNICATION, 0x07);
    assert_eq!(BASE_CLASS_SYSTEM_PERIPHERAL, 0x08);
    assert_eq!(BASE_CLASS_INPUT, 0x09);
    assert_eq!(BASE_CLASS_DOCKING, 0x0A);
    assert_eq!(BASE_CLASS_PROCESSOR, 0x0B);
    assert_eq!(BASE_CLASS_SERIAL_BUS, 0x0C);
    assert_eq!(BASE_CLASS_WIRELESS, 0x0D);
    assert_eq!(BASE_CLASS_INTELLIGENT_IO, 0x0E);
    assert_eq!(BASE_CLASS_SATELLITE, 0x0F);
    assert_eq!(BASE_CLASS_ENCRYPTION, 0x10);
    assert_eq!(BASE_CLASS_SIGNAL_PROCESSING, 0x11);
    assert_eq!(BASE_CLASS_PROCESSING_ACCELERATOR, 0x12);
    assert_eq!(BASE_CLASS_INSTRUMENTATION, 0x13);
    assert_eq!(BASE_CLASS_VENDOR_SPECIFIC, 0xFF);
}

#[test]
fn test_pci_ai_processing_accelerator() {
    assert!(is_processing_accelerator(BASE_CLASS_PROCESSING_ACCELERATOR));
    assert!(!is_processing_accelerator(BASE_CLASS_DISPLAY));
    assert!(!is_processing_accelerator(BASE_CLASS_MASS_STORAGE));
}

#[test]
fn test_pci_nvme_and_sata() {
    // NVMe: Class 0x01, Subclass 0x08, ProgIf 0x02
    assert!(is_nvme(0x01, 0x08, 0x02));
    assert!(!is_nvme(0x01, 0x06, 0x01)); // SATA AHCI is not NVMe

    // SATA AHCI: Class 0x01, Subclass 0x06, ProgIf 0x01
    assert!(is_sata_ahci(0x01, 0x06, 0x01));
    assert!(!is_sata_ahci(0x01, 0x08, 0x02));
}

#[test]
fn test_pci_multimedia_and_audio() {
    assert!(is_multimedia(BASE_CLASS_MULTIMEDIA));
    assert!(is_audio_hda(BASE_CLASS_MULTIMEDIA, SUBCLASS_HDA));
    assert!(!is_audio_hda(BASE_CLASS_MULTIMEDIA, SUBCLASS_VIDEO));
}

#[test]
fn test_pci_serial_bus_and_usb() {
    assert!(is_serial_bus(BASE_CLASS_SERIAL_BUS));
    assert!(is_usb(BASE_CLASS_SERIAL_BUS, SUBCLASS_USB));
    assert!(is_usb_xhci(
        BASE_CLASS_SERIAL_BUS,
        SUBCLASS_USB,
        PROG_IF_USB_XHCI
    ));
    assert!(is_usb_ehci(
        BASE_CLASS_SERIAL_BUS,
        SUBCLASS_USB,
        PROG_IF_USB_EHCI
    ));
    assert!(is_smbus(BASE_CLASS_SERIAL_BUS, SUBCLASS_SMBUS));
}

#[test]
fn test_pci_wireless_and_bluetooth() {
    assert!(is_wireless(BASE_CLASS_WIRELESS));
    assert!(is_wifi(BASE_CLASS_WIRELESS, SUBCLASS_WIFI));
    assert!(is_bluetooth(BASE_CLASS_WIRELESS, SUBCLASS_BLUETOOTH));
}
