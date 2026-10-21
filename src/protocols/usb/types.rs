//! # USB types
//!
//! Transfer directions and speeds.

/// A transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbDirection(u8);

impl UsbDirection {
    /// Data in.
    pub const IN: Self = Self(0);

    /// Data out.
    pub const OUT: Self = Self(1);

    /// No data.
    pub const NO_DATA: Self = Self(2);

    /// Creates a value from a raw byte.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u8 {
        self.0
    }
}

/// A device speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbSpeed(u8);

impl UsbSpeed {
    /// Full speed.
    pub const FULL: Self = Self(0);

    /// Low speed.
    pub const LOW: Self = Self(1);

    /// High speed.
    pub const HIGH: Self = Self(2);

    /// Super speed.
    pub const SUPER: Self = Self(3);

    /// Creates a value from a raw byte.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u8 {
        self.0
    }
}

/// A setup request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UsbDeviceRequest {
    /// The request type.
    pub request_type: u8,
    /// The request.
    pub request: u8,
    /// The value.
    pub value: u16,
    /// The index.
    pub index: u16,
    /// The length.
    pub length: u16,
}

const _: () = {
    assert!(core::mem::size_of::<UsbDeviceRequest>() == 8);
};
