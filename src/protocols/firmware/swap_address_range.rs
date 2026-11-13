//! # Swap Address Range Protocol
//!
//! Provides hardware and software abstraction for swapping the primary boot block
//! and backup block of firmware volumes (`TopSwap`), ensuring fail-safe firmware
//! updates, recovery transitions, and RTC power failure detection.

#![allow(clippy::doc_markdown, clippy::borrow_as_ptr, clippy::ref_as_ptr)]

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// Swap Address Range Protocol GUID.
///
/// `{1259f60d-b754-468e-a789-4db85d55e87e}`
pub const SWAP_ADDRESS_RANGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1259_F60D,
    0xB754,
    0x468E,
    [0xA7, 0x89, 0x4D, 0xB8, 0x5D, 0x55, 0xE8, 0x7E],
);

/// SMM Swap Address Range Protocol GUID.
///
/// `{67c4f112-3385-4e55-9c5b-c05b717c4228}`
pub const SMM_SWAP_ADDRESS_RANGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x67C4_F112,
    0x3385,
    0x4E55,
    [0x9C, 0x5B, 0xC0, 0x5B, 0x71, 0x7C, 0x42, 0x28],
);

/// Lock capability flags for swap operations.
pub mod swap_lock {
    /// Swapping does not support locking.
    pub const UNSUPPORTED: u8 = 0;
    /// Software-controlled lock mechanism.
    pub const SOFTWARE: u8 = 1;
    /// Hardware register/strapping lock mechanism.
    pub const HARDWARE: u8 = 2;
}

/// Address range locations of boot and backup blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapRangeLocation {
    /// Base physical address of current boot block.
    pub boot_block_base: EfiPhysicalAddress,
    /// Size in bytes of current boot block.
    pub boot_block_size: usize,
    /// Base physical address of current backup block.
    pub backup_block_base: EfiPhysicalAddress,
    /// Size in bytes of current backup block.
    pub backup_block_size: usize,
}

/// Raw function pointer for querying block locations.
pub type EfiGetRangeLocation = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    boot_block_base: *mut EfiPhysicalAddress,
    boot_block_size: *mut usize,
    backup_block_base: *mut EfiPhysicalAddress,
    backup_block_size: *mut usize,
) -> EfiStatus;

/// Raw function pointer for querying swap state.
pub type EfiGetSwapState = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    swap_state: *mut Bool,
) -> EfiStatus;

/// Raw function pointer for setting swap state.
pub type EfiSetSwapState = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    new_swap_state: Bool,
) -> EfiStatus;

/// Raw function pointer for querying RTC power status.
pub type EfiGetRtcPowerStatus = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    rtc_power_failed: *mut Bool,
) -> EfiStatus;

/// Raw function pointer for querying lock capability.
pub type EfiGetSwapLockCapability = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    lock_capability: *mut u8,
) -> EfiStatus;

/// Raw function pointer for setting swap lock state.
pub type EfiSetSwapLock = unsafe extern "efiapi" fn(
    this: *mut EfiSwapAddressRangeProtocol,
    lock_capability: u8,
    new_lock_state: Bool,
) -> EfiStatus;

/// EFI Swap Address Range Protocol interface structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiSwapAddressRangeProtocol {
    /// Get base physical addresses and sizes of boot and backup blocks.
    pub get_range_location: EfiGetRangeLocation,
    /// Query whether boot and backup blocks are currently swapped.
    pub get_swap_state: EfiGetSwapState,
    /// Perform or revert block swapping.
    pub set_swap_state: EfiSetSwapState,
    /// Check whether RTC battery/power failed.
    pub get_rtc_power_status: EfiGetRtcPowerStatus,
    /// Retrieve supported lock mechanisms.
    pub get_swap_lock_capability: EfiGetSwapLockCapability,
    /// Acquire or release designated swap locks.
    pub set_swap_lock: EfiSetSwapLock,
}

/// High-level safe wrapper around [`EfiSwapAddressRangeProtocol`].
pub struct SwapAddressRange<'a> {
    proto: &'a mut EfiSwapAddressRangeProtocol,
}

impl<'a> SwapAddressRange<'a> {
    /// Wraps a mutable reference to [`EfiSwapAddressRangeProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiSwapAddressRangeProtocol) -> Self {
        Self { proto }
    }

    /// Queries the physical address ranges of boot and backup blocks.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on hardware or platform failure.
    pub fn get_range_location(&mut self) -> Result<SwapRangeLocation, EfiStatus> {
        let mut boot_base = EfiPhysicalAddress::NULL;
        let mut boot_size: usize = 0;
        let mut backup_base = EfiPhysicalAddress::NULL;
        let mut backup_size: usize = 0;

        let status = unsafe {
            (self.proto.get_range_location)(
                self.proto,
                &mut boot_base,
                &mut boot_size,
                &mut backup_base,
                &mut backup_size,
            )
        };

        if status.is_error() {
            Err(status)
        } else {
            Ok(SwapRangeLocation {
                boot_block_base: boot_base,
                boot_block_size: boot_size,
                backup_block_base: backup_base,
                backup_block_size: backup_size,
            })
        }
    }

    /// Queries whether the boot block and backup block are currently swapped.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on platform failure.
    pub fn is_swapped(&mut self) -> Result<bool, EfiStatus> {
        let mut state: Bool = 0;
        let status = unsafe { (self.proto.get_swap_state)(self.proto, &mut state) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(state != 0)
        }
    }

    /// Swaps the boot block and backup block (`true`) or restores original mapping (`false`).
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if the hardware swap operation was aborted or denied.
    pub fn set_swapped(&mut self, swap: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_swap_state)(self.proto, u8::from(swap)) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Checks if a Real-Time Clock (RTC) power failure occurred.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on hardware failure.
    pub fn is_rtc_power_failed(&mut self) -> Result<bool, EfiStatus> {
        let mut failed: Bool = 0;
        let status = unsafe { (self.proto.get_rtc_power_status)(self.proto, &mut failed) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(failed != 0)
        }
    }

    /// Queries supported swap lock methods (`UNSUPPORTED`, `SOFTWARE`, `HARDWARE`).
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn get_lock_capability(&mut self) -> Result<u8, EfiStatus> {
        let mut cap: u8 = 0;
        let status = unsafe { (self.proto.get_swap_lock_capability)(self.proto, &mut cap) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(cap)
        }
    }

    /// Acquires (`true`) or releases (`false`) the specified swap lock.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if the lock cannot be configured.
    pub fn set_lock(&mut self, lock_cap: u8, acquire: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_swap_lock)(self.proto, lock_cap, u8::from(acquire)) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
