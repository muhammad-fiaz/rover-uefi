// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Pre-EFI Initialization (PEI) services and interfaces.

use core::mem::size_of;
use rover_uefi::protocols::firmware::{
    BOOT_ASSUMING_NO_CONFIGURATION_CHANGES, BOOT_IN_RECOVERY_MODE, BOOT_ON_FLASH_UPDATE,
    BOOT_ON_S3_RESUME, BOOT_WITH_DEFAULT_SETTINGS, BOOT_WITH_FULL_CONFIGURATION,
    BOOT_WITH_MINIMAL_CONFIGURATION, EFI_PEI_DECOMPRESS_PPI_GUID,
    EFI_PEI_END_OF_PEI_PHASE_PPI_GUID, EFI_PEI_PPI_DESCRIPTOR_NOTIFY_CALLBACK,
    EFI_PEI_PPI_DESCRIPTOR_NOTIFY_DISPATCH, EFI_PEI_PPI_DESCRIPTOR_PPI,
    EFI_PEI_PPI_DESCRIPTOR_TERMINATE_LIST, EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID,
    EFI_PEI_SECURITY2_PPI_GUID, EFI_PEI_STALL_PPI_GUID, EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID,
    EFI_PEI_TEMPORARY_RAM_SUPPORT_PPI_GUID, EfiPeiNotifyDescriptor, EfiPeiPpiDescriptor,
    EfiPeiServices, PEI_SERVICES_REVISION, PEI_SERVICES_SIGNATURE,
};

#[test]
fn pei_constants_and_guids() {
    assert_eq!(PEI_SERVICES_SIGNATURE, 0x5652_4553_2049_4550);
    assert_eq!(PEI_SERVICES_REVISION, (1 << 16) | 0x0050);

    assert_eq!(EFI_PEI_PPI_DESCRIPTOR_PPI, 0x10);
    assert_eq!(EFI_PEI_PPI_DESCRIPTOR_NOTIFY_CALLBACK, 0x20);
    assert_eq!(EFI_PEI_PPI_DESCRIPTOR_NOTIFY_DISPATCH, 0x40);
    assert_eq!(EFI_PEI_PPI_DESCRIPTOR_TERMINATE_LIST, 0x8000_0000);

    assert_eq!(BOOT_WITH_FULL_CONFIGURATION, 0);
    assert_eq!(BOOT_WITH_MINIMAL_CONFIGURATION, 1);
    assert_eq!(BOOT_ASSUMING_NO_CONFIGURATION_CHANGES, 2);
    assert_eq!(BOOT_WITH_DEFAULT_SETTINGS, 4);
    assert_eq!(BOOT_ON_S3_RESUME, 0x11);
    assert_eq!(BOOT_ON_FLASH_UPDATE, 0x12);
    assert_eq!(BOOT_IN_RECOVERY_MODE, 0x20);

    assert!(!EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID.is_zero());
    assert!(!EFI_PEI_END_OF_PEI_PHASE_PPI_GUID.is_zero());
    assert!(!EFI_PEI_STALL_PPI_GUID.is_zero());
    assert!(!EFI_PEI_TEMPORARY_RAM_SUPPORT_PPI_GUID.is_zero());
    assert!(!EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID.is_zero());
    assert!(!EFI_PEI_SECURITY2_PPI_GUID.is_zero());
    assert!(!EFI_PEI_DECOMPRESS_PPI_GUID.is_zero());
}

#[test]
fn pei_descriptors_and_table_layout() {
    assert!(size_of::<EfiPeiPpiDescriptor>() >= 12);
    assert!(size_of::<EfiPeiNotifyDescriptor>() >= 12);
    assert!(size_of::<EfiPeiServices>() > 100);
}
