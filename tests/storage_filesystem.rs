//! Tests for file system helpers.

use core::mem::{offset_of, size_of};

use rover_uefi::abi::types::Char16;
use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::storage::filesystem::directory::is_end_of_directory;
use rover_uefi::protocols::storage::filesystem::file::{
    EfiFileProtocol, FILE_DIRECTORY, FILE_MODE_CREATE, FILE_MODE_READ, FILE_MODE_WRITE,
};
use rover_uefi::protocols::storage::filesystem::info::EfiFileIoToken;
use rover_uefi::protocols::storage::filesystem::metadata::{
    EfiFileInfo, EfiFileSystemInfo, SIZE_OF_EFI_FILE_INFO, SIZE_OF_EFI_FILE_SYSTEM_INFO,
    validate_file_info,
};
use rover_uefi::protocols::storage::filesystem::validation::{
    checked_data_size, validate_file_name, validate_open_mode,
};
use rover_uefi::protocols::storage::guid::{
    FILE_INFO_ID, FILE_SYSTEM_INFO_ID, FILE_SYSTEM_VOLUME_LABEL_ID, SIMPLE_FILE_SYSTEM_GUID,
};

#[test]
fn guid_values() {
    assert_eq!(
        FILE_INFO_ID.to_bytes(),
        [
            0x92, 0x6E, 0x57, 0x09, 0x3F, 0x6D, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        FILE_SYSTEM_INFO_ID.to_bytes(),
        [
            0x93, 0x6E, 0x57, 0x09, 0x3F, 0x6D, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        SIMPLE_FILE_SYSTEM_GUID.to_bytes(),
        [
            0x22, 0x5B, 0x4E, 0x96, 0x59, 0x64, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(FILE_SYSTEM_VOLUME_LABEL_ID.to_bytes()),
        FILE_SYSTEM_VOLUME_LABEL_ID
    );
}

#[test]
fn protocol_layouts() {
    assert_eq!(
        size_of::<EfiFileProtocol>(),
        8 + 14 * core::mem::size_of::<usize>()
    );
    assert_eq!(size_of::<EfiFileInfo>(), 88);
    assert_eq!(offset_of!(EfiFileInfo, file_name), SIZE_OF_EFI_FILE_INFO);
    assert_eq!(
        offset_of!(EfiFileSystemInfo, volume_label),
        SIZE_OF_EFI_FILE_SYSTEM_INFO
    );
    assert_eq!(FILE_MODE_READ, 1);
    assert_eq!(FILE_MODE_WRITE, 2);
    assert_eq!(FILE_MODE_CREATE, 0x8000_0000_0000_0000);
    assert_eq!(FILE_DIRECTORY, 0x10);
    assert_eq!(
        size_of::<EfiFileIoToken>(),
        4 * core::mem::size_of::<usize>()
    );
}

#[test]
fn name_helpers() {
    let name: [Char16; 3] = [0x41, 0x42, 0x00];
    assert!(validate_file_name(&name));
    let bad: [Char16; 2] = [0x41, 0x42];
    assert!(!validate_file_name(&bad));
    assert!(validate_open_mode(FILE_MODE_READ));
    assert!(validate_open_mode(FILE_MODE_READ | FILE_MODE_WRITE));
    assert!(validate_open_mode(
        FILE_MODE_READ | FILE_MODE_WRITE | FILE_MODE_CREATE
    ));
    assert!(!validate_open_mode(0));
    assert!(!validate_open_mode(FILE_MODE_WRITE));
    assert_eq!(checked_data_size(4, 2), Some(8));
    assert!(is_end_of_directory(0));
    assert!(!is_end_of_directory(10));
}

#[test]
fn metadata_validation() {
    let mut buffer = [0_u8; 84];
    let size: u64 = 84;
    buffer[0..8].copy_from_slice(&size.to_le_bytes());
    buffer[82] = 0;
    buffer[83] = 0;
    assert!(validate_file_info(&buffer));
    buffer[0] = 0xFF;
    assert!(!validate_file_info(&buffer));
}
