//! # SPI I/O Protocol
//!
//! Defined in the UEFI PI Specification, this protocol provides managed SPI data
//! transactions between the SPI host controller and a SPI peripheral device.

use super::hc::SpiTransactionType;
use crate::base::EfiStatus;
use core::ffi::c_void;

/// SPI I/O attribute: supports 2-bit data bus width.
pub const SPI_IO_SUPPORTS_2_BIT_DATA_BUS_WIDTH: u32 = 1 << 0;
/// SPI I/O attribute: supports 4-bit data bus width.
pub const SPI_IO_SUPPORTS_4_BIT_DATA_BUS_WIDTH: u32 = 1 << 1;
/// SPI I/O attribute: supports 8-bit data bus width.
pub const SPI_IO_SUPPORTS_8_BIT_DATA_BUS_WIDTH: u32 = 1 << 2;
/// SPI I/O attribute: transfer size includes opcode byte.
pub const SPI_IO_TRANSFER_SIZE_INCLUDES_OPCODE: u32 = 1 << 3;
/// SPI I/O attribute: transfer size includes address bytes.
pub const SPI_IO_TRANSFER_SIZE_INCLUDES_ADDRESS: u32 = 1 << 4;

/// Function pointer for `EFI_SPI_IO_PROTOCOL_TRANSACTION`.
pub type EfiSpiIoProtocolTransaction = unsafe extern "efiapi" fn(
    this: *const EfiSpiIoProtocol,
    transaction_type: SpiTransactionType,
    debug_transaction: bool,
    clock_hz: u32,
    bus_width: u32,
    frame_size: u32,
    write_bytes: u32,
    write_buffer: *const u8,
    read_bytes: u32,
    read_buffer: *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_SPI_IO_PROTOCOL_UPDATE_SPI_PERIPHERAL`.
pub type EfiSpiIoProtocolUpdateSpiPeripheral = unsafe extern "efiapi" fn(
    this: *const EfiSpiIoProtocol,
    spi_peripheral: *const c_void,
) -> EfiStatus;

/// Raw ABI protocol structure for SPI I/O (`EFI_SPI_IO_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSpiIoProtocol {
    /// Pointer to current peripheral descriptor.
    pub spi_peripheral: *const c_void,
    /// Pointer to original peripheral descriptor.
    pub original_spi_peripheral: *const c_void,
    /// Bitmask of supported frame sizes.
    pub frame_size_support_mask: u32,
    /// Maximum supported transfer length in bytes.
    pub maximum_transfer_bytes: u32,
    /// Transaction and bus width attributes.
    pub attributes: u32,
    /// Pointer to legacy SPI controller protocol if present.
    pub legacy_spi_protocol: *const c_void,
    /// Initiates a SPI transaction.
    pub transaction: EfiSpiIoProtocolTransaction,
    /// Updates the peripheral descriptor associated with this I/O instance.
    pub update_spi_peripheral: EfiSpiIoProtocolUpdateSpiPeripheral,
}

/// Safe wrapper around [`EfiSpiIoProtocol`].
pub struct SpiIo<'a> {
    proto: &'a EfiSpiIoProtocol,
}

impl<'a> SpiIo<'a> {
    /// Wraps a reference to [`EfiSpiIoProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSpiIoProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSpiIoProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSpiIoProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns the maximum transfer length in bytes.
    #[must_use]
    pub const fn maximum_transfer_bytes(&self) -> u32 {
        self.proto.maximum_transfer_bytes
    }

    /// Returns the frame size support bitmask.
    #[must_use]
    pub const fn frame_size_support_mask(&self) -> u32 {
        self.proto.frame_size_support_mask
    }

    /// Returns the capability attribute flags.
    #[must_use]
    pub const fn attributes(&self) -> u32 {
        self.proto.attributes
    }

    /// Executes a full-duplex SPI transaction where equal amounts of data are written and read simultaneously.
    ///
    /// # Errors
    ///
    /// Returns an error status if the transaction cannot be completed or buffers differ in length.
    pub fn full_duplex(
        &self,
        write_buf: &[u8],
        read_buf: &mut [u8],
        bus_width: u32,
        frame_size: u32,
        clock_hz: u32,
    ) -> Result<(), EfiStatus> {
        if write_buf.len() != read_buf.len() {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }

        let len = u32::try_from(write_buf.len()).unwrap_or(u32::MAX);
        let status = unsafe {
            (self.proto.transaction)(
                self.proto,
                SpiTransactionType::FullDuplex,
                false,
                clock_hz,
                bus_width,
                frame_size,
                len,
                write_buf.as_ptr(),
                len,
                read_buf.as_mut_ptr(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Executes a write-only SPI transaction.
    ///
    /// # Errors
    ///
    /// Returns an error status if writing to the SPI peripheral fails.
    pub fn write_only(
        &self,
        write_buf: &[u8],
        bus_width: u32,
        frame_size: u32,
        clock_hz: u32,
    ) -> Result<(), EfiStatus> {
        let len = u32::try_from(write_buf.len()).unwrap_or(u32::MAX);
        let status = unsafe {
            (self.proto.transaction)(
                self.proto,
                SpiTransactionType::WriteOnly,
                false,
                clock_hz,
                bus_width,
                frame_size,
                len,
                write_buf.as_ptr(),
                0,
                core::ptr::null_mut(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Executes a read-only SPI transaction.
    ///
    /// # Errors
    ///
    /// Returns an error status if reading from the SPI peripheral fails.
    pub fn read_only(
        &self,
        read_buf: &mut [u8],
        bus_width: u32,
        frame_size: u32,
        clock_hz: u32,
    ) -> Result<(), EfiStatus> {
        let len = u32::try_from(read_buf.len()).unwrap_or(u32::MAX);
        let status = unsafe {
            (self.proto.transaction)(
                self.proto,
                SpiTransactionType::ReadOnly,
                false,
                clock_hz,
                bus_width,
                frame_size,
                0,
                core::ptr::null(),
                len,
                read_buf.as_mut_ptr(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Executes a write-then-read SPI transaction (commonly used for SPI flash commands followed by data read).
    ///
    /// # Errors
    ///
    /// Returns an error status if the command or read sequence fails.
    pub fn write_then_read(
        &self,
        write_buf: &[u8],
        read_buf: &mut [u8],
        bus_width: u32,
        frame_size: u32,
        clock_hz: u32,
    ) -> Result<(), EfiStatus> {
        let w_len = u32::try_from(write_buf.len()).unwrap_or(u32::MAX);
        let r_len = u32::try_from(read_buf.len()).unwrap_or(u32::MAX);
        let status = unsafe {
            (self.proto.transaction)(
                self.proto,
                SpiTransactionType::WriteThenRead,
                false,
                clock_hz,
                bus_width,
                frame_size,
                w_len,
                write_buf.as_ptr(),
                r_len,
                read_buf.as_mut_ptr(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
