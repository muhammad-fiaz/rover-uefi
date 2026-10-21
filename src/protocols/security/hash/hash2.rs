//! # Hash2 protocol
//!
//! The firmware interface for padded message hashing.
//!
//! The protocol has five entries. Hashing is finalized by the
//! supporting driver, so padding is handled internally. The
//! no-pad algorithm identifiers are not compatible with this
//! protocol.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// The Hash2 protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHash2Protocol {
    /// Get hash size.
    pub get_hash_size:
        unsafe extern "efiapi" fn(*mut EfiHash2Protocol, *mut EfiGuid, *mut UIntN) -> EfiStatus,
    /// Compute a hash.
    pub hash: unsafe extern "efiapi" fn(
        *mut EfiHash2Protocol,
        *mut EfiGuid,
        *mut u8,
        UIntN,
        *mut u8,
    ) -> EfiStatus,
    /// Initialize a digest calculation.
    pub hash_init: unsafe extern "efiapi" fn(*mut EfiHash2Protocol, *mut EfiGuid) -> EfiStatus,
    /// Update a digest calculation.
    pub hash_update: unsafe extern "efiapi" fn(*mut EfiHash2Protocol, *mut u8, UIntN) -> EfiStatus,
    /// Finalize a digest calculation.
    pub hash_final: unsafe extern "efiapi" fn(*mut EfiHash2Protocol, *mut u8) -> EfiStatus,
}

/// A borrowed Hash2 view.
pub struct Hash2Protocol<'a> {
    protocol: &'a EfiHash2Protocol,
}

impl<'a> Hash2Protocol<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiHash2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiHash2Protocol {
        self.protocol
    }

    /// Returns the hash size for an algorithm.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_hash_size(&self, algorithm: &EfiGuid) -> Result<UIntN, EfiStatus> {
        let mut size: UIntN = 0;
        // SAFETY: The algorithm reference and stack local are
        // valid for the call.
        let status = unsafe {
            (self.protocol.get_hash_size)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::from_ref(algorithm).cast_mut(),
                &raw mut size,
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Computes a hash.
    ///
    /// The output buffer must be large enough for the algorithm
    /// digest size as reported by
    /// [`Hash2Protocol::get_hash_size`].
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn hash(
        &self,
        algorithm: &EfiGuid,
        message: &[u8],
        output: &mut [u8],
    ) -> Result<(), EfiStatus> {
        if output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: Slices are valid for the call.
        let status = unsafe {
            (self.protocol.hash)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::from_ref(algorithm).cast_mut(),
                message.as_ptr().cast_mut(),
                message.len(),
                output.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initializes a digest calculation.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn hash_init(&self, algorithm: &EfiGuid) -> Result<(), EfiStatus> {
        // SAFETY: The algorithm reference is valid for the call.
        let status = unsafe {
            (self.protocol.hash_init)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::from_ref(algorithm).cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Updates a digest calculation.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn hash_update(&self, message: &[u8]) -> Result<(), EfiStatus> {
        if message.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice is valid for the call.
        let status = unsafe {
            (self.protocol.hash_update)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                message.as_ptr().cast_mut(),
                message.len(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Finalizes a digest calculation.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn hash_final(&self, output: &mut [u8]) -> Result<(), EfiStatus> {
        if output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice is valid for the call.
        let status = unsafe {
            (self.protocol.hash_final)(
                core::ptr::from_ref(self.protocol).cast_mut(),
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

impl core::fmt::Debug for Hash2Protocol<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Hash2Protocol")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
