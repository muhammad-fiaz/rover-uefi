//! # SPI Host Controller protocol
//!
//! UEFI PI 1.8+ `EFI_SPI_HC_PROTOCOL`.

use super::guid::EFI_SPI_HC_PROTOCOL_GUID;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// SPI Host Controller attributes.
pub mod hc_attributes {
    /// Host supports write-only operations.
    pub const SUPPORTS_WRITE_ONLY_OPERATIONS: u32 = 1 << 0;
    /// Host supports read-only operations.
    pub const SUPPORTS_READ_ONLY_OPERATIONS: u32 = 1 << 1;
    /// Host supports write-then-read operations.
    pub const SUPPORTS_WRITE_THEN_READ_OPERATIONS: u32 = 1 << 2;
    /// Transmit frame in MSB.
    pub const TX_FRAME_IN_MOST_SIGNIFICANT_BITS: u32 = 1 << 3;
    /// Receive frame in MSB.
    pub const RX_FRAME_IN_MOST_SIGNIFICANT_BITS: u32 = 1 << 4;
    /// Host supports 2-bit data bus width.
    pub const SUPPORTS_2_BIT_DATA_BUS_WIDTH: u32 = 1 << 5;
    /// Host supports 4-bit data bus width.
    pub const SUPPORTS_4_BIT_DATA_BUS_WIDTH: u32 = 1 << 6;
    /// Host supports 8-bit data bus width.
    pub const SUPPORTS_8_BIT_DATA_BUS_WIDTH: u32 = 1 << 7;
    /// Transfer size includes opcode.
    pub const TRANSFER_SIZE_INCLUDES_OPCODE: u32 = 1 << 8;
    /// Transfer size includes address.
    pub const TRANSFER_SIZE_INCLUDES_ADDRESS: u32 = 1 << 9;
}

/// SPI Transaction types.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpiTransactionType {
    /// Data flowing in both directions between host and peripheral.
    FullDuplex = 0,
    /// Data flowing from host to peripheral.
    WriteOnly = 1,
    /// Data flowing from peripheral to host.
    ReadOnly = 2,
    /// Write command/address followed by read data.
    WriteThenRead = 3,
}

/// SPI Bus Transaction description.
#[repr(C)]
pub struct EfiSpiBusTransaction {
    /// Pointer to `EFI_SPI_PERIPHERAL`.
    pub spi_peripheral: *const c_void,
    /// Transaction type.
    pub transaction_type: SpiTransactionType,
    /// Set true when transaction debugging is desired.
    pub debug_transaction: bool,
    /// SPI bus width in bits (1, 2, 4, 8).
    pub bus_width: u32,
    /// Frame size in bits (1-32).
    pub frame_size: u32,
    /// Length of write buffer in bytes.
    pub write_bytes: u32,
    /// Write data buffer.
    pub write_buffer: *const u8,
    /// Length of read buffer in bytes.
    pub read_bytes: u32,
    /// Read data buffer.
    pub read_buffer: *mut u8,
}

/// Raw UEFI PI `EFI_SPI_HC_PROTOCOL`.
#[repr(C)]
pub struct EfiSpiHcProtocol {
    /// Host controller attributes.
    pub attributes: u32,
    /// Mask of supported frame sizes.
    pub frame_size_support_mask: u32,
    /// Maximum transfer size in bytes.
    pub maximum_transfer_bytes: u32,
    /// Assert or deassert chip select line.
    pub chip_select: unsafe extern "efiapi" fn(
        this: *const EfiSpiHcProtocol,
        spi_peripheral: *const c_void,
        pin_value: bool,
    ) -> EfiStatus,
    /// Configure SPI clock.
    pub clock: unsafe extern "efiapi" fn(
        this: *const EfiSpiHcProtocol,
        spi_peripheral: *const c_void,
        clock_hz: *mut u32,
    ) -> EfiStatus,
    /// Execute SPI transaction.
    pub transaction: unsafe extern "efiapi" fn(
        this: *const EfiSpiHcProtocol,
        bus_transaction: *mut EfiSpiBusTransaction,
    ) -> EfiStatus,
}

/// Safe wrapper around `EfiSpiHcProtocol`.
pub struct SpiHcProtocol<'a> {
    raw: &'a EfiSpiHcProtocol,
}

impl<'a> SpiHcProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = EFI_SPI_HC_PROTOCOL_GUID;

    /// Creates a safe wrapper from a raw protocol reference.
    #[must_use]
    pub const fn new(raw: &'a EfiSpiHcProtocol) -> Self {
        Self { raw }
    }

    /// Controller attributes.
    #[must_use]
    pub const fn attributes(&self) -> u32 {
        self.raw.attributes
    }

    /// Mask of supported frame sizes.
    #[must_use]
    pub const fn frame_size_support_mask(&self) -> u32 {
        self.raw.frame_size_support_mask
    }

    /// Maximum transfer size in bytes.
    #[must_use]
    pub const fn maximum_transfer_bytes(&self) -> u32 {
        self.raw.maximum_transfer_bytes
    }

    /// Manipulates the chip select pin.
    ///
    /// # Safety
    /// `spi_peripheral` must be a valid pointer to an `EFI_SPI_PERIPHERAL` structure or null.
    ///
    /// # Errors
    /// Returns an error if the host controller fails to assert/deassert chip select.
    pub unsafe fn chip_select(
        &self,
        spi_peripheral: *const c_void,
        pin_value: bool,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.chip_select)(self.raw, spi_peripheral, pin_value) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Configures the clock frequency.
    ///
    /// # Safety
    /// `spi_peripheral` must be a valid pointer to an `EFI_SPI_PERIPHERAL` structure or null.
    ///
    /// # Errors
    /// Returns an error if the host controller cannot support the frequency.
    pub unsafe fn set_clock(
        &self,
        spi_peripheral: *const c_void,
        requested_hz: u32,
    ) -> Result<u32, EfiStatus> {
        let mut actual_hz = requested_hz;
        let status = unsafe { (self.raw.clock)(self.raw, spi_peripheral, &raw mut actual_hz) };
        if status.is_success() {
            Ok(actual_hz)
        } else {
            Err(status)
        }
    }

    /// Executes a transaction.
    ///
    /// # Safety
    /// `bus_transaction` must point to a valid `EfiSpiBusTransaction` structure with valid buffer pointers.
    ///
    /// # Errors
    /// Returns an error if the transaction fails on the SPI bus.
    pub unsafe fn execute_transaction(
        &self,
        bus_transaction: &mut EfiSpiBusTransaction,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.transaction)(
                self.raw,
                core::ptr::from_mut::<EfiSpiBusTransaction>(bus_transaction),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
