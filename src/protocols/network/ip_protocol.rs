//! # IP protocols
//!
//! The firmware interfaces for IPv4 and IPv6 packet sessions.
//!
//! The IPv4 protocol has eight function entries. The IPv6
//! protocol has nine entries including neighbor management.
//! Structured configuration data remains firmware-owned.

use core::ffi::c_void;

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// The IPv4 protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIp4Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiIp4Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiIp4Protocol, *mut c_void) -> EfiStatus,
    /// Manages multicast groups.
    pub groups: unsafe extern "efiapi" fn(*mut EfiIp4Protocol, Bool, *mut c_void) -> EfiStatus,
    /// Gets routes.
    pub routes:
        unsafe extern "efiapi" fn(*mut EfiIp4Protocol, *mut UIntN, *mut c_void) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: unsafe extern "efiapi" fn(
        *mut EfiIp4Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Receives a packet.
    pub receive: unsafe extern "efiapi" fn(
        *mut EfiIp4Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiIp4Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiIp4Protocol) -> EfiStatus,
}

/// The IPv6 protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIp6Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiIp6Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiIp6Protocol, *mut c_void) -> EfiStatus,
    /// Manages multicast groups.
    pub groups: unsafe extern "efiapi" fn(*mut EfiIp6Protocol, Bool, *mut c_void) -> EfiStatus,
    /// Gets routes.
    pub routes:
        unsafe extern "efiapi" fn(*mut EfiIp6Protocol, *mut UIntN, *mut c_void) -> EfiStatus,
    /// Manages neighbors.
    pub neighbors:
        unsafe extern "efiapi" fn(*mut EfiIp6Protocol, *mut UIntN, *mut c_void) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: unsafe extern "efiapi" fn(
        *mut EfiIp6Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Receives a packet.
    pub receive: unsafe extern "efiapi" fn(
        *mut EfiIp6Protocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiIp6Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiIp6Protocol) -> EfiStatus,
}

/// A borrowed IPv4 view.
pub struct Ip4<'a> {
    protocol: &'a EfiIp4Protocol,
}

impl<'a> Ip4<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiIp4Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiIp4Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Ip4<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Ip4")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed IPv6 view.
pub struct Ip6<'a> {
    protocol: &'a EfiIp6Protocol,
}

impl<'a> Ip6<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiIp6Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiIp6Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Ip6<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Ip6")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
