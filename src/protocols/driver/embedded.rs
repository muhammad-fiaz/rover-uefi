//! # Embedded Platform Device and Peripheral Protocols
//!
//! Provides protocol abstractions for memory-mapped non-PCI embedded devices,
//! external register-based peripheral devices, and simplified USB device-mode
//! peripheral controllers used in embedded bootloaders and mobile platforms.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

// ---------------------------------------------------------------------------
// Protocol GUIDs
// ---------------------------------------------------------------------------

/// Embedded Memory-Mapped Device Protocol GUID: `{bf4b9d10-13ec-43dd-8880-e90b718f27de}`.
pub const EMBEDDED_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBF4B_9D10,
    0x13EC,
    0x43DD,
    [0x88, 0x80, 0xE9, 0x0B, 0x71, 0x8F, 0x27, 0xDE],
);

/// Embedded External Peripheral Device Protocol GUID: `{735f8c64-d696-44d0-bdf2-447fd05a5406}`.
pub const EMBEDDED_EXTERNAL_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x735F_8C64,
    0xD696,
    0x44D0,
    [0xBD, 0xF2, 0x44, 0x7F, 0xD0, 0x5A, 0x54, 0x06],
);

/// Simplified USB Device Mode Protocol GUID: `{021bd2ca-51d2-11e3-8e56-b75417c70b44}`.
pub const USB_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x021B_D2CA,
    0x51D2,
    0x11E3,
    [0x8E, 0x56, 0xB7, 0x54, 0x17, 0xC7, 0x0B, 0x44],
);

// ---------------------------------------------------------------------------
// Embedded Device Protocol
// ---------------------------------------------------------------------------

/// Embedded Memory-Mapped Device Protocol interface.
///
/// Published on handles for memory-mapped devices that exist outside standard
/// PCI root bus enumeration, providing device identification and base address.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmbeddedDeviceProtocol {
    /// Vendor identifier.
    pub vendor_id: u16,
    /// Device identifier.
    pub device_id: u16,
    /// Revision identifier.
    pub revision_id: u16,
    /// Subsystem identifier.
    pub subsystem_id: u16,
    /// Subsystem vendor identifier.
    pub subsystem_vendor_id: u16,
    /// PCI-compatible class code triplet.
    pub class_code: [u8; 3],
    /// Header size.
    pub header_size: u8,
    /// MMIO base address of the device.
    pub base_address: UIntN,
}

// ---------------------------------------------------------------------------
// Embedded External Device Protocol
// ---------------------------------------------------------------------------

/// Prototype to read contiguous external device registers.
pub type EmbeddedExternalDeviceRead = unsafe extern "efiapi" fn(
    this: *mut EmbeddedExternalDeviceProtocol,
    register: UIntN,
    length: UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// Prototype to write contiguous external device registers.
pub type EmbeddedExternalDeviceWrite = unsafe extern "efiapi" fn(
    this: *mut EmbeddedExternalDeviceProtocol,
    register: UIntN,
    length: UIntN,
    buffer: *const c_void,
) -> EfiStatus;

/// Embedded External Peripheral Device Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmbeddedExternalDeviceProtocol {
    /// Reads contiguous external device registers.
    pub read: Option<EmbeddedExternalDeviceRead>,
    /// Writes contiguous external device registers.
    pub write: Option<EmbeddedExternalDeviceWrite>,
}

/// Safe client wrapper around [`EmbeddedExternalDeviceProtocol`].
#[derive(Debug)]
pub struct EmbeddedExternalDevice<'a> {
    proto: &'a mut EmbeddedExternalDeviceProtocol,
}

impl<'a> EmbeddedExternalDevice<'a> {
    /// Creates a safe wrapper around a mutable reference to [`EmbeddedExternalDeviceProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EmbeddedExternalDeviceProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EmbeddedExternalDeviceProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EmbeddedExternalDeviceProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Reads contiguous device registers into the destination buffer.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn read_registers(&mut self, register: usize, buffer: &mut [u8]) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.read else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe {
            func(
                self.proto,
                register,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes contiguous device registers from the source buffer.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn write_registers(&mut self, register: usize, buffer: &[u8]) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.write else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, register, buffer.len(), buffer.as_ptr().cast()) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// USB Device Mode Protocol
// ---------------------------------------------------------------------------

/// Callback invoked when USB device received data from host.
pub type UsbDeviceRxCallback = unsafe extern "efiapi" fn(size: UIntN, buffer: *mut c_void);

/// Callback invoked when USB device completed transmission to host.
pub type UsbDeviceTxCallback = unsafe extern "efiapi" fn(endpoint_index: u8);

/// Starts USB device mode peripheral controller and responds to enumeration.
pub type UsbDeviceStart = unsafe extern "efiapi" fn(
    device_descriptor: *const c_void,
    descriptors: *const *const c_void,
    rx_callback: Option<UsbDeviceRxCallback>,
    tx_callback: Option<UsbDeviceTxCallback>,
) -> EfiStatus;

/// Puts data in the transmit buffer for transmission on the next IN token.
pub type UsbDeviceSend =
    unsafe extern "efiapi" fn(endpoint_index: u8, size: UIntN, buffer: *const c_void) -> EfiStatus;

/// Simplified USB Device Mode Protocol (`USB_DEVICE_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UsbDeviceProtocol {
    /// Restarts USB peripheral controller and responds to enumeration.
    pub start: Option<UsbDeviceStart>,
    /// Sends data on the specified endpoint.
    pub send: Option<UsbDeviceSend>,
}

/// Safe client wrapper around [`UsbDeviceProtocol`].
#[derive(Debug)]
pub struct UsbDevice<'a> {
    proto: &'a mut UsbDeviceProtocol,
}

impl<'a> UsbDevice<'a> {
    /// Creates a safe wrapper around a mutable reference to [`UsbDeviceProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut UsbDeviceProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`UsbDeviceProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut UsbDeviceProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Sends data to the host on the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn send(&mut self, endpoint: u8, data: &[u8]) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.send else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(endpoint, data.len(), data.as_ptr().cast()) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
