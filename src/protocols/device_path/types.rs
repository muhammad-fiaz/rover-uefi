//! # Device path types
//!
//! Header and constants for device paths.
//!
//! The header is 4 bytes: type, subtype, and
//! a little-endian length that includes the
//! header itself. The length array is stored as
//! bytes so the struct does not require packed
//! alignment. Payloads are variable length and
//! are exposed as byte slices in the iterator.

/// Hardware device path.
pub const DEVICE_PATH_HARDWARE: u8 = 0x01;

/// ACPI device path.
pub const DEVICE_PATH_ACPI: u8 = 0x02;

/// Messaging device path.
pub const DEVICE_PATH_MESSAGING: u8 = 0x03;

/// Media device path.
pub const DEVICE_PATH_MEDIA: u8 = 0x04;

/// BIOS boot specification device path.
pub const DEVICE_PATH_BIOS_BOOT: u8 = 0x05;

/// End of device path.
pub const DEVICE_PATH_END: u8 = 0x7F;

/// End this instance subtype.
pub const END_INSTANCE: u8 = 0x01;

/// End entire path subtype.
pub const END_ENTIRE: u8 = 0xFF;

/// Hardware PCI subtype.
pub const HW_PCI: u8 = 0x01;

/// Hardware PCCARD subtype.
pub const HW_PCCARD: u8 = 0x02;

/// Hardware memory mapped subtype.
pub const HW_MEMMAP: u8 = 0x03;

/// Hardware vendor subtype.
pub const HW_VENDOR: u8 = 0x04;

/// Hardware controller subtype.
pub const HW_CONTROLLER: u8 = 0x05;

/// Hardware BMC subtype.
pub const HW_BMC: u8 = 0x06;

/// ACPI HID subtype.
pub const ACPI_HID: u8 = 0x01;

/// ACPI extended HID subtype.
pub const ACPI_EXTENDED_HID: u8 = 0x02;

/// ACPI ADR subtype.
pub const ACPI_ADR: u8 = 0x03;

/// ACPI NVDIMM subtype.
pub const ACPI_NVDIMM: u8 = 0x04;

/// Messaging ATAPI subtype.
pub const MSG_ATAPI: u8 = 0x01;

/// Messaging SCSI subtype.
pub const MSG_SCSI: u8 = 0x02;

/// Messaging Fibre Channel subtype.
pub const MSG_FIBRE_CHANNEL: u8 = 0x03;

/// Messaging USB subtype.
pub const MSG_USB: u8 = 0x05;

/// Messaging SATA subtype.
pub const MSG_SATA: u8 = 0x0A;

/// Messaging `NVMe` subtype.
pub const MSG_NVME: u8 = 0x19;

/// Media HD subtype.
pub const MEDIA_HD: u8 = 0x01;

/// Media file system subtype.
pub const MEDIA_FILE_SYSTEM: u8 = 0x04;

/// Media RAM disk subtype.
pub const MEDIA_RAM_DISK: u8 = 0x09;

/// BIOS boot specification subtype.
pub const BBS_BOOT: u8 = 0x01;

/// A device path header.
///
/// The length includes this header and the
/// payload. The struct is `#[repr(C)]` with
/// size 4 and alignment 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiDevicePath {
    /// The device path type.
    pub device_type: u8,
    /// The subtype.
    pub subtype: u8,
    /// The length in bytes as little-endian.
    pub length: [u8; 2],
}

impl EfiDevicePath {
    /// Returns the node length in bytes.
    #[must_use]
    pub const fn node_length(&self) -> usize {
        u16::from_le_bytes(self.length) as usize
    }

    /// Returns whether this node ends an instance.
    #[must_use]
    pub const fn is_end_instance(&self) -> bool {
        self.device_type == DEVICE_PATH_END && self.subtype == END_INSTANCE
    }

    /// Returns whether this node ends the entire path.
    #[must_use]
    pub const fn is_end_entire(&self) -> bool {
        self.device_type == DEVICE_PATH_END && self.subtype == END_ENTIRE
    }

    /// Creates a header.
    ///
    /// Lengths above `u16::MAX` are clamped.
    #[must_use]
    pub fn new(device_type: u8, subtype: u8, length: usize) -> Self {
        Self {
            device_type,
            subtype,
            length: u16::try_from(length).unwrap_or(u16::MAX).to_le_bytes(),
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<EfiDevicePath>() == 4);
    assert!(core::mem::align_of::<EfiDevicePath>() == 1);
    assert!(core::mem::offset_of!(EfiDevicePath, device_type) == 0);
    assert!(core::mem::offset_of!(EfiDevicePath, subtype) == 1);
    assert!(core::mem::offset_of!(EfiDevicePath, length) == 2);
};
