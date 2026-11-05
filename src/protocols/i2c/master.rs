//! # I2C Master and I/O protocols
//!
//! UEFI PI 1.3+ `EFI_I2C_MASTER_PROTOCOL` and `EFI_I2C_IO_PROTOCOL`.

use super::guid::{EFI_I2C_IO_PROTOCOL_GUID, EFI_I2C_MASTER_PROTOCOL_GUID};
use crate::abi::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// A 10-bit slave address flag.
pub const I2C_ADDRESSING_10_BIT: usize = 0x8000_0000;

/// I2C Operation flags.
pub mod i2c_flags {
    /// Read operation when set, write when clear.
    pub const READ: u32 = 0x0000_0001;
    /// `SMBus` operation.
    pub const SMBUS_OPERATION: u32 = 0x0001_0000;
    /// `SMBus` block operation.
    pub const SMBUS_BLOCK: u32 = 0x0002_0000;
    /// `SMBus` process call operation.
    pub const SMBUS_PROCESS_CALL: u32 = 0x0004_0000;
    /// `SMBus` packet error code (PEC).
    pub const SMBUS_PEC: u32 = 0x0008_0000;
}

/// I2C Controller Capabilities.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EfiI2cControllerCapabilities {
    /// Structure size in bytes.
    pub structure_size_in_bytes: u32,
    /// Maximum receive bytes in transaction.
    pub maximum_receive_bytes: u32,
    /// Maximum transmit bytes in transaction.
    pub maximum_transmit_bytes: u32,
    /// Maximum total bytes in transaction.
    pub maximum_total_bytes: u32,
}

/// Description of an I2C operation segment.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EfiI2cOperation {
    /// Operation flags (e.g. `i2c_flags::READ`).
    pub flags: u32,
    /// Number of bytes to send or receive. Zero indicates ping.
    pub length_in_bytes: u32,
    /// Data buffer pointer.
    pub buffer: *mut u8,
}

/// Description of an I2C request packet containing one or more operations.
#[repr(C)]
pub struct EfiI2cRequestPacket {
    /// Number of operations in packet.
    pub operation_count: UIntN,
    /// Flexible array member in C. Must contain at least one operation.
    pub operations: [EfiI2cOperation; 1],
}

/// Raw UEFI PI `EFI_I2C_MASTER_PROTOCOL`.
#[repr(C)]
pub struct EfiI2cMasterProtocol {
    /// Set bus clock frequency in Hertz.
    pub set_bus_frequency: unsafe extern "efiapi" fn(
        this: *const EfiI2cMasterProtocol,
        bus_clock_hertz: *mut UIntN,
    ) -> EfiStatus,
    /// Reset the controller.
    pub reset: unsafe extern "efiapi" fn(this: *const EfiI2cMasterProtocol) -> EfiStatus,
    /// Start an I2C transaction.
    pub start_request: unsafe extern "efiapi" fn(
        this: *const EfiI2cMasterProtocol,
        slave_address: UIntN,
        request_packet: *mut EfiI2cRequestPacket,
        event: *mut c_void,
        i2c_status: *mut EfiStatus,
    ) -> EfiStatus,
    /// Capabilities of the I2C host controller.
    pub capabilities: *const EfiI2cControllerCapabilities,
}

/// Safe wrapper around `EfiI2cMasterProtocol`.
pub struct I2cMasterProtocol<'a> {
    raw: &'a EfiI2cMasterProtocol,
}

impl<'a> I2cMasterProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = EFI_I2C_MASTER_PROTOCOL_GUID;

    /// Creates a safe wrapper from a raw protocol reference.
    #[must_use]
    pub const fn new(raw: &'a EfiI2cMasterProtocol) -> Self {
        Self { raw }
    }

    /// Returns the controller capabilities if provided.
    #[must_use]
    pub fn capabilities(&self) -> Option<EfiI2cControllerCapabilities> {
        if self.raw.capabilities.is_null() {
            None
        } else {
            // SAFETY: Verified non-null above.
            Some(unsafe { *self.raw.capabilities })
        }
    }

    /// Sets the bus clock frequency in Hertz, returning the actual negotiated frequency.
    ///
    /// # Errors
    /// Returns an error if the controller cannot support this frequency.
    pub fn set_bus_frequency(&self, requested_hz: usize) -> Result<usize, EfiStatus> {
        let mut actual = requested_hz as UIntN;
        let status = unsafe { (self.raw.set_bus_frequency)(self.raw, &raw mut actual) };
        if status.is_success() {
            Ok(actual as usize)
        } else {
            Err(status)
        }
    }

    /// Resets the I2C host controller.
    ///
    /// # Errors
    /// Returns an error if reset fails or controller is busy.
    pub fn reset(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.reset)(self.raw) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Synchronously starts an I2C transaction request.
    ///
    /// # Safety
    /// `request_packet` must point to a valid `EfiI2cRequestPacket` with properly initialized buffer pointers.
    ///
    /// # Errors
    /// Returns an error if the transaction fails or the device responds with NACK.
    pub unsafe fn start_request_sync(
        &self,
        slave_address: usize,
        request_packet: &mut EfiI2cRequestPacket,
    ) -> Result<(), EfiStatus> {
        let mut completion_status = EfiStatus::SUCCESS;
        let status = unsafe {
            (self.raw.start_request)(
                self.raw,
                slave_address as UIntN,
                core::ptr::from_mut::<EfiI2cRequestPacket>(request_packet),
                core::ptr::null_mut(),
                &raw mut completion_status,
            )
        };
        if status.is_success() {
            if completion_status.is_success() {
                Ok(())
            } else {
                Err(completion_status)
            }
        } else {
            Err(status)
        }
    }
}

/// Raw UEFI PI `EFI_I2C_IO_PROTOCOL`.
#[repr(C)]
pub struct EfiI2cIoProtocol {
    /// Queue request for execution.
    pub queue_request: unsafe extern "efiapi" fn(
        this: *const EfiI2cIoProtocol,
        slave_address_index: UIntN,
        event: *mut c_void,
        request_packet: *mut EfiI2cRequestPacket,
        i2c_status: *mut EfiStatus,
    ) -> EfiStatus,
    /// Device GUID.
    pub device_guid: *const EfiGuid,
    /// Device index.
    pub device_index: u32,
    /// Hardware revision.
    pub hardware_revision: u32,
    /// Capabilities pointer.
    pub capabilities: *const EfiI2cControllerCapabilities,
}

/// Safe wrapper around `EfiI2cIoProtocol`.
pub struct I2cIoProtocol<'a> {
    raw: &'a EfiI2cIoProtocol,
}

impl<'a> I2cIoProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = EFI_I2C_IO_PROTOCOL_GUID;

    /// Creates a safe wrapper from a raw protocol reference.
    #[must_use]
    pub const fn new(raw: &'a EfiI2cIoProtocol) -> Self {
        Self { raw }
    }

    /// Device GUID if available.
    #[must_use]
    pub fn device_guid(&self) -> Option<EfiGuid> {
        if self.raw.device_guid.is_null() {
            None
        } else {
            // SAFETY: Verified non-null above.
            Some(unsafe { *self.raw.device_guid })
        }
    }

    /// Device index within the system.
    #[must_use]
    pub const fn device_index(&self) -> u32 {
        self.raw.device_index
    }

    /// Hardware revision.
    #[must_use]
    pub const fn hardware_revision(&self) -> u32 {
        self.raw.hardware_revision
    }

    /// Synchronously queues an I2C transaction request.
    ///
    /// # Safety
    /// `request_packet` must point to a valid `EfiI2cRequestPacket` with properly initialized buffer pointers.
    ///
    /// # Errors
    /// Returns an error if the transaction fails or device does not respond.
    pub unsafe fn queue_request_sync(
        &self,
        slave_address_index: usize,
        request_packet: &mut EfiI2cRequestPacket,
    ) -> Result<(), EfiStatus> {
        let mut completion_status = EfiStatus::SUCCESS;
        let status = unsafe {
            (self.raw.queue_request)(
                self.raw,
                slave_address_index as UIntN,
                core::ptr::null_mut(),
                core::ptr::from_mut::<EfiI2cRequestPacket>(request_packet),
                &raw mut completion_status,
            )
        };
        if status.is_success() {
            if completion_status.is_success() {
                Ok(())
            } else {
                Err(completion_status)
            }
        } else {
            Err(status)
        }
    }
}
