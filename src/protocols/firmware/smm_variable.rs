//! # Management Mode (SMM) Variable and Firmware Volume Block Protocols
//!
//! Provides protocol definitions for variable storage services and block-oriented
//! firmware volume access in the SMM runtime environment.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::firmvol::EfiFirmwareVolumeBlock2Protocol;
use crate::tables::runtime::{
    EfiGetNextVariableName, EfiGetVariable, EfiQueryVariableInfo, EfiSetVariable,
};
use core::ffi::c_void;

/// SMM Variable Protocol GUID: `{ed32d533-99e6-4209-9cc0-2d72cdd998a7}`.
pub const EFI_SMM_VARIABLE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xED32_D533,
    0x99E6,
    0x4209,
    [0x9C, 0xC0, 0x2D, 0x72, 0xCD, 0xD9, 0x98, 0xA7],
);

/// SMM Firmware Volume Block Protocol GUID: `{d326d041-bd31-4c01-b5a8-628be87f0653}`.
pub const EFI_SMM_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD326_D041,
    0xBD31,
    0x4C01,
    [0xB5, 0xA8, 0x62, 0x8B, 0xE8, 0x7F, 0x06, 0x53],
);

/// Low-level SMM Variable Protocol table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSmmVariableProtocol {
    /// Retrieves a variable's value in SMM.
    pub smm_get_variable: EfiGetVariable,
    /// Enumerates the current variable names in SMM.
    pub smm_get_next_variable_name: EfiGetNextVariableName,
    /// Sets the value of a variable in SMM.
    pub smm_set_variable: EfiSetVariable,
    /// Returns information about the EFI variables in SMM.
    pub smm_query_variable_info: EfiQueryVariableInfo,
}

/// SMM Firmware Volume Block Protocol alias.
///
/// In the SMM environment, this protocol provides the same layout and functionality
/// as [`EfiFirmwareVolumeBlock2Protocol`], accessible after `ExitPmAuth`.
pub type EfiSmmFirmwareVolumeBlockProtocol = EfiFirmwareVolumeBlock2Protocol;

/// Safe client wrapper around [`EfiSmmVariableProtocol`].
#[derive(Clone, Copy, Debug)]
pub struct SmmVariable {
    proto: *const EfiSmmVariableProtocol,
}

impl SmmVariable {
    /// Wraps a raw pointer to [`EfiSmmVariableProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiSmmVariableProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self { proto })
        }
    }

    /// Retrieves a variable's value in SMM.
    ///
    /// # Safety
    ///
    /// Pointers must be valid for the duration of the call.
    pub unsafe fn get_variable(
        &self,
        name: *mut Char16,
        vendor_guid: *mut EfiGuid,
        attributes: *mut u32,
        data_size: *mut UIntN,
        data: *mut c_void,
    ) -> EfiStatus {
        unsafe { ((*self.proto).smm_get_variable)(name, vendor_guid, attributes, data_size, data) }
    }

    /// Sets the value of a variable in SMM.
    ///
    /// # Safety
    ///
    /// Pointers must be valid for the duration of the call.
    pub unsafe fn set_variable(
        &self,
        name: *mut Char16,
        vendor_guid: *mut EfiGuid,
        attributes: u32,
        data_size: UIntN,
        data: *mut c_void,
    ) -> EfiStatus {
        unsafe { ((*self.proto).smm_set_variable)(name, vendor_guid, attributes, data_size, data) }
    }
}
