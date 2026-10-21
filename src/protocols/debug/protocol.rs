//! # Debug protocols
//!
//! The firmware interfaces for debug support and debug port.
//!
//! The debug support protocol has an architecture field
//! followed by four function entries. The debug port protocol
//! has four function entries.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::EfiStatus;

use super::types::InstructionSetArchitecture;

/// The debug support protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDebugSupportProtocol {
    /// The processor architecture.
    pub isa: InstructionSetArchitecture,
    /// Gets the maximum processor index.
    pub get_maximum_processor_index:
        unsafe extern "efiapi" fn(*mut EfiDebugSupportProtocol, *mut UIntN) -> EfiStatus,
    /// Registers a periodic callback.
    pub register_periodic_callback:
        unsafe extern "efiapi" fn(*mut EfiDebugSupportProtocol, UIntN, usize) -> EfiStatus,
    /// Registers an exception callback.
    pub register_exception_callback:
        unsafe extern "efiapi" fn(*mut EfiDebugSupportProtocol, UIntN, usize, isize) -> EfiStatus,
    /// Invalidates instruction cache.
    pub invalidate_instruction_cache: unsafe extern "efiapi" fn(
        *mut EfiDebugSupportProtocol,
        UIntN,
        *mut c_void,
        u64,
    ) -> EfiStatus,
}

/// The debug port protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDebugPortProtocol {
    /// Resets the debug port.
    pub reset: unsafe extern "efiapi" fn(*mut EfiDebugPortProtocol) -> EfiStatus,
    /// Writes data.
    pub write: unsafe extern "efiapi" fn(
        *mut EfiDebugPortProtocol,
        u32,
        *mut UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Reads data.
    pub read: unsafe extern "efiapi" fn(
        *mut EfiDebugPortProtocol,
        u32,
        *mut UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Polls for data.
    pub poll: unsafe extern "efiapi" fn(*mut EfiDebugPortProtocol) -> EfiStatus,
}

/// A borrowed debug support view.
pub struct DebugSupport<'a> {
    protocol: &'a EfiDebugSupportProtocol,
}

impl<'a> DebugSupport<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiDebugSupportProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDebugSupportProtocol {
        self.protocol
    }

    /// Gets the maximum processor index.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn maximum_processor_index(&self) -> Result<UIntN, EfiStatus> {
        let mut maximum: UIntN = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_maximum_processor_index)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut maximum,
            )
        };
        if status.is_success() {
            Ok(maximum)
        } else {
            Err(status)
        }
    }

    /// Returns the processor architecture.
    #[must_use]
    pub const fn isa(&self) -> InstructionSetArchitecture {
        self.protocol.isa
    }
}

impl core::fmt::Debug for DebugSupport<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DebugSupport")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed debug port view.
pub struct DebugPort<'a> {
    protocol: &'a EfiDebugPortProtocol,
}

impl<'a> DebugPort<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiDebugPortProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDebugPortProtocol {
        self.protocol
    }

    /// Resets the debug port.
    #[must_use]
    pub fn reset(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.reset)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Polls for available data.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Writes data to the debug port.
    ///
    /// Returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the write fails.
    pub fn write(&self, timeout: u32, data: &[u8]) -> Result<UIntN, EfiStatus> {
        let mut size = data.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            (self.protocol.write)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                timeout,
                &raw mut size,
                data.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Reads data from the debug port.
    ///
    /// Returns the number of bytes actually read.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the read fails.
    pub fn read(&self, timeout: u32, buffer: &mut [u8]) -> Result<UIntN, EfiStatus> {
        let mut size = buffer.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            (self.protocol.read)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                timeout,
                &raw mut size,
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }
}

impl core::fmt::Debug for DebugPort<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DebugPort")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
