//! # Driver Health Protocol
//!
//! Defined by the UEFI Specification Section 11.8: Driver Health Protocol.
//!
//! Enables drivers to report their operational and configuration health to the
//! boot manager or firmware setup utilities, and request repairs or reconfiguration.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// The `EFI_DRIVER_HEALTH_PROTOCOL_GUID`.
pub const EFI_DRIVER_HEALTH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2A52_4C80,
    0x530E,
    0x4EDD,
    [0x93, 0x8C, 0xC1, 0x06, 0x46, 0x6E, 0xD8, 0x40],
);

/// Driver health status returned by `GetHealthStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiDriverHealthStatus {
    /// Controller and driver are completely healthy.
    Healthy = 0,
    /// Controller requires repair before full operation.
    RepairRequired = 1,
    /// Controller requires user or administrative configuration.
    ConfigurationRequired = 2,
    /// Controller has failed and cannot be repaired.
    Failed = 3,
    /// Controller requires reconnection.
    ReconnectRequired = 4,
}

/// HII string or message descriptor associated with health notification.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiDriverHealthHiiMessage {
    /// HII package list handle.
    pub hii_handle: EfiHandle,
    /// String identifier in the HII database.
    pub string_id: u16,
    /// Message visual attributes.
    pub message_code: u64,
}

/// Notification callback invoked during interactive or background repair.
pub type EfiDriverHealthNotify = unsafe extern "efiapi" fn(
    repaired_percent: UIntN,
    type_message: *const EfiDriverHealthHiiMessage,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_HEALTH_GET_HEALTH_STATUS`.
pub type EfiDriverHealthGetHealthStatus = unsafe extern "efiapi" fn(
    this: *const EfiDriverHealthProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    health_status: *mut EfiDriverHealthStatus,
    message_list: *mut *mut EfiDriverHealthHiiMessage,
    form_list: *mut *mut EfiDriverHealthHiiMessage,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_HEALTH_REPAIR`.
pub type EfiDriverHealthRepair = unsafe extern "efiapi" fn(
    this: *const EfiDriverHealthProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    progress_notify: EfiDriverHealthNotify,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DRIVER_HEALTH_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverHealthProtocol {
    /// Retrieves current operational and configuration health.
    pub get_health_status: EfiDriverHealthGetHealthStatus,
    /// Requests automated or assisted repair of the controller.
    pub repair: EfiDriverHealthRepair,
}
