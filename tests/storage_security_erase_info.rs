// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Storage Security Command, Erase Block, and Disk Info protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::storage::{
    EFI_DISK_INFO_AHCI_INTERFACE_GUID, EFI_DISK_INFO_IDE_INTERFACE_GUID,
    EFI_DISK_INFO_NVME_INTERFACE_GUID, EFI_DISK_INFO_PROTOCOL_GUID,
    EFI_DISK_INFO_SCSI_INTERFACE_GUID, EFI_DISK_INFO_SD_MMC_INTERFACE_GUID,
    EFI_DISK_INFO_UFS_INTERFACE_GUID, EFI_DISK_INFO_USB_INTERFACE_GUID,
    EFI_ERASE_BLOCK_PROTOCOL_GUID, EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID,
    EfiStorageSecurityCommandProtocol, StorageSecurityCommand,
};

#[test]
fn storage_protocols_guids() {
    assert_eq!(
        EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID,
        EfiGuid::new(
            0xC88B_0B6D,
            0x0DFC,
            0x49A7,
            [0x9C, 0xB4, 0x49, 0x07, 0x4B, 0x4C, 0x3A, 0x78],
        )
    );
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

unsafe extern "efiapi" fn mock_receive_data(
    _this: *const EfiStorageSecurityCommandProtocol,
    media_id: u32,
    _timeout: u64,
    _security_protocol_id: u8,
    _security_protocol_specific_data: u16,
    payload_buffer_size: UIntN,
    payload_buffer: *mut c_void,
    payload_transfer_size: *mut UIntN,
) -> EfiStatus {
    if media_id != 1 || payload_buffer.is_null() || payload_transfer_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        let to_copy = core::cmp::min(payload_buffer_size, 4);
        let dest = core::slice::from_raw_parts_mut(payload_buffer.cast::<u8>(), to_copy);
        dest.copy_from_slice(&[0x01, 0x02, 0x03, 0x04][..to_copy]);
        *payload_transfer_size = to_copy as UIntN;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_send_data(
    _this: *const EfiStorageSecurityCommandProtocol,
    media_id: u32,
    _timeout: u64,
    _security_protocol_id: u8,
    _security_protocol_specific_data: u16,
    payload_buffer_size: UIntN,
    payload_buffer: *const c_void,
) -> EfiStatus {
    if media_id != 1 || (payload_buffer_size > 0 && payload_buffer.is_null()) {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_storage_security_command_client() {
    let mock_proto = EfiStorageSecurityCommandProtocol {
        receive_data: mock_receive_data,
        send_data: mock_send_data,
    };

    let client = StorageSecurityCommand::new(&mock_proto);

    let mut buf = [0u8; 4];
    let transferred = client
        .receive_data(1, 100_000, 0x01, 0x0001, &mut buf)
        .unwrap_or(0);
    assert_eq!(transferred, 4);
    assert_eq!(&buf, &[0x01, 0x02, 0x03, 0x04]);

    let payload = [0xAA, 0xBB];
    assert!(client.send_data(1, 100_000, 0x01, 0x0001, &payload).is_ok());
    assert!(
        client
            .send_data(2, 100_000, 0x01, 0x0001, &payload)
            .is_err()
    );
}
