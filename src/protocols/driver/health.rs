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
    0x2A53_4210,
    0x9280,
    0x41D8,
    [0xAE, 0x79, 0xCA, 0xDA, 0x01, 0xA2, 0xB1, 0x27],
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
    /// Controller requires system reboot.
    RebootRequired = 5,
}

/// HII string or message descriptor associated with health notification.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiDriverHealthHiiMessage {
    /// HII package list handle.
    pub hii_handle: EfiHandle,
    /// String identifier in the HII database.
    pub string_id: u16,
    /// Message visual attributes or error/warning code.
    pub message_code: u64,
}

/// Notification callback invoked during repair operation to report progress.
pub type EfiDriverHealthRepairNotify =
    unsafe extern "efiapi" fn(value: UIntN, limit: UIntN) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_HEALTH_GET_HEALTH_STATUS`.
pub type EfiDriverHealthGetHealthStatus = unsafe extern "efiapi" fn(
    this: *const EfiDriverHealthProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    health_status: *mut EfiDriverHealthStatus,
    message_list: *mut *mut EfiDriverHealthHiiMessage,
    form_hii_handle: *mut EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_HEALTH_REPAIR`.
pub type EfiDriverHealthRepair = unsafe extern "efiapi" fn(
    this: *const EfiDriverHealthProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    repair_notify: Option<EfiDriverHealthRepairNotify>,
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

/// Safe client wrapper around `EfiDriverHealthProtocol`.
pub struct DriverHealth<'a> {
    proto: &'a EfiDriverHealthProtocol,
}

impl<'a> DriverHealth<'a> {
    /// Creates a new `DriverHealth` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiDriverHealthProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverHealthProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiDriverHealthProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverHealthProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *const EfiDriverHealthProtocol {
        core::ptr::from_ref(self.proto)
    }

    /// Retrieves the health status of a controller in the platform.
    ///
    /// Returns `(health_status, message_list_ptr, optional_form_hii_handle)`.
    ///
    /// # Errors
    ///
    /// Returns an error status if querying health status fails.
    pub fn get_health_status(
        &self,
        controller: Option<EfiHandle>,
        child: Option<EfiHandle>,
    ) -> Result<
        (
            EfiDriverHealthStatus,
            *mut EfiDriverHealthHiiMessage,
            Option<EfiHandle>,
        ),
        EfiStatus,
    > {
        let controller_handle =
            controller.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut health_status = EfiDriverHealthStatus::Healthy;
        let mut message_list: *mut EfiDriverHealthHiiMessage = core::ptr::null_mut();
        let mut form_hii = EfiHandle::from_ptr(core::ptr::null_mut());

        let status = unsafe {
            (self.proto.get_health_status)(
                self.proto,
                controller_handle,
                child_handle,
                core::ptr::from_mut(&mut health_status),
                core::ptr::from_mut(&mut message_list),
                core::ptr::from_mut(&mut form_hii),
            )
        };

        if status == EfiStatus::SUCCESS {
            let form_opt = if form_hii.is_null() {
                None
            } else {
                Some(form_hii)
            };
            Ok((health_status, message_list, form_opt))
        } else {
            Err(status)
        }
    }

    /// Performs a repair operation on a controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if repair fails.
    pub fn repair(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        repair_notify: Option<EfiDriverHealthRepairNotify>,
    ) -> Result<(), EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let status =
            unsafe { (self.proto.repair)(self.proto, controller, child_handle, repair_notify) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
