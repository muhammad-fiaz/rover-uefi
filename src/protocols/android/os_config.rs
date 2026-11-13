//! # Android OS Configuration Protocol and Client API
//!
//! Provides services for kernel `bootconfig` parameter fixups, dynamic Device Tree
//! (`DTB`) and overlay (`DTBO`) selection, and Flattened Image Tree (`FIT`) configuration
//! filtering during boot of Android and embedded Linux platforms.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// `GBL` EFI OS Configuration Protocol GUID.
pub const GBL_EFI_OS_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xDDA0_D135,
    0xAA5B,
    0x42FF,
    [0x85, 0xAC, 0xE3, 0xAD, 0x6E, 0xFB, 0x46, 0x19],
);

/// `GBL` EFI OS Configuration Protocol Revision (1.0).
pub const GBL_EFI_OS_CONFIGURATION_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Device tree types recognized by `GBL`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiDeviceTreeType {
    /// Base flattened device tree (`DTB`).
    DeviceTree = 0,
    /// Device tree overlay (`DTBO`).
    Overlay = 1,
    /// Protected virtual machine device tree overlay (`PVM` DA).
    PvmDaOverlay = 2,
}

/// Device tree sources.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiDeviceTreeSource {
    /// Extracted from `boot` partition image.
    Boot = 0,
    /// Extracted from `vendor_boot` partition image.
    VendorBoot = 1,
    /// Extracted from dedicated `dtbo` partition.
    Dtbo = 2,
    /// Extracted from dedicated `dtb` partition.
    Dtb = 3,
}

/// Metadata describing a candidate device tree.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiDeviceTreeMetadata {
    /// Origin partition source (`GblEfiDeviceTreeSource`).
    pub source: u32,
    /// Tree structure type (`GblEfiDeviceTreeType`).
    pub tree_type: u32,
    /// Hardware platform identifier.
    pub id: u32,
    /// Hardware revision identifier.
    pub rev: u32,
    /// Custom metadata buffer size.
    pub custom_size: usize,
    /// Pointer to custom metadata bytes.
    pub custom: *const u8,
}

/// Verified device tree entry for platform filtering.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiVerifiedDeviceTree {
    /// Associated metadata.
    pub metadata: GblEfiDeviceTreeMetadata,
    /// 8-byte aligned device tree blob pointer.
    pub device_tree: *const c_void,
    /// Non-zero if selected by firmware for merging and boot.
    pub selected: bool,
}

/// Function pointer to fix up kernel `bootconfig` parameters.
pub type GblEfiFixupBootConfig = unsafe extern "efiapi" fn(
    this: *mut GblEfiOsConfigurationProtocol,
    boot_config_size: usize,
    boot_config: *const u8,
    fixup_buffer_size: *mut usize,
    fixup: *mut u8,
) -> EfiStatus;

/// Function pointer to select matching device trees and overlays.
pub type GblEfiSelectDeviceTrees = unsafe extern "efiapi" fn(
    this: *mut GblEfiOsConfigurationProtocol,
    num_device_trees: usize,
    device_trees: *mut GblEfiVerifiedDeviceTree,
) -> EfiStatus;

/// Function pointer to select `FIT` configuration node offset.
pub type GblEfiSelectFitConfiguration = unsafe extern "efiapi" fn(
    this: *mut GblEfiOsConfigurationProtocol,
    fit_size: usize,
    fit: *const u8,
    metadata_size: usize,
    metadata: *const u8,
    selected_configuration_offset: *mut usize,
) -> EfiStatus;

/// Google Bootloader OS Configuration Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiOsConfigurationProtocol {
    /// Protocol revision (`GBL_EFI_OS_CONFIGURATION_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Augment or patch kernel bootconfig parameters.
    pub fixup_boot_config: Option<GblEfiFixupBootConfig>,
    /// Filter and select device tree and overlay images.
    pub select_device_trees: Option<GblEfiSelectDeviceTrees>,
    /// Select target configuration within flattened image tree (`FIT`).
    pub select_fit_configuration: Option<GblEfiSelectFitConfiguration>,
}

/// Safe client wrapper for `GBL` OS Configuration.
#[derive(Debug)]
pub struct OsConfiguration<'a> {
    protocol: &'a mut GblEfiOsConfigurationProtocol,
}

impl<'a> OsConfiguration<'a> {
    /// Create a new safe view of the OS configuration protocol.
    #[must_use]
    pub fn new(protocol: &'a mut GblEfiOsConfigurationProtocol) -> Self {
        Self { protocol }
    }

    /// Select matching device trees from candidate slice.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn select_device_trees(
        &mut self,
        device_trees: &mut [GblEfiVerifiedDeviceTree],
    ) -> Result<(), EfiStatus> {
        if let Some(select_fn) = self.protocol.select_device_trees {
            let status =
                unsafe { select_fn(self.protocol, device_trees.len(), device_trees.as_mut_ptr()) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
