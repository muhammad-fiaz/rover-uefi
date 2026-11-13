//! Integration tests for Variable Check and SMM Variable Check protocols.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::security::var_check::{
    EfiSmmVarCheckProtocol, EfiVarCheckProtocol, SMM_VAR_CHECK_PROTOCOL_GUID,
    SetVariableCheckHandler, SmmVarCheck, VAR_CHECK_PROTOCOL_GUID,
    VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY, VAR_CHECK_VARIABLE_PROPERTY_REVISION, VarCheck,
    VariableProperty,
};

static mut MOCK_PROP: VariableProperty = VariableProperty {
    revision: VAR_CHECK_VARIABLE_PROPERTY_REVISION,
    property: VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY,
    attributes: 0x07,
    min_size: 16,
    max_size: 1024,
};

static mut HANDLER_REGISTERED: bool = false;

unsafe extern "efiapi" fn mock_check_handler(
    _variable_name: *const u16,
    _vendor_guid: *const EfiGuid,
    _attributes: u32,
    _data_size: usize,
    _data: *const core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_handler(handler: SetVariableCheckHandler) -> EfiStatus {
    if handler as usize == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        HANDLER_REGISTERED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_prop(
    _name: *const u16,
    _guid: *const EfiGuid,
    prop: *const VariableProperty,
) -> EfiStatus {
    if prop.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        MOCK_PROP = *prop;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_prop(
    _name: *const u16,
    _guid: *const EfiGuid,
    prop: *mut VariableProperty,
) -> EfiStatus {
    if prop.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *prop = MOCK_PROP;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_var_check_constants_and_guids() {
    assert_eq!(
        VAR_CHECK_PROTOCOL_GUID,
        EfiGuid::new(
            0xAF23_B340,
            0x97B4,
            0x4685,
            [0x8D, 0x4F, 0xA3, 0xF2, 0x81, 0x69, 0xB2, 0x1D],
        )
    );
    assert_eq!(
        SMM_VAR_CHECK_PROTOCOL_GUID,
        EfiGuid::new(
            0xB0D8_F3C1,
            0xB7DE,
            0x4C11,
            [0xBC, 0x89, 0x2F, 0xB5, 0x62, 0xC8, 0xC4, 0x11],
        )
    );
    assert_eq!(VAR_CHECK_VARIABLE_PROPERTY_REVISION, 1);
    assert_eq!(VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY, 1);
}

#[test]
#[allow(clippy::cast_lossless)]
fn test_var_check_wrapper() {
    let mock_proto = EfiVarCheckProtocol {
        register_set_variable_check_handler: mock_register_handler,
        variable_property_set: mock_set_prop,
        variable_property_get: mock_get_prop,
    };

    let var_check = unsafe { VarCheck::from_raw(&raw const mock_proto) };
    assert!(var_check.register_check_handler(mock_check_handler).is_ok());
    assert!(unsafe { HANDLER_REGISTERED });

    let var_name = [b'T' as u16, b'e' as u16, b's' as u16, b't' as u16, 0];
    let guid = EfiGuid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);

    let new_prop = VariableProperty {
        revision: VAR_CHECK_VARIABLE_PROPERTY_REVISION,
        property: VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY,
        attributes: 0x03,
        min_size: 32,
        max_size: 512,
    };

    assert!(var_check.set_property(&var_name, &guid, &new_prop).is_ok());

    let fetched = var_check.get_property(&var_name, &guid).unwrap_or_default();
    assert_eq!(fetched, new_prop);
}

#[test]
#[allow(clippy::cast_lossless)]
fn test_smm_var_check_wrapper() {
    let mock_proto = EfiSmmVarCheckProtocol {
        smm_register_set_variable_check_handler: mock_register_handler,
        smm_variable_property_set: mock_set_prop,
        smm_variable_property_get: mock_get_prop,
    };

    let smm_var_check = unsafe { SmmVarCheck::from_raw(&raw const mock_proto) };
    let var_name = [b'S' as u16, b'M' as u16, b'M' as u16, 0];
    let guid = EfiGuid::new(0xAA, 0xBB, 0xCC, [1, 2, 3, 4, 5, 6, 7, 8]);

    let prop = VariableProperty {
        revision: VAR_CHECK_VARIABLE_PROPERTY_REVISION,
        property: 0,
        attributes: 0x07,
        min_size: 4,
        max_size: 64,
    };

    assert!(smm_var_check.set_property(&var_name, &guid, &prop).is_ok());
    let fetched = smm_var_check
        .get_property(&var_name, &guid)
        .unwrap_or_default();
    assert_eq!(fetched, prop);
}
