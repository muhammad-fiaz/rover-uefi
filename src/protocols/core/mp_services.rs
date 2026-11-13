//! # Multiprocessor (MP) Services Protocol
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 2:
//! Driver Execution Environment Core Interface.
//!
//! Provides control over processor cores, processor enumeration, AP startup,
//! BSP reassignment, and AP enable/disable controls.

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_MP_SERVICES_PROTOCOL_GUID`.
pub const EFI_MP_SERVICES_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3FDD_A605,
    0xA76E,
    0x4F46,
    [0xAD, 0x29, 0x12, 0xF4, 0x53, 0x1B, 0x3D, 0x08],
);

// Processor status flags
/// Processor is the Boot Strap Processor (BSP).
pub const PROCESSOR_AS_BSP_BIT: u32 = 0x0000_0001;
/// Processor is enabled.
pub const PROCESSOR_ENABLED_BIT: u32 = 0x0000_0002;
/// Processor has healthy diagnostic status.
pub const PROCESSOR_HEALTH_STATUS_BIT: u32 = 0x0000_0004;

/// Physical location of a processor core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiCpuPhysicalLocation {
    /// Zero-based physical package number.
    pub package: u32,
    /// Zero-based physical core number within package.
    pub core: u32,
    /// Zero-based physical thread number within core.
    pub thread: u32,
}

/// Extended physical location with tile / module / die levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiCpuPhysicalLocation2 {
    /// Zero-based physical package number.
    pub package: u32,
    /// Zero-based physical die number.
    pub die: u32,
    /// Zero-based physical tile number.
    pub tile: u32,
    /// Zero-based physical module number.
    pub module: u32,
    /// Zero-based physical core number.
    pub core: u32,
    /// Zero-based physical thread number.
    pub thread: u32,
}

/// Detailed information on a processor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiProcessorInformation {
    /// Unique hardware processor identifier (e.g. APIC ID).
    pub processor_id: u64,
    /// Bitmask indicating BSP, enabled, and health status.
    pub status_flag: u32,
    /// Physical location of processor core.
    pub location: EfiCpuPhysicalLocation,
    /// Extended physical location.
    pub extended_information: EfiCpuPhysicalLocation2,
}

/// Procedure callback executed on Application Processors.
pub type EfiApProcedure = unsafe extern "efiapi" fn(buffer: *mut c_void);

/// Function pointer for `EFI_MP_SERVICES_GET_NUMBER_OF_PROCESSORS`.
pub type EfiMpServicesGetNumberOfProcessors = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    number_of_processors: *mut UIntN,
    number_of_enabled_processors: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_GET_PROCESSOR_INFO`.
pub type EfiMpServicesGetProcessorInfo = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    processor_number: UIntN,
    processor_info_buffer: *mut EfiProcessorInformation,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_STARTUP_ALL_APS`.
pub type EfiMpServicesStartupAllAPs = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    procedure: EfiApProcedure,
    single_thread: Bool,
    wait_event: EfiEvent,
    timeout_in_micro_seconds: UIntN,
    procedure_argument: *mut c_void,
    failed_cpu_list: *mut *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_STARTUP_THIS_AP`.
pub type EfiMpServicesStartupThisAP = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    procedure: EfiApProcedure,
    processor_number: UIntN,
    wait_event: EfiEvent,
    timeout_in_micro_seconds: UIntN,
    procedure_argument: *mut c_void,
    finished: *mut Bool,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_SWITCH_BSP`.
pub type EfiMpServicesSwitchBsp = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    processor_number: UIntN,
    enable_old_bsp: Bool,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_ENABLEDISABLEAP`.
pub type EfiMpServicesEnableDisableAP = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    processor_number: UIntN,
    enable_ap: Bool,
    health_flag: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_MP_SERVICES_WHOAMI`.
pub type EfiMpServicesWhoAmI = unsafe extern "efiapi" fn(
    this: *const EfiMpServicesProtocol,
    processor_number: *mut UIntN,
) -> EfiStatus;

/// The raw ABI structure for `EFI_MP_SERVICES_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMpServicesProtocol {
    /// Gets total processor count and enabled processor count.
    pub get_number_of_processors: EfiMpServicesGetNumberOfProcessors,
    /// Gets detailed processor information by index.
    pub get_processor_info: EfiMpServicesGetProcessorInfo,
    /// Dispatches code execution across all application processors.
    pub startup_all_aps: EfiMpServicesStartupAllAPs,
    /// Dispatches code execution on a specific application processor.
    pub startup_this_ap: EfiMpServicesStartupThisAP,
    /// Reassigns the Boot Strap Processor (BSP).
    pub switch_bsp: EfiMpServicesSwitchBsp,
    /// Enables or disables an Application Processor.
    pub enable_disable_ap: EfiMpServicesEnableDisableAP,
    /// Returns the caller processor index.
    pub who_am_i: EfiMpServicesWhoAmI,
}

/// Safe wrapper around `EFI_MP_SERVICES_PROTOCOL`.
pub struct MpServices<'a> {
    protocol: &'a EfiMpServicesProtocol,
}

impl<'a> MpServices<'a> {
    /// Creates a new view borrowing the protocol instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiMpServicesProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the total count of processors and the count of enabled processors.
    ///
    /// # Errors
    /// Returns an error if the query fails.
    pub fn processor_counts(&self) -> Result<(usize, usize), EfiStatus> {
        let mut total = UIntN::default();
        let mut enabled = UIntN::default();
        // SAFETY: Pointers reference local variables.
        let status = unsafe {
            (self.protocol.get_number_of_processors)(
                self.protocol,
                core::ptr::from_mut(&mut total),
                core::ptr::from_mut(&mut enabled),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok((total, enabled))
        }
    }

    /// Retrieves processor information for the specified processor index.
    ///
    /// # Errors
    /// Returns an error if the processor index is invalid or query fails.
    pub fn processor_info(&self, index: usize) -> Result<EfiProcessorInformation, EfiStatus> {
        let mut info = EfiProcessorInformation {
            processor_id: 0,
            status_flag: 0,
            location: EfiCpuPhysicalLocation {
                package: 0,
                core: 0,
                thread: 0,
            },
            extended_information: EfiCpuPhysicalLocation2 {
                package: 0,
                die: 0,
                tile: 0,
                module: 0,
                core: 0,
                thread: 0,
            },
        };
        // SAFETY: Pointer references local variable.
        let status = unsafe {
            (self.protocol.get_processor_info)(self.protocol, index, core::ptr::from_mut(&mut info))
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(info)
        }
    }

    /// Returns the zero-based index of the currently executing processor core.
    ///
    /// # Errors
    /// Returns an error if the query fails.
    pub fn current_processor_index(&self) -> Result<usize, EfiStatus> {
        let mut index = UIntN::default();
        // SAFETY: Pointer references local variable.
        let status =
            unsafe { (self.protocol.who_am_i)(self.protocol, core::ptr::from_mut(&mut index)) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(index)
        }
    }
}

/// GUID for Multiprocessor Information 2 HOB (`{417a7f64-f4e9-4b32-846a-5cc4d8621879}`).
pub const MP_INFORMATION2_GUID: EfiGuid = EfiGuid::new(
    0x417A_7F64,
    0xF4E9,
    0x4B32,
    [0x84, 0x6A, 0x5C, 0xC4, 0xD8, 0x62, 0x18, 0x79],
);

/// Revision for `MpInformation2HobData`.
pub const MP_INFORMATION2_HOB_REVISION: u8 = 1;

/// Standard symmetrical core type.
pub const CORE_TYPE_STANDARD: u8 = 0;
/// Performance core (P-core) in hybrid heterogeneous CPU architectures.
pub const CORE_TYPE_PERFORMANCE: u8 = 1;
/// Efficient core (E-core) in hybrid heterogeneous CPU architectures.
pub const CORE_TYPE_EFFICIENT: u8 = 2;

/// Heterogeneous processor entry in MP Information 2 HOB.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MpInformation2Entry {
    /// Processor architectural location and flags.
    pub processor_info: EfiProcessorInformation,
    /// Core type classification (`CORE_TYPE_*`).
    pub core_type: u8,
    /// Reserved alignment padding.
    pub reserved: [u8; 7],
}

impl MpInformation2Entry {
    /// Check if this processor is a performance core (P-core).
    #[must_use]
    pub const fn is_performance_core(&self) -> bool {
        self.core_type == CORE_TYPE_PERFORMANCE
    }

    /// Check if this processor is an efficiency core (E-core).
    #[must_use]
    pub const fn is_efficient_core(&self) -> bool {
        self.core_type == CORE_TYPE_EFFICIENT
    }
}

/// Header for the Multiprocessor Information 2 HOB describing heterogeneous cores.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MpInformation2HobData {
    /// Number of processors described.
    pub number_of_processors: u16,
    /// Size of each `MpInformation2Entry` in bytes.
    pub entry_size: u16,
    /// Structure revision (`MP_INFORMATION2_HOB_REVISION`).
    pub version: u8,
    /// Reserved padding.
    pub reserved: [u8; 3],
    /// Processor index offset.
    pub processor_index: u64,
}
