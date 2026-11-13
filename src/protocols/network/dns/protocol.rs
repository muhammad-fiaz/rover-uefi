//! # DNS protocols
//!
//! The firmware interfaces for DNS lookups.
//!
//! Both the IPv4 and IPv6 DNS protocols have eight entries.

use core::ffi::c_void;

use crate::base::EfiStatus;

/// The IPv4 DNS protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDns4Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void) -> EfiStatus,
    /// Resolves a host name to addresses.
    pub host_name_to_ip:
        unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Resolves an address to a host name.
    pub ip_to_host_name:
        unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Performs a general lookup.
    pub general_look_up:
        unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Updates the DNS cache.
    pub update_dns_cache: unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiDns4Protocol) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiDns4Protocol, *mut c_void) -> EfiStatus,
}

/// The IPv6 DNS protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDns6Protocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void) -> EfiStatus,
    /// Resolves a host name to addresses.
    pub host_name_to_ip:
        unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Resolves an address to a host name.
    pub ip_to_host_name:
        unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Performs a general lookup.
    pub general_look_up:
        unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Updates the DNS cache.
    pub update_dns_cache: unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiDns6Protocol) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiDns6Protocol, *mut c_void) -> EfiStatus,
}

/// A borrowed IPv4 DNS view.
pub struct Dns4<'a> {
    protocol: &'a EfiDns4Protocol,
}

impl<'a> Dns4<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiDns4Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDns4Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Dns4<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Dns4")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed IPv6 DNS view.
pub struct Dns6<'a> {
    protocol: &'a EfiDns6Protocol,
}

impl<'a> Dns6<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiDns6Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDns6Protocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Dns6<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Dns6")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
