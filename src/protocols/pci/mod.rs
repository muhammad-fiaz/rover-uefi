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
