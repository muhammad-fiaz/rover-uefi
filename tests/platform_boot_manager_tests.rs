//! Integration tests for Platform Boot Manager Protocol.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::platform_boot_manager::*;

static mut REFRESH_INVOKED: bool = false;

unsafe extern "efiapi" fn mock_refresh_all_boot_options(
    boot_options: *const EfiBootManagerLoadOption,
    boot_options_count: UIntN,
    updated_boot_options: *mut *mut EfiBootManagerLoadOption,
    updated_boot_options_count: *mut UIntN,
) -> EfiStatus {
    if boot_options.is_null() && boot_options_count > 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        REFRESH_INVOKED = true;
        *updated_boot_options = 0x5000 as *mut EfiBootManagerLoadOption;
        *updated_boot_options_count = boot_options_count + 1;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_platform_boot_manager_guids_and_constants() {
    assert_eq!(
        PLATFORM_BOOT_MANAGER_PROTOCOL_GUID,
        EfiGuid::new(
            0xAA17_ADD4,
            0x756C,
            0x460D,
            [0x94, 0xB8, 0x43, 0x88, 0xD7, 0xFB, 0x3E, 0x59]
        )
    );

    assert_eq!(PLATFORM_BOOT_MANAGER_PROTOCOL_REVISION, 1);

    assert_eq!(BootManagerLoadOptionType::Driver as u32, 0);
    assert_eq!(BootManagerLoadOptionType::SysPrep as u32, 1);
    assert_eq!(BootManagerLoadOptionType::Boot as u32, 2);
    assert_eq!(BootManagerLoadOptionType::PlatformRecovery as u32, 3);

    assert_eq!(load_option_attributes::LOAD_OPTION_ACTIVE, 0x0000_0001);
    assert_eq!(
        load_option_attributes::LOAD_OPTION_FORCE_RECONNECT,
        0x0000_0002
    );
    assert_eq!(load_option_attributes::LOAD_OPTION_HIDDEN, 0x0000_0008);
    assert_eq!(
        load_option_attributes::LOAD_OPTION_CATEGORY_MASK,
        0x0000_1F00
    );
    assert_eq!(
        load_option_attributes::LOAD_OPTION_CATEGORY_BOOT,
        0x0000_0000
    );
    assert_eq!(
        load_option_attributes::LOAD_OPTION_CATEGORY_APP,
        0x0000_0100
    );
}

#[test]
fn test_platform_boot_manager_client_wrapper() {
    let raw_proto = EfiPlatformBootManagerProtocol {
        revision: PLATFORM_BOOT_MANAGER_PROTOCOL_REVISION,
        refresh_all_boot_options: mock_refresh_all_boot_options,
    };

    let client = unsafe { PlatformBootManager::from_raw(&raw const raw_proto) };

    assert_eq!(client.revision(), 1);

    let dummy_option = EfiBootManagerLoadOption {
        option_number: 1,
        option_type: BootManagerLoadOptionType::Boot,
        attributes: load_option_attributes::LOAD_OPTION_ACTIVE,
        description: core::ptr::null(),
        file_path: core::ptr::null(),
        optional_data: core::ptr::null(),
        optional_data_size: 0,
        vendor_guid: EfiGuid::ZERO,
        status: EfiStatus::SUCCESS,
    };

    let options = [dummy_option];
    let (updated_ptr, count) = match client.refresh_all_boot_options(&options) {
        Ok(res) => res,
        Err(e) => panic!("refresh_all_boot_options failed: {e:?}"),
    };

    assert!(unsafe { REFRESH_INVOKED });
    assert_eq!(count, 2);
    assert_eq!(updated_ptr, 0x5000 as *mut EfiBootManagerLoadOption);
}
