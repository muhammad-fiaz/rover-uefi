//! # Platform Boot Manager Protocol
//!
//! Provides the protocol interface to customize and refresh auto-enumerated platform
//! boot options, NV boot options, and platform boot policies during the BDS phase.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::types::EfiDevicePath;

/// Platform Boot Manager Protocol GUID: `{aa17add4-756c-460d-94b8-4388d7fb3e59}`.
pub const PLATFORM_BOOT_MANAGER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAA17_ADD4,
    0x756C,
    0x460D,
    [0x94, 0xB8, 0x43, 0x88, 0xD7, 0xFB, 0x3E, 0x59],
);

/// Platform Boot Manager Protocol Revision (1.0).
pub const PLATFORM_BOOT_MANAGER_PROTOCOL_REVISION: u64 = 0x0000_0001;

/// Type of boot manager load option.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BootManagerLoadOptionType {
    /// UEFI driver load option (`Driver####`).
    Driver = 0,
    /// System preparation load option (`SysPrep####`).
    SysPrep = 1,
    /// Boot load option (`Boot####`).
    Boot = 2,
    /// Platform recovery load option (`PlatformRecovery####`).
    PlatformRecovery = 3,
    /// Maximum option delimiter.
    Max = 4,
}

/// Load option attributes defined in UEFI Specification.
pub mod load_option_attributes {
    /// Load option is active and will be considered during boot processing.
    pub const LOAD_OPTION_ACTIVE: u32 = 0x0000_0001;
    /// Driver load option requires reconnecting all controllers after dispatch.
    pub const LOAD_OPTION_FORCE_RECONNECT: u32 = 0x0000_0002;
    /// Option is hidden from standard user interfaces.
    pub const LOAD_OPTION_HIDDEN: u32 = 0x0000_0008;
    /// Mask isolating category bits.
    pub const LOAD_OPTION_CATEGORY_MASK: u32 = 0x0000_1F00;
    /// Standard OS boot target.
    pub const LOAD_OPTION_CATEGORY_BOOT: u32 = 0x0000_0000;
    /// Interactive firmware application / shell.
    pub const LOAD_OPTION_CATEGORY_APP: u32 = 0x0000_0100;
}

/// Low-level load option representation used by the Boot Manager.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiBootManagerLoadOption {
    /// Option index number (e.g. 0x0001 for `Boot0001`).
    pub option_number: UIntN,
    /// Type of load option (`Boot`, `Driver`, `SysPrep`, `PlatformRecovery`).
    pub option_type: BootManagerLoadOptionType,
    /// Option attributes bitmask (`LOAD_OPTION_ACTIVE`, etc.).
    pub attributes: u32,
    /// Null-terminated UTF-16 description string.
    pub description: *const u16,
    /// Device path to the boot image.
    pub file_path: *const EfiDevicePath,
    /// Optional binary data passed to the loaded image.
    pub optional_data: *const u8,
    /// Size of `optional_data` in bytes.
    pub optional_data_size: u32,
    /// Vendor GUID for vendor-specific option options.
    pub vendor_guid: EfiGuid,
    /// Execution status from boot attempt.
    pub status: EfiStatus,
}

/// Function pointer prototype for `REFRESH_ALL_BOOT_OPTIONS`.
pub type PlatformBootManagerRefreshAllBootOptions = unsafe extern "efiapi" fn(
    boot_options: *const EfiBootManagerLoadOption,
    boot_options_count: UIntN,
    updated_boot_options: *mut *mut EfiBootManagerLoadOption,
    updated_boot_options_count: *mut UIntN,
) -> EfiStatus;

/// Low-level Platform Boot Manager Protocol table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPlatformBootManagerProtocol {
    /// Protocol revision (`PLATFORM_BOOT_MANAGER_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Customizes and refreshes all boot options.
    pub refresh_all_boot_options: PlatformBootManagerRefreshAllBootOptions,
}

/// Safe client wrapper around `EfiPlatformBootManagerProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PlatformBootManager {
    proto: *const EfiPlatformBootManagerProtocol,
}

impl PlatformBootManager {
    /// Wraps a raw pointer to `EfiPlatformBootManagerProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPlatformBootManagerProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        unsafe { (*self.proto).revision }
    }

    /// Refreshes and customizes platform boot options based on enumerated options.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    /// Returns `EfiStatus::UNSUPPORTED` if platform overrides are unsupported.
    /// Returns `EfiStatus::INVALID_PARAMETER` if inputs are invalid.
    pub fn refresh_all_boot_options(
        &self,
        boot_options: &[EfiBootManagerLoadOption],
    ) -> Result<(*mut EfiBootManagerLoadOption, usize), EfiStatus> {
        let mut updated: *mut EfiBootManagerLoadOption = core::ptr::null_mut();
        let mut count: UIntN = 0;

        let status = unsafe {
            ((*self.proto).refresh_all_boot_options)(
                boot_options.as_ptr(),
                boot_options.len(),
                core::ptr::addr_of_mut!(updated),
                core::ptr::addr_of_mut!(count),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((updated, count))
        } else {
            Err(status)
        }
    }
}
