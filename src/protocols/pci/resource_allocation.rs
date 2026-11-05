//! # PCI Host Bridge Resource Allocation and Hot Plug Protocols
//!
//! Defined by the UEFI Specification Section 14.2: PCI Host Bridge Resource Allocation Protocol
//! and Section 14.3: PCI Hot Plug Initialization Protocol.
//!
//! Governs bus numbering, MMIO and I/O aperture assignment, and ACPI/hot-plug controller
//! initialization across PCI root bridges.

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// The `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID`.
pub const EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCF96_6496,
    0x11B3,
    0x48F2,
    [0xAE, 0x77, 0xCE, 0xDE, 0xA2, 0x76, 0x93, 0x75],
);

/// The `EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID`.
pub const EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAA6C_88E2,
    0x221F,
    0x477E,
    [0x98, 0x86, 0x86, 0x07, 0x4C, 0xE7, 0x64, 0x10],
);

/// Phase of the PCI bus scanning and resource allocation state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiPciHostBridgeResourceAllocationPhase {
    /// Reset host bridge and root bridges.
    NotifyBeforeBusEnumeration = 0,
    /// Bus numbering and sizing complete.
    NotifyAfterBusEnumeration = 1,
    /// Resource allocations have been programed into devices.
    NotifyBeforeAllocateResources = 2,
    /// Resources assigned and host bridge apertures configured.
    NotifyAfterAllocateResources = 3,
    /// Maximum phase count.
    MaxPciHostBridgeResourceAllocationPhase = 4,
}

// Allocation attributes
/// Root bridge supports combining bus numbers.
pub const EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM: u64 = 0x0000_0001;
/// Root bridge requires memory aperture aligned to power of two.
pub const EFI_PCI_HOST_BRIDGE_MEM64_DECODE: u64 = 0x0000_0002;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_NOTIFY_PHASE`.
pub type EfiPciHostBridgeResourceAllocationNotifyPhase = unsafe extern "efiapi" fn(
    this: *const EfiPciHostBridgeResourceAllocationProtocol,
    phase: EfiPciHostBridgeResourceAllocationPhase,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_GET_NEXT_ROOT_BRIDGE`.
pub type EfiPciHostBridgeResourceAllocationGetNextRootBridge =
    unsafe extern "efiapi" fn(
        this: *const EfiPciHostBridgeResourceAllocationProtocol,
        root_bridge_handle: *mut EfiHandle,
    ) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_GET_ATTRIBUTES`.
pub type EfiPciHostBridgeResourceAllocationGetAttributes = unsafe extern "efiapi" fn(
    this: *const EfiPciHostBridgeResourceAllocationProtocol,
    root_bridge_handle: EfiHandle,
    attributes: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_START_BUS_ENUMERATION`.
pub type EfiPciHostBridgeResourceAllocationStartBusEnumeration =
    unsafe extern "efiapi" fn(
        this: *const EfiPciHostBridgeResourceAllocationProtocol,
        root_bridge_handle: EfiHandle,
        configuration: *mut *mut c_void,
    ) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_SET_BUS_NUMBERS`.
pub type EfiPciHostBridgeResourceAllocationSetBusNumbers = unsafe extern "efiapi" fn(
    this: *const EfiPciHostBridgeResourceAllocationProtocol,
    root_bridge_handle: EfiHandle,
    configuration: *const c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_SUBMIT_RESOURCES`.
pub type EfiPciHostBridgeResourceAllocationSubmitResources = unsafe extern "efiapi" fn(
    this: *const EfiPciHostBridgeResourceAllocationProtocol,
    root_bridge_handle: EfiHandle,
    configuration: *const c_void,
)
    -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_GET_PROPOSED_RESOURCES`.
pub type EfiPciHostBridgeResourceAllocationGetProposedResources =
    unsafe extern "efiapi" fn(
        this: *const EfiPciHostBridgeResourceAllocationProtocol,
        root_bridge_handle: EfiHandle,
        configuration: *mut *mut c_void,
    ) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PREPROCESS_CONTROLLER`.
pub type EfiPciHostBridgeResourceAllocationPreprocessController =
    unsafe extern "efiapi" fn(
        this: *const EfiPciHostBridgeResourceAllocationProtocol,
        root_bridge_handle: EfiHandle,
        pci_address: u64,
        phase: EfiPciHostBridgeResourceAllocationPhase,
    ) -> EfiStatus;

/// The raw ABI structure for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciHostBridgeResourceAllocationProtocol {
    /// Notifies the host bridge of execution phase changes.
    pub notify_phase: EfiPciHostBridgeResourceAllocationNotifyPhase,
    /// Iterates next root bridge handle managed by this host bridge.
    pub get_next_root_bridge: EfiPciHostBridgeResourceAllocationGetNextRootBridge,
    /// Retrieves allocation attributes for a root bridge.
    pub get_alloc_attributes: EfiPciHostBridgeResourceAllocationGetAttributes,
    /// Prepares root bridge for bus scanning.
    pub start_bus_enumeration: EfiPciHostBridgeResourceAllocationStartBusEnumeration,
    /// Sets allocated bus numbers into root bridge.
    pub set_bus_numbers: EfiPciHostBridgeResourceAllocationSetBusNumbers,
    /// Submits requested BAR resource requirements to host bridge.
    pub submit_resources: EfiPciHostBridgeResourceAllocationSubmitResources,
    /// Retrieves proposed memory and I/O windows.
    pub get_proposed_resources: EfiPciHostBridgeResourceAllocationGetProposedResources,
    /// Invoked before and after initialization of specific controllers.
    pub preprocess_controller: EfiPciHostBridgeResourceAllocationPreprocessController,
}

/// Function pointer for `EFI_PCI_HOT_PLUG_INIT_GET_ROOT_HPC_LIST`.
pub type EfiPciHotPlugInitGetRootHpcList = unsafe extern "efiapi" fn(
    this: *const EfiPciHotPlugInitProtocol,
    hpc_count: *mut UIntN,
    hpc_list: *mut *mut EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOT_PLUG_INIT_INITIALIZE_ROOT_HPC`.
pub type EfiPciHotPlugInitInitializeRootHpc = unsafe extern "efiapi" fn(
    this: *const EfiPciHotPlugInitProtocol,
    hpc_handle: EfiHandle,
    hpc_pci_address: u64,
    event: *const c_void,
    hpc_state: *mut Bool,
) -> EfiStatus;

/// The raw ABI structure for `EFI_PCI_HOT_PLUG_INIT_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciHotPlugInitProtocol {
    /// Returns the list of root hot-plug controllers.
    pub get_root_hpc_list: EfiPciHotPlugInitGetRootHpcList,
    /// Initializes a specific root hot plug controller.
    pub initialize_root_hpc: EfiPciHotPlugInitInitializeRootHpc,
    /// Returns private data associated with a hot-plug controller.
    pub get_hpc_user_data: unsafe extern "efiapi" fn(
        this: *const EfiPciHotPlugInitProtocol,
        hpc_handle: EfiHandle,
        user_data: *mut *mut c_void,
    ) -> EfiStatus,
}
