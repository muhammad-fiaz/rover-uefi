//! # Storage Security Command Protocol
//!
//! Defined by the UEFI Specification Section 14.5: Storage Security Command Protocol.
//!
//! Provides trusted command transport (e.g. TCG Opal, IEEE 1667) to underlying storage
//! controllers and self-encrypting drives (SED).

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID`.
pub const EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xC88B_0B6D,
    0x0DFC,
    0x49A7,
    [0x9C, 0xB4, 0x49, 0x07, 0x4B, 0x4C, 0x3A, 0x78],
);

/// Function pointer for `EFI_STORAGE_SECURITY_RECEIVE_DATA`.
pub type EfiStorageSecurityReceiveData = unsafe extern "efiapi" fn(
    this: *const EfiStorageSecurityCommandProtocol,
    media_id: u32,
    timeout: u64,
    security_protocol_id: u8,
    security_protocol_specific_data: u16,
    payload_buffer_size: UIntN,
    payload_buffer: *mut c_void,
    payload_transfer_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_STORAGE_SECURITY_SEND_DATA`.
pub type EfiStorageSecuritySendData = unsafe extern "efiapi" fn(
    this: *const EfiStorageSecurityCommandProtocol,
    media_id: u32,
    timeout: u64,
    security_protocol_id: u8,
    security_protocol_specific_data: u16,
    payload_buffer_size: UIntN,
    payload_buffer: *const c_void,
) -> EfiStatus;

/// The raw ABI structure for `EFI_STORAGE_SECURITY_COMMAND_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiStorageSecurityCommandProtocol {
    /// Receives security protocol data from the storage device.
    pub receive_data: EfiStorageSecurityReceiveData,
    /// Sends security protocol data to the storage device.
    pub send_data: EfiStorageSecuritySendData,
}

/// Safe wrapper around `EFI_STORAGE_SECURITY_COMMAND_PROTOCOL`.
pub struct StorageSecurityCommand<'a> {
    protocol: &'a EfiStorageSecurityCommandProtocol,
}

impl<'a> StorageSecurityCommand<'a> {
    /// Creates a new view borrowing the protocol instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiStorageSecurityCommandProtocol) -> Self {
        Self { protocol }
    }

    /// Wraps a raw pointer to [`EfiStorageSecurityCommandProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to a valid and initialized protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiStorageSecurityCommandProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                protocol: unsafe { &*proto },
            })
        }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiStorageSecurityCommandProtocol {
        self.protocol
    }

    /// Receives security protocol data from a storage medium.
    ///
    /// Returns the number of bytes transferred into `payload_buffer`.
    ///
    /// # Errors
    ///
    /// Returns an error status if the security protocol command fails.
    pub fn receive_data(
        &self,
        media_id: u32,
        timeout_100ns: u64,
        security_protocol_id: u8,
        security_protocol_specific_data: u16,
        payload_buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut transfer_size: UIntN = 0;
        let buffer_size = payload_buffer.len() as UIntN;

        let status = unsafe {
            (self.protocol.receive_data)(
                self.protocol,
                media_id,
                timeout_100ns,
                security_protocol_id,
                security_protocol_specific_data,
                buffer_size,
                payload_buffer.as_mut_ptr().cast::<c_void>(),
                core::ptr::from_mut(&mut transfer_size),
            )
        };

        if status == EfiStatus::SUCCESS || status == EfiStatus::WARN_BUFFER_TOO_SMALL {
            Ok(transfer_size as usize)
        } else {
            Err(status)
        }
    }

    /// Sends security protocol data to a storage medium.
    ///
    /// # Errors
    ///
    /// Returns an error status if the security protocol command fails.
    pub fn send_data(
        &self,
        media_id: u32,
        timeout_100ns: u64,
        security_protocol_id: u8,
        security_protocol_specific_data: u16,
        payload_buffer: &[u8],
    ) -> Result<(), EfiStatus> {
        let buffer_size = payload_buffer.len() as UIntN;

        let status = unsafe {
            (self.protocol.send_data)(
                self.protocol,
                media_id,
                timeout_100ns,
                security_protocol_id,
                security_protocol_specific_data,
                buffer_size,
                payload_buffer.as_ptr().cast::<c_void>(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
