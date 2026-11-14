//! # Management Mode (SMM) Variable and Firmware Volume Block Protocols
//!
//! Provides protocol definitions for variable storage services and block-oriented
//! firmware volume access in the SMM runtime environment. Also defines structures
//! and function identifiers for communicating with the SMM variable driver across
//! the SMI boundary via the Management Mode communication buffer.

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

/// SMM Variable Write Protocol GUID: `{93ba1826-dffb-45dd-82a7-e7dcaa3bbdf3}`.
pub const EFI_SMM_VARIABLE_WRITE_GUID: EfiGuid = EfiGuid::new(
    0x93BA_1826,
    0xDFFB,
    0x45DD,
    [0x82, 0xA7, 0xE7, 0xDC, 0xAA, 0x3B, 0xBD, 0xF3],
);

/// SMM Firmware Volume Block Protocol GUID: `{d326d041-bd31-4c01-b5a8-628be87f0653}`.
pub const EFI_SMM_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD326_D041,
    0xBD31,
    0x4C01,
    [0xB5, 0xA8, 0x62, 0x8B, 0xE8, 0x7F, 0x06, 0x53],
);

// --- SMM Variable Communication Function Identifiers ---

/// Function 1: Get variable value across SMM boundary.
pub const SMM_VARIABLE_FUNCTION_GET_VARIABLE: usize = 1;
/// Function 2: Get next variable name across SMM boundary.
pub const SMM_VARIABLE_FUNCTION_GET_NEXT_VARIABLE_NAME: usize = 2;
/// Function 3: Set variable value across SMM boundary.
pub const SMM_VARIABLE_FUNCTION_SET_VARIABLE: usize = 3;
/// Function 4: Query variable storage capacity across SMM boundary.
pub const SMM_VARIABLE_FUNCTION_QUERY_VARIABLE_INFO: usize = 4;
/// Function 5: Notify Ready To Boot event to SMM variable driver.
pub const SMM_VARIABLE_FUNCTION_READY_TO_BOOT: usize = 5;
/// Function 6: Notify Exit Boot Services event to SMM variable driver.
pub const SMM_VARIABLE_FUNCTION_EXIT_BOOT_SERVICE: usize = 6;
/// Function 7: Retrieve variable storage statistics.
pub const SMM_VARIABLE_FUNCTION_GET_STATISTICS: usize = 7;
/// Function 8: Lock variable against modifications until platform reset.
pub const SMM_VARIABLE_FUNCTION_LOCK_VARIABLE: usize = 8;
/// Function 9: Set variable check property policy.
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_SET: usize = 9;
/// Function 10: Get variable check property policy.
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_GET: usize = 10;
/// Function 11: Query maximum supported payload size.
pub const SMM_VARIABLE_FUNCTION_GET_PAYLOAD_SIZE: usize = 11;
/// Function 12: Initialize runtime variable cache context.
pub const SMM_VARIABLE_FUNCTION_INIT_RUNTIME_VARIABLE_CACHE_CONTEXT: usize = 12;
/// Function 13: Synchronize runtime variable cache.
pub const SMM_VARIABLE_FUNCTION_SYNC_RUNTIME_CACHE: usize = 13;
/// Function 14: Query runtime variable cache information.
pub const SMM_VARIABLE_FUNCTION_GET_RUNTIME_CACHE_INFO: usize = 14;

/// Header for communication messages directed to the SMM variable driver.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmVariableCommunicateHeader {
    /// Function code identifying operation (`SMM_VARIABLE_FUNCTION_*`).
    pub function: usize,
    /// Status code returned by the SMM handler.
    pub return_status: EfiStatus,
}

/// Payload structure for `SMM_VARIABLE_FUNCTION_GET_VARIABLE` and `SET_VARIABLE`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmVariableCommunicateAccessVariable {
    /// Variable namespace GUID.
    pub guid: EfiGuid,
    /// Size of data payload in bytes.
    pub data_size: usize,
    /// Size of null-terminated UTF-16 variable name in bytes.
    pub name_size: usize,
    /// Variable attributes bitmask.
    pub attributes: u32,
}

/// Payload structure for `SMM_VARIABLE_FUNCTION_GET_NEXT_VARIABLE_NAME`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmVariableCommunicateGetNextVariableName {
    /// Variable namespace GUID.
    pub guid: EfiGuid,
    /// Size in bytes of name buffer.
    pub name_size: usize,
}

/// Alias for variable lock payload.
pub type SmmVariableCommunicateLockVariable = SmmVariableCommunicateGetNextVariableName;

/// Payload structure for `SMM_VARIABLE_FUNCTION_QUERY_VARIABLE_INFO`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmmVariableCommunicateQueryVariableInfo {
    /// Maximum variable storage capacity in bytes.
    pub maximum_variable_storage_size: u64,
    /// Remaining variable storage in bytes.
    pub remaining_variable_storage_size: u64,
    /// Maximum individual variable size in bytes.
    pub maximum_variable_size: u64,
    /// Target variable attributes bitmask.
    pub attributes: u32,
}

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
