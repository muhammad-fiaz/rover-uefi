//! # Pixels
//!
//! Pixel formats and BLT pixels.
//!
//! The format is a `u32` value. The raw
//! value is preserved so unknown firmware
//! values remain representable.

/// A pixel format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PixelFormat(u32);

impl PixelFormat {
    /// Red-green-blue reserved 8 bits per color.
    pub const RGB_RESERVED: Self = Self(0);

    /// Blue-green-red reserved 8 bits per color.
    pub const BGR_RESERVED: Self = Self(1);

    /// Pixel bitmask format.
    pub const BITMASK: Self = Self(2);

    /// BLT only format.
    pub const BLT_ONLY: Self = Self(3);

    /// Maximum format value.
    pub const MAX: Self = Self(4);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value when the raw integer names
    /// a defined format.
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

/// A pixel bitmask.
///
/// The struct is `#[repr(C)]` with four
/// fields matching the specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiPixelBitmask {
    /// The red mask.
    pub red_mask: u32,
    /// The green mask.
    pub green_mask: u32,
    /// The blue mask.
    pub blue_mask: u32,
    /// The reserved mask.
    pub reserved_mask: u32,
}

/// A BLT pixel.
///
/// The struct is `#[repr(C)]` with byte order
/// blue, green, red, reserved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiGraphicsOutputBltPixel {
    /// The blue component.
    pub blue: u8,
    /// The green component.
    pub green: u8,
    /// The red component.
    pub red: u8,
    /// The reserved component.
    pub reserved: u8,
}

const _: () = {
    assert!(core::mem::size_of::<EfiPixelBitmask>() == 16);
    assert!(core::mem::size_of::<EfiGraphicsOutputBltPixel>() == 4);
    assert!(core::mem::offset_of!(EfiGraphicsOutputBltPixel, blue) == 0);
    assert!(core::mem::offset_of!(EfiGraphicsOutputBltPixel, green) == 1);
    assert!(core::mem::offset_of!(EfiGraphicsOutputBltPixel, red) == 2);
    assert!(core::mem::offset_of!(EfiGraphicsOutputBltPixel, reserved) == 3);
};
