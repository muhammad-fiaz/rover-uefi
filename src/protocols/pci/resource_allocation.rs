//! # PCI Host Bridge Resource Allocation, Fixed Resources, Hot Plug, and Platform Protocols
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 5:
//! - Section 14.2: PCI Host Bridge Resource Allocation Protocol
//! - Section 14.3: PCI Hot Plug Initialization Protocol
//! - Section 14.4: PCI Hot Plug Request Protocol
//! - Section 14.5: PCI Platform Protocol
//! - Host Bridge Fixed Resource Protocol for fixed aperture allocations
//!
//! Governs bus numbering, MMIO and I/O aperture assignment, fixed BAR allocations,
//! ACPI/hot-plug controller initialization, and platform Option ROM / policy hooks
//! across PCI root bridges.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiHandle, EfiStatus};
use crate::protocols::device_path::types::EfiDevicePath;
use core::ffi::c_void;

/// The `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID`.
pub const EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCF80_34BE,
    0x6768,
    0x4D8B,
    [0xB7, 0x39, 0x7C, 0xCE, 0x68, 0x3A, 0x9F, 0xBE],
);

/// The `EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID`.
pub const EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAA0E_8BC1,
    0xDABC,
    0x46B0,
    [0xA8, 0x44, 0x37, 0xB8, 0x16, 0x9B, 0x2B, 0xEA],
);

/// The `EFI_PCI_HOTPLUG_REQUEST_PROTOCOL_GUID`.
pub const EFI_PCI_HOTPLUG_REQUEST_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x19CB_87AB,
    0x2CB9,
    0x4665,
    [0x83, 0x60, 0xDD, 0xCF, 0x60, 0x54, 0xF7, 0x9D],
);

/// The `EFI_PCI_PLATFORM_PROTOCOL_GUID`.
pub const EFI_PCI_PLATFORM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x07D7_5280,
    0x27D4,
    0x4D69,
    [0x90, 0xD0, 0x56, 0x43, 0xE2, 0x38, 0xB3, 0x41],
);

/// PCI Host Bridge Fixed Resource Protocol GUID.
pub const PCI_HOST_BRIDGE_FIXED_RESOURCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBB46_5AEA,
    0x5165,
    0x4FDF,
    [0xB4, 0xB0, 0xFD, 0x3A, 0x1D, 0x4C, 0xC6, 0xD8],
);

/// PCI Host Bridge Combine Memory and Prefetchable Memory attribute bit.
pub const EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM: u64 = 0x0000_0001;
/// PCI Host Bridge 64-bit Memory Decoding attribute bit.
pub const EFI_PCI_HOST_BRIDGE_MEM64_DECODE: u64 = 0x0000_0002;

/// Requested resource could be fulfilled.
pub const EFI_RESOURCE_SATISFIED: u64 = 0x0000_0000_0000_0000;
/// Requested resource could not be fulfilled.
pub const EFI_RESOURCE_NOT_SATISFIED: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// Phase of the PCI host bridge enumeration process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiPciHostBridgeResourceAllocationPhase {
    /// Reset host bridge apertures and internal data structures before enumeration.
    BeginEnumeration = 0,
    /// Bus allocation phase is about to begin.
    BeginBusAllocation = 1,
    /// Bus allocation and bus programming phase is complete.
    EndBusAllocation = 2,
    /// Resource allocation phase is about to begin.
    BeginResourceAllocation = 3,
    /// Allocate resources per previously submitted requests for all root bridges.
    AllocateResources = 4,
    /// Program host bridge hardware to decode allocated resources.
    SetResources = 5,
    /// De-allocate previously allocated resources and reset apertures.
    FreeResources = 6,
    /// Resource allocation phase is completed.
    EndResourceAllocation = 7,
    /// Host bridge enumeration is fully completed.
    EndEnumeration = 8,
    /// Maximum enumeration phase delimiter.
    MaxPciHostBridgeEnumerationPhase = 9,
}

/// Notification points for individual PCI controllers during enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiPciControllerResourceAllocationPhase {
    /// Notification before enumerating child bus behind a PCI-PCI bridge.
    BeforeChildBusEnumeration = 0,
    /// Notification before probing BAR registers for every valid function.
    BeforeResourceCollection = 1,
}

/// Execution phase for platform chipset notifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiPciExecutionPhase {
    /// Entry point before host bridge driver notification.
    BeforePciHostBridge = 0,
    /// Exit point after host bridge driver notification.
    AfterPciHostBridge = 1,
    /// Maximum chipset phase delimiter.
    MaximumChipsetPhase = 2,
}

/// Platform policy with respect to ISA and VGA I/O aliasing.
pub mod pci_platform_policy {
    /// Does not set aside ISA or VGA I/O resources.
    pub const RESERVE_NONE_IO_ALIAS: u32 = 0x0000;
    /// Sets aside ISA I/O range and all aliases.
    pub const RESERVE_ISA_IO_ALIAS: u32 = 0x0001;
    /// Sets aside ISA I/O range `0x100..=0x3FF` without aliases.
    pub const RESERVE_ISA_IO_NO_ALIAS: u32 = 0x0002;
    /// Sets aside VGA I/O ranges and all aliases.
    pub const RESERVE_VGA_IO_ALIAS: u32 = 0x0004;
    /// Sets aside VGA I/O ranges without aliases.
    pub const RESERVE_VGA_IO_NO_ALIAS: u32 = 0x0008;
}

/// State of a Hot Plug Controller (HPC).
pub mod hpc_state {
    /// HPC completed initialization but was not enabled in hardware.
    pub const INITIALIZED: u16 = 0x01;
    /// HPC completed initialization and was enabled. Resource padding is required.
    pub const ENABLED: u16 = 0x02;
}

/// Location definition for a PCI Hot Plug Controller.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiHpcLocation {
    /// Device path to the root HPC.
    pub hpc_device_path: *const EfiDevicePath,
    /// Device path to the Hot Plug Bus (HPB) controlled by the root HPC.
    pub hpb_device_path: *const EfiDevicePath,
}

/// Describes how resource padding should be applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiHpcPaddingAttributes {
    /// Apply padding at a PCI bus level.
    PaddingPciBus = 0,
    /// Apply padding at a PCI root bridge level.
    PaddingPciRootBridge = 1,
}

/// Hot plug operation requested from the PCI bus driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiPciHotplugOperation {
    /// Request to create handles and allocate resources for specified devices.
    Add = 0,
    /// Request to destroy handles and release resources for specified devices.
    Remove = 1,
}

// ---------------------------------------------------------------------------
// PCI Host Bridge Resource Allocation Protocol ABI
// ---------------------------------------------------------------------------

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
        phase: EfiPciControllerResourceAllocationPhase,
    ) -> EfiStatus;

/// Raw ABI structure for `EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL`.
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

// ---------------------------------------------------------------------------
// PCI Host Bridge Fixed Resource Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer to submit fixed memory or I/O ranges.
pub type EfiPciHostBridgeSubmitFixedResources = unsafe extern "efiapi" fn(
    this: *const EfiPciHostBridgeFixedResourceProtocol,
    root_bridge_handle: EfiHandle,
    configuration: *const c_void,
) -> EfiStatus;

/// Raw ABI structure for PCI Host Bridge Fixed Resource Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciHostBridgeFixedResourceProtocol {
    /// Submits fixed I/O and memory ranges decoded below the root bridge.
    pub submit_fixed_resources: EfiPciHostBridgeSubmitFixedResources,
}

// ---------------------------------------------------------------------------
// PCI Hot Plug Init Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_PCI_HOT_PLUG_INIT_GET_ROOT_HPC_LIST`.
pub type EfiPciHotPlugInitGetRootHpcList = unsafe extern "efiapi" fn(
    this: *const EfiPciHotPlugInitProtocol,
    hpc_count: *mut UIntN,
    hpc_list: *mut *mut EfiHpcLocation,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOT_PLUG_INIT_INITIALIZE_ROOT_HPC`.
pub type EfiPciHotPlugInitInitializeRootHpc = unsafe extern "efiapi" fn(
    this: *const EfiPciHotPlugInitProtocol,
    hpc_device_path: *const EfiDevicePath,
    hpc_pci_address: u64,
    event: *const c_void,
    hpc_state: *mut u16,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_HOT_PLUG_INIT_GET_RESOURCE_PADDING`.
pub type EfiPciHotPlugInitGetResourcePadding = unsafe extern "efiapi" fn(
    this: *const EfiPciHotPlugInitProtocol,
    hpc_device_path: *const EfiDevicePath,
    hpc_pci_address: u64,
    hpc_state: *mut u16,
    padding: *mut *mut c_void,
    attributes: *mut EfiHpcPaddingAttributes,
) -> EfiStatus;

/// Raw ABI structure for `EFI_PCI_HOT_PLUG_INIT_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciHotPlugInitProtocol {
    /// Returns the list of root hot-plug controllers.
    pub get_root_hpc_list: EfiPciHotPlugInitGetRootHpcList,
    /// Initializes a specific root hot plug controller.
    pub initialize_root_hpc: EfiPciHotPlugInitInitializeRootHpc,
    /// Returns resource padding required by the hot plug bus.
    pub get_resource_padding: EfiPciHotPlugInitGetResourcePadding,
}

// ---------------------------------------------------------------------------
// PCI Hot Plug Request Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_PCI_HOTPLUG_REQUEST_NOTIFY`.
pub type EfiPciHotplugRequestNotify = unsafe extern "efiapi" fn(
    this: *const EfiPciHotplugRequestProtocol,
    operation: EfiPciHotplugOperation,
    controller: EfiHandle,
    remaining_device_path: *const EfiDevicePath,
    number_of_children: *mut u8,
    child_handle_buffer: *mut EfiHandle,
) -> EfiStatus;

/// Raw ABI structure for `EFI_PCI_HOTPLUG_REQUEST_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciHotplugRequestProtocol {
    /// Notifies the PCI bus driver of hot-plug events.
    pub notify: EfiPciHotplugRequestNotify,
}

// ---------------------------------------------------------------------------
// PCI Platform Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_PCI_PLATFORM_PHASE_NOTIFY`.
pub type EfiPciPlatformPhaseNotify = unsafe extern "efiapi" fn(
    this: *const EfiPciPlatformProtocol,
    host_bridge: EfiHandle,
    phase: EfiPciHostBridgeResourceAllocationPhase,
    exec_phase: EfiPciExecutionPhase,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_PLATFORM_PREPROCESS_CONTROLLER`.
pub type EfiPciPlatformPreprocessController = unsafe extern "efiapi" fn(
    this: *const EfiPciPlatformProtocol,
    host_bridge: EfiHandle,
    root_bridge: EfiHandle,
    pci_address: u64,
    phase: EfiPciControllerResourceAllocationPhase,
    exec_phase: EfiPciExecutionPhase,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_PLATFORM_GET_PLATFORM_POLICY`.
pub type EfiPciPlatformGetPlatformPolicy = unsafe extern "efiapi" fn(
    this: *const EfiPciPlatformProtocol,
    pci_policy: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_PCI_PLATFORM_GET_PCI_ROM`.
pub type EfiPciPlatformGetPciRom = unsafe extern "efiapi" fn(
    this: *const EfiPciPlatformProtocol,
    pci_handle: EfiHandle,
    rom_image: *mut *mut c_void,
    rom_size: *mut UIntN,
) -> EfiStatus;

/// Raw ABI structure for `EFI_PCI_PLATFORM_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciPlatformProtocol {
    /// Notifies platform driver of bus enumeration phases.
    pub platform_notify: EfiPciPlatformPhaseNotify,
    /// Notifies platform driver of individual controller initialization steps.
    pub platform_prep_controller: EfiPciPlatformPreprocessController,
    /// Retrieves platform enumeration policy with respect to legacy ISA/VGA.
    pub get_platform_policy: EfiPciPlatformGetPlatformPolicy,
    /// Retrieves optional platform-specific Option ROM for a device.
    pub get_pci_rom: EfiPciPlatformGetPciRom,
}

// ---------------------------------------------------------------------------
// Safe Native Rust Client Wrappers
// ---------------------------------------------------------------------------

/// Safe wrapper around `EfiPciHostBridgeResourceAllocationProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PciHostBridgeResourceAllocation {
    proto: *const EfiPciHostBridgeResourceAllocationProtocol,
}

impl PciHostBridgeResourceAllocation {
    /// Wraps a raw pointer to `EfiPciHostBridgeResourceAllocationProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPciHostBridgeResourceAllocationProtocol) -> Self {
        Self { proto }
    }

    /// Notifies the host bridge driver of the upcoming enumeration phase.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if the phase is invalid.
    /// Returns `EfiStatus::NOT_READY` or `EfiStatus::DEVICE_ERROR` on allocation or hardware failure.
    pub fn notify_phase(
        &self,
        phase: EfiPciHostBridgeResourceAllocationPhase,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).notify_phase)(self.proto, phase) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the handle of the next PCI root bridge associated with this host bridge.
    ///
    /// Pass `None` to retrieve the first root bridge handle.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` when enumeration reaches the end.
    /// Returns `EfiStatus::INVALID_PARAMETER` if `prev_handle` is invalid.
    pub fn get_next_root_bridge(
        &self,
        prev_handle: Option<EfiHandle>,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = prev_handle.unwrap_or(EfiHandle::NULL);
        let status = unsafe {
            ((*self.proto).get_next_root_bridge)(self.proto, core::ptr::addr_of_mut!(handle))
        };
        if status == EfiStatus::SUCCESS {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Returns the allocation attributes of a PCI root bridge.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `root_bridge_handle` is invalid.
    pub fn get_alloc_attributes(&self, root_bridge_handle: EfiHandle) -> Result<u64, EfiStatus> {
        let mut attrs = 0u64;
        let status = unsafe {
            ((*self.proto).get_alloc_attributes)(
                self.proto,
                root_bridge_handle,
                core::ptr::addr_of_mut!(attrs),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(attrs)
        } else {
            Err(status)
        }
    }

    /// Prepares the root bridge for bus enumeration and returns bus resource descriptors.
    ///
    /// # Errors
    ///
    /// Returns an error status on hardware failure or invalid handle.
    pub fn start_bus_enumeration(
        &self,
        root_bridge_handle: EfiHandle,
    ) -> Result<*mut c_void, EfiStatus> {
        let mut config = core::ptr::null_mut();
        let status = unsafe {
            ((*self.proto).start_bus_enumeration)(
                self.proto,
                root_bridge_handle,
                core::ptr::addr_of_mut!(config),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(config)
        } else {
            Err(status)
        }
    }

    /// Programs the root bridge hardware to decode the specified bus range.
    ///
    /// # Safety
    ///
    /// `configuration` must point to a valid ACPI bus resource descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error status if the configuration descriptor is invalid or programming fails.
    pub unsafe fn set_bus_numbers(
        &self,
        root_bridge_handle: EfiHandle,
        configuration: *const c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).set_bus_numbers)(self.proto, root_bridge_handle, configuration)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Submits resource requirements for the specified root bridge.
    ///
    /// # Safety
    ///
    /// `configuration` must point to valid ACPI memory and I/O resource descriptors.
    ///
    /// # Errors
    ///
    /// Returns an error status if the resource request is invalid or cannot be accepted.
    pub unsafe fn submit_resources(
        &self,
        root_bridge_handle: EfiHandle,
        configuration: *const c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).submit_resources)(self.proto, root_bridge_handle, configuration)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves proposed memory and I/O windows for the specified root bridge.
    ///
    /// # Errors
    ///
    /// Returns an error status if proposed resources cannot be retrieved.
    pub fn get_proposed_resources(
        &self,
        root_bridge_handle: EfiHandle,
    ) -> Result<*mut c_void, EfiStatus> {
        let mut config = core::ptr::null_mut();
        let status = unsafe {
            ((*self.proto).get_proposed_resources)(
                self.proto,
                root_bridge_handle,
                core::ptr::addr_of_mut!(config),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(config)
        } else {
            Err(status)
        }
    }

    /// Preinitializes individual PCI controllers before bus enumeration.
    ///
    /// # Errors
    ///
    /// Returns an error status on controller preinitialization failure.
    pub fn preprocess_controller(
        &self,
        root_bridge_handle: EfiHandle,
        pci_address: u64,
        phase: EfiPciControllerResourceAllocationPhase,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).preprocess_controller)(
                self.proto,
                root_bridge_handle,
                pci_address,
                phase,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around `EfiPciHostBridgeFixedResourceProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PciHostBridgeFixedResource {
    proto: *const EfiPciHostBridgeFixedResourceProtocol,
}

impl PciHostBridgeFixedResource {
    /// Wraps a raw pointer to `EfiPciHostBridgeFixedResourceProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPciHostBridgeFixedResourceProtocol) -> Self {
        Self { proto }
    }

    /// Submits fixed I/O and memory ranges decoded below the specified root bridge.
    ///
    /// # Safety
    ///
    /// `configuration` must point to valid ACPI QWORD address space descriptors.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `root_bridge_handle` or `configuration` is invalid.
    /// Returns `EfiStatus::ACCESS_DENIED` if fixed ranges have already been claimed and not released.
    pub unsafe fn submit_fixed_resources(
        &self,
        root_bridge_handle: EfiHandle,
        configuration: *const c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).submit_fixed_resources)(self.proto, root_bridge_handle, configuration)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around `EfiPciHotPlugInitProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PciHotPlugInit {
    proto: *const EfiPciHotPlugInitProtocol,
}

impl PciHotPlugInit {
    /// Wraps a raw pointer to `EfiPciHotPlugInitProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPciHotPlugInitProtocol) -> Self {
        Self { proto }
    }

    /// Returns the list of root Hot Plug Controllers (HPCs).
    ///
    /// # Errors
    ///
    /// Returns an error status on resource allocation failure.
    pub fn get_root_hpc_list(&self) -> Result<(*mut EfiHpcLocation, usize), EfiStatus> {
        let mut count: UIntN = 0;
        let mut list: *mut EfiHpcLocation = core::ptr::null_mut();
        let status = unsafe {
            ((*self.proto).get_root_hpc_list)(
                self.proto,
                core::ptr::addr_of_mut!(count),
                core::ptr::addr_of_mut!(list),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((list, count))
        } else {
            Err(status)
        }
    }

    /// Initializes a specific root Hot Plug Controller (HPC).
    ///
    /// # Errors
    ///
    /// Returns an error status if the HPC is unsupported or initialization fails.
    pub fn initialize_root_hpc(
        &self,
        hpc_device_path: &EfiDevicePath,
        hpc_pci_address: u64,
        event: Option<EfiEvent>,
    ) -> Result<u16, EfiStatus> {
        let mut state = 0u16;
        let event_ptr = event.map_or(core::ptr::null(), |e| e.to_ptr());
        let status = unsafe {
            ((*self.proto).initialize_root_hpc)(
                self.proto,
                core::ptr::from_ref(hpc_device_path),
                hpc_pci_address,
                event_ptr,
                core::ptr::addr_of_mut!(state),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(state)
        } else {
            Err(status)
        }
    }

    /// Returns resource padding required by the hot plug bus.
    ///
    /// # Errors
    ///
    /// Returns an error status if the HPC is unsupported or padding cannot be determined.
    pub fn get_resource_padding(
        &self,
        hpc_device_path: &EfiDevicePath,
        hpc_pci_address: u64,
    ) -> Result<(u16, *mut c_void, EfiHpcPaddingAttributes), EfiStatus> {
        let mut state = 0u16;
        let mut padding = core::ptr::null_mut();
        let mut attrs = EfiHpcPaddingAttributes::PaddingPciBus;
        let status = unsafe {
            ((*self.proto).get_resource_padding)(
                self.proto,
                core::ptr::from_ref(hpc_device_path),
                hpc_pci_address,
                core::ptr::addr_of_mut!(state),
                core::ptr::addr_of_mut!(padding),
                core::ptr::addr_of_mut!(attrs),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((state, padding, attrs))
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around `EfiPciHotplugRequestProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PciHotplugRequest {
    proto: *const EfiPciHotplugRequestProtocol,
}

impl PciHotplugRequest {
    /// Wraps a raw pointer to `EfiPciHotplugRequestProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPciHotplugRequestProtocol) -> Self {
        Self { proto }
    }

    /// Notifies the PCI bus driver of a hot-plug event (add or remove).
    ///
    /// # Errors
    ///
    /// Returns an error status if operation fails or parameters are invalid.
    pub fn notify(
        &self,
        operation: EfiPciHotplugOperation,
        controller: EfiHandle,
        remaining_device_path: Option<&EfiDevicePath>,
        children: &mut [EfiHandle],
    ) -> Result<u8, EfiStatus> {
        let mut num_children = u8::try_from(children.len()).unwrap_or(u8::MAX);
        let dev_path_ptr = remaining_device_path.map_or(core::ptr::null(), core::ptr::from_ref);
        let status = unsafe {
            ((*self.proto).notify)(
                self.proto,
                operation,
                controller,
                dev_path_ptr,
                core::ptr::addr_of_mut!(num_children),
                children.as_mut_ptr(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(num_children)
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around `EfiPciPlatformProtocol`.
#[derive(Clone, Copy, Debug)]
pub struct PciPlatform {
    proto: *const EfiPciPlatformProtocol,
}

impl PciPlatform {
    /// Wraps a raw pointer to `EfiPciPlatformProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPciPlatformProtocol) -> Self {
        Self { proto }
    }

    /// Notifies the platform driver of the current enumeration phase.
    ///
    /// # Errors
    ///
    /// Returns an error status if notification fails.
    pub fn notify_phase(
        &self,
        host_bridge: EfiHandle,
        phase: EfiPciHostBridgeResourceAllocationPhase,
        exec_phase: EfiPciExecutionPhase,
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { ((*self.proto).platform_notify)(self.proto, host_bridge, phase, exec_phase) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Notifies the platform driver before or after configuring an individual PCI controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if preinitialization fails.
    pub fn prep_controller(
        &self,
        host_bridge: EfiHandle,
        root_bridge: EfiHandle,
        pci_address: u64,
        phase: EfiPciControllerResourceAllocationPhase,
        exec_phase: EfiPciExecutionPhase,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).platform_prep_controller)(
                self.proto,
                host_bridge,
                root_bridge,
                pci_address,
                phase,
                exec_phase,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves the platform policy regarding ISA and VGA I/O aliasing.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if policy cannot be retrieved.
    pub fn get_platform_policy(&self) -> Result<u32, EfiStatus> {
        let mut policy = 0u32;
        let status = unsafe {
            ((*self.proto).get_platform_policy)(self.proto, core::ptr::addr_of_mut!(policy))
        };
        if status == EfiStatus::SUCCESS {
            Ok(policy)
        } else {
            Err(status)
        }
    }

    /// Retrieves the platform-specific Option ROM for a device, if available.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if no platform Option ROM is present.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` or `EfiStatus::DEVICE_ERROR` on failure.
    pub fn get_pci_rom(&self, pci_handle: EfiHandle) -> Result<(*mut c_void, usize), EfiStatus> {
        let mut rom_image = core::ptr::null_mut();
        let mut rom_size: UIntN = 0;
        let status = unsafe {
            ((*self.proto).get_pci_rom)(
                self.proto,
                pci_handle,
                core::ptr::addr_of_mut!(rom_image),
                core::ptr::addr_of_mut!(rom_size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((rom_image, rom_size))
        } else {
            Err(status)
        }
    }
}
