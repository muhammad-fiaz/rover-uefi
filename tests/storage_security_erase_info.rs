// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Storage Security Command, Erase Block, and Disk Info protocols.

use rover_uefi::protocols::storage::{
    EFI_DISK_INFO_AHCI_INTERFACE_GUID, EFI_DISK_INFO_IDE_INTERFACE_GUID,
    EFI_DISK_INFO_NVME_INTERFACE_GUID, EFI_DISK_INFO_PROTOCOL_GUID,
    EFI_DISK_INFO_SCSI_INTERFACE_GUID, EFI_DISK_INFO_SD_MMC_INTERFACE_GUID,
    EFI_DISK_INFO_UFS_INTERFACE_GUID, EFI_DISK_INFO_USB_INTERFACE_GUID,
    EFI_ERASE_BLOCK_PROTOCOL_GUID, EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID,
};

#[test]
fn storage_protocols_guids() {
    assert!(!EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID.is_zero());
    assert!(!EFI_ERASE_BLOCK_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DISK_INFO_PROTOCOL_GUID.is_zero());

    assert!(!EFI_DISK_INFO_IDE_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_SCSI_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_USB_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_AHCI_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_NVME_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_SD_MMC_INTERFACE_GUID.is_zero());
    assert!(!EFI_DISK_INFO_UFS_INTERFACE_GUID.is_zero());
}
