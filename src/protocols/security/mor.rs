//! # TCG Memory Overwrite Request Control (MOR)
//!
//! Implements the TCG Platform Reset Attack Mitigation Specification and
//! the Secure MOR Lock extension for preventing cold-boot secret extraction.

use crate::base::EfiGuid;

/// TCG Memory Only Reset Control GUID (`MEMORY_ONLY_RESET_CONTROL_GUID`).
pub const MEMORY_ONLY_RESET_CONTROL_GUID: EfiGuid = EfiGuid::new(
    0xE209_39BE,
    0x32D4,
    0x41BE,
    [0xA1, 0x50, 0x89, 0x7F, 0x85, 0xD4, 0x98, 0x29],
);

/// Microsoft Secure MOR Lock GUID (`MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_GUID`).
pub const MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_GUID: EfiGuid = EfiGuid::new(
    0xBB98_3CCF,
    0x151D,
    0x40E1,
    [0xA0, 0x7B, 0x4A, 0x17, 0xBE, 0x16, 0x82, 0x92],
);

/// Memory Overwrite Request variable name in UTF-16.
pub const MEMORY_OVERWRITE_REQUEST_VARIABLE_NAME: &[u16] = &[
    'M' as u16, 'e' as u16, 'm' as u16, 'o' as u16, 'r' as u16, 'y' as u16, 'O' as u16, 'v' as u16,
    'e' as u16, 'r' as u16, 'w' as u16, 'r' as u16, 'i' as u16, 't' as u16, 'e' as u16, 'R' as u16,
    'e' as u16, 'q' as u16, 'u' as u16, 'e' as u16, 's' as u16, 't' as u16, 'C' as u16, 'o' as u16,
    'n' as u16, 't' as u16, 'r' as u16, 'o' as u16, 'l' as u16, 0,
];

/// Memory Overwrite Request Control Lock variable name in UTF-16.
pub const MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_NAME: &[u16] = &[
    'M' as u16, 'e' as u16, 'm' as u16, 'o' as u16, 'r' as u16, 'y' as u16, 'O' as u16, 'v' as u16,
    'e' as u16, 'r' as u16, 'w' as u16, 'r' as u16, 'i' as u16, 't' as u16, 'e' as u16, 'R' as u16,
    'e' as u16, 'q' as u16, 'u' as u16, 'e' as u16, 's' as u16, 't' as u16, 'C' as u16, 'o' as u16,
    'n' as u16, 't' as u16, 'r' as u16, 'o' as u16, 'l' as u16, 'L' as u16, 'o' as u16, 'c' as u16,
    'k' as u16, 0,
];

/// Bit 0: Firmware MUST clear memory on restart when set.
pub const MOR_CLEAR_MEMORY_BIT_MASK: u8 = 0x01;
/// Bit 4: Firmware MUST NOT autodetect clean OS shutdown when set.
pub const MOR_DISABLEAUTODETECT_BIT_MASK: u8 = 0x10;

/// Secure MOR Lock State.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MorLockState {
    /// MOR state is unlocked and can be modified.
    Unlocked = 0,
    /// MOR state is locked without a key (locked until reboot).
    LockedWithoutKey = 1,
    /// MOR state is locked with an 8-byte key.
    LockedWithKey = 2,
}

/// Helper utilities for Memory Overwrite Request bitmask encoding/decoding.
pub struct MemoryOverwriteControl;

impl MemoryOverwriteControl {
    /// Returns whether the `ClearMemory` bit is set in the MOR byte.
    #[must_use]
    pub const fn should_clear_memory(mor_byte: u8) -> bool {
        (mor_byte & MOR_CLEAR_MEMORY_BIT_MASK) != 0
    }

    /// Returns whether the `DisableAutoDetect` bit is set in the MOR byte.
    #[must_use]
    pub const fn disable_autodetect(mor_byte: u8) -> bool {
        (mor_byte & MOR_DISABLEAUTODETECT_BIT_MASK) != 0
    }

    /// Encode a MOR control byte from component flags.
    #[must_use]
    pub const fn encode(clear_memory: bool, disable_autodetect: bool) -> u8 {
        let mut byte = 0u8;
        if clear_memory {
            byte |= MOR_CLEAR_MEMORY_BIT_MASK;
        }
        if disable_autodetect {
            byte |= MOR_DISABLEAUTODETECT_BIT_MASK;
        }
        byte
    }
}
