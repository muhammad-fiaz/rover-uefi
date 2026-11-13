//! # PCI Device Classes
//!
//! Standard PCI base class codes, subclass codes, programming interfaces,
//! and hardware classification helpers according to the PCI Local Bus Specification
//! and PCI Code and ID Assignment Specification.

// Base Classes (0x00 .. 0x13, 0xFF)

/// Unclassified / legacy device base class.
pub const BASE_CLASS_OLD: u8 = 0x00;
/// Mass storage controller base class.
pub const BASE_CLASS_MASS_STORAGE: u8 = 0x01;
/// Network controller base class.
pub const BASE_CLASS_NETWORK: u8 = 0x02;
/// Display controller base class.
pub const BASE_CLASS_DISPLAY: u8 = 0x03;
/// Multimedia controller base class (audio, video).
pub const BASE_CLASS_MULTIMEDIA: u8 = 0x04;
/// Memory controller base class (RAM, Flash).
pub const BASE_CLASS_MEMORY: u8 = 0x05;
/// Bridge device base class (Host, PCI-to-PCI, `CardBus`).
pub const BASE_CLASS_BRIDGE: u8 = 0x06;
/// Simple communication controller base class (UART, modem).
pub const BASE_CLASS_COMMUNICATION: u8 = 0x07;
/// Generic system peripheral base class (PIC, DMA, Timer, RTC, IOMMU).
pub const BASE_CLASS_SYSTEM_PERIPHERAL: u8 = 0x08;
/// Input device controller base class (Keyboard, Mouse, Touch).
pub const BASE_CLASS_INPUT: u8 = 0x09;
/// Docking station base class.
pub const BASE_CLASS_DOCKING: u8 = 0x0A;
/// Processor base class (x86, ARM, RISC-V, co-processor).
pub const BASE_CLASS_PROCESSOR: u8 = 0x0B;
/// Serial bus controller base class (`FireWire`, USB, `SMBus`).
pub const BASE_CLASS_SERIAL_BUS: u8 = 0x0C;
/// Wireless controller base class (Bluetooth, `WiFi`, RF).
pub const BASE_CLASS_WIRELESS: u8 = 0x0D;
/// Intelligent I/O controller base class (I2O).
pub const BASE_CLASS_INTELLIGENT_IO: u8 = 0x0E;
/// Satellite communication controller base class.
pub const BASE_CLASS_SATELLITE: u8 = 0x0F;
/// Encryption and decryption controller base class.
pub const BASE_CLASS_ENCRYPTION: u8 = 0x10;
/// Signal processing controller base class.
pub const BASE_CLASS_SIGNAL_PROCESSING: u8 = 0x11;
/// Processing accelerator base class (AI, NPU, TPU, FPGA accelerators).
pub const BASE_CLASS_PROCESSING_ACCELERATOR: u8 = 0x12;
/// Non-essential instrumentation base class.
pub const BASE_CLASS_INSTRUMENTATION: u8 = 0x13;
/// Vendor-specific / miscellaneous device base class.
pub const BASE_CLASS_VENDOR_SPECIFIC: u8 = 0xFF;

// Subclasses for Mass Storage (0x01)
/// SCSI bus controller subclass.
pub const SUBCLASS_SCSI: u8 = 0x00;
/// IDE controller subclass.
pub const SUBCLASS_IDE: u8 = 0x01;
/// Floppy disk controller subclass.
pub const SUBCLASS_FLOPPY: u8 = 0x02;
/// IPI bus controller subclass.
pub const SUBCLASS_IPI: u8 = 0x03;
/// RAID controller subclass.
pub const SUBCLASS_RAID: u8 = 0x04;
/// ATA controller with ADMA subclass.
pub const SUBCLASS_ATA: u8 = 0x05;
/// Serial ATA (SATA) controller subclass.
pub const SUBCLASS_SATA: u8 = 0x06;
/// Serial Attached SCSI (SAS) controller subclass.
pub const SUBCLASS_SAS: u8 = 0x07;
/// Non-Volatile Memory (`NVMe` / NVMHCI) solid state subclass.
pub const SUBCLASS_NVME: u8 = 0x08;
/// Universal Flash Storage (UFS) subclass.
pub const SUBCLASS_UFS: u8 = 0x09;

// Subclasses for Network (0x02)
/// Ethernet controller subclass.
pub const SUBCLASS_ETHERNET: u8 = 0x00;
/// Token ring controller subclass.
pub const SUBCLASS_TOKEN_RING: u8 = 0x01;
/// FDDI controller subclass.
pub const SUBCLASS_FDDI: u8 = 0x02;
/// ATM network controller subclass.
pub const SUBCLASS_ATM: u8 = 0x03;
/// ISDN controller subclass.
pub const SUBCLASS_ISDN: u8 = 0x04;
/// `WorldFip` controller subclass.
pub const SUBCLASS_WORLDFIP: u8 = 0x05;
/// PICMG multi-computing subclass.
pub const SUBCLASS_PICMG: u8 = 0x06;
/// InfiniBand network controller subclass.
pub const SUBCLASS_INFINIBAND: u8 = 0x07;
/// Host fabric controller subclass.
pub const SUBCLASS_FABRIC: u8 = 0x08;

// Subclasses for Display (0x03)
/// VGA compatible controller subclass.
pub const SUBCLASS_VGA: u8 = 0x00;
/// XGA controller subclass.
pub const SUBCLASS_XGA: u8 = 0x01;
/// 3D controller (not VGA compatible) subclass.
pub const SUBCLASS_3D: u8 = 0x02;

// Subclasses for Multimedia (0x04)
/// Video device subclass.
pub const SUBCLASS_VIDEO: u8 = 0x00;
/// Audio device (legacy sound) subclass.
pub const SUBCLASS_AUDIO: u8 = 0x01;
/// Computer telephony device subclass.
pub const SUBCLASS_TELEPHONY: u8 = 0x02;
/// High Definition Audio (HDA) / Mixed mode subclass.
pub const SUBCLASS_HDA: u8 = 0x03;

// Subclasses for Bridge (0x06)
/// Host bridge subclass.
pub const SUBCLASS_BRIDGE_HOST: u8 = 0x00;
/// ISA bridge subclass.
pub const SUBCLASS_BRIDGE_ISA: u8 = 0x01;
/// PCI-to-PCI bridge subclass.
pub const SUBCLASS_BRIDGE_P2P: u8 = 0x04;
/// `CardBus` bridge subclass.
pub const SUBCLASS_BRIDGE_CARDBUS: u8 = 0x07;

// Subclasses for Serial Bus (0x0C)
/// `FireWire` (IEEE 1394) subclass.
pub const SUBCLASS_FIREWIRE: u8 = 0x00;
/// ACCESS bus subclass.
pub const SUBCLASS_ACCESS_BUS: u8 = 0x01;
/// SSA subclass.
pub const SUBCLASS_SSA: u8 = 0x02;
/// USB controller subclass.
pub const SUBCLASS_USB: u8 = 0x03;
/// Fibre Channel subclass.
pub const SUBCLASS_FIBRE_CHANNEL: u8 = 0x04;
/// `SMBus` (System Management Bus) controller subclass.
pub const SUBCLASS_SMBUS: u8 = 0x05;
/// MIPI I3C host controller subclass.
pub const SUBCLASS_I3C: u8 = 0x07;

// Subclasses for Wireless (0x0D)
/// iRDA compatible controller subclass.
pub const SUBCLASS_IRDA: u8 = 0x00;
/// Consumer IR controller subclass.
pub const SUBCLASS_CONSUMER_IR: u8 = 0x01;
/// RF / Bluetooth controller subclass.
pub const SUBCLASS_BLUETOOTH: u8 = 0x11;
/// 802.11a/b/g/n/ac/ax `WiFi` controller subclass.
pub const SUBCLASS_WIFI: u8 = 0x20;

// Subclasses for Processing Accelerators (0x12)
/// Generic AI / Neural Processing Unit (NPU) / Tensor Processing Unit (TPU) subclass.
pub const SUBCLASS_PROCESSING_ACCELERATOR: u8 = 0x00;

// Programming Interfaces for USB (Base 0x0C, Sub 0x03)
/// USB UHCI programming interface.
pub const PROG_IF_USB_UHCI: u8 = 0x00;
/// USB OHCI programming interface.
pub const PROG_IF_USB_OHCI: u8 = 0x10;
/// USB EHCI programming interface.
pub const PROG_IF_USB_EHCI: u8 = 0x20;
/// USB XHCI programming interface.
pub const PROG_IF_USB_XHCI: u8 = 0x30;
/// USB4 host interface.
pub const PROG_IF_USB4: u8 = 0x40;
/// USB device (not host controller).
pub const PROG_IF_USB_DEVICE: u8 = 0xFE;

// Programming Interfaces for Storage
/// `NVMe` NVMHCI 1.0+ programming interface.
pub const PROG_IF_NVME: u8 = 0x02;
/// AHCI 1.0 SATA programming interface.
pub const PROG_IF_SATA_AHCI: u8 = 0x01;

// Classification Helper Functions

/// Returns whether the class represents a mass storage controller.
#[must_use]
pub const fn is_mass_storage(base_class: u8) -> bool {
    base_class == BASE_CLASS_MASS_STORAGE
}

/// Returns whether the device is an `NVMe` solid state controller.
#[must_use]
pub const fn is_nvme(base_class: u8, subclass: u8, prog_if: u8) -> bool {
    base_class == BASE_CLASS_MASS_STORAGE && subclass == SUBCLASS_NVME && prog_if == PROG_IF_NVME
}

/// Returns whether the device is a SATA AHCI controller.
#[must_use]
pub const fn is_sata_ahci(base_class: u8, subclass: u8, prog_if: u8) -> bool {
    base_class == BASE_CLASS_MASS_STORAGE
        && subclass == SUBCLASS_SATA
        && prog_if == PROG_IF_SATA_AHCI
}

/// Returns whether the class represents a network controller.
#[must_use]
pub const fn is_network(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_NETWORK && subclass == SUBCLASS_ETHERNET
}

/// Returns whether the device is an Ethernet controller.
#[must_use]
pub const fn is_ethernet(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_NETWORK && subclass == SUBCLASS_ETHERNET
}

/// Returns whether the class represents a display controller (e.g. GPU).
#[must_use]
pub const fn is_display(base_class: u8) -> bool {
    base_class == BASE_CLASS_DISPLAY
}

/// Returns whether the class represents a multimedia controller (video, audio).
#[must_use]
pub const fn is_multimedia(base_class: u8) -> bool {
    base_class == BASE_CLASS_MULTIMEDIA
}

/// Returns whether the device is a High Definition Audio (HDA) controller.
#[must_use]
pub const fn is_audio_hda(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_MULTIMEDIA && subclass == SUBCLASS_HDA
}

/// Returns whether the class represents a PCI bridge.
#[must_use]
pub const fn is_bridge(base_class: u8) -> bool {
    base_class == BASE_CLASS_BRIDGE
}

/// Returns whether the class represents a system peripheral (timers, IOMMU, etc.).
#[must_use]
pub const fn is_system_peripheral(base_class: u8) -> bool {
    base_class == BASE_CLASS_SYSTEM_PERIPHERAL
}

/// Returns whether the class represents an input device controller.
#[must_use]
pub const fn is_input(base_class: u8) -> bool {
    base_class == BASE_CLASS_INPUT
}

/// Returns whether the class represents a serial bus controller.
#[must_use]
pub const fn is_serial_bus(base_class: u8) -> bool {
    base_class == BASE_CLASS_SERIAL_BUS
}

/// Returns whether the device is a USB host controller.
#[must_use]
pub const fn is_usb(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_SERIAL_BUS && subclass == SUBCLASS_USB
}

/// Returns whether the device is an XHCI (USB 3.x) host controller.
#[must_use]
pub const fn is_usb_xhci(base_class: u8, subclass: u8, prog_if: u8) -> bool {
    base_class == BASE_CLASS_SERIAL_BUS && subclass == SUBCLASS_USB && prog_if == PROG_IF_USB_XHCI
}

/// Returns whether the device is an EHCI (USB 2.0) host controller.
#[must_use]
pub const fn is_usb_ehci(base_class: u8, subclass: u8, prog_if: u8) -> bool {
    base_class == BASE_CLASS_SERIAL_BUS && subclass == SUBCLASS_USB && prog_if == PROG_IF_USB_EHCI
}

/// Returns whether the device is an `SMBus` host controller.
#[must_use]
pub const fn is_smbus(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_SERIAL_BUS && subclass == SUBCLASS_SMBUS
}

/// Returns whether the device is a wireless controller (Bluetooth, `WiFi`).
#[must_use]
pub const fn is_wireless(base_class: u8) -> bool {
    base_class == BASE_CLASS_WIRELESS
}

/// Returns whether the device is a `WiFi` (802.11) network adapter.
#[must_use]
pub const fn is_wifi(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_WIRELESS && subclass == SUBCLASS_WIFI
}

/// Returns whether the device is a Bluetooth controller.
#[must_use]
pub const fn is_bluetooth(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_WIRELESS && subclass == SUBCLASS_BLUETOOTH
}

/// Returns whether the device is a Processing Accelerator (AI / NPU / TPU / FPGA).
#[must_use]
pub const fn is_processing_accelerator(base_class: u8) -> bool {
    base_class == BASE_CLASS_PROCESSING_ACCELERATOR
}
