//! # BLT operations
//!
//! Block transfer operations.
//!
//! The operation is a `u32` value. Delta is the
//! byte stride of a buffer row. Zero means the
//! buffer is tightly packed.

use crate::abi::types::UIntN;

/// A BLT operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BltOperation(u32);

impl BltOperation {
    /// Fill the video rectangle with one pixel.
    pub const VIDEO_FILL: Self = Self(0);

    /// Copy video to the BLT buffer.
    pub const VIDEO_TO_BLT_BUFFER: Self = Self(1);

    /// Copy the BLT buffer to video.
    pub const BUFFER_TO_VIDEO: Self = Self(2);

    /// Copy video to video.
    pub const VIDEO_TO_VIDEO: Self = Self(3);

    /// Maximum operation value.
    pub const MAX: Self = Self(4);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value when the raw integer
    /// names a defined operation.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 4 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

/// A BLT rectangle.
///
/// Coordinates and sizes are in pixels except
/// `delta`, which is in bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BltRectangle {
    /// The source X coordinate.
    pub source_x: UIntN,
    /// The source Y coordinate.
    pub source_y: UIntN,
    /// The destination X coordinate.
    pub destination_x: UIntN,
    /// The destination Y coordinate.
    pub destination_y: UIntN,
    /// The rectangle width.
    pub width: UIntN,
    /// The rectangle height.
    pub height: UIntN,
    /// The buffer row stride in bytes.
    pub delta: UIntN,
}

/// A complete BLT request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BltRequest {
    /// The BLT operation.
    pub operation: BltOperation,
    /// The rectangle.
    pub rectangle: BltRectangle,
}
