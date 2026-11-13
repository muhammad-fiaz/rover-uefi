//! # Embedded General Purpose Input/Output (GPIO) Protocol
//!
//! Provides hardware pin control, input reading, output driving, special function assignment,
//! and internal pull-up / pull-down resistor configuration.

#![allow(clippy::doc_markdown)]

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// Embedded GPIO Protocol `GUID`.
///
/// `{17a0a3d7-c0a5-4635-bbd5-072187dfe2ee}`
pub const EMBEDDED_GPIO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x17A0_A3D7,
    0xC0A5,
    0x4635,
    [0xBB, 0xD5, 0x07, 0x21, 0x87, 0xDF, 0xE2, 0xEE],
);

/// Platform GPIO Protocol `GUID`.
///
/// `{52ce9845-5af4-43e2-bafd-230812547ac2}`
pub const PLATFORM_GPIO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x52CE_9845,
    0x5AF4,
    0x43E2,
    [0xBA, 0xFD, 0x23, 0x08, 0x12, 0x54, 0x7A, 0xC2],
);

/// A composite GPIO identifier combining a 16-bit controller port and 16-bit pin index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpioPin(pub usize);

impl GpioPin {
    /// Creates a composite GPIO pin identifier from controller port and pin offset.
    #[must_use]
    pub const fn new(port: u16, pin: u16) -> Self {
        Self(((port as usize) << 16) | (pin as usize))
    }

    /// Extracts the controller port number (upper 16 bits).
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn port(self) -> u16 {
        (self.0 >> 16) as u16
    }

    /// Extracts the pin index within the controller port (lower 16 bits).
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn pin(self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }
}

/// Operational direction and alternate function modes for a GPIO pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GpioMode {
    /// Input mode (high impedance).
    Input = 0x00,
    /// Driven low output (logical 0).
    Output0 = 0x0E,
    /// Driven high output (logical 1).
    Output1 = 0x0F,
    /// Alternate hardware peripheral function 2.
    SpecialFunction2 = 0x02,
    /// Alternate hardware peripheral function 3.
    SpecialFunction3 = 0x03,
    /// Alternate hardware peripheral function 4.
    SpecialFunction4 = 0x04,
    /// Alternate hardware peripheral function 5.
    SpecialFunction5 = 0x05,
    /// Alternate hardware peripheral function 6.
    SpecialFunction6 = 0x06,
    /// Alternate hardware peripheral function 7.
    SpecialFunction7 = 0x07,
}

impl GpioMode {
    /// Decodes a raw 32-bit integer into a [`GpioMode`].
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0x00 => Some(Self::Input),
            0x0E => Some(Self::Output0),
            0x0F => Some(Self::Output1),
            0x02 => Some(Self::SpecialFunction2),
            0x03 => Some(Self::SpecialFunction3),
            0x04 => Some(Self::SpecialFunction4),
            0x05 => Some(Self::SpecialFunction5),
            0x06 => Some(Self::SpecialFunction6),
            0x07 => Some(Self::SpecialFunction7),
            _ => None,
        }
    }
}

/// Internal resistor pull configuration for a GPIO pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GpioPull {
    /// Floating / no internal bias.
    None = 0,
    /// Internal weak pull-up resistor enabled.
    PullUp = 1,
    /// Internal weak pull-down resistor enabled.
    PullDown = 2,
}

/// Raw function pointer for reading the current binary state of a GPIO pin.
pub type EmbeddedGpioGet = unsafe extern "efiapi" fn(
    this: *mut EmbeddedGpioProtocol,
    gpio: GpioPin,
    value: *mut UIntN,
) -> EfiStatus;

/// Raw function pointer for configuring the operational mode of a GPIO pin.
pub type EmbeddedGpioSet = unsafe extern "efiapi" fn(
    this: *mut EmbeddedGpioProtocol,
    gpio: GpioPin,
    mode: GpioMode,
) -> EfiStatus;

/// Raw function pointer for reading the active mode of a GPIO pin.
pub type EmbeddedGpioGetMode = unsafe extern "efiapi" fn(
    this: *mut EmbeddedGpioProtocol,
    gpio: GpioPin,
    mode: *mut GpioMode,
) -> EfiStatus;

/// Raw function pointer for setting internal bias resistors.
pub type EmbeddedGpioSetPull = unsafe extern "efiapi" fn(
    this: *mut EmbeddedGpioProtocol,
    gpio: GpioPin,
    direction: GpioPull,
) -> EfiStatus;

/// Raw Embedded GPIO Protocol structure.
#[derive(Debug)]
#[repr(C)]
pub struct EmbeddedGpioProtocol {
    /// Reads pin level.
    pub get: EmbeddedGpioGet,
    /// Sets pin mode.
    pub set: EmbeddedGpioSet,
    /// Queries active pin mode.
    pub get_mode: EmbeddedGpioGetMode,
    /// Configures internal bias pull resistors.
    pub set_pull: EmbeddedGpioSetPull,
}

/// High-level safe wrapper around [`EmbeddedGpioProtocol`].
pub struct EmbeddedGpio<'a> {
    proto: &'a mut EmbeddedGpioProtocol,
}

impl<'a> EmbeddedGpio<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut EmbeddedGpioProtocol) -> Self {
        Self { proto }
    }

    /// Reads the logic level of `pin` (0 or 1).
    ///
    /// # Errors
    ///
    /// Returns an error status if the hardware pin query fails.
    pub fn get(&mut self, pin: GpioPin) -> Result<usize, EfiStatus> {
        let mut value: UIntN = 0;
        let status = unsafe { (self.proto.get)(self.proto, pin, &raw mut value) };
        if status.is_success() {
            Ok(value)
        } else {
            Err(status)
        }
    }

    /// Configures the mode of `pin`.
    ///
    /// # Errors
    ///
    /// Returns an error status if the pin mode transition is rejected by hardware.
    pub fn set(&mut self, pin: GpioPin, mode: GpioMode) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set)(self.proto, pin, mode) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves the current mode of `pin`.
    ///
    /// # Errors
    ///
    /// Returns an error status if reading pin configuration fails.
    pub fn get_mode(&mut self, pin: GpioPin) -> Result<GpioMode, EfiStatus> {
        let mut mode = GpioMode::Input;
        let status = unsafe { (self.proto.get_mode)(self.proto, pin, &raw mut mode) };
        if status.is_success() {
            Ok(mode)
        } else {
            Err(status)
        }
    }

    /// Configures internal pull-up / pull-down resistors for `pin`.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting pull state fails or is unsupported on `pin`.
    pub fn set_pull(&mut self, pin: GpioPin, pull: GpioPull) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_pull)(self.proto, pin, pull) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
