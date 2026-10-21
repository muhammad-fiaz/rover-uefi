//! # RNG protocol
//!
//! The firmware interface for random number generation.
//!
//! The protocol has exactly two entries. The first entry in the
//! algorithm list returned by `GetInfo` is the default algorithm
//! for the driver. A null algorithm pointer selects the default.
//! Firmware RNG quality depends on the platform implementation;
//! the presence of this protocol does not by itself guarantee
//! any particular entropy quality.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// Gets algorithm information.
pub type EfiRngGetInfo =
    unsafe extern "efiapi" fn(*mut EfiRngProtocol, *mut UIntN, *mut EfiGuid) -> EfiStatus;

/// Gets random bytes.
pub type EfiRngGetRng =
    unsafe extern "efiapi" fn(*mut EfiRngProtocol, *mut EfiGuid, UIntN, *mut u8) -> EfiStatus;

/// The RNG protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiRngProtocol {
    /// Get algorithm information.
    pub get_info: EfiRngGetInfo,
    /// Get random bytes.
    pub get_rng: EfiRngGetRng,
}

/// A borrowed RNG view.
pub struct RngProtocol<'a> {
    protocol: &'a EfiRngProtocol,
}

impl<'a> RngProtocol<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiRngProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiRngProtocol {
        self.protocol
    }

    /// Requests the required algorithm list size.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn algorithm_list_size(&self) -> Result<UIntN, EfiStatus> {
        let mut size: UIntN = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_info)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut size,
                core::ptr::null_mut(),
            )
        };
        if status.is_success() || status == EfiStatus::BUFFER_TOO_SMALL {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Gets random bytes with the default algorithm.
    ///
    /// A null algorithm pointer is passed to firmware, which
    /// selects the default algorithm.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_rng_default(&self, output: &mut [u8]) -> Result<(), EfiStatus> {
        if output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice pointer is valid for `output.len()` bytes.
        let status = unsafe {
            (self.protocol.get_rng)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::null_mut(),
                output.len(),
                output.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Gets random bytes with a specific algorithm.
    ///
    /// The caller must not assume a different algorithm is
    /// substituted when the requested one is unsupported;
    /// firmware reports `EFI_UNSUPPORTED` in that case.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_rng(&self, algorithm: &EfiGuid, output: &mut [u8]) -> Result<(), EfiStatus> {
        if output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The algorithm reference and slice pointer are
        // valid for the call.
        let status = unsafe {
            (self.protocol.get_rng)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::from_ref(algorithm).cast_mut(),
                output.len(),
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

impl core::fmt::Debug for RngProtocol<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RngProtocol")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
