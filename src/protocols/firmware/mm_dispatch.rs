//! # Management Mode (MM / SMM) Dispatch Protocols
//!
//! Provides the standard child MMI / SMI dispatch service interfaces defined by the
//! UEFI Platform Initialization (PI) Specification Volume 4: Management Mode Core Interface.
//!
//! Includes software MMI dispatch, sleep state (Sx) dispatch, periodic timer dispatch,
//! power button dispatch, standby button dispatch, I/O trap dispatch, General Purpose
//! Input (GPI) dispatch, and USB MMI dispatch protocols.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use crate::protocols::firmware::mm_system_table::EfiMmiHandler;

// --- 1. Software MMI Dispatch Protocol ---

/// MM Software Dispatch Protocol `GUID` (`EFI_MM_SW_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_SW_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x18A3_C6DC,
    0x5EEA,
    0x48C8,
    [0xA1, 0xC1, 0xB5, 0x33, 0x89, 0xF9, 0x89, 0x99],
);

/// SMM Software Dispatch 2 Protocol `GUID` (`EFI_SMM_SW_DISPATCH2_PROTOCOL_GUID`).
pub const EFI_SMM_SW_DISPATCH2_PROTOCOL_GUID: EfiGuid = EFI_MM_SW_DISPATCH_PROTOCOL_GUID;

/// Context passed when registering a software MMI handler.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmSwRegisterContext {
    /// The software MMI / SMI input value (e.g., APMC command value).
    pub sw_mmi_input_value: UIntN,
}

/// Context supplied to a software MMI handler during execution.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmSwContext {
    /// 0-based index of the logical CPU that triggered the software MMI.
    pub sw_mmi_cpu_index: UIntN,
    /// Value written to the command port.
    pub command_port: u8,
    /// Value written to the data port.
    pub data_port: u8,
}

/// Raw interface structure for the MM Software Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmSwDispatchProtocol {
    /// Registers a handler for a specific software MMI value.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmSwDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *mut MmSwRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a previously registered software MMI handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmSwDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
    /// Maximum supported software interrupt value.
    pub maximum_swi_value: UIntN,
}

/// Safe idiomatic wrapper for the MM Software Dispatch Protocol.
#[derive(Debug)]
pub struct MmSwDispatch<'a> {
    protocol: &'a MmSwDispatchProtocol,
}

impl<'a> MmSwDispatch<'a> {
    /// MM Software Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_SW_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmSwDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a child handler for the specified software MMI value.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if value is already in use.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &mut MmSwRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw mut *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a previously registered handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the maximum software interrupt value supported by the platform.
    #[must_use]
    pub fn maximum_swi_value(&self) -> usize {
        self.protocol.maximum_swi_value
    }
}

// --- 2. Sleep-State (Sx) Dispatch Protocol ---

/// MM Sx Dispatch Protocol `GUID` (`EFI_MM_SX_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_SX_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x456D_2859,
    0xA84B,
    0x4E47,
    [0xA2, 0xEE, 0x32, 0x76, 0xD8, 0x86, 0x99, 0x7D],
);

/// SMM Sx Dispatch 2 Protocol `GUID` (`EFI_SMM_SX_DISPATCH2_PROTOCOL_GUID`).
pub const EFI_SMM_SX_DISPATCH2_PROTOCOL_GUID: EfiGuid = EFI_MM_SX_DISPATCH_PROTOCOL_GUID;

/// ACPI sleep state types S0 through S5.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SleepType {
    /// Working state S0.
    S0 = 0,
    /// Sleeping state S1.
    S1 = 1,
    /// Sleeping state S2.
    S2 = 2,
    /// Sleeping state S3 (Suspend-to-RAM).
    S3 = 3,
    /// Sleeping state S4 (Hibernate).
    S4 = 4,
    /// Soft Off S5.
    S5 = 5,
}

/// Sleep state phase (entry into or exit from sleep state).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SleepPhase {
    /// Entering the sleep state.
    Entry = 0,
    /// Exiting the sleep state.
    Exit = 1,
}

/// Context for registering an Sx sleep state handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MmSxRegisterContext {
    /// The target sleep state.
    pub sleep_type: SleepType,
    /// Entry or exit phase.
    pub phase: SleepPhase,
}

/// Raw interface structure for the MM Sx Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmSxDispatchProtocol {
    /// Registers a handler for an Sx transition event.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmSxDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmSxRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters an Sx transition handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmSxDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM Sx Dispatch Protocol.
#[derive(Debug)]
pub struct MmSxDispatch<'a> {
    protocol: &'a MmSxDispatchProtocol,
}

impl<'a> MmSxDispatch<'a> {
    /// MM Sx Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_SX_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmSxDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a handler for an Sx sleep state transition.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if context is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmSxRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a previously registered Sx transition handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 3. Periodic Timer Dispatch Protocol ---

/// MM Periodic Timer Dispatch Protocol `GUID` (`EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4CEC_368E,
    0x8E8E,
    0x4D71,
    [0x8B, 0xE1, 0x95, 0x8C, 0x45, 0xFC, 0x8A, 0x53],
);

/// SMM Periodic Timer Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_PERIODIC_TIMER_DISPATCH2_PROTOCOL_GUID: EfiGuid =
    EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID;

/// Registration context for periodic timer MMIs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmPeriodicTimerRegisterContext {
    /// Minimum period in 100-nanosecond units.
    pub period: u64,
    /// Desired tick interval between MMIs in 100-nanosecond units.
    pub mmi_tick_interval: u64,
}

/// Context structure supplied during periodic timer MMI execution.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmPeriodicTimerContext {
    /// Elapsed time in 100-nanosecond units since the last handler execution.
    pub elapsed_time: u64,
}

/// Raw interface structure for the MM Periodic Timer Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmPeriodicTimerDispatchProtocol {
    /// Registers a periodic timer MMI handler.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmPeriodicTimerDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmPeriodicTimerRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a periodic timer MMI handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmPeriodicTimerDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
    /// Returns the supported tick intervals for periodic timer MMIs.
    pub get_next_mmi_interval: unsafe extern "efiapi" fn(
        this: *const MmPeriodicTimerDispatchProtocol,
        mmi_tick_interval: *mut *mut u64,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM Periodic Timer Dispatch Protocol.
#[derive(Debug)]
pub struct MmPeriodicTimerDispatch<'a> {
    protocol: &'a MmPeriodicTimerDispatchProtocol,
}

impl<'a> MmPeriodicTimerDispatch<'a> {
    /// MM Periodic Timer Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmPeriodicTimerDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a periodic timer handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if context is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmPeriodicTimerRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a periodic timer handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 4. Power Button Dispatch Protocol ---

/// MM Power Button Dispatch Protocol `GUID` (`EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1B11_83FA,
    0x1823,
    0x46A7,
    [0x88, 0x72, 0x9C, 0x57, 0x87, 0x55, 0x40, 0x9D],
);

/// SMM Power Button Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_POWER_BUTTON_DISPATCH2_PROTOCOL_GUID: EfiGuid =
    EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID;

/// Power Button activation phases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PowerButtonPhase {
    /// Invoked upon pressing the power button.
    Entry = 0,
    /// Invoked upon releasing the power button.
    Exit = 1,
}

/// Registration context for the power button dispatch handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MmPowerButtonRegisterContext {
    /// Power button phase.
    pub phase: PowerButtonPhase,
}

/// Raw interface structure for the MM Power Button Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmPowerButtonDispatchProtocol {
    /// Registers a handler for power button events.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmPowerButtonDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmPowerButtonRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a power button handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmPowerButtonDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM Power Button Dispatch Protocol.
#[derive(Debug)]
pub struct MmPowerButtonDispatch<'a> {
    protocol: &'a MmPowerButtonDispatchProtocol,
}

impl<'a> MmPowerButtonDispatch<'a> {
    /// MM Power Button Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmPowerButtonDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a power button event handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if context is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmPowerButtonRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a power button handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 5. Standby Button Dispatch Protocol ---

/// MM Standby Button Dispatch Protocol `GUID` (`EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x7300_C4A1,
    0x43F2,
    0x4017,
    [0xA5, 0x1B, 0xC8, 0x1A, 0x7F, 0x40, 0x58, 0x5B],
);

/// SMM Standby Button Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_STANDBY_BUTTON_DISPATCH2_PROTOCOL_GUID: EfiGuid =
    EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID;

/// Standby Button activation phases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StandbyButtonPhase {
    /// Invoked upon pressing the standby button.
    Entry = 0,
    /// Invoked upon releasing the standby button.
    Exit = 1,
}

/// Registration context for the standby button dispatch handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MmStandbyButtonRegisterContext {
    /// Standby button phase.
    pub phase: StandbyButtonPhase,
}

/// Raw interface structure for the MM Standby Button Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmStandbyButtonDispatchProtocol {
    /// Registers a handler for standby button events.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmStandbyButtonDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmStandbyButtonRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a standby button handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmStandbyButtonDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM Standby Button Dispatch Protocol.
#[derive(Debug)]
pub struct MmStandbyButtonDispatch<'a> {
    protocol: &'a MmStandbyButtonDispatchProtocol,
}

impl<'a> MmStandbyButtonDispatch<'a> {
    /// MM Standby Button Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmStandbyButtonDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a standby button event handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if context is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmStandbyButtonRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a standby button handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 6. I/O Trap Dispatch Protocol ---

/// MM I/O Trap Dispatch Protocol `GUID` (`EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x58DC_368D,
    0x7BFA,
    0x4E77,
    [0xAB, 0xBC, 0x0E, 0x29, 0x41, 0x8D, 0xF9, 0x30],
);

/// SMM I/O Trap Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_IO_TRAP_DISPATCH2_PROTOCOL_GUID: EfiGuid = EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID;

/// I/O Trap types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MmIoTrapType {
    /// Trap on I/O write operations.
    WriteTrap = 0,
    /// Trap on I/O read operations.
    ReadTrap = 1,
    /// Trap on both read and write operations.
    ReadWriteTrap = 2,
}

/// Registration context for an I/O trap handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MmIoTrapRegisterContext {
    /// Base I/O address for trap (or 0 to auto-allocate).
    pub address: u16,
    /// Length of I/O trap window in bytes.
    pub length: u16,
    /// Type of trap (read, write, or both).
    pub trap_type: MmIoTrapType,
}

/// Context information supplied during I/O trap handler invocation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmIoTrapContext {
    /// Data value that was written during an I/O write trap.
    pub write_data: u32,
}

/// Raw interface structure for the MM I/O Trap Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmIoTrapDispatchProtocol {
    /// Registers an I/O trap handler.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmIoTrapDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *mut MmIoTrapRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters an I/O trap handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmIoTrapDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM I/O Trap Dispatch Protocol.
#[derive(Debug)]
pub struct MmIoTrapDispatch<'a> {
    protocol: &'a MmIoTrapDispatchProtocol,
}

impl<'a> MmIoTrapDispatch<'a> {
    /// MM I/O Trap Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmIoTrapDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers an I/O trap handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `context` is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &mut MmIoTrapRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw mut *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters an I/O trap handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 7. General Purpose Input (GPI) Dispatch Protocol ---

/// MM GPI Dispatch Protocol `GUID` (`EFI_MM_GPI_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_GPI_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2556_6B03,
    0xB577,
    0x4CBF,
    [0x95, 0x8C, 0xED, 0x66, 0x3E, 0xA2, 0x43, 0x80],
);

/// SMM GPI Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_GPI_DISPATCH2_PROTOCOL_GUID: EfiGuid = EFI_MM_GPI_DISPATCH_PROTOCOL_GUID;

/// Registration context for a General Purpose Input (GPI) MMI handler.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct MmGpiRegisterContext {
    /// Logical GPI pin number that generates the MMI.
    pub gpi_num: u64,
}

/// Raw interface structure for the MM GPI Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmGpiDispatchProtocol {
    /// Registers a GPI MMI handler.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmGpiDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmGpiRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a GPI MMI handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmGpiDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
    /// Number of GPI inputs supported by the hardware.
    pub num_supported_gpis: UIntN,
}

/// Safe idiomatic wrapper for the MM GPI Dispatch Protocol.
#[derive(Debug)]
pub struct MmGpiDispatch<'a> {
    protocol: &'a MmGpiDispatchProtocol,
}

impl<'a> MmGpiDispatch<'a> {
    /// MM GPI Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_GPI_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmGpiDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a GPI MMI handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `context` is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmGpiRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a GPI MMI handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the number of GPI inputs supported by the hardware.
    #[must_use]
    pub fn num_supported_gpis(&self) -> usize {
        self.protocol.num_supported_gpis
    }
}

// --- 8. USB MMI Dispatch Protocol ---

/// MM USB Dispatch Protocol `GUID` (`EFI_MM_USB_DISPATCH_PROTOCOL_GUID`).
pub const EFI_MM_USB_DISPATCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEE9B_8D90,
    0xC5A6,
    0x40A2,
    [0xBD, 0xE2, 0x52, 0x55, 0x8D, 0x33, 0xCC, 0xA1],
);

/// SMM USB Dispatch 2 Protocol `GUID`.
pub const EFI_SMM_USB_DISPATCH2_PROTOCOL_GUID: EfiGuid = EFI_MM_USB_DISPATCH_PROTOCOL_GUID;

/// USB MMI event types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbMmiType {
    /// Legacy emulation event (e.g. keyboard port trap).
    UsbLegacy = 0,
    /// USB resume / wake event from low-power state.
    UsbWake = 1,
}

/// Registration context for a USB MMI handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MmUsbRegisterContext {
    /// Event type (legacy emulation or wake).
    pub mmi_type: UsbMmiType,
    /// Pointer to device path describing USB host controller location.
    pub device: *mut EfiDevicePath,
}

/// Raw interface structure for the MM USB Dispatch Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct MmUsbDispatchProtocol {
    /// Registers a handler for USB MMI events.
    pub register: unsafe extern "efiapi" fn(
        this: *const MmUsbDispatchProtocol,
        dispatch_function: EfiMmiHandler,
        register_context: *const MmUsbRegisterContext,
        dispatch_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a USB MMI handler.
    pub unregister: unsafe extern "efiapi" fn(
        this: *const MmUsbDispatchProtocol,
        dispatch_handle: EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic wrapper for the MM USB Dispatch Protocol.
#[derive(Debug)]
pub struct MmUsbDispatch<'a> {
    protocol: &'a MmUsbDispatchProtocol,
}

impl<'a> MmUsbDispatch<'a> {
    /// MM USB Dispatch Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_MM_USB_DISPATCH_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a MmUsbDispatchProtocol) -> Self {
        Self { protocol }
    }

    /// Registers a USB MMI handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `context` is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn register(
        &self,
        handler: EfiMmiHandler,
        context: &MmUsbRegisterContext,
    ) -> Result<EfiHandle, EfiStatus> {
        let mut handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.register)(self.protocol, handler, &raw const *context, &raw mut handle)
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Unregisters a USB MMI handler.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is invalid.
    pub fn unregister(&self, handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
