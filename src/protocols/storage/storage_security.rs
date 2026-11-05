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
    0x0BF2,
    0x4186,
    [0x82, 0x61, 0x00, 0x12, 0xDD, 0xD2, 0xEE, 0x70],
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

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiStorageSecurityCommandProtocol {
        self.protocol
    }
}
