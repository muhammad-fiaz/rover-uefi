//! # Hash protocol
//!
//! The firmware interface for message hashing.
//!
//! The hash operation takes an extend flag, a message pointer,
//! a 64-bit message size and an output pointer. The extend flag
//! selects a new hash or extension of an existing hash value.
//! The caller allocates the output buffer using the size
//! returned by `GetHashSize`.

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiGuid, EfiStatus};

/// The hash protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHashProtocol {
    /// Get hash size.
    pub get_hash_size:
        unsafe extern "efiapi" fn(*mut EfiHashProtocol, *mut EfiGuid, *mut UIntN) -> EfiStatus,
    /// Compute a hash.
    pub hash: unsafe extern "efiapi" fn(
        *mut EfiHashProtocol,
        *mut EfiGuid,
        Bool,
        *mut u8,
        u64,
        *mut u8,
    ) -> EfiStatus,
}

/// A borrowed hash view.
pub struct HashProtocol<'a> {
    protocol: &'a EfiHashProtocol,
}

impl<'a> HashProtocol<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiHashProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiHashProtocol {
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

    /// Computes a new hash without extension.
    ///
    /// The output buffer must be large enough for the algorithm
    /// digest size as reported by [`HashProtocol::get_hash_size`].
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
        if message.is_empty() || output.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let message_size = u64::try_from(message.len()).unwrap_or(u64::MAX);
        // SAFETY: Slices are valid for the call and the output
        // buffer is caller allocated.
        let status = unsafe {
            (self.protocol.hash)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::from_ref(algorithm).cast_mut(),
                crate::abi::types::EFI_FALSE,
                message.as_ptr().cast_mut(),
                message_size,
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

impl core::fmt::Debug for HashProtocol<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("HashProtocol")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
