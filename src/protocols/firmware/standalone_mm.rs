//! # Standalone Management Mode (MM) Infrastructure
//!
//! Provides entry point definitions, GUID-based HOB structures for MMRAM region
//! reservation and multiprocessor information, and MM communication dispatch structures
//! for Standalone MM execution in isolated partitions (e.g. ARM Secure Partitions / FF-A).

use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};

/// HOB GUID for reserving MMRAM memory candidate regions in PEI/SEC:
/// `{0703f912-bf8d-4e2a-be07-ab272525c592}`.
pub const EFI_MM_PEI_MMRAM_MEMORY_RESERVE_GUID: EfiGuid = EfiGuid::new(
    0x0703_F912,
    0xBF8D,
    0x4E2A,
    [0xBE, 0x07, 0xAB, 0x27, 0x25, 0x25, 0xC5, 0x92],
);

/// HOB GUID for passing multiprocessor static information to Standalone MM:
/// `{ba33f15d-4000-45c1-8e88-f91692d457e3}`.
pub const MP_INFORMATION_HOB_GUID: EfiGuid = EfiGuid::new(
    0xBA33_F15D,
    0x4000,
    0x45C1,
    [0x8E, 0x88, 0xF9, 0x16, 0x92, 0xD4, 0x57, 0xE3],
);

/// GUID for MM Firmware Volume dispatch event:
/// `{b65694cc-09e3-4c3b-b5cd-05f44d3cdbff}`.
pub const MM_FV_DISPATCH_GUID: EfiGuid = EfiGuid::new(
    0xB656_94CC,
    0x09E3,
    0x4C3B,
    [0xB5, 0xCD, 0x05, 0xF4, 0x4D, 0x3C, 0xDB, 0xFF],
);

/// Entry point signature for a Standalone MM driver.
pub type MmImageEntryPoint = unsafe extern "efiapi" fn(
    image_handle: EfiHandle,
    mm_system_table: *mut core::ffi::c_void,
) -> EfiStatus;

/// Entry point signature for the Standalone MM foundation core.
pub type MmFoundationEntryPoint = unsafe extern "efiapi" fn(hob_start: *mut core::ffi::c_void);

/// Data payload for `MM_FV_DISPATCH_GUID` communication buffer.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiMmCommunicateFvDispatchData {
    /// Physical address of the firmware volume.
    pub address: EfiPhysicalAddress,
    /// Size of the firmware volume in bytes.
    pub size: u64,
}

/// Communication message wrapper for dispatching firmware volumes into Standalone MM.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiMmCommunicateFvDispatch {
    /// Header GUID matching `MM_FV_DISPATCH_GUID`.
    pub header_guid: EfiGuid,
    /// Message data length in bytes.
    pub message_length: usize,
    /// Dispatch data payload.
    pub data: EfiMmCommunicateFvDispatchData,
}

/// Header for MMRAM reservation HOB descriptor block.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiMmramHobDescriptorBlockHeader {
    /// Number of reserved MMRAM regions described in this HOB.
    pub number_of_mm_reserved_regions: u32,
}

/// Header for multiprocessor information HOB in Standalone MM.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MpInformationHobHeader {
    /// Total number of processors present in the platform.
    pub number_of_processors: u64,
    /// Number of enabled processors available for execution.
    pub number_of_enabled_processors: u64,
}
