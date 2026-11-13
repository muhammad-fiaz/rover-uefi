//! # Variable Check Protocol and SMM Variable Check Protocol
//!
//! Provides services for runtime and Management Mode (SMM) variable write
//! validation, sanitization hooks, and variable property constraints
//! (attributes, minimum/maximum payload sizes, read-only status).

use crate::base::{EfiGuid, EfiStatus};

/// Variable Check Protocol GUID: `{af23b340-97b4-4685-8d4f-a3f28169b21d}`.
pub const VAR_CHECK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAF23_B340,
    0x97B4,
    0x4685,
    [0x8D, 0x4F, 0xA3, 0xF2, 0x81, 0x69, 0xB2, 0x1D],
);

/// SMM Variable Check Protocol GUID: `{b0d8f3c1-b7de-4c11-bc89-2fb562c8c411}`.
pub const SMM_VAR_CHECK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB0D8_F3C1,
    0xB7DE,
    0x4C11,
    [0xBC, 0x89, 0x2F, 0xB5, 0x62, 0xC8, 0xC4, 0x11],
);

/// Current revision of the `VariableProperty` structure.
pub const VAR_CHECK_VARIABLE_PROPERTY_REVISION: u16 = 0x0001;

/// Property flag: Variable is marked read-only.
pub const VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY: u16 = 1 << 0;

/// Structure describing variable property constraints.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VariableProperty {
    /// Revision of this structure (`VAR_CHECK_VARIABLE_PROPERTY_REVISION`).
    pub revision: u16,
    /// Property flags (e.g. `VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY`).
    pub property: u16,
    /// Required attributes mask for the variable.
    pub attributes: u32,
    /// Minimum allowed payload size in bytes.
    pub min_size: usize,
    /// Maximum allowed payload size in bytes.
    pub max_size: usize,
}

/// Function pointer for custom `SetVariable` check handlers.
pub type SetVariableCheckHandler = unsafe extern "efiapi" fn(
    variable_name: *const u16,
    vendor_guid: *const EfiGuid,
    attributes: u32,
    data_size: usize,
    data: *const core::ffi::c_void,
) -> EfiStatus;

/// Function pointer for registering a `SetVariable` check handler.
pub type VarCheckRegisterSetVariableCheckHandler =
    unsafe extern "efiapi" fn(handler: SetVariableCheckHandler) -> EfiStatus;

/// Function pointer for setting variable properties.
pub type VarCheckVariablePropertySet = unsafe extern "efiapi" fn(
    name: *const u16,
    guid: *const EfiGuid,
    variable_property: *const VariableProperty,
) -> EfiStatus;

/// Function pointer for querying variable properties.
pub type VarCheckVariablePropertyGet = unsafe extern "efiapi" fn(
    name: *const u16,
    guid: *const EfiGuid,
    variable_property: *mut VariableProperty,
) -> EfiStatus;

/// Low-level Variable Check Protocol interface.
#[repr(C)]
#[derive(Debug)]
pub struct EfiVarCheckProtocol {
    /// Registers a custom `SetVariable` check handler.
    pub register_set_variable_check_handler: VarCheckRegisterSetVariableCheckHandler,
    /// Sets variable properties for a named variable.
    pub variable_property_set: VarCheckVariablePropertySet,
    /// Retrieves variable properties for a named variable.
    pub variable_property_get: VarCheckVariablePropertyGet,
}

/// Low-level SMM Variable Check Protocol interface.
#[repr(C)]
#[derive(Debug)]
pub struct EfiSmmVarCheckProtocol {
    /// Registers a custom SMM `SetVariable` check handler.
    pub smm_register_set_variable_check_handler: VarCheckRegisterSetVariableCheckHandler,
    /// Sets variable properties for a named variable in SMM.
    pub smm_variable_property_set: VarCheckVariablePropertySet,
    /// Retrieves variable properties for a named variable in SMM.
    pub smm_variable_property_get: VarCheckVariablePropertyGet,
}

/// Safe client wrapper around the Variable Check Protocol.
#[derive(Debug, Clone, Copy)]
pub struct VarCheck {
    proto: *const EfiVarCheckProtocol,
}

impl VarCheck {
    /// Wraps a raw pointer to `EfiVarCheckProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiVarCheckProtocol` table.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiVarCheckProtocol) -> Self {
        Self { proto }
    }

    /// Registers a `SetVariable` check handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handler` is invalid.
    /// Returns `EfiStatus::ACCESS_DENIED` if boot phase transition has occurred.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register_check_handler(
        &self,
        handler: SetVariableCheckHandler,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).register_set_variable_check_handler)(handler) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Sets property constraints for a variable.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `name` is empty or not null-terminated.
    /// Returns `EfiStatus::ACCESS_DENIED` if boot phase transition has occurred.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if registration fails.
    pub fn set_property(
        &self,
        name: &[u16],
        guid: &EfiGuid,
        property: &VariableProperty,
    ) -> Result<(), EfiStatus> {
        if name.is_empty() || name[name.len() - 1] != 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let status =
            unsafe { ((*self.proto).variable_property_set)(name.as_ptr(), guid, property) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves property constraints for a variable.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `name` is empty or not null-terminated.
    /// Returns `EfiStatus::NOT_FOUND` if the variable property is not registered.
    pub fn get_property(
        &self,
        name: &[u16],
        guid: &EfiGuid,
    ) -> Result<VariableProperty, EfiStatus> {
        if name.is_empty() || name[name.len() - 1] != 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let mut prop = VariableProperty::default();
        let status =
            unsafe { ((*self.proto).variable_property_get)(name.as_ptr(), guid, &raw mut prop) };
        if status == EfiStatus::SUCCESS {
            Ok(prop)
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around the SMM Variable Check Protocol.
#[derive(Debug, Clone, Copy)]
pub struct SmmVarCheck {
    proto: *const EfiSmmVarCheckProtocol,
}

impl SmmVarCheck {
    /// Wraps a raw pointer to `EfiSmmVarCheckProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiSmmVarCheckProtocol` table.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiSmmVarCheckProtocol) -> Self {
        Self { proto }
    }

    /// Registers an SMM `SetVariable` check handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handler` is invalid.
    /// Returns `EfiStatus::ACCESS_DENIED` if ready to boot has signaled.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register_check_handler(
        &self,
        handler: SetVariableCheckHandler,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).smm_register_set_variable_check_handler)(handler) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Sets property constraints for a variable in SMM.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `name` is empty or not null-terminated.
    /// Returns `EfiStatus::ACCESS_DENIED` if boot phase transition has occurred.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if registration fails.
    pub fn set_property(
        &self,
        name: &[u16],
        guid: &EfiGuid,
        property: &VariableProperty,
    ) -> Result<(), EfiStatus> {
        if name.is_empty() || name[name.len() - 1] != 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let status =
            unsafe { ((*self.proto).smm_variable_property_set)(name.as_ptr(), guid, property) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves property constraints for a variable in SMM.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `name` is empty or not null-terminated.
    /// Returns `EfiStatus::NOT_FOUND` if the variable property is not registered.
    pub fn get_property(
        &self,
        name: &[u16],
        guid: &EfiGuid,
    ) -> Result<VariableProperty, EfiStatus> {
        if name.is_empty() || name[name.len() - 1] != 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let mut prop = VariableProperty::default();
        let status = unsafe {
            ((*self.proto).smm_variable_property_get)(name.as_ptr(), guid, &raw mut prop)
        };
        if status == EfiStatus::SUCCESS {
            Ok(prop)
        } else {
            Err(status)
        }
    }
}
