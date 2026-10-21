//! # Driver binding
//!
//! The firmware interface for driver lifecycle management.

use crate::abi::types::UIntN;
use crate::base::{EfiHandle, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// The driver binding protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverBindingProtocol {
    /// Tests whether the driver supports a controller.
    pub supported: unsafe extern "efiapi" fn(
        *mut EfiDriverBindingProtocol,
        EfiHandle,
        *mut EfiDevicePath,
    ) -> EfiStatus,
    /// Starts a controller.
    pub start: unsafe extern "efiapi" fn(
        *mut EfiDriverBindingProtocol,
        EfiHandle,
        *mut EfiDevicePath,
    ) -> EfiStatus,
    /// Stops a controller.
    pub stop: unsafe extern "efiapi" fn(
        *mut EfiDriverBindingProtocol,
        EfiHandle,
        UIntN,
        *mut EfiHandle,
    ) -> EfiStatus,
    /// The driver version.
    pub version: u32,
    /// The driver image handle.
    pub image_handle: EfiHandle,
    /// The driver binding handle.
    pub driver_binding_handle: EfiHandle,
}

/// A borrowed driver binding view.
pub struct DriverBinding<'a> {
    protocol: &'a EfiDriverBindingProtocol,
}

impl<'a> DriverBinding<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiDriverBindingProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDriverBindingProtocol {
        self.protocol
    }

    /// Tests whether the driver supports a controller.
    ///
    /// # Safety
    ///
    /// The controller handle must be valid and the device-path
    /// pointer must be null or point to a valid firmware path.
    #[must_use]
    pub unsafe fn supported(
        &self,
        controller: EfiHandle,
        remaining: *mut EfiDevicePath,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity.
        unsafe {
            (self.protocol.supported)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                controller,
                remaining,
            )
        }
    }

    /// Starts a controller.
    ///
    /// # Safety
    ///
    /// The controller handle must be valid, the device-path
    /// pointer must be null or valid, and the driver must have
    /// previously reported support for the same parameters.
    #[must_use]
    pub unsafe fn start(&self, controller: EfiHandle, remaining: *mut EfiDevicePath) -> EfiStatus {
        // SAFETY: The caller guarantees the start contract.
        unsafe {
            (self.protocol.start)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                controller,
                remaining,
            )
        }
    }

    /// Stops a controller.
    ///
    /// # Safety
    ///
    /// The controller handle must have been started by this
    /// driver and every child handle must have been created by
    /// this driver during start.
    #[must_use]
    pub unsafe fn stop(&self, controller: EfiHandle, children: &mut [EfiHandle]) -> EfiStatus {
        // SAFETY: The caller guarantees handle validity.
        unsafe {
            (self.protocol.stop)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                controller,
                children.len(),
                children.as_mut_ptr(),
            )
        }
    }
}

impl core::fmt::Debug for DriverBinding<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DriverBinding")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
