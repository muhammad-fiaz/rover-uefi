//! # Component names
//!
//! The firmware interfaces for driver and controller names.

use crate::abi::types::{Char8, Char16};
use crate::base::{EfiHandle, EfiStatus};

/// The component name protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiComponentNameProtocol {
    /// Gets the driver name.
    pub get_driver_name: unsafe extern "efiapi" fn(
        *mut EfiComponentNameProtocol,
        *mut Char8,
        *mut *mut Char16,
    ) -> EfiStatus,
    /// Gets the controller name.
    pub get_controller_name: unsafe extern "efiapi" fn(
        *mut EfiComponentNameProtocol,
        EfiHandle,
        *mut Char8,
        *mut *mut Char16,
    ) -> EfiStatus,
    /// The supported languages string.
    pub supported_languages: *mut Char8,
}

/// The component name 2 protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiComponentName2Protocol {
    /// Gets the driver name.
    pub get_driver_name: unsafe extern "efiapi" fn(
        *mut EfiComponentName2Protocol,
        *mut Char8,
        *mut *mut Char16,
    ) -> EfiStatus,
    /// Gets the controller name.
    pub get_controller_name: unsafe extern "efiapi" fn(
        *mut EfiComponentName2Protocol,
        EfiHandle,
        *mut Char8,
        *mut *mut Char16,
    ) -> EfiStatus,
    /// The supported languages string.
    pub supported_languages: *mut Char8,
}

/// A borrowed component name view.
pub struct ComponentName<'a> {
    protocol: &'a EfiComponentNameProtocol,
}

impl<'a> ComponentName<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiComponentNameProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiComponentNameProtocol {
        self.protocol
    }
}

impl core::fmt::Debug for ComponentName<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ComponentName")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed component name 2 view.
pub struct ComponentName2<'a> {
    protocol: &'a EfiComponentName2Protocol,
}

impl<'a> ComponentName2<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiComponentName2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiComponentName2Protocol {
        self.protocol
    }
}

impl core::fmt::Debug for ComponentName2<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ComponentName2")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
