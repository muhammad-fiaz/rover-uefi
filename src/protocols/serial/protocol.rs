//! # Serial I/O protocol
//!
//! The firmware interface for UART-style serial devices.
//!
//! The struct holds a revision, six function entries, the
//! mode pointer and the device-type pointer in specification
//! order. Wrappers use caller slices; the buffer size in/out
//! parameter is handled through a stack local.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

use super::types::{ParityType, StopBitsType};

/// The serial I/O mode structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiSerialIoMode {
    /// The control-bit mask supported by the device.
    pub control_mask: u32,
    /// The current timeout in microseconds.
    pub timeout: u32,
    /// The current baud rate.
    pub baud_rate: u64,
    /// The receive FIFO depth.
    pub receive_fifo_depth: u32,
    /// The data-bits setting.
    pub data_bits: u32,
    /// The parity setting.
    pub parity: u32,
    /// The stop-bits setting.
    pub stop_bits: u32,
}

/// The serial I/O protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSerialIoProtocol {
    /// The protocol revision.
    pub revision: u32,
    /// Resets the device.
    pub reset: unsafe extern "efiapi" fn(*mut EfiSerialIoProtocol) -> EfiStatus,
    /// Sets device attributes.
    pub set_attributes: unsafe extern "efiapi" fn(
        *mut EfiSerialIoProtocol,
        u64,
        u32,
        u32,
        ParityType,
        u8,
        StopBitsType,
    ) -> EfiStatus,
    /// Sets control bits.
    pub set_control: unsafe extern "efiapi" fn(*mut EfiSerialIoProtocol, u32) -> EfiStatus,
    /// Gets control bits.
    pub get_control: unsafe extern "efiapi" fn(*mut EfiSerialIoProtocol, *mut u32) -> EfiStatus,
    /// Writes data.
    pub write: unsafe extern "efiapi" fn(
        *mut EfiSerialIoProtocol,
        *mut UIntN,
        *mut core::ffi::c_void,
    ) -> EfiStatus,
    /// Reads data.
    pub read: unsafe extern "efiapi" fn(
        *mut EfiSerialIoProtocol,
        *mut UIntN,
        *mut core::ffi::c_void,
    ) -> EfiStatus,
    /// The mode data for the device.
    pub mode: *mut EfiSerialIoMode,
    /// The device-type identifier, revision 1.1.
    pub device_type_guid: *mut EfiGuid,
}

/// A borrowed serial view.
pub struct SerialIo<'a> {
    protocol: &'a EfiSerialIoProtocol,
}

impl<'a> SerialIo<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiSerialIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSerialIoProtocol {
        self.protocol
    }

    /// Resets the device.
    #[must_use]
    pub fn reset(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.reset)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Writes data to the device.
    ///
    /// Returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the write fails.
    pub fn write(&self, data: &[u8]) -> Result<UIntN, EfiStatus> {
        let mut size = data.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            (self.protocol.write)(
                core::ptr::from_ref(self.protocol).cast_mut(),
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

    /// Reads data from the device.
    ///
    /// Returns the number of bytes actually read.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the read fails.
    pub fn read(&self, buffer: &mut [u8]) -> Result<UIntN, EfiStatus> {
        let mut size = buffer.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            (self.protocol.read)(
                core::ptr::from_ref(self.protocol).cast_mut(),
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

    /// Gets control bits.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the read fails.
    pub fn get_control(&self) -> Result<u32, EfiStatus> {
        let mut control: u32 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_control)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut control,
            )
        };
        if status.is_success() {
            Ok(control)
        } else {
            Err(status)
        }
    }

    /// Sets control bits.
    #[must_use]
    pub fn set_control(&self, control: u32) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_control)(core::ptr::from_ref(self.protocol).cast_mut(), control)
        }
    }

    /// Sets device attributes.
    #[must_use]
    pub fn set_attributes(
        &self,
        baud_rate: u64,
        receive_fifo_depth: u32,
        timeout: u32,
        parity: ParityType,
        data_bits: u8,
        stop_bits: StopBitsType,
    ) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_attributes)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                baud_rate,
                receive_fifo_depth,
                timeout,
                parity,
                data_bits,
                stop_bits,
            )
        }
    }
}

impl core::fmt::Debug for SerialIo<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SerialIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
