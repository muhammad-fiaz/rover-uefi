//! # TCG2 protocol
//!
//! The firmware interface for platform measurement.
//!
//! The protocol has exactly seven entries in the applicable
//! specification revision: capability, event log, hash log
//! extend event, submit command, active PCR banks get and set,
//! and result of set active PCR banks. Physical addresses in
//! this interface are opaque firmware identifiers; safe code in
//! this crate never dereferences them. Callers supply mappings
//! through the firmware protocol contract.

use crate::abi::types::Bool;
use crate::base::{EfiPhysicalAddress, EfiStatus};

use super::capability::EfiTcg2BootServiceCapability;
use super::event::EfiTcg2Event;

/// The TCG2 protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTcg2Protocol {
    /// Get capability.
    pub get_capability: unsafe extern "efiapi" fn(
        *mut EfiTcg2Protocol,
        *mut EfiTcg2BootServiceCapability,
    ) -> EfiStatus,
    /// Get event log.
    pub get_event_log: unsafe extern "efiapi" fn(
        *mut EfiTcg2Protocol,
        u32,
        *mut EfiPhysicalAddress,
        *mut EfiPhysicalAddress,
        *mut Bool,
    ) -> EfiStatus,
    /// Hash and optionally extend an event.
    pub hash_log_extend_event: unsafe extern "efiapi" fn(
        *mut EfiTcg2Protocol,
        u64,
        EfiPhysicalAddress,
        u64,
        *mut EfiTcg2Event,
    ) -> EfiStatus,
    /// Submit a command.
    pub submit_command:
        unsafe extern "efiapi" fn(*mut EfiTcg2Protocol, u32, *mut u8, u32, *mut u8) -> EfiStatus,
    /// Get active PCR banks.
    pub get_active_pcr_banks:
        unsafe extern "efiapi" fn(*mut EfiTcg2Protocol, *mut u32) -> EfiStatus,
    /// Set active PCR banks.
    pub set_active_pcr_banks: unsafe extern "efiapi" fn(*mut EfiTcg2Protocol, u32) -> EfiStatus,
    /// Get result of set active PCR banks.
    pub get_result_of_set_active_pcr_banks:
        unsafe extern "efiapi" fn(*mut EfiTcg2Protocol, *mut u32, *mut u32) -> EfiStatus,
}

/// A borrowed TCG2 view.
pub struct Tcg2Protocol<'a> {
    protocol: &'a EfiTcg2Protocol,
}

impl<'a> Tcg2Protocol<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiTcg2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiTcg2Protocol {
        self.protocol
    }

    /// Gets capability information into caller-provided storage.
    ///
    /// The caller sets the size field of `capability` to the
    /// allocated size. Firmware fills the fields that fit.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_capability(
        &self,
        capability: &mut EfiTcg2BootServiceCapability,
    ) -> Result<(), EfiStatus> {
        // SAFETY: The stack caller buffer is valid for the call.
        let status = unsafe {
            (self.protocol.get_capability)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                capability,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Gets the active PCR bank bitmap.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_active_pcr_banks(&self) -> Result<u32, EfiStatus> {
        let mut banks: u32 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_active_pcr_banks)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut banks,
            )
        };
        if status.is_success() {
            Ok(banks)
        } else {
            Err(status)
        }
    }

    /// Sets the active PCR bank bitmap.
    ///
    /// At least one bit must be set.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn set_active_pcr_banks(&self, banks: u32) -> Result<(), EfiStatus> {
        if banks == 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The borrowed protocol is valid for the call.
        let status = unsafe {
            (self.protocol.set_active_pcr_banks)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                banks,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Gets the result of the last set active PCR banks call.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_result_of_set_active_pcr_banks(&self) -> Result<(u32, u32), EfiStatus> {
        let mut operation_present: u32 = 0;
        let mut response: u32 = 0;
        // SAFETY: Stack locals are valid for out parameters.
        let status = unsafe {
            (self.protocol.get_result_of_set_active_pcr_banks)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut operation_present,
                &raw mut response,
            )
        };
        if status.is_success() {
            Ok((operation_present, response))
        } else {
            Err(status)
        }
    }

    /// Submits a command.
    ///
    /// Input and output buffers are caller owned. Firmware does
    /// not retain them after the call returns.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn submit_command(&self, input: &[u8], output: &mut [u8]) -> Result<(), EfiStatus> {
        if input.is_empty() || output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let input_size = u32::try_from(input.len()).unwrap_or(u32::MAX);
        let output_size = u32::try_from(output.len()).unwrap_or(u32::MAX);
        // SAFETY: Slices are valid for the call.
        let status = unsafe {
            (self.protocol.submit_command)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                input_size,
                input.as_ptr().cast_mut(),
                output_size,
                output.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

impl core::fmt::Debug for Tcg2Protocol<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Tcg2Protocol")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
