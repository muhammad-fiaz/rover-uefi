//! # PXE base code protocol
//!
//! The firmware interface for pre-execution network boot.
//!
//! The struct holds the revision, twelve function entries and
//! the mode pointer in specification order. Mode data is
//! firmware-owned and represented with an opaque pointer.

use core::ffi::c_void;

use crate::abi::types::Bool;
use crate::base::EfiStatus;

/// The PXE base code protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPxeBaseCodeProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// Starts the protocol.
    pub start: unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol, Bool) -> EfiStatus,
    /// Stops the protocol.
    pub stop: unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol) -> EfiStatus,
    /// Performs dynamic host configuration.
    pub dhcp: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        Bool,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Discovers a boot server.
    pub discover: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        Bool,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Transfers a file with multicast support.
    pub mtftp: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Writes a UDP packet.
    pub udp_write: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Reads a UDP packet.
    pub udp_read: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Sets the IP filter.
    pub set_ip_filter:
        unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol, *mut c_void) -> EfiStatus,
    /// Performs address resolution.
    pub arp: unsafe extern "efiapi" fn(
        *mut EfiPxeBaseCodeProtocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Sets protocol parameters.
    pub set_parameters:
        unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol, *mut c_void) -> EfiStatus,
    /// Sets the station IP address.
    pub set_station_ip:
        unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol, *mut c_void) -> EfiStatus,
    /// Sets packet data.
    pub set_packets:
        unsafe extern "efiapi" fn(*mut EfiPxeBaseCodeProtocol, *mut c_void) -> EfiStatus,
    /// The mode data for the device.
    pub mode: *mut c_void,
}

/// A borrowed PXE base code view.
pub struct PxeBaseCode<'a> {
    protocol: &'a EfiPxeBaseCodeProtocol,
}

impl<'a> PxeBaseCode<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiPxeBaseCodeProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiPxeBaseCodeProtocol {
        self.protocol
    }

    /// Stops the protocol.
    #[must_use]
    pub fn stop(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.stop)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for PxeBaseCode<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PxeBaseCode")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
