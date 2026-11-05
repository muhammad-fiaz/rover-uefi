// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Management Mode System Table (MMST).

use core::mem::size_of;
use rover_uefi::protocols::firmware::{
    EFI_MM_SYSTEM_TABLE_GUID, EFI_SMM_SYSTEM_TABLE2_GUID, EfiMmSystemTable,
    MM_SYSTEM_TABLE_REVISION, MM_SYSTEM_TABLE_SIGNATURE,
};

#[test]
fn mmst_constants_and_guids() {
    assert_eq!(MM_SYSTEM_TABLE_SIGNATURE, 0x5453_4D4D);
    assert_eq!(MM_SYSTEM_TABLE_REVISION, (1 << 16) | 0x0050);
    assert!(!EFI_MM_SYSTEM_TABLE_GUID.is_zero());
    assert_eq!(EFI_MM_SYSTEM_TABLE_GUID, EFI_SMM_SYSTEM_TABLE2_GUID);
}

#[test]
fn mmst_layout() {
    assert!(size_of::<EfiMmSystemTable>() > 100);
}
