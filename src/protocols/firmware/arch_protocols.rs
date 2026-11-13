//! # PI Architectural Protocols
//!
//! Defined in PI Specification Volume 2: DXE Foundation.
//!
//! The DXE Foundation relies on architectural protocols to abstract processor,
//! platform, and runtime dependencies. Once all required architectural protocols
//! are published, the DXE Foundation enters its fully initialized state and
//! begins scheduling and dispatching non-architectural drivers.

use core::ffi::c_void;

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use crate::protocols::device_path::types::EfiDevicePath;

/// Reset Architectural Protocol `GUID`.
pub const EFI_RESET_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x27CF_AC88,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Real Time Clock Architectural Protocol `GUID`.
pub const EFI_REAL_TIME_CLOCK_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x27CF_AC87,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Watchdog Timer Architectural Protocol `GUID`.
pub const EFI_WATCHDOG_TIMER_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x665E_3FF5,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Monotonic Counter Architectural Protocol `GUID`.
pub const EFI_MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1DA9_7072,
    0xBDDC,
    0x4B30,
    [0x99, 0xF1, 0x72, 0xA0, 0xB5, 0x6F, 0xFF, 0x2A],
);

/// Metronome Architectural Protocol `GUID`.
pub const EFI_METRONOME_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB2,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// Timer Architectural Protocol `GUID`.
pub const EFI_TIMER_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB3,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// Variable Architectural Protocol `GUID`.
pub const EFI_VARIABLE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1E56_68E2,
    0x8481,
    0x11D4,
    [0xBC, 0xF1, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// Variable Write Architectural Protocol `GUID`.
pub const EFI_VARIABLE_WRITE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6441_F818,
    0x6362,
    0x4E44,
    [0xB5, 0x70, 0x7D, 0xBA, 0x31, 0xDD, 0x24, 0x53],
);

/// Runtime Architectural Protocol `GUID`.
pub const EFI_RUNTIME_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB7DF_B4E1,
    0x052F,
    0x449F,
    [0x87, 0xBE, 0x98, 0x18, 0xFC, 0x91, 0xB7, 0x33],
);

/// Security Architectural Protocol `GUID`.
pub const EFI_SECURITY_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA464_23E3,
    0x4617,
    0x49F1,
    [0xB9, 0xFF, 0xD1, 0xBF, 0xA9, 0x11, 0x58, 0x39],
);

/// Security2 Architectural Protocol `GUID`.
pub const EFI_SECURITY2_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x94AB_2F58,
    0x1438,
    0x4EF1,
    [0x91, 0x52, 0x18, 0x94, 0x1A, 0x3A, 0x0E, 0x68],
);

/// CPU Architectural Protocol `GUID`.
pub const EFI_CPU_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB1,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

// --- Metronome Protocol Types ---

/// Function pointer to wait for a number of platform ticks.
pub type EfiMetronomeWaitForTick =
    unsafe extern "efiapi" fn(this: *mut EfiMetronomeArchProtocol, tick_number: u32) -> EfiStatus;

/// Metronome Architectural Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiMetronomeArchProtocol {
    /// Wait for platform timer ticks.
    pub wait_for_tick: EfiMetronomeWaitForTick,
    /// Period of platform's time source in 100ns units (between 10us and 200us).
    pub tick_period: u32,
}

// --- Timer Protocol Types ---

/// Timer interrupt notification callback.
pub type EfiTimerNotify = unsafe extern "efiapi" fn(time: u64);

/// Register timer interrupt notification callback.
pub type EfiTimerRegisterHandler = unsafe extern "efiapi" fn(
    this: *mut EfiTimerArchProtocol,
    notify_function: Option<EfiTimerNotify>,
) -> EfiStatus;

/// Set timer period in 100ns units.
pub type EfiTimerSetTimerPeriod =
    unsafe extern "efiapi" fn(this: *mut EfiTimerArchProtocol, timer_period: u64) -> EfiStatus;

/// Get timer period in 100ns units.
pub type EfiTimerGetTimerPeriod =
    unsafe extern "efiapi" fn(this: *mut EfiTimerArchProtocol, timer_period: *mut u64) -> EfiStatus;

/// Generate software timer interrupt.
pub type EfiTimerGenerateSoftInterrupt =
    unsafe extern "efiapi" fn(this: *mut EfiTimerArchProtocol) -> EfiStatus;

/// Timer Architectural Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTimerArchProtocol {
    /// Registers or unregisters timer interrupt handler.
    pub register_handler: EfiTimerRegisterHandler,
    /// Sets timer interrupt period.
    pub set_timer_period: EfiTimerSetTimerPeriod,
    /// Gets current timer period.
    pub get_timer_period: EfiTimerGetTimerPeriod,
    /// Triggers software timer interrupt.
    pub generate_soft_interrupt: EfiTimerGenerateSoftInterrupt,
}

// --- Watchdog Timer Protocol Types ---

/// Watchdog timer notify handler.
pub type EfiWatchdogTimerNotify = unsafe extern "efiapi" fn(time: u64);

/// Register watchdog handler.
pub type EfiWatchdogTimerRegisterHandler = unsafe extern "efiapi" fn(
    this: *mut EfiWatchdogTimerArchProtocol,
    notify_function: Option<EfiWatchdogTimerNotify>,
) -> EfiStatus;

/// Set watchdog timer period in 100ns units.
pub type EfiWatchdogTimerSetTimerPeriod = unsafe extern "efiapi" fn(
    this: *mut EfiWatchdogTimerArchProtocol,
    timer_period: u64,
) -> EfiStatus;

/// Get watchdog timer period in 100ns units.
pub type EfiWatchdogTimerGetTimerPeriod = unsafe extern "efiapi" fn(
    this: *mut EfiWatchdogTimerArchProtocol,
    timer_period: *mut u64,
) -> EfiStatus;

/// Watchdog Timer Architectural Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiWatchdogTimerArchProtocol {
    /// Registers a notification callback called before system reset.
    pub register_handler: EfiWatchdogTimerRegisterHandler,
    /// Sets watchdog timeout period in 100ns units.
    pub set_timer_period: EfiWatchdogTimerSetTimerPeriod,
    /// Retrieves current watchdog period.
    pub get_timer_period: EfiWatchdogTimerGetTimerPeriod,
}

// --- Security Protocols ---

/// Authenticate image file callback (Security Arch).
pub type EfiSecurityFileAuthenticationState = unsafe extern "efiapi" fn(
    this: *mut EfiSecurityArchProtocol,
    authentication_status: u32,
    file: *const EfiDevicePath,
) -> EfiStatus;

/// Security Architectural Protocol interface (PI 1.2.1+).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiSecurityArchProtocol {
    /// Verifies authentication status of an image file from a firmware volume.
    pub file_authentication_state: EfiSecurityFileAuthenticationState,
}

/// Authenticate image file callback (Security2 Arch).
pub type EfiSecurity2FileAuthentication = unsafe extern "efiapi" fn(
    this: *mut EfiSecurity2ArchProtocol,
    file: *const EfiDevicePath,
    file_buffer: *const c_void,
    file_size: usize,
    boot_policy: Bool,
) -> EfiStatus;

/// Security2 Architectural Protocol interface (PI 1.2.1+).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiSecurity2ArchProtocol {
    /// Measures or verifies a UEFI image prior to execution.
    pub file_authentication: EfiSecurity2FileAuthentication,
}

// --- CPU Architectural Protocol ---

/// Processor cache flush type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiCpuFlushType {
    /// Write back and invalidate data caches.
    WriteBackInvalidate = 0,
    /// Write back modified data caches.
    WriteBack = 1,
    /// Invalidate data caches without writing back.
    Invalidate = 2,
}

/// Processor initialization type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiCpuInitType {
    /// Normal CPU initialization.
    Init = 0,
}

/// Flush CPU data cache.
pub type EfiCpuFlushDataCache = unsafe extern "efiapi" fn(
    this: *mut EfiCpuArchProtocol,
    start: EfiPhysicalAddress,
    length: u64,
    flush_type: EfiCpuFlushType,
) -> EfiStatus;

/// Enable processor interrupts.
pub type EfiCpuEnableInterrupt =
    unsafe extern "efiapi" fn(this: *mut EfiCpuArchProtocol) -> EfiStatus;

/// Disable processor interrupts.
pub type EfiCpuDisableInterrupt =
    unsafe extern "efiapi" fn(this: *mut EfiCpuArchProtocol) -> EfiStatus;

/// Get interrupt enable state.
pub type EfiCpuGetInterruptState =
    unsafe extern "efiapi" fn(this: *mut EfiCpuArchProtocol, state: *mut Bool) -> EfiStatus;

/// Initialize processor.
pub type EfiCpuInit = unsafe extern "efiapi" fn(
    this: *mut EfiCpuArchProtocol,
    init_type: EfiCpuInitType,
) -> EfiStatus;

/// Architecture-independent interrupt handler function pointer.
pub type EfiCpuInterruptHandler =
    unsafe extern "efiapi" fn(interrupt_type: u32, system_context: *const c_void);

/// Register processor interrupt/exception handler.
pub type EfiCpuRegisterInterruptHandler = unsafe extern "efiapi" fn(
    this: *mut EfiCpuArchProtocol,
    interrupt_type: u32,
    interrupt_handler: Option<EfiCpuInterruptHandler>,
) -> EfiStatus;

/// Read internal processor timer counter.
pub type EfiCpuGetTimerValue = unsafe extern "efiapi" fn(
    this: *mut EfiCpuArchProtocol,
    timer_index: u32,
    timer_value: *mut u64,
    timer_period: *mut u64,
) -> EfiStatus;

/// Set physical memory caching/protection attributes.
pub type EfiCpuSetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiCpuArchProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// CPU Architectural Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiCpuArchProtocol {
    /// Flushes processor data caches.
    pub flush_data_cache: EfiCpuFlushDataCache,
    /// Enables CPU interrupts.
    pub enable_interrupt: EfiCpuEnableInterrupt,
    /// Disables CPU interrupts.
    pub disable_interrupt: EfiCpuDisableInterrupt,
    /// Queries CPU interrupt enable status.
    pub get_interrupt_state: EfiCpuGetInterruptState,
    /// Initializes processor subsystem.
    pub init: EfiCpuInit,
    /// Registers an interrupt or exception vector handler.
    pub register_interrupt_handler: EfiCpuRegisterInterruptHandler,
    /// Reads processor timer counter and period.
    pub get_timer_value: EfiCpuGetTimerValue,
    /// Modifies architectural memory caching attributes.
    pub set_memory_attributes: EfiCpuSetMemoryAttributes,
    /// Total number of hardware timers available in the CPU.
    pub number_of_timers: u32,
    /// Minimum required alignment (in bytes) for DMA buffer allocations.
    pub dma_buffer_alignment: u32,
}
