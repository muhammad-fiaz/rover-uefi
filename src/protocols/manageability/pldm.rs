//! # Platform Level Data Model (PLDM)
//!
//! Provides the PLDM protocol interface defined by DMTF DSP0240
//! and UEFI Manageability specifications.

use crate::base::{EfiGuid, EfiStatus};

/// The PLDM Protocol `GUID`.
pub const PLDM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6099_7616,
    0xDB70,
    0x4B5F,
    [0x86, 0xA4, 0x09, 0x58, 0xA3, 0x71, 0x47, 0xB4],
);

/// PLDM Message Types.
pub mod pldm_type {
    /// PLDM Base and control commands.
    pub const BASE: u8 = 0x00;
    /// Platform monitoring and control.
    pub const PLATFORM_MONITORING: u8 = 0x02;
    /// BIOS control and configuration.
    pub const BIOS_CONTROL: u8 = 0x03;
    /// FRU data.
    pub const FRU_DATA: u8 = 0x04;
    /// Firmware update.
    pub const FIRMWARE_UPDATE: u8 = 0x05;
    /// Redfish device enablement.
    pub const REDFISH_DEVICE_ENABLEMENT: u8 = 0x06;
    /// OEM type.
    pub const OEM: u8 = 0x3F;
}

/// PLDM Base commands.
pub mod base_cmd {
    /// Set Terminus ID.
    pub const SET_TID: u8 = 0x01;
    /// Get Terminus ID.
    pub const GET_TID: u8 = 0x02;
    /// Get PLDM Version.
    pub const GET_PLDM_VERSION: u8 = 0x03;
    /// Get PLDM Types.
    pub const GET_PLDM_TYPES: u8 = 0x04;
    /// Get PLDM Commands.
    pub const GET_PLDM_COMMANDS: u8 = 0x05;
}

/// Submits a PLDM command byte stream.
pub type EfiPldmSubmitCommand = unsafe extern "efiapi" fn(
    this: *mut EfiPldmProtocol,
    pldm_type: u8,
    command: u8,
    pldm_terminus_source_id: u8,
    pldm_terminus_destination_id: u8,
    request_data: *const u8,
    request_data_size: u32,
    response_data: *mut u8,
    response_data_size: *mut u32,
) -> EfiStatus;

/// The EFI PLDM Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPldmProtocol {
    /// Submits a PLDM command and receives response.
    pub submit_command: EfiPldmSubmitCommand,
}
