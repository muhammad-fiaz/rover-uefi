//! # SMM / MM Control2 Protocol
//!
//! Initiates synchronous SMI/MMI activations and acknowledges/clears them.

use crate::base::{EfiGuid, EfiStatus};

/// SMM/MM Control2 Protocol GUID: `{843dc720-ab1e-42cb-9357-8a0078f3561b}`.
pub const SMM_CONTROL2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x843d_c720,
    0xab1e,
    0x42cb,
    [0x93, 0x57, 0x8a, 0x00, 0x78, 0xf3, 0x56, 0x1b],
);

/// Function pointer for `EFI_MM_ACTIVATE`.
pub type EfiSmmActivate2 = unsafe extern "efiapi" fn(
    this: *const EfiSmmControl2Protocol,
    command_port: *mut u8,
    data_port: *mut u8,
    periodic: bool,
    activation_interval: usize,
) -> EfiStatus;

/// Function pointer for `EFI_MM_DEACTIVATE`.
pub type EfiSmmDeactivate2 =
    unsafe extern "efiapi" fn(this: *const EfiSmmControl2Protocol, periodic: bool) -> EfiStatus;

/// `EFI_SMM_CONTROL2_PROTOCOL` interface table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSmmControl2Protocol {
    /// Generates an SMI/MMI activation.
    pub trigger: EfiSmmActivate2,
    /// Clears any system state created in response to trigger.
    pub clear: EfiSmmDeactivate2,
    /// Minimum interval at which platform can set periodic activation.
    pub minimum_trigger_period: usize,
}

/// Safe client wrapper around the SMM Control2 Protocol.
#[derive(Debug, Clone, Copy)]
pub struct SmmControl2 {
    proto: *const EfiSmmControl2Protocol,
}

impl SmmControl2 {
    /// Wraps an SMM Control2 protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiSmmControl2Protocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiSmmControl2Protocol) -> Self {
        Self { proto }
    }

    /// Triggers a synchronous SMI/MMI activation.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if timing is unsupported.
    /// Returns `EfiStatus::INVALID_PARAMETER` if activation period is unsupported.
    /// Returns `EfiStatus::NOT_STARTED` if MM base services have not been initialized.
    pub fn trigger(
        &self,
        command_port: Option<&mut u8>,
        data_port: Option<&mut u8>,
        periodic: bool,
        interval: usize,
    ) -> Result<(), EfiStatus> {
        let cmd_ptr = match command_port {
            Some(p) => core::ptr::from_mut(p),
            None => core::ptr::null_mut(),
        };
        let data_ptr = match data_port {
            Some(p) => core::ptr::from_mut(p),
            None => core::ptr::null_mut(),
        };

        let status = unsafe {
            let func = (*self.proto).trigger;
            func(self.proto, cmd_ptr, data_ptr, periodic, interval)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Clears and deasserts an SMI/MMI activation.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if the source cannot be cleared.
    pub fn clear(&self, periodic: bool) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).clear;
            func(self.proto, periodic)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the minimum trigger period in microseconds.
    #[must_use]
    pub fn minimum_trigger_period(&self) -> usize {
        unsafe { (*self.proto).minimum_trigger_period }
    }
}
