//! # USB 2 host controller
//!
//! The `EFI_USB2_HC_PROTOCOL`.
//!
//! The wrapper borrows the firmware protocol.
//! Transfers require device addresses, speeds,
//! and translator data that the caller must
//! supply correctly.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use crate::protocols::usb::types::{UsbDeviceRequest, UsbDirection};

/// The root hub port status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UsbPortStatus {
    /// The port status bits.
    pub port_status: u16,
    /// The change bits.
    pub port_change_status: u16,
}

/// Queries root hub status.
pub type EfiUsb2HcGetRootHubPortStatus =
    unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, u8, *mut UsbPortStatus) -> EfiStatus;

/// Flushes the controller.
pub type EfiUsb2HcReset = unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, u16) -> EfiStatus;

/// The USB 2 host controller protocol.
///
/// The struct is `#[repr(C)]`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUsb2HcProtocol {
    /// Get capability.
    pub get_capability:
        unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, *mut u8, *mut u8, *mut u8) -> EfiStatus,
    /// Reset.
    pub reset: EfiUsb2HcReset,
    /// Get state.
    pub get_state: unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, *mut u32) -> EfiStatus,
    /// Set state.
    pub set_state: unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, u32) -> EfiStatus,
    /// Control transfer.
    pub control_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        UIntN,
        *mut UsbDeviceRequest,
        UsbDirection,
        *mut c_void,
        *mut UIntN,
        UIntN,
        *mut c_void,
        *mut u32,
    ) -> EfiStatus,
    /// Bulk transfer.
    pub bulk_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        u8,
        UIntN,
        u8,
        *mut *mut c_void,
        *mut UIntN,
        *mut u8,
        UIntN,
        *mut c_void,
        *mut u32,
    ) -> EfiStatus,
    /// Async interrupt.
    pub async_interrupt_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        u8,
        UIntN,
        crate::abi::types::Bool,
        *mut u8,
        UIntN,
        UIntN,
        *mut c_void,
        Option<crate::protocols::usb::usb_io::protocol::EfiAsyncUsbTransferCallback>,
        *mut c_void,
    ) -> EfiStatus,
    /// Sync interrupt.
    pub sync_interrupt_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        u8,
        UIntN,
        *mut c_void,
        *mut UIntN,
        *mut u8,
        UIntN,
        *mut c_void,
        *mut u32,
    ) -> EfiStatus,
    /// Isochronous.
    pub isochronous_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        u8,
        UIntN,
        u8,
        *mut *mut c_void,
        UIntN,
        *mut c_void,
        *mut u32,
    ) -> EfiStatus,
    /// Async isochronous.
    pub async_isochronous_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsb2HcProtocol,
        u8,
        u8,
        u8,
        UIntN,
        u8,
        *mut *mut c_void,
        UIntN,
        *mut c_void,
        Option<crate::protocols::usb::usb_io::protocol::EfiAsyncUsbTransferCallback>,
        *mut c_void,
    ) -> EfiStatus,
    /// Get root hub status.
    pub get_root_hub_port_status: EfiUsb2HcGetRootHubPortStatus,
    /// Set port feature.
    pub set_root_hub_port_feature:
        unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, u8, u32) -> EfiStatus,
    /// Clear port feature.
    pub clear_root_hub_port_feature:
        unsafe extern "efiapi" fn(*mut EfiUsb2HcProtocol, u8, u32) -> EfiStatus,
    /// Major revision.
    pub major_revision: u16,
    /// Minor revision.
    pub minor_revision: u16,
}

const _: () = {
    assert!(
        core::mem::size_of::<EfiUsb2HcProtocol>()
            == if core::mem::size_of::<usize>() == 8 {
                112
            } else {
                56
            }
    );
};

/// A typed USB 2 host controller view.
pub struct Usb2Hc<'a> {
    protocol: &'a EfiUsb2HcProtocol,
}

impl<'a> Usb2Hc<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiUsb2HcProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiUsb2HcProtocol {
        self.protocol
    }

    /// Gets root hub port status.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// status cannot be retrieved.
    pub fn get_root_hub_port_status(&self, port: u8) -> Result<UsbPortStatus, EfiStatus> {
        let mut status = UsbPortStatus {
            port_status: 0,
            port_change_status: 0,
        };
        // SAFETY: The stack local is valid for the out parameter.
        let result = unsafe {
            (self.protocol.get_root_hub_port_status)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                port,
                &raw mut status,
            )
        };
        if result.is_success() {
            Ok(status)
        } else {
            Err(result)
        }
    }

    /// Resets the controller.
    #[must_use]
    pub fn reset(&self, attributes: u16) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.reset)(core::ptr::from_ref(self.protocol).cast_mut(), attributes) }
    }
}

impl core::fmt::Debug for Usb2Hc<'_> {
    /// Formats without dereferencing firmware
    /// state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Usb2Hc")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
