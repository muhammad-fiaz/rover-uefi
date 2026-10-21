//! # Intel Firmware Support Package (FSP) 2.x
//!
//! Provides headers, signatures, and interfaces described in the
//! Intel Firmware Support Package External Architecture Specification v2.0 - v2.4.

use crate::base::EfiStatus;

/// Fixed offset of the FSP Info Header in an FSP binary image.
pub const FSP_INFO_HEADER_OFFSET: usize = 0x94;

/// FSP Info Header signature: `'FSPH'` (`0x4850_5346`).
pub const FSP_INFO_HEADER_SIGNATURE: u32 = 0x4850_5346;

/// Image attribute: Graphics support.
pub const IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT: u16 = 1 << 0;
/// Image attribute: Dispatch mode support.
pub const IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT: u16 = 1 << 1;
/// Image attribute: 64-bit mode support.
pub const IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT: u16 = 1 << 2;

/// FSP execution mode: IA32.
pub const FSP_IA32: u8 = 0;
/// FSP execution mode: X64.
pub const FSP_X64: u8 = 1;

/// FSP Information Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspInfoHeader {
    /// Signature ('FSPH').
    pub signature: u32,
    /// Length of the FSP Information Header.
    pub header_length: u32,
    /// Reserved.
    pub reserved1: [u8; 2],
    /// Specification version in BCD format (e.g. 0x24 for v2.4).
    pub spec_version: u8,
    /// Revision of the FSP Information Header.
    pub header_revision: u8,
    /// Revision of the FSP binary.
    pub image_revision: u32,
    /// Image identifier string.
    pub image_id: [u8; 8],
    /// Size of the FSP binary image in bytes.
    pub image_size: u32,
    /// Base address of the FSP binary image.
    pub image_base: u32,
    /// Image attribute flags.
    pub image_attribute: u16,
    /// Component attribute flags.
    pub component_attribute: u16,
    /// Offset to configuration region (UPD).
    pub cfg_region_offset: u32,
    /// Size of configuration region.
    pub cfg_region_size: u32,
    /// Reserved.
    pub reserved2: [u8; 4],
    /// Entry offset for `TempRamInit` API.
    pub temp_ram_init_entry_offset: u32,
    /// Reserved.
    pub reserved3: [u8; 4],
    /// Entry offset for `FspNotifyPhase` API.
    pub notify_phase_entry_offset: u32,
    /// Entry offset for `FspMemoryInit` API.
    pub fsp_memory_init_entry_offset: u32,
    /// Entry offset for `TempRamExit` API.
    pub temp_ram_exit_entry_offset: u32,
    /// Entry offset for `FspSiliconInit` API.
    pub fsp_silicon_init_entry_offset: u32,
    /// Entry offset for `FspMultiPhaseSiInit` API.
    pub fsp_multi_phase_si_init_entry_offset: u32,
}

/// FSP notification phase.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FspPhase {
    /// After PCI enumeration complete.
    AfterPciEnumeration = 0x20,
    /// Ready to boot.
    ReadyToBoot = 0x40,
    /// End of firmware execution.
    EndOfFirmware = 0xF0,
}

/// Notification parameter structure passed to `FspNotifyPhase`.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspNotifyParams {
    /// Notification phase.
    pub phase: u32,
}

/// `TempRamInit` entry point function prototype.
pub type FspTempRamInit = unsafe extern "efiapi" fn(temp_ram_init_params: *const ()) -> EfiStatus;

/// `FspMemoryInit` entry point function prototype.
pub type FspMemoryInit = unsafe extern "efiapi" fn(
    fsp_memory_init_params: *const (),
    hob_list_ptr: *mut *const (),
) -> EfiStatus;

/// `TempRamExit` entry point function prototype.
pub type FspTempRamExit = unsafe extern "efiapi" fn(temp_ram_exit_params: *const ()) -> EfiStatus;

/// `FspSiliconInit` entry point function prototype.
pub type FspSiliconInit =
    unsafe extern "efiapi" fn(fsp_silicon_init_params: *const ()) -> EfiStatus;

/// `FspNotifyPhase` entry point function prototype.
pub type FspNotifyPhase =
    unsafe extern "efiapi" fn(notify_params: *const FspNotifyParams) -> EfiStatus;
