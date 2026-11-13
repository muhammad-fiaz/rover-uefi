//! Integration tests for Platform To Driver Configuration protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::driver::{
    EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID,
    EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL_GUID, EfiConfigureClpParameterBlock,
    EfiPlatformConfigurationAction, EfiPlatformToDriverConfigurationProtocol,
    PlatformToDriverConfiguration,
};

static mut LAST_ACTION: Option<EfiPlatformConfigurationAction> = None;

struct SyncClp(EfiConfigureClpParameterBlock);
unsafe impl Sync for SyncClp {}

static DUMMY_CLP_BLOCK: SyncClp = SyncClp(EfiConfigureClpParameterBlock {
    clp_command: c"show".as_ptr().cast::<u8>(),
    clp_command_length: 5,
    clp_return_string: core::ptr::null_mut(),
    clp_return_string_length: 0,
    clp_cmd_status: 0,
    clp_error_value: 0,
    clp_msg_code: 0,
});

unsafe extern "efiapi" fn mock_query(
    _this: *const EfiPlatformToDriverConfigurationProtocol,
    _controller_handle: EfiHandle,
    _child_handle: EfiHandle,
    instance: *const UIntN,
    parameter_type_guid: *mut *const EfiGuid,
    parameter_block: *mut *mut c_void,
    parameter_block_size: *mut UIntN,
) -> EfiStatus {
    if instance.is_null()
        || parameter_type_guid.is_null()
        || parameter_block.is_null()
        || parameter_block_size.is_null()
    {
        return EfiStatus::INVALID_PARAMETER;
    }

    let inst = unsafe { *instance };
    if inst == 0 {
        unsafe {
            *parameter_type_guid =
                core::ptr::from_ref(&EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID);
            *parameter_block = core::ptr::from_ref(&DUMMY_CLP_BLOCK.0)
                .cast_mut()
                .cast::<c_void>();
            *parameter_block_size = core::mem::size_of::<EfiConfigureClpParameterBlock>() as UIntN;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_response(
    _this: *const EfiPlatformToDriverConfigurationProtocol,
    _controller_handle: EfiHandle,
    _child_handle: EfiHandle,
    instance: *const UIntN,
    parameter_type_guid: *const EfiGuid,
    _parameter_block: *const c_void,
    _parameter_block_size: UIntN,
    configuration_action: EfiPlatformConfigurationAction,
) -> EfiStatus {
    if instance.is_null() || parameter_type_guid.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        LAST_ACTION = Some(configuration_action);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_platform_to_driver_guid_and_constants() {
    assert_eq!(
        EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL_GUID,
        EfiGuid::new(
            0x642C_D590,
            0x8059,
            0x4C0A,
            [0xA9, 0x58, 0xC5, 0xEC, 0x07, 0xD2, 0x3C, 0x4B],
        )
    );
    assert_eq!(
        EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID,
        EfiGuid::new(
            0x345E_CC0E,
            0x0CB6,
            0x4B75,
            [0xBB, 0x57, 0x1B, 0x12, 0x9C, 0x47, 0x33, 0x3E],
        )
    );
    assert_eq!(EfiPlatformConfigurationAction::None as u32, 0);
    assert_eq!(EfiPlatformConfigurationAction::StopController as u32, 1);
    assert_eq!(EfiPlatformConfigurationAction::RestartController as u32, 2);
    assert_eq!(EfiPlatformConfigurationAction::RestartPlatform as u32, 3);
    assert_eq!(EfiPlatformConfigurationAction::NvramFailed as u32, 4);
    assert_eq!(EfiPlatformConfigurationAction::UnsupportedGuid as u32, 5);
}

#[test]
fn test_platform_to_driver_query_and_response() {
    let mock_proto = EfiPlatformToDriverConfigurationProtocol {
        query: mock_query,
        response: mock_response,
    };

    let p2d = PlatformToDriverConfiguration::new(&mock_proto);

    let ctrl_handle = EfiHandle::from_ptr(0x1000 as *mut c_void);

    // Instance 0 query returns valid CLP parameter block
    let (guid_ptr, block_ptr, block_size) =
        p2d.query(ctrl_handle, None, 0)
            .unwrap_or((core::ptr::null(), core::ptr::null_mut(), 0));

    assert_eq!(
        unsafe { *guid_ptr },
        EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID
    );
    assert_eq!(
        block_size,
        core::mem::size_of::<EfiConfigureClpParameterBlock>()
    );
    assert!(!block_ptr.is_null());

    // Instance 1 query returns NOT_FOUND
    assert_eq!(
        p2d.query(ctrl_handle, None, 1).err(),
        Some(EfiStatus::NOT_FOUND)
    );

    let block_slice = unsafe { core::slice::from_raw_parts(block_ptr.cast::<u8>(), block_size) };

    // Send response
    let resp_res = p2d.response(
        ctrl_handle,
        None,
        0,
        &EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID,
        block_slice,
        EfiPlatformConfigurationAction::None,
    );
    assert!(resp_res.is_ok());
    assert_eq!(
        unsafe { LAST_ACTION },
        Some(EfiPlatformConfigurationAction::None)
    );
}
