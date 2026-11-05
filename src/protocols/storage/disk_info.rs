//! # Disk Info Protocol
//!
//! Defined by the UEFI Specification Section 13.12: Disk Info Protocol.
//!
//! Returns hardware-specific device inquiry, identify, and sense data for
//! IDE, SCSI, USB, AHCI, `NVMe`, SD/MMC, and UFS controllers.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_DISK_INFO_PROTOCOL_GUID`.
pub const EFI_DISK_INFO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD432_A67F,
    0x14DC,
    0x483B,
    [0xB2, 0x25, 0x00, 0xC6, 0x5C, 0x4B, 0x02, 0x87],
);

/// IDE interface type GUID.
pub const EFI_DISK_INFO_IDE_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x5E94_8351,
    0x0553,
    0x4AC6,
    [0x9E, 0x1F, 0x9E, 0x10, 0xC7, 0x39, 0xC7, 0xEC],
);

/// SCSI interface type GUID.
pub const EFI_DISK_INFO_SCSI_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x08F7_4D40,
    0xB9A6,
    0x4251,
    [0x88, 0x26, 0x0F, 0xDC, 0xA1, 0x04, 0x88, 0x4E],
);

/// USB interface type GUID.
pub const EFI_DISK_INFO_USB_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x0808_4346,
    0x8929,
    0x4057,
    [0xBD, 0x0C, 0x24, 0x58, 0x48, 0x88, 0x73, 0x41],
);

/// AHCI interface type GUID.
pub const EFI_DISK_INFO_AHCI_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x9E49_8AA4,
    0x4494,
    0x436E,
    [0x80, 0x2E, 0x31, 0x4F, 0x70, 0x10, 0x42, 0x4D],
);

/// `NVMe` interface type GUID.
pub const EFI_DISK_INFO_NVME_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x3CD2_080B,
    0x985D,
    0x4D14,
    [0xAB, 0xD5, 0x58, 0xC5, 0x77, 0x4D, 0xCD, 0x55],
);

/// SD/MMC interface type GUID.
pub const EFI_DISK_INFO_SD_MMC_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0x8DEE_C97E,
    0x0423,
    0x425C,
    [0xA5, 0xCE, 0x34, 0x8B, 0xAE, 0x64, 0xA1, 0x46],
);

/// UFS interface type GUID.
pub const EFI_DISK_INFO_UFS_INTERFACE_GUID: EfiGuid = EfiGuid::new(
    0xD811_40A7,
    0xCF77,
    0x41AB,
    [0x80, 0x2C, 0x5E, 0xA6, 0x46, 0xC0, 0xC8, 0x60],
);

/// Function pointer for `EFI_DISK_INFO_INQUIRY`.
pub type EfiDiskInfoInquiry = unsafe extern "efiapi" fn(
    this: *const EfiDiskInfoProtocol,
    inquiry_data: *mut c_void,
    inquiry_data_size: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_DISK_INFO_IDENTIFY`.
pub type EfiDiskInfoIdentify = unsafe extern "efiapi" fn(
    this: *const EfiDiskInfoProtocol,
    identify_data: *mut c_void,
    identify_data_size: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_DISK_INFO_SENSE_DATA`.
pub type EfiDiskInfoSenseData = unsafe extern "efiapi" fn(
    this: *const EfiDiskInfoProtocol,
    sense_data: *mut c_void,
    sense_data_size: *mut u32,
    sense_data_number: *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_DISK_INFO_WHICH_IDE`.
pub type EfiDiskInfoWhichIde = unsafe extern "efiapi" fn(
    this: *const EfiDiskInfoProtocol,
    ide_channel: *mut u32,
    ide_device: *mut u32,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DISK_INFO_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDiskInfoProtocol {
    /// Interface type GUID identifying device class (IDE, SCSI, AHCI, `NVMe`, etc.).
    pub interface: EfiGuid,
    /// Returns standard SCSI/ATAPI inquiry data.
    pub inquiry: EfiDiskInfoInquiry,
    /// Returns ATA identify or `NVMe` controller identify data.
    pub identify: EfiDiskInfoIdentify,
    /// Returns SCSI request sense or error diagnostic data.
    pub sense_data: EfiDiskInfoSenseData,
    /// Returns IDE channel and master/slave device index.
    pub which_ide: EfiDiskInfoWhichIde,
}
