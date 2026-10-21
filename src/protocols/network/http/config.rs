//! # HTTP configuration
//!
//! Configuration helpers.

/// An HTTP version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HttpVersion(u32);

impl HttpVersion {
    /// HTTP 1.0.
    pub const V10: Self = Self(0);

    /// HTTP 1.1.
    pub const V11: Self = Self(1);

    /// Creates a value from a raw integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}
