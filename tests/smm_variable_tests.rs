//! Management Mode (SMM) variable and firmware volume block integration tests.

use rover_uefi::abi::types::{Char16, UIntN};
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::smm_variable::{
    EFI_SMM_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID, EFI_SMM_VARIABLE_PROTOCOL_GUID,
    EfiSmmVariableProtocol, SmmVariable,
};
use rover_uefi::tables::runtime::EfiQueryVariableInfo;

#[test]
fn test_smm_variable_guids() {
    let expected_var = EfiGuid::new(
        0xED32_D533,
        0x99E6,
        0x4209,
        [0x9C, 0xC0, 0x2D, 0x72, 0xCD, 0xD9, 0x98, 0xA7],
    );
    assert_eq!(EFI_SMM_VARIABLE_PROTOCOL_GUID, expected_var);

    let expected_fvb = EfiGuid::new(
        0xD326_D041,
        0xBD31,
        0x4C01,
        [0xB5, 0xA8, 0x62, 0x8B, 0xE8, 0x7F, 0x06, 0x53],
    );
    assert_eq!(EFI_SMM_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID, expected_fvb);
}

unsafe extern "efiapi" fn mock_smm_get_variable(
    _name: *mut Char16,
    _vendor_guid: *mut EfiGuid,
    attributes: *mut u32,
    data_size: *mut UIntN,
    _data: *mut core::ffi::c_void,
) -> EfiStatus {
    if !attributes.is_null() {
        unsafe { *attributes = 7 };
    }
    if !data_size.is_null() {
        unsafe { *data_size = 4 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_smm_set_variable(
    _name: *mut Char16,
    _vendor_guid: *mut EfiGuid,
    _attributes: u32,
    _data_size: UIntN,
    _data: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_smm_get_next_name(
    _name_size: *mut UIntN,
    _name: *mut Char16,
    _vendor_guid: *mut EfiGuid,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

unsafe extern "efiapi" fn mock_smm_query_info(
    _attributes: u32,
    _max_storage_size: *mut u64,
    _remaining_size: *mut u64,
    _max_var_size: *mut u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_smm_variable_wrapper() {
    let mock_proto = EfiSmmVariableProtocol {
        smm_get_variable: mock_smm_get_variable,
        smm_get_next_variable_name: mock_smm_get_next_name,
        smm_set_variable: mock_smm_set_variable,
        smm_query_variable_info: mock_smm_query_info as EfiQueryVariableInfo,
    };

    let Some(smm_var) = (unsafe { SmmVariable::from_raw(&raw const mock_proto) }) else {
        panic!("failed to wrap smm variable protocol");
    };

    let mut name = [0u16; 8];
    let mut guid = EfiGuid::ZERO;
    let mut attr: u32 = 0;
    let mut size: UIntN = 0;
    let mut data = [0u8; 8];

    let status = unsafe {
        smm_var.get_variable(
            name.as_mut_ptr(),
            &raw mut guid,
            &raw mut attr,
            &raw mut size,
            data.as_mut_ptr().cast(),
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(attr, 7);
    assert_eq!(size, 4);

    let set_status = unsafe {
        smm_var.set_variable(
            name.as_mut_ptr(),
            &raw mut guid,
            attr,
            size,
            data.as_mut_ptr().cast(),
        )
    };
    assert_eq!(set_status, EfiStatus::SUCCESS);
}
