//! # Serial types
//!
//! Control bits, parity and stop-bit definitions.

/// Clear-to-send control bit (read-only).
pub const SERIAL_CLEAR_TO_SEND: u32 = 0x0000_0010;

/// Data-set-ready control bit (read-only).
pub const SERIAL_DATA_SET_READY: u32 = 0x0000_0020;

/// Ring-indicate control bit (read-only).
pub const SERIAL_RING_INDICATE: u32 = 0x0000_0040;

/// Carrier-detect control bit (read-only).
pub const SERIAL_CARRIER_DETECT: u32 = 0x0000_0080;

/// Input-buffer-empty control bit (read-only).
pub const SERIAL_INPUT_BUFFER_EMPTY: u32 = 0x0000_0100;

/// Output-buffer-empty control bit (read-only).
pub const SERIAL_OUTPUT_BUFFER_EMPTY: u32 = 0x0000_0200;

/// Request-to-send control bit (write-only).
pub const SERIAL_REQUEST_TO_SEND: u32 = 0x0000_0002;

/// Data-terminal-ready control bit (write-only).
pub const SERIAL_DATA_TERMINAL_READY: u32 = 0x0000_0001;

/// Hardware loopback control bit.
pub const SERIAL_HARDWARE_LOOPBACK_ENABLE: u32 = 0x0000_1000;

/// Software loopback control bit.
pub const SERIAL_SOFTWARE_LOOPBACK_ENABLE: u32 = 0x0000_2000;

/// Hardware flow-control enable bit.
pub const SERIAL_HARDWARE_FLOW_CONTROL_ENABLE: u32 = 0x0000_4000;

/// A parity type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ParityType(u32);

impl ParityType {
    /// Default parity.
    pub const DEFAULT: Self = Self(0);

    /// No parity.
    pub const NO_PARITY: Self = Self(1);

    /// Even parity.
    pub const EVEN: Self = Self(2);

    /// Odd parity.
    pub const ODD: Self = Self(3);

    /// Mark parity.
    pub const MARK: Self = Self(4);

    /// Space parity.
    pub const SPACE: Self = Self(5);

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

/// A stop-bits type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StopBitsType(u32);

impl StopBitsType {
    /// Default stop bits.
    pub const DEFAULT: Self = Self(0);

    /// One stop bit.
    pub const ONE: Self = Self(1);

    /// One and a half stop bits.
    pub const ONE_FIVE: Self = Self(2);

    /// Two stop bits.
    pub const TWO: Self = Self(3);

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

/// Returns whether a data-bits value is in range.
#[must_use]
pub fn validate_data_bits(data_bits: u8) -> bool {
    data_bits == 0 || (5..=8).contains(&data_bits)
}

/// Returns whether a parity value is defined.
#[must_use]
pub fn validate_parity(parity: ParityType) -> bool {
    parity.to_raw() <= 5
}

/// Returns whether a stop-bits value is defined.
#[must_use]
pub fn validate_stop_bits(stop_bits: StopBitsType) -> bool {
    stop_bits.to_raw() <= 3
}
