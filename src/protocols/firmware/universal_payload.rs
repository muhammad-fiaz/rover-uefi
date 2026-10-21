//! # Universal Payload
//!
//! Provides the Universal Payload information and generic header structures
//! defined in the Universal Payload Specification.

use core::ffi::c_void;

/// Universal Payload identifier signature (`'PLDH'`).
pub const UNIVERSAL_PAYLOAD_IDENTIFIER: u32 = 0x4844_4C50;

/// Universal Payload Information Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadInfoHeader {
    /// Identifier ('PLDH').
    pub identifier: u32,
    /// Length of this header in bytes.
    pub header_length: u32,
    /// Specification revision.
    pub spec_revision: u16,
    /// Reserved.
    pub reserved: [u8; 2],
    /// Payload revision.
    pub revision: u32,
    /// Attribute flags.
    pub attribute: u32,
    /// Capability flags.
    pub capability: u32,
    /// Producer ID string (ASCII, space-padded).
    pub producer_id: [u8; 16],
    /// Image ID string (ASCII, space-padded).
    pub image_id: [u8; 16],
}

/// Generic header preceding all Universal Payload HOB structures.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadGenericHeader {
    /// Structure revision.
    pub revision: u8,
    /// Reserved.
    pub reserved: u8,
    /// Structure length including header.
    pub length: u16,
}

/// Universal Payload entry point function prototype.
pub type UniversalPayloadEntryPoint = unsafe extern "efiapi" fn(hob_list: *const c_void);
