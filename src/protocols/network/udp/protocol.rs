//! # UDP protocols
//!
//! The firmware interfaces for UDP sessions.
//!
//! The IPv4 UDP protocol has eight entries. The IPv6 UDP
//! protocol has seven entries without route management.

use core::ffi::c_void;

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// The IPv4 UDP protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUdp4Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiUdp4Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiUdp4Protocol, *mut c_void) -> EfiStatus,
    /// Manages multicast groups.
    pub groups: unsafe extern "efiapi" fn(*mut EfiUdp4Protocol, Bool, *mut c_void) -> EfiStatus,
    /// Gets routes.
    pub routes:
        unsafe extern "efiapi" fn(*mut EfiUdp4Protocol, *mut UIntN, *mut c_void) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: unsafe extern "efiapi" fn(
        *mut EfiUdp4Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Receives a packet.
    pub receive: unsafe extern "efiapi" fn(
        *mut EfiUdp4Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiUdp4Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiUdp4Protocol) -> EfiStatus,
}

/// The IPv6 UDP protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUdp6Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiUdp6Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiUdp6Protocol, *mut c_void) -> EfiStatus,
    /// Manages multicast groups.
    pub groups: unsafe extern "efiapi" fn(*mut EfiUdp6Protocol, Bool, *mut c_void) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: unsafe extern "efiapi" fn(
        *mut EfiUdp6Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Receives a packet.
    pub receive: unsafe extern "efiapi" fn(
        *mut EfiUdp6Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiUdp6Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiUdp6Protocol) -> EfiStatus,
}

/// A borrowed IPv4 UDP view.
pub struct Udp4<'a> {
    protocol: &'a EfiUdp4Protocol,
}

impl<'a> Udp4<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiUdp4Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiUdp4Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Udp4<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Udp4")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed IPv6 UDP view.
pub struct Udp6<'a> {
    protocol: &'a EfiUdp6Protocol,
}

impl<'a> Udp6<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiUdp6Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiUdp6Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Udp6<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Udp6")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
