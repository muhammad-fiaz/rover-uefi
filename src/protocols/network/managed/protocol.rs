//! # Managed network protocol
//!
//! The firmware interface for managed packet sessions.
//!
//! The struct has eight function entries in specification order.
//! Structured configuration data remains firmware-owned and is
//! represented with opaque pointers.

use core::ffi::c_void;

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// The managed network protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiManagedNetworkProtocol {
    /// Gets mode data.
    pub get_mode_data:
        unsafe extern "efiapi" fn(*mut EfiManagedNetworkProtocol, *mut c_void) -> EfiStatus,
    /// Configures the session.
    pub configure:
        unsafe extern "efiapi" fn(*mut EfiManagedNetworkProtocol, *mut c_void) -> EfiStatus,
    /// Converts a multicast IP address.
    pub mcast_ip_to_mac: unsafe extern "efiapi" fn(
        *mut EfiManagedNetworkProtocol,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Manages multicast groups.
    pub groups:
        unsafe extern "efiapi" fn(*mut EfiManagedNetworkProtocol, Bool, *mut c_void) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: unsafe extern "efiapi" fn(
        *mut EfiManagedNetworkProtocol,
        *mut c_void,
        *mut c_void,
        UIntN,
        UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Receives a packet.
    pub receive: unsafe extern "efiapi" fn(
        *mut EfiManagedNetworkProtocol,
        *mut c_void,
        *mut c_void,
        *mut UIntN,
        *mut UIntN,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(
        *mut EfiManagedNetworkProtocol,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiManagedNetworkProtocol) -> EfiStatus,
}

/// A borrowed managed network view.
pub struct ManagedNetwork<'a> {
    protocol: &'a EfiManagedNetworkProtocol,
}

impl<'a> ManagedNetwork<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiManagedNetworkProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiManagedNetworkProtocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for ManagedNetwork<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ManagedNetwork")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
