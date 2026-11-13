//! # Platform To Driver Configuration Protocol
//!
//! Defined in the UEFI specification, this protocol is optionally produced by the
//! platform firmware and consumed by UEFI drivers during `Start()` to receive
//! platform-specific configuration parameters and policies.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// The `EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL_GUID`.
pub const EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x642C_D590,
    0x8059,
    0x4C0A,
    [0xA9, 0x58, 0xC5, 0xEC, 0x07, 0xD2, 0x3C, 0x4B],
);

/// GUID for DMTF SM CLP configuration parameters passed via Platform To Driver Configuration.
pub const EFI_PLATFORM_TO_DRIVER_CONFIGURATION_CLP_GUID: EfiGuid = EfiGuid::new(
    0x345E_CC0E,
    0x0CB6,
    0x4B75,
    [0xBB, 0x57, 0x1B, 0x12, 0x9C, 0x47, 0x33, 0x3E],
);

/// Actions that a UEFI driver instructs the platform to take after applying a configuration block.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EfiPlatformConfigurationAction {
    /// Controller is usable with updated settings; no special action required.
    None = 0,
    /// Controller is not in a usable state and must be disconnected/stopped immediately.
    StopController = 1,
    /// Controller must be stopped and restarted before it can be used again.
    RestartController = 2,
    /// Platform requires a system reboot before settings take effect.
    RestartPlatform = 3,
    /// Settings were applied, but writing to non-volatile storage failed.
    NvramFailed = 4,
    /// Driver did not recognize the `ParameterTypeGuid`; platform should query next format.
    UnsupportedGuid = 5,
}

/// Parameter block structure for DMTF SM CLP configuration.
#[repr(C)]
#[derive(Debug)]
pub struct EfiConfigureClpParameterBlock {
    /// Pointer to null-terminated UTF-8 CLP command string.
    pub clp_command: *const u8,
    /// Length of the CLP command in bytes.
    pub clp_command_length: u32,
    /// Pointer to buffer where driver returns null-terminated response string.
    pub clp_return_string: *mut u8,
    /// Length of return response string buffer.
    pub clp_return_string_length: u32,
    /// CLP command execution status code.
    pub clp_cmd_status: u8,
    /// CLP processing error value.
    pub clp_error_value: u8,
    /// CLP message code (bit 15: OEM flag; bits 14-0: message code).
    pub clp_msg_code: u16,
}

/// Function pointer for `EFI_PLATFORM_TO_DRIVER_CONFIGURATION_QUERY`.
pub type EfiPlatformToDriverConfigurationQuery = unsafe extern "efiapi" fn(
    this: *const EfiPlatformToDriverConfigurationProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    instance: *const UIntN,
    parameter_type_guid: *mut *const EfiGuid,
    parameter_block: *mut *mut c_void,
    parameter_block_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_PLATFORM_TO_DRIVER_CONFIGURATION_RESPONSE`.
pub type EfiPlatformToDriverConfigurationResponse = unsafe extern "efiapi" fn(
    this: *const EfiPlatformToDriverConfigurationProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    instance: *const UIntN,
    parameter_type_guid: *const EfiGuid,
    parameter_block: *const c_void,
    parameter_block_size: UIntN,
    configuration_action: EfiPlatformConfigurationAction,
) -> EfiStatus;

/// Raw ABI protocol table for Platform To Driver Configuration.
#[repr(C)]
#[derive(Debug)]
pub struct EfiPlatformToDriverConfigurationProtocol {
    /// Queries the platform for configuration parameter blocks.
    pub query: EfiPlatformToDriverConfigurationQuery,
    /// Notifies the platform of the driver's response and actions taken.
    pub response: EfiPlatformToDriverConfigurationResponse,
}

/// Safe wrapper around [`EfiPlatformToDriverConfigurationProtocol`].
pub struct PlatformToDriverConfiguration<'a> {
    proto: &'a EfiPlatformToDriverConfigurationProtocol,
}

impl<'a> PlatformToDriverConfiguration<'a> {
    /// Wraps a reference to [`EfiPlatformToDriverConfigurationProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiPlatformToDriverConfigurationProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiPlatformToDriverConfigurationProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to a valid and initialized protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiPlatformToDriverConfigurationProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Queries the platform for configuration parameters for a controller or child controller.
    ///
    /// Returns a tuple containing:
    /// - Pointer to the parameter type GUID identifying the parameter block format.
    /// - Pointer to the raw parameter block allocated by the platform.
    /// - Size in bytes of the parameter block.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` when all instances have been queried, or an error status
    /// if the query failed.
    pub fn query(
        &self,
        controller_handle: EfiHandle,
        child_handle: Option<EfiHandle>,
        instance: usize,
    ) -> Result<(*const EfiGuid, *mut c_void, usize), EfiStatus> {
        let child = child_handle.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let inst_val = instance as UIntN;
        let mut type_guid: *const EfiGuid = core::ptr::null();
        let mut block_ptr: *mut c_void = core::ptr::null_mut();
        let mut block_size: UIntN = 0;

        let status = unsafe {
            (self.proto.query)(
                self.proto,
                controller_handle,
                child,
                core::ptr::from_ref(&inst_val),
                core::ptr::from_mut(&mut type_guid),
                core::ptr::from_mut(&mut block_ptr),
                core::ptr::from_mut(&mut block_size),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((type_guid, block_ptr, block_size as usize))
        } else {
            Err(status)
        }
    }

    /// Informs the platform how the configuration parameter block was used and what action is required.
    ///
    /// # Errors
    ///
    /// Returns an error status if the response submission fails.
    pub fn response(
        &self,
        controller_handle: EfiHandle,
        child_handle: Option<EfiHandle>,
        instance: usize,
        parameter_type_guid: &EfiGuid,
        parameter_block: &[u8],
        action: EfiPlatformConfigurationAction,
    ) -> Result<(), EfiStatus> {
        let child = child_handle.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let inst_val = instance as UIntN;

        let status = unsafe {
            (self.proto.response)(
                self.proto,
                controller_handle,
                child,
                core::ptr::from_ref(&inst_val),
                parameter_type_guid,
                parameter_block.as_ptr().cast::<c_void>(),
                parameter_block.len() as UIntN,
                action,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
