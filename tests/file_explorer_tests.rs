//! Integration tests for File Explorer Protocol.

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::device_path::EfiDevicePath;
use rover_uefi::protocols::storage::file_explorer::{
    EfiFileExplorerChooseHandler, EfiFileExplorerProtocol, FILE_EXPLORER_PROTOCOL_GUID,
    FileExplorer,
};

static mut CHOSEN_PATH_CALLS: usize = 0;
static mut MOCK_TARGET_DEVICE_PATH: EfiDevicePath = EfiDevicePath {
    device_type: 0x01,
    subtype: 0x01,
    length: [4, 0],
};

unsafe extern "efiapi" fn mock_choose_file(
    _root_dir: *const EfiDevicePath,
    _file_type: *const u16,
    handler: Option<EfiFileExplorerChooseHandler>,
    file: *mut *mut EfiDevicePath,
) -> EfiStatus {
    unsafe {
        CHOSEN_PATH_CALLS += 1;
        if let Some(cb) = handler {
            let _ = cb(&raw const MOCK_TARGET_DEVICE_PATH);
        }
        *file = &raw mut MOCK_TARGET_DEVICE_PATH;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn sample_choose_handler(_file: *const EfiDevicePath) -> Bool {
    1
}

#[test]
fn test_file_explorer_guid() {
    assert_eq!(
        FILE_EXPLORER_PROTOCOL_GUID,
        EfiGuid::new(
            0x2C03_C536,
            0x4594,
            0x4515,
            [0x9E, 0x7A, 0xD3, 0xD2, 0x04, 0xFE, 0x13, 0x63],
        )
    );
}

#[test]
#[allow(clippy::cast_lossless)]
fn test_file_explorer_client() {
    let mock_proto = EfiFileExplorerProtocol {
        choose_file: mock_choose_file,
    };

    let explorer = unsafe { FileExplorer::from_raw(&raw const mock_proto) };
    let ext = [b'e' as u16, b'f' as u16, b'i' as u16, 0];

    let result = explorer.choose_file(None, Some(&ext), Some(sample_choose_handler));
    assert!(result.is_ok());
    assert_eq!(unsafe { CHOSEN_PATH_CALLS }, 1);
    let chosen = result.unwrap_or(core::ptr::null_mut());
    assert!(!chosen.is_null());
}
