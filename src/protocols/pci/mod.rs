//! # PCI
//!
//! PCI protocol and configuration parsing.

pub mod access;
pub mod bar;
pub mod capabilities;
pub mod class;
pub mod cxl;
pub mod device;
pub mod enumeration;
pub mod guid;
pub mod header;
pub mod resource_allocation;
pub mod types;
pub mod validation;

pub use access::{EfiPciIoProtocol, EfiPciRootBridgeIoProtocol, PciIo, PciRootBridgeIo};
pub use capabilities::{
    Capability, CapabilityIter, ExtendedCapability, ExtendedCapabilityIter, MAX_CAPABILITIES,
    MAX_EXT_CAPABILITIES, PCI_CAP_AF, PCI_CAP_AGP, PCI_CAP_AGP8X, PCI_CAP_BRIDGE_SUBSYS,
    PCI_CAP_CPCI_CRC, PCI_CAP_DEBUG, PCI_CAP_EA, PCI_CAP_EXP, PCI_CAP_HOTPLUG, PCI_CAP_HOTSWAP,
    PCI_CAP_HT, PCI_CAP_MSI, PCI_CAP_MSIX, PCI_CAP_PCIX, PCI_CAP_PM, PCI_CAP_SATA,
    PCI_CAP_SECURE_DEV, PCI_CAP_SLOT_ID, PCI_CAP_VENDOR, PCI_CAP_VPD, PCI_EXP_EXT_CAP_ACS,
    PCI_EXP_EXT_CAP_AER, PCI_EXP_EXT_CAP_ARI, PCI_EXP_EXT_CAP_ATS, PCI_EXP_EXT_CAP_CAC,
    PCI_EXP_EXT_CAP_DLF, PCI_EXP_EXT_CAP_DOE, PCI_EXP_EXT_CAP_DPA, PCI_EXP_EXT_CAP_DPC,
    PCI_EXP_EXT_CAP_DSN, PCI_EXP_EXT_CAP_DVSEC, PCI_EXP_EXT_CAP_IDE, PCI_EXP_EXT_CAP_L1SS,
    PCI_EXP_EXT_CAP_LNR, PCI_EXP_EXT_CAP_LTR, PCI_EXP_EXT_CAP_MCAST, PCI_EXP_EXT_CAP_MFVC,
    PCI_EXP_EXT_CAP_MRIOV, PCI_EXP_EXT_CAP_PASID, PCI_EXP_EXT_CAP_PL_16GT, PCI_EXP_EXT_CAP_PMUX,
    PCI_EXP_EXT_CAP_PRI, PCI_EXP_EXT_CAP_PTM, PCI_EXP_EXT_CAP_PWRB, PCI_EXP_EXT_CAP_RCEC,
    PCI_EXP_EXT_CAP_RCILC, PCI_EXP_EXT_CAP_RCLD, PCI_EXP_EXT_CAP_RCRB, PCI_EXP_EXT_CAP_REBAR,
    PCI_EXP_EXT_CAP_SECPCI, PCI_EXP_EXT_CAP_SRIOV, PCI_EXP_EXT_CAP_TPH, PCI_EXP_EXT_CAP_VC,
    PCI_EXP_EXT_CAP_VC9, PCI_EXP_EXT_CAP_VNDR, capability_at, capability_iter,
    extended_capability_at, extended_capability_iter, find_capability, find_extended_capability,
};
pub use class::*;
pub use cxl::{
    CXL_IO_PROTOCOL_GUID, CxlIoProtocol, CxlIoProtocolDvsecAccess, CxlIoProtocolRegblockAccess,
    EfiCxlIoProtocol,
};
pub use guid::{PCI_IO_GUID, PCI_ROOT_BRIDGE_IO_GUID};
pub use resource_allocation::{
    EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM, EFI_PCI_HOST_BRIDGE_MEM64_DECODE,
    EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID, EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID,
    EfiPciHostBridgeResourceAllocationGetAttributes,
    EfiPciHostBridgeResourceAllocationGetNextRootBridge,
    EfiPciHostBridgeResourceAllocationGetProposedResources,
    EfiPciHostBridgeResourceAllocationNotifyPhase, EfiPciHostBridgeResourceAllocationPhase,
    EfiPciHostBridgeResourceAllocationPreprocessController,
    EfiPciHostBridgeResourceAllocationProtocol, EfiPciHostBridgeResourceAllocationSetBusNumbers,
    EfiPciHostBridgeResourceAllocationStartBusEnumeration,
    EfiPciHostBridgeResourceAllocationSubmitResources, EfiPciHotPlugInitGetRootHpcList,
    EfiPciHotPlugInitInitializeRootHpc, EfiPciHotPlugInitProtocol,
};
