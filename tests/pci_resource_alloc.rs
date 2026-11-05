// SPDX-License-Identifier: Apache-2.0
//! Unit tests for PCI Host Bridge Resource Allocation and Hot Plug protocols.

use rover_uefi::protocols::pci::{
    EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM, EFI_PCI_HOST_BRIDGE_MEM64_DECODE,
    EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID, EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID,
    EfiPciHostBridgeResourceAllocationPhase,
};

#[test]
fn pci_resource_allocation_guids_and_phases() {
    assert!(!EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID.is_zero());
    assert!(!EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID.is_zero());

    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::NotifyBeforeBusEnumeration as u32,
        0
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::NotifyAfterBusEnumeration as u32,
        1
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::NotifyBeforeAllocateResources as u32,
        2
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::NotifyAfterAllocateResources as u32,
        3
    );

    assert_eq!(EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM, 0x01);
    assert_eq!(EFI_PCI_HOST_BRIDGE_MEM64_DECODE, 0x02);
}
