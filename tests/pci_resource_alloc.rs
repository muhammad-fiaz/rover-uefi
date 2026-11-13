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
        EfiPciHostBridgeResourceAllocationPhase::BeginEnumeration as u32,
        0
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::BeginBusAllocation as u32,
        1
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::EndBusAllocation as u32,
        2
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::BeginResourceAllocation as u32,
        3
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::AllocateResources as u32,
        4
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::SetResources as u32,
        5
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::FreeResources as u32,
        6
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::EndResourceAllocation as u32,
        7
    );
    assert_eq!(
        EfiPciHostBridgeResourceAllocationPhase::EndEnumeration as u32,
        8
    );

    assert_eq!(EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM, 0x01);
    assert_eq!(EFI_PCI_HOST_BRIDGE_MEM64_DECODE, 0x02);
}
