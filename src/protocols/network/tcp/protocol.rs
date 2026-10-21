//! # TCP protocols
//!
//! The firmware interfaces for TCP sessions.
//!
//! The IPv4 TCP protocol has ten entries including route
//! management. The IPv6 TCP protocol has nine entries
//! without route management.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::EfiStatus;

/// The IPv4 TCP protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTcp4Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void) -> EfiStatus,
    /// Gets routes.
    pub routes:
        unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut UIntN, *mut c_void) -> EfiStatus,
    /// Connects.
    pub connect: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void) -> EfiStatus,
    /// Accepts.
    pub accept: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void) -> EfiStatus,
    /// Transmits data.
    pub transmit:
        unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Receives data.
    pub receive:
        unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Closes the session.
    pub close: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiTcp4Protocol) -> EfiStatus,
}

/// The IPv6 TCP protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTcp6Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void) -> EfiStatus,
    /// Connects.
    pub connect: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void) -> EfiStatus,
    /// Accepts.
    pub accept: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void) -> EfiStatus,
    /// Transmits data.
    pub transmit:
        unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Receives data.
    pub receive:
        unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Closes the session.
    pub close: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiTcp6Protocol) -> EfiStatus,
}

/// A borrowed IPv4 TCP view.
pub struct Tcp4<'a> {
    protocol: &'a EfiTcp4Protocol,
}

impl<'a> Tcp4<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiTcp4Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiTcp4Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Tcp4<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Tcp4")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed IPv6 TCP view.
pub struct Tcp6<'a> {
    protocol: &'a EfiTcp6Protocol,
}

impl<'a> Tcp6<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiTcp6Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiTcp6Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Tcp6<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Tcp6")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// Legacy alias for the IPv4 TCP view.
pub type TcpProtocol<'a> = Tcp4<'a>;
