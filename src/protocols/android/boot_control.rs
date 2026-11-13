//! # Android Boot Control Protocol and Client API
//!
//! Provides A/B slot selection, boot target manipulation, remaining boot attempt
//! tracking, and loaded OS metadata handoff according to the Google Bootloader (`GBL`)
//! specification for Android and `ChromiumOS`.

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// `GBL` EFI Boot Control Protocol GUID.
pub const GBL_EFI_BOOT_CONTROL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD382_DB1B,
    0x9AC2,
    0x11F0,
    [0x84, 0xC7, 0x04, 0x7B, 0xCB, 0xA9, 0x60, 0x19],
);

/// `GBL` EFI Boot Control Protocol revision (1.0).
pub const GBL_EFI_BOOT_CONTROL_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Reason why a slot is marked unbootable.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiUnbootableReason {
    /// Unknown reason.
    Unknown = 0,
    /// No remaining retry attempts left.
    NoMoreTries = 1,
    /// Unbootable due to system update.
    SystemUpdate = 2,
    /// User requested unbootable status.
    UserRequested = 3,
    /// Cryptographic verification failure.
    VerificationFailure = 4,
}

/// One-shot boot modes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GblEfiOneShotBootMode {
    /// Normal boot mode.
    None = 0,
    /// One-shot bootloader mode (fastboot).
    Bootloader = 1,
    /// One-shot recovery mode.
    Recovery = 2,
}

/// Android A/B slot descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GblEfiSlotInfo {
    /// Slot suffix as a UTF-8 character (e.g. `'a'` or `'b'`).
    pub suffix: u32,
    /// Reason slot is unbootable (`GblEfiUnbootableReason`).
    pub unbootable_reason: u8,
    /// Priority of the slot (higher values indicate higher priority).
    pub priority: u8,
    /// Number of remaining boot tries before marking unbootable.
    pub remaining_tries: u8,
    /// Non-zero if the slot has booted successfully previously.
    pub successful: u8,
}

/// Loaded OS image buffers passed to firmware.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GblEfiLoadedOs {
    /// Size of kernel buffer in bytes.
    pub kernel_size: usize,
    /// Physical address of loaded kernel.
    pub kernel: EfiPhysicalAddress,
    /// Size of ramdisk buffer in bytes.
    pub ramdisk_size: usize,
    /// Physical address of loaded ramdisk.
    pub ramdisk: EfiPhysicalAddress,
    /// Size of device tree buffer in bytes.
    pub device_tree_size: usize,
    /// Physical address of loaded device tree.
    pub device_tree: EfiPhysicalAddress,
    /// Reserved fields for future ABI expansion.
    pub reserved: [u64; 8],
}

/// Retrieve the number of available slots.
pub type GblEfiBootControlGetSlotCount = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootControlProtocol,
    slot_count: *mut u8,
) -> EfiStatus;

/// Retrieve slot information for the given index.
pub type GblEfiBootControlGetSlotInfo = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootControlProtocol,
    index: u8,
    info: *mut GblEfiSlotInfo,
) -> EfiStatus;

/// Retrieve slot information for the current active slot.
pub type GblEfiBootControlGetCurrentSlot = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootControlProtocol,
    info: *mut GblEfiSlotInfo,
) -> EfiStatus;

/// Set the active slot for subsequent boot attempts.
pub type GblEfiBootControlSetActiveSlot =
    unsafe extern "efiapi" fn(this: *mut GblEfiBootControlProtocol, index: u8) -> EfiStatus;

/// Query one-shot boot mode requested by the OS.
pub type GblEfiBootControlGetOneShotBootMode =
    unsafe extern "efiapi" fn(this: *mut GblEfiBootControlProtocol, mode: *mut u32) -> EfiStatus;

/// Notify firmware of the loaded OS image descriptors.
pub type GblEfiBootControlHandleLoadedOs = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootControlProtocol,
    os: *const GblEfiLoadedOs,
) -> EfiStatus;

/// Google Bootloader (`GBL`) Boot Control Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiBootControlProtocol {
    /// Protocol revision (`GBL_EFI_BOOT_CONTROL_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Query total number of slots.
    pub get_slot_count: Option<GblEfiBootControlGetSlotCount>,
    /// Query slot information by slot index.
    pub get_slot_info: Option<GblEfiBootControlGetSlotInfo>,
    /// Query active slot information.
    pub get_current_slot: Option<GblEfiBootControlGetCurrentSlot>,
    /// Set designated slot active.
    pub set_active_slot: Option<GblEfiBootControlSetActiveSlot>,
    /// Get one-shot boot mode.
    pub get_one_shot_boot_mode: Option<GblEfiBootControlGetOneShotBootMode>,
    /// Handoff loaded OS metadata to firmware.
    pub handle_loaded_os: Option<GblEfiBootControlHandleLoadedOs>,
}

/// Safe client wrapper for the `GBL` Boot Control protocol.
#[derive(Debug)]
pub struct BootControl<'a> {
    protocol: &'a mut GblEfiBootControlProtocol,
}

impl<'a> BootControl<'a> {
    /// Create a new safe view of the Boot Control protocol.
    #[must_use]
    pub fn new(protocol: &'a mut GblEfiBootControlProtocol) -> Self {
        Self { protocol }
    }

    /// Query the number of A/B slots available.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn slot_count(&mut self) -> Result<u8, EfiStatus> {
        if let Some(count_fn) = self.protocol.get_slot_count {
            let mut count = 0u8;
            let status = unsafe { count_fn(self.protocol, &raw mut count) };
            if status == EfiStatus::SUCCESS {
                Ok(count)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Query current active slot descriptor.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn current_slot(&mut self) -> Result<GblEfiSlotInfo, EfiStatus> {
        if let Some(cur_fn) = self.protocol.get_current_slot {
            let mut info = GblEfiSlotInfo::default();
            let status = unsafe { cur_fn(self.protocol, &raw mut info) };
            if status == EfiStatus::SUCCESS {
                Ok(info)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Set designated slot active.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn set_active_slot(&mut self, slot_index: u8) -> Result<(), EfiStatus> {
        if let Some(set_fn) = self.protocol.set_active_slot {
            let status = unsafe { set_fn(self.protocol, slot_index) };
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
