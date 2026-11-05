//! # CPU architectural protocol
//!
//! Defined in PI Specification Volume 2: DXE.
//!
//! Abstracts processor-specific functions from the DXE Foundation, including
//! cache flushing, interrupt enable/disable, interrupt/exception hooking,
//! reading internal processor timers, and memory attribute configuration.

use core::ffi::c_void;

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// The CPU architectural protocol `GUID`.
pub const EFI_CPU_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB1,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// CPU cache flush type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EfiCpuFlushType {
    /// Write-back dirty cache lines and invalidate.
    WriteBackInvalidate = 0,
    /// Write-back dirty cache lines without invalidating.
    WriteBack = 1,
    /// Invalidate cache lines without writing back.
    Invalidate = 2,
}

/// CPU initialization type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EfiCpuInitType {
    /// Standard processor INIT.
    Init = 0,
}

/// Interrupt or exception callback handler signature.
pub type EfiCpuInterruptHandler = unsafe extern "efiapi" fn(isize, *const c_void);

/// Flushes processor data cache.
pub type EfiCpuFlushDataCache = unsafe extern "efiapi" fn(
    *mut EfiCpuArchProtocol,
    EfiPhysicalAddress,
    u64,
    EfiCpuFlushType,
) -> EfiStatus;

/// Enables interrupt processing on the processor.
pub type EfiCpuEnableInterrupt = unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol) -> EfiStatus;

/// Disables interrupt processing on the processor.
pub type EfiCpuDisableInterrupt = unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol) -> EfiStatus;

/// Queries the processor's current interrupt enable state.
pub type EfiCpuGetInterruptState =
    unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol, *mut Bool) -> EfiStatus;

/// Generates an INIT on the processor.
pub type EfiCpuInit =
    unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol, EfiCpuInitType) -> EfiStatus;

/// Registers an interrupt or exception handler for the processor.
pub type EfiCpuRegisterInterruptHandler = unsafe extern "efiapi" fn(
    *mut EfiCpuArchProtocol,
    isize,
    Option<EfiCpuInterruptHandler>,
) -> EfiStatus;

/// Reads a processor timer value.
pub type EfiCpuGetTimerValue =
    unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol, u32, *mut u64, *mut u64) -> EfiStatus;

/// Modifies processor memory attributes for a physical address range.
pub type EfiCpuSetMemoryAttributes =
    unsafe extern "efiapi" fn(*mut EfiCpuArchProtocol, EfiPhysicalAddress, u64, u64) -> EfiStatus;

/// The CPU architectural protocol interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiCpuArchProtocol {
    /// Flushes a range of processor data cache.
    pub flush_data_cache: EfiCpuFlushDataCache,
    /// Enables interrupts on the processor.
    pub enable_interrupt: EfiCpuEnableInterrupt,
    /// Disables interrupts on the processor.
    pub disable_interrupt: EfiCpuDisableInterrupt,
    /// Retrieves current interrupt enable state.
    pub get_interrupt_state: EfiCpuGetInterruptState,
    /// Generates a processor INIT.
    pub init: EfiCpuInit,
    /// Hooks or unhooks an interrupt or exception vector.
    pub register_interrupt_handler: EfiCpuRegisterInterruptHandler,
    /// Reads an internal processor timer.
    pub get_timer_value: EfiCpuGetTimerValue,
    /// Configures memory attributes for a physical range.
    pub set_memory_attributes: EfiCpuSetMemoryAttributes,
    /// Number of timers available in the processor.
    pub number_of_timers: u32,
    /// Alignment required for DMA buffer allocations in bytes.
    pub dma_buffer_alignment: u32,
}

/// A safe wrapper for [`EfiCpuArchProtocol`].
pub struct CpuArch<'a> {
    protocol: &'a EfiCpuArchProtocol,
}

impl<'a> CpuArch<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiCpuArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns a reference to the raw protocol structure.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiCpuArchProtocol {
        self.protocol
    }

    /// Returns the number of timers available in the processor.
    #[must_use]
    pub const fn number_of_timers(&self) -> u32 {
        self.protocol.number_of_timers
    }

    /// Returns the DMA buffer alignment requirement in bytes.
    #[must_use]
    pub const fn dma_buffer_alignment(&self) -> u32 {
        self.protocol.dma_buffer_alignment
    }

    /// Flushes processor data cache for the specified physical address range.
    ///
    /// # Errors
    ///
    /// Returns an error if the flush type is unsupported or a device error occurs.
    pub fn flush_data_cache(
        &self,
        start: EfiPhysicalAddress,
        length: u64,
        flush_type: EfiCpuFlushType,
    ) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status = unsafe { (self.protocol.flush_data_cache)(ptr, start, length, flush_type) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Enables interrupt processing on the processor.
    ///
    /// # Errors
    ///
    /// Returns an error if interrupts could not be enabled.
    pub fn enable_interrupt(&self) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status = unsafe { (self.protocol.enable_interrupt)(ptr) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Disables interrupt processing on the processor.
    ///
    /// # Errors
    ///
    /// Returns an error if interrupts could not be disabled.
    pub fn disable_interrupt(&self) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status = unsafe { (self.protocol.disable_interrupt)(ptr) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Queries whether interrupts are currently enabled on the processor.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_interrupt_state(&self) -> Result<bool, EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let mut state: Bool = 0;
        let status = unsafe { (self.protocol.get_interrupt_state)(ptr, &raw mut state) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(state != 0)
        }
    }

    /// Reads a processor timer value and its frequency period in femtoseconds.
    ///
    /// Returns `(timer_value, timer_period_femtoseconds)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the timer index is invalid or the timer is unreadable.
    pub fn get_timer_value(&self, timer_index: u32) -> Result<(u64, u64), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let mut timer_value: u64 = 0;
        let mut timer_period: u64 = 0;
        let status = unsafe {
            (self.protocol.get_timer_value)(
                ptr,
                timer_index,
                &raw mut timer_value,
                &raw mut timer_period,
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok((timer_value, timer_period))
        }
    }

    /// Sets memory caching attributes for a physical address range.
    ///
    /// # Errors
    ///
    /// Returns an error if the address range or attribute combination is invalid.
    pub fn set_memory_attributes(
        &self,
        base_address: EfiPhysicalAddress,
        length: u64,
        attributes: u64,
    ) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status =
            unsafe { (self.protocol.set_memory_attributes)(ptr, base_address, length, attributes) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Debug for CpuArch<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CpuArch")
            .field("number_of_timers", &self.protocol.number_of_timers)
            .field("dma_buffer_alignment", &self.protocol.dma_buffer_alignment)
            .finish()
    }
}
