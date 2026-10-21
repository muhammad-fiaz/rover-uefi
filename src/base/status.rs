//! # Status codes
//!
//! The `UEFI` status type and the named status values
//! defined by the specification.
//!
//! Every firmware service, protocol function and image
//! entry point returns an `EFI_STATUS`. The raw value
//! is a pointer-sized unsigned integer. Success is `0`,
//! warnings have the error bit clear with a non-zero
//! code, and errors have the high bit set.
//!
//! The error bit is the highest bit of the pointer-sized
//! integer, so its value depends on the target pointer
//! width. This module computes it from
//! `usize::BITS` so the constants are correct on both
//! 32-bit and 64-bit targets.

/// The high bit set in every error status code.
///
/// The value is `1` shifted left by the number of bits
/// minus one in `usize`. On 64-bit targets this is
/// `0x8000_0000_0000_0000`, and on 32-bit targets it is
/// `0x8000_0000`.
pub const ERROR_BIT: usize = 1usize << (usize::BITS - 1);

/// The largest warning code value.
///
/// Warning codes occupy the range below [`ERROR_BIT`]
/// with a non-zero value, so the maximum is one less
/// than the error bit.
pub const MAX_WARNING: usize = ERROR_BIT - 1;

/// An opaque `UEFI` status code.
///
/// The wrapper preserves the exact raw value returned
/// by firmware. Construct values with [`EfiStatus::from_raw`]
/// or with the named associated constants such as
/// [`EfiStatus::SUCCESS`] and [`EfiStatus::INVALID_PARAMETER`].
/// Inspect the raw value with [`EfiStatus::to_raw`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EfiStatus(usize);

impl EfiStatus {
    /// The operation completed successfully.
    ///
    /// The raw value is `0`.
    pub const SUCCESS: Self = Self(0);

    /// A loading error occurred.
    pub const LOAD_ERROR: Self = Self(ERROR_BIT | 0x1);

    /// An invalid parameter was supplied.
    pub const INVALID_PARAMETER: Self = Self(ERROR_BIT | 0x2);

    /// The operation is not supported.
    pub const UNSUPPORTED: Self = Self(ERROR_BIT | 0x3);

    /// The buffer size was invalid.
    pub const BAD_BUFFER_SIZE: Self = Self(ERROR_BIT | 0x4);

    /// The buffer was too small.
    pub const BUFFER_TOO_SMALL: Self = Self(ERROR_BIT | 0x5);

    /// The device or service is not ready.
    pub const NOT_READY: Self = Self(ERROR_BIT | 0x6);

    /// A device error occurred.
    pub const DEVICE_ERROR: Self = Self(ERROR_BIT | 0x7);

    /// The media is write protected.
    pub const WRITE_PROTECTED: Self = Self(ERROR_BIT | 0x8);

    /// Resources are exhausted.
    pub const OUT_OF_RESOURCES: Self = Self(ERROR_BIT | 0x9);

    /// The volume is corrupted.
    pub const VOLUME_CORRUPTED: Self = Self(ERROR_BIT | 0x0A);

    /// The volume is full.
    pub const VOLUME_FULL: Self = Self(ERROR_BIT | 0x0B);

    /// No media is present.
    pub const NO_MEDIA: Self = Self(ERROR_BIT | 0x0C);

    /// The media has changed.
    pub const MEDIA_CHANGED: Self = Self(ERROR_BIT | 0x0D);

    /// The requested item was not found.
    pub const NOT_FOUND: Self = Self(ERROR_BIT | 0x0E);

    /// Access was denied.
    pub const ACCESS_DENIED: Self = Self(ERROR_BIT | 0x0F);

    /// No response was received.
    pub const NO_RESPONSE: Self = Self(ERROR_BIT | 0x10);

    /// No mapping exists for the request.
    pub const NO_MAPPING: Self = Self(ERROR_BIT | 0x11);

    /// A timeout occurred.
    pub const TIMEOUT: Self = Self(ERROR_BIT | 0x12);

    /// The operation has not been started.
    pub const NOT_STARTED: Self = Self(ERROR_BIT | 0x13);

    /// The operation is already started.
    pub const ALREADY_STARTED: Self = Self(ERROR_BIT | 0x14);

    /// The operation was aborted.
    pub const ABORTED: Self = Self(ERROR_BIT | 0x15);

    /// An `ICMP` error occurred.
    pub const ICMP_ERROR: Self = Self(ERROR_BIT | 0x16);

    /// A `TFTP` error occurred.
    pub const TFTP_ERROR: Self = Self(ERROR_BIT | 0x17);

    /// A protocol error occurred.
    pub const PROTOCOL_ERROR: Self = Self(ERROR_BIT | 0x18);

    /// An incompatible version was supplied.
    pub const INCOMPATIBLE_VERSION: Self = Self(ERROR_BIT | 0x19);

    /// A security violation was detected.
    pub const SECURITY_VIOLATION: Self = Self(ERROR_BIT | 0x1A);

    /// A CRC error occurred.
    pub const CRC_ERROR: Self = Self(ERROR_BIT | 0x1B);

    /// The end of the media was reached.
    pub const END_OF_MEDIA: Self = Self(ERROR_BIT | 0x1C);

    /// The end of a file was reached.
    pub const END_OF_FILE: Self = Self(ERROR_BIT | 0x1F);

    /// An invalid language was supplied.
    pub const INVALID_LANGUAGE: Self = Self(ERROR_BIT | 0x20);

    /// Compromised data was detected.
    pub const COMPROMISED_DATA: Self = Self(ERROR_BIT | 0x21);

    /// An IP address conflict was detected.
    pub const IP_ADDRESS_CONFLICT: Self = Self(ERROR_BIT | 0x22);

    /// An HTTP error occurred.
    pub const HTTP_ERROR: Self = Self(ERROR_BIT | 0x23);

    /// An unknown glyph was encountered during output.
    pub const WARN_UNKNOWN_GLYPH: Self = Self(0x1);

    /// A delete operation failed.
    pub const WARN_DELETE_FAILURE: Self = Self(0x2);

    /// A write operation failed.
    pub const WARN_WRITE_FAILURE: Self = Self(0x3);

    /// A buffer was too small for a warning result.
    pub const WARN_BUFFER_TOO_SMALL: Self = Self(0x4);

    /// Stale data was returned.
    pub const WARN_STALE_DATA: Self = Self(0x5);

    /// A file system warning occurred.
    pub const WARN_FILE_SYSTEM: Self = Self(0x6);

    /// A reset is required.
    pub const WARN_RESET_REQUIRED: Self = Self(0x7);

    /// Creates a status from a raw firmware value.
    ///
    /// The value is stored without validation. Firmware
    /// may return values that do not match a named
    /// constant; the raw value is always preserved.
    #[must_use]
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    /// Returns the raw firmware value.
    #[must_use]
    pub const fn to_raw(self) -> usize {
        self.0
    }

    /// Returns whether the status is an error.
    ///
    /// An error has [`ERROR_BIT`] set.
    #[must_use]
    pub const fn is_error(self) -> bool {
        self.0 & ERROR_BIT != 0
    }

    /// Returns whether the status is a warning.
    ///
    /// A warning has the error bit clear and a non-zero
    /// value.
    #[must_use]
    pub const fn is_warning(self) -> bool {
        self.0 != 0 && self.0 & ERROR_BIT == 0
    }

    /// Returns whether the status is success.
    ///
    /// Success is the raw value `0`.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.0 == 0
    }

    /// Returns the error or warning code.
    ///
    /// The code is the low portion of the raw value
    /// with [`ERROR_BIT`] cleared. For success the
    /// code is `0`. The result is the numeric code
    /// only; use [`EfiStatus::is_error`] and
    /// [`EfiStatus::is_warning`] to distinguish class.
    #[must_use]
    pub const fn code(self) -> usize {
        self.0 & !ERROR_BIT
    }
}

impl core::fmt::Debug for EfiStatus {
    /// Formats the status as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("EfiStatus").field(&self.0).finish()
    }
}
