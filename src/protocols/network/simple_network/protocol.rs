//! # Simple network protocol
//!
//! The firmware interface for packet transmission and reception.
//!
//! The struct holds the revision, thirteen function entries,
//! the wait-for-packet event and the mode pointer in
//! specification order.

use core::ffi::c_void;

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiEvent, EfiStatus};

use super::mode::EfiSimpleNetworkMode;

/// The transmit function.
pub type EfiSimpleNetworkTransmit = unsafe extern "efiapi" fn(
    *mut EfiSimpleNetworkProtocol,
    UIntN,
    UIntN,
    *mut c_void,
    *mut c_void,
    *mut c_void,
) -> EfiStatus;

/// The receive function.
pub type EfiSimpleNetworkReceive = unsafe extern "efiapi" fn(
    *mut EfiSimpleNetworkProtocol,
    *mut UIntN,
    *mut UIntN,
    *mut c_void,
    *mut c_void,
    *mut c_void,
) -> EfiStatus;

/// The simple network protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimpleNetworkProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// Starts the device.
    pub start: unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol) -> EfiStatus,
    /// Stops the device.
    pub stop: unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol) -> EfiStatus,
    /// Initializes the device.
    pub initialize:
        unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol, *mut c_void) -> EfiStatus,
    /// Resets the device.
    pub reset: unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol, Bool) -> EfiStatus,
    /// Shuts down the device.
    pub shutdown: unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol) -> EfiStatus,
    /// Sets receive filters.
    pub receive_filters: unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol, u32) -> EfiStatus,
    /// Sets the station address.
    pub station_address:
        unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol, Bool, *mut c_void) -> EfiStatus,
    /// Gets statistics.
    pub statistics:
        unsafe extern "efiapi" fn(*mut EfiSimpleNetworkProtocol, *mut c_void) -> EfiStatus,
    /// Converts a multicast IP address to a MAC address.
    pub mcast_ip_to_mac: unsafe extern "efiapi" fn(
        *mut EfiSimpleNetworkProtocol,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Writes non-volatile data.
    pub nv_data: unsafe extern "efiapi" fn(
        *mut EfiSimpleNetworkProtocol,
        Bool,
        UIntN,
        UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Gets device status.
    pub get_status: unsafe extern "efiapi" fn(
        *mut EfiSimpleNetworkProtocol,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Transmits a packet.
    pub transmit: EfiSimpleNetworkTransmit,
    /// Receives a packet.
    pub receive: EfiSimpleNetworkReceive,
    /// The event signaled when a packet is received.
    pub wait_for_packet: EfiEvent,
    /// The mode data for the device.
    pub mode: *mut EfiSimpleNetworkMode,
}

/// A typed simple network view.
pub struct SimpleNetwork<'a> {
    protocol: &'a EfiSimpleNetworkProtocol,
}

impl<'a> SimpleNetwork<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimpleNetworkProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSimpleNetworkProtocol {
        self.protocol
    }

    /// Starts the device.
    #[must_use]
    pub fn start(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.start)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Stops the device.
    #[must_use]
    pub fn stop(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.stop)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Transmits a packet.
    ///
    /// # Safety
    ///
    /// The buffers must remain alive until transmission completes
    /// and sizes must describe the packet correctly.
    #[must_use]
    pub unsafe fn transmit(
        &self,
        header_size: UIntN,
        buffer_size: UIntN,
        buffer: *mut c_void,
        source: *mut c_void,
        destination: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees buffer validity.
        unsafe {
            (self.protocol.transmit)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                header_size,
                buffer_size,
                buffer,
                source,
                destination,
            )
        }
    }
}

impl core::fmt::Debug for SimpleNetwork<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SimpleNetwork")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
