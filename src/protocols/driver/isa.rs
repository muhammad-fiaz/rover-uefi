//! # ISA Host Controller Protocol
//!
//! Provides registration and aperture decoding for legacy ISA and `LPC` (Low Pin Count)
//! devices (such as Super I/O chipsets, legacy UARTs, PS/2 controllers, and parallel ports)
//! defined in the UEFI Platform Initialization (PI) Specification (Vol 5).

use crate::base::{EfiGuid, EfiStatus};

/// EFI ISA Host Controller Protocol GUID.
pub const EFI_ISA_HC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBCDA_F080,
    0x1BDE,
    0x4E22,
    [0xAE, 0x6A, 0x43, 0x54, 0x1E, 0x12, 0x8E, 0xC4],
);

/// EFI ISA Host Controller Service Binding Protocol GUID.
pub const EFI_ISA_HC_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xFAD7_933A,
    0x6C21,
    0x4234,
    [0xA4, 0x34, 0x0A, 0x8A, 0x0D, 0x2B, 0x07, 0x81],
);

/// Function prototype to open an I/O aperture in the ISA Host Controller.
pub type EfiIsaHcOpenIo = unsafe extern "efiapi" fn(
    this: *const EfiIsaHcProtocol,
    io_address: u16,
    io_length: u16,
    io_aperture_handle: *mut u64,
) -> EfiStatus;

/// Function prototype to close a previously opened I/O aperture.
pub type EfiIsaHcCloseIo =
    unsafe extern "efiapi" fn(this: *const EfiIsaHcProtocol, io_aperture_handle: u64) -> EfiStatus;

/// EFI ISA Host Controller Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiIsaHcProtocol {
    /// Version of this protocol.
    pub version: u32,
    /// Opens an I/O aperture for positively-decoded ISA devices.
    pub open_io_aperture: Option<EfiIsaHcOpenIo>,
    /// Closes an I/O aperture.
    pub close_io_aperture: Option<EfiIsaHcCloseIo>,
}

/// Safe wrapper around [`EfiIsaHcProtocol`].
pub struct IsaHostController<'a> {
    proto: &'a EfiIsaHcProtocol,
}

impl<'a> IsaHostController<'a> {
    /// Wraps a reference to [`EfiIsaHcProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiIsaHcProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiIsaHcProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiIsaHcProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns the ISA host controller protocol version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.proto.version
    }

    /// Opens an I/O aperture for `io_address..io_address + io_length`.
    ///
    /// Returns the opened aperture handle on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if opening aperture fails or function is missing.
    pub fn open_io_aperture(&self, io_address: u16, io_length: u16) -> Result<u64, EfiStatus> {
        let Some(func) = self.proto.open_io_aperture else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let mut handle = 0u64;
        let status = unsafe {
            func(
                self.proto,
                io_address,
                io_length,
                core::ptr::from_mut(&mut handle),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Closes a previously opened I/O aperture handle.
    ///
    /// # Errors
    ///
    /// Returns an error status if closing aperture fails or function is missing.
    pub fn close_io_aperture(&self, aperture_handle: u64) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.close_io_aperture else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, aperture_handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
