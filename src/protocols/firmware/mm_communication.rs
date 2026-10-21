//! # Management Mode (MM) Communication Protocols
//!
//! Declarations for `EFI_MM_COMMUNICATION_PROTOCOL` and `EFI_MM_COMMUNICATION2_PROTOCOL`
//! according to the UEFI Platform Initialization (PI) Specification.
//!
//! Provides communication between DXE/UEFI drivers and MMI/SMI handlers executing in MM/SMM.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// PI MM Communication Protocol GUID: `{c68ed8e2-9dc6-4cbd-9d94-db65acc5c332}`.
pub const MM_COMMUNICATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xc68e_d8e2,
    0x9dc6,
    0x4cbd,
    [0x9d, 0x94, 0xdb, 0x65, 0xac, 0xc5, 0xc3, 0x32],
);

/// PI MM Communication2 Protocol GUID: `{378daedc-f06b-4446-8314-40ab933c87a3}`.
pub const MM_COMMUNICATION2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x378d_aedc,
    0xf06b,
    0x4446,
    [0x83, 0x14, 0x40, 0xab, 0x93, 0x3c, 0x87, 0xa3],
);

/// Header beginning every MM communication message buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct EfiMmCommunicateHeader {
    /// Disambiguates message format and target handler.
    pub header_guid: EfiGuid,
    /// Size of message data payload in bytes (does not include header size).
    pub message_length: usize,
}

/// Function pointer for `EFI_MM_COMMUNICATE`.
pub type EfiMmCommunicate = unsafe extern "efiapi" fn(
    this: *const EfiMmCommunicationProtocol,
    comm_buffer: *mut c_void,
    comm_size: *mut usize,
) -> EfiStatus;

/// `EFI_MM_COMMUNICATION_PROTOCOL` interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMmCommunicationProtocol {
    /// Sends a message buffer to MMRAM handler.
    pub communicate: EfiMmCommunicate,
}

/// Function pointer for `EFI_MM_COMMUNICATE2`.
pub type EfiMmCommunicate2 = unsafe extern "efiapi" fn(
    this: *const EfiMmCommunication2Protocol,
    comm_buffer_physical: *mut c_void,
    comm_buffer_virtual: *mut c_void,
    comm_size: *mut usize,
) -> EfiStatus;

/// `EFI_MM_COMMUNICATION2_PROTOCOL` interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMmCommunication2Protocol {
    /// Sends physical and virtual message buffers to MM handler.
    pub communicate: EfiMmCommunicate2,
}
