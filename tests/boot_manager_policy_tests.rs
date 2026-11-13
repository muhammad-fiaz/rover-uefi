//! Integration tests for Boot Manager Policy protocol.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::device_path::types::EfiDevicePath;
use rover_uefi::protocols::driver::{
    BootManagerPolicy, EFI_BOOT_MANAGER_POLICY_CONNECT_ALL_GUID,
    EFI_BOOT_MANAGER_POLICY_CONSOLE_GUID, EFI_BOOT_MANAGER_POLICY_NETWORK_GUID,
    EFI_BOOT_MANAGER_POLICY_PROTOCOL_GUID, EFI_BOOT_MANAGER_POLICY_PROTOCOL_REVISION,
    EFI_BOOT_MANAGER_POLICY_STORAGE_GUID, EfiBootManagerPolicyProtocol,
};

static mut LAST_CONNECTED_CLASS: Option<EfiGuid> = None;
static mut CONNECTED_ALL_PATHS: bool = false;

unsafe extern "efiapi" fn mock_connect_device_path(
    _this: *const EfiBootManagerPolicyProtocol,
    device_path: *const EfiDevicePath,
    _recursive: bool,
) -> EfiStatus {
    if device_path.is_null() {
        unsafe { CONNECTED_ALL_PATHS = true };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_connect_device_class(
    _this: *const EfiBootManagerPolicyProtocol,
    class: *const EfiGuid,
) -> EfiStatus {
    if class.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        LAST_CONNECTED_CLASS = Some(*class);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_boot_manager_policy_guids_and_constants() {
    assert_eq!(
        EFI_BOOT_MANAGER_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0xFEDF_8E0C,
            0xE147,
            0x11E3,
            [0x99, 0x03, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
        )
    );
    assert_eq!(
        EFI_BOOT_MANAGER_POLICY_CONSOLE_GUID,
        EfiGuid::new(
            0xCAB0_E94C,
            0xE15F,
            0x11E3,
            [0x91, 0x8D, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
        )
    );
    assert_eq!(
        EFI_BOOT_MANAGER_POLICY_NETWORK_GUID,
        EfiGuid::new(
            0xD041_59DC,
            0xE15F,
            0x11E3,
            [0xB2, 0x61, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
        )
    );
    assert_eq!(
        EFI_BOOT_MANAGER_POLICY_CONNECT_ALL_GUID,
        EfiGuid::new(
            0x113B_2126,
            0xFC8A,
            0x11E3,
            [0xBD, 0x6C, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
        )
    );
    assert_eq!(
        EFI_BOOT_MANAGER_POLICY_STORAGE_GUID,
        EfiGuid::new(
            0xCD68_FE79,
            0xD3CB,
            0x436E,
            [0xA8, 0x50, 0xF4, 0x43, 0xC8, 0x8C, 0xFB, 0x49],
        )
    );
    assert_eq!(EFI_BOOT_MANAGER_POLICY_PROTOCOL_REVISION, 0x0001_0000);
}

#[test]
fn test_boot_manager_policy_operations() {
    let mock_proto = EfiBootManagerPolicyProtocol {
        revision: EFI_BOOT_MANAGER_POLICY_PROTOCOL_REVISION,
        connect_device_path: mock_connect_device_path,
        connect_device_class: mock_connect_device_class,
    };

    let policy = BootManagerPolicy::new(&mock_proto);
    assert_eq!(policy.revision(), EFI_BOOT_MANAGER_POLICY_PROTOCOL_REVISION);

    assert!(policy.connect_device_path(None, false).is_ok());
    assert!(unsafe { CONNECTED_ALL_PATHS });

    assert!(policy.connect_consoles().is_ok());
    assert_eq!(
        unsafe { LAST_CONNECTED_CLASS },
        Some(EFI_BOOT_MANAGER_POLICY_CONSOLE_GUID)
    );

    assert!(policy.connect_network().is_ok());
    assert_eq!(
        unsafe { LAST_CONNECTED_CLASS },
        Some(EFI_BOOT_MANAGER_POLICY_NETWORK_GUID)
    );

    assert!(policy.connect_all().is_ok());
    assert_eq!(
        unsafe { LAST_CONNECTED_CLASS },
        Some(EFI_BOOT_MANAGER_POLICY_CONNECT_ALL_GUID)
    );

    assert!(policy.connect_storage().is_ok());
    assert_eq!(
        unsafe { LAST_CONNECTED_CLASS },
        Some(EFI_BOOT_MANAGER_POLICY_STORAGE_GUID)
    );
}
