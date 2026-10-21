//! # Device path
//!
//! The `EFI_DEVICE_PATH_PROTOCOL` and
//! parsing helpers.
//!
//! Callers provide byte buffers. The iterator
//! validates lengths and never reads beyond
//! the buffer. Use [`validate_device_path`]
//! before trusting firmware data.

pub mod iterator;
pub mod types;
pub mod utilities;
pub mod validation;

pub use iterator::{DevicePathIter, DevicePathNode, device_path_iter, is_end_node};
pub use types::{
    ACPI_ADR, ACPI_EXTENDED_HID, ACPI_HID, ACPI_NVDIMM, BBS_BOOT, DEVICE_PATH_ACPI,
    DEVICE_PATH_BIOS_BOOT, DEVICE_PATH_END, DEVICE_PATH_HARDWARE, DEVICE_PATH_MEDIA,
    DEVICE_PATH_MESSAGING, END_ENTIRE, END_INSTANCE, EfiDevicePath, HW_BMC, HW_CONTROLLER,
    HW_MEMMAP, HW_PCCARD, HW_PCI, HW_VENDOR, MEDIA_FILE_SYSTEM, MEDIA_HD, MEDIA_RAM_DISK,
    MSG_ATAPI, MSG_FIBRE_CHANNEL, MSG_NVME, MSG_SATA, MSG_SCSI, MSG_USB,
};
pub use utilities::{InstanceIter, MAX_DEVICE_PATH_SIZE, checked_node_end, device_path_instances};
pub use validation::{node_count, validate_device_path, validated_length};
