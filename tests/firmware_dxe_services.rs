// SPDX-License-Identifier: Apache-2.0
//! Unit tests for DXE Services Table and GCD structures.

use core::mem::size_of;
use rover_uefi::protocols::firmware::{
    DXE_SERVICES_REVISION, DXE_SERVICES_SIGNATURE, DXE_SERVICES_TABLE_GUID, EfiDxeServices,
    EfiGcdAllocateType, EfiGcdIoSpaceDescriptor, EfiGcdIoType, EfiGcdMemorySpaceDescriptor,
    EfiGcdMemoryType,
};

#[test]
fn dxe_services_constants_and_types() {
    assert_eq!(DXE_SERVICES_SIGNATURE, 0x5652_4553_2045_5844);
    assert_eq!(DXE_SERVICES_REVISION, (1 << 16) | 0x0050);
    assert!(!DXE_SERVICES_TABLE_GUID.is_zero());

    assert_eq!(EfiGcdMemoryType::NonExistent as u32, 0);
    assert_eq!(EfiGcdMemoryType::SystemMemory as u32, 2);
    assert_eq!(EfiGcdMemoryType::MemoryMappedIo as u32, 3);
    assert_eq!(EfiGcdMemoryType::PersistentMemory as u32, 4);

    assert_eq!(EfiGcdAllocateType::AllocateAnySearchBottomUp as u32, 0);
    assert_eq!(EfiGcdAllocateType::AllocateAddress as u32, 2);

    assert_eq!(EfiGcdIoType::Io as u32, 2);
}

#[test]
fn dxe_services_table_layout() {
    assert!(size_of::<EfiGcdMemorySpaceDescriptor>() >= 32);
    assert!(size_of::<EfiGcdIoSpaceDescriptor>() >= 24);
    assert!(size_of::<EfiDxeServices>() > 100);
}
