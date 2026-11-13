//! # Generic Memory Test Protocol
//!
//! Provides platform memory diagnostics, testing, and error span isolation
//! during pre-boot initialization.

use crate::base::EfiStatus;

/// Generic Memory Test Protocol GUID: `{30e3d083-9d3c-4fc3-a462-17c7e12850e3}`.
pub const GENERIC_MEMORY_TEST_PROTOCOL_GUID: crate::base::EfiGuid = crate::base::EfiGuid::new(
    0x30E3_D083,
    0x9D3C,
    0x4FC3,
    [0xA4, 0x62, 0x17, 0xC7, 0xE1, 0x28, 0x50, 0xE3],
);

/// Memory test coverage level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MemoryTestLevel {
    /// Quick walk of memory range boundaries.
    Quick = 0,
    /// Sparse walking bit pattern test.
    Sparse = 1,
    /// Extensive cell-by-cell write/read pattern validation.
    Extensive = 2,
}

/// Function pointer for `InitializeMemoryTest`.
pub type EfiInitializeMemoryTest = unsafe extern "efiapi" fn(
    this: *const EfiGenericMemoryTestProtocol,
    level: MemoryTestLevel,
    require_key_stroke: *mut bool,
) -> EfiStatus;

/// Function pointer for `PerformMemoryTest`.
pub type EfiPerformMemoryTest = unsafe extern "efiapi" fn(
    this: *const EfiGenericMemoryTestProtocol,
    tested_operation: *mut u32,
    percent_complete: *mut u32,
    error_detected: *mut bool,
) -> EfiStatus;

/// Function pointer for `Finished`.
pub type EfiMemoryTestFinished =
    unsafe extern "efiapi" fn(this: *const EfiGenericMemoryTestProtocol) -> EfiStatus;

/// Low-level Generic Memory Test Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiGenericMemoryTestProtocol {
    /// Initializes testing parameters and coverage.
    pub initialize_memory_test: EfiInitializeMemoryTest,
    /// Performs an incremental slice of memory testing.
    pub perform_memory_test: EfiPerformMemoryTest,
    /// Concludes memory test phase.
    pub finished: EfiMemoryTestFinished,
}

/// Status of an incremental memory test step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryTestProgress {
    /// Current testing operation identifier.
    pub operation: u32,
    /// Percentage completed (0 to 100).
    pub percent_complete: u32,
    /// Whether an uncorrectable memory fault was detected.
    pub error_detected: bool,
}

/// Safe client wrapper around the Generic Memory Test Protocol.
#[derive(Debug, Clone, Copy)]
pub struct GenericMemoryTest {
    proto: *const EfiGenericMemoryTestProtocol,
}

impl GenericMemoryTest {
    /// Wraps a Generic Memory Test protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiGenericMemoryTestProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiGenericMemoryTestProtocol) -> Self {
        Self { proto }
    }

    /// Initializes memory testing at the specified coverage level.
    ///
    /// Returns `true` if platform requires a keystroke before proceeding.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if level is unsupported.
    pub fn initialize(&self, level: MemoryTestLevel) -> Result<bool, EfiStatus> {
        let mut require_key = false;
        let status = unsafe {
            let func = (*self.proto).initialize_memory_test;
            func(self.proto, level, &raw mut require_key)
        };
        if status == EfiStatus::SUCCESS {
            Ok(require_key)
        } else {
            Err(status)
        }
    }

    /// Performs an incremental test step.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_READY` if not initialized.
    /// Returns `EfiStatus::DEVICE_ERROR` on fatal test failure.
    pub fn perform_step(&self) -> Result<MemoryTestProgress, EfiStatus> {
        let mut op = 0_u32;
        let mut pct = 0_u32;
        let mut err = false;
        let status = unsafe {
            let func = (*self.proto).perform_memory_test;
            func(self.proto, &raw mut op, &raw mut pct, &raw mut err)
        };
        if status == EfiStatus::SUCCESS {
            Ok(MemoryTestProgress {
                operation: op,
                percent_complete: pct,
                error_detected: err,
            })
        } else {
            Err(status)
        }
    }

    /// Concludes the memory testing sequence.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_READY` if test was not completed.
    pub fn finished(&self) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).finished;
            func(self.proto)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
