//! # SPI NOR Flash protocol
//!
//! UEFI PI 1.6+ `EFI_SPI_NOR_FLASH_PROTOCOL` and extended SPI NOR Flash protocol.

use super::guid::{EFI_SPI_NOR_FLASH_PROTOCOL_GUID, SPI_NOR_FLASH_PROTOCOL_EX_GUID};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Raw UEFI PI `EFI_SPI_NOR_FLASH_PROTOCOL`.
#[repr(C)]
pub struct EfiSpiNorFlashProtocol {
    /// Pointer to `EFI_SPI_PERIPHERAL`.
    pub spi_peripheral: *const c_void,
    /// Flash size in bytes.
    pub flash_size: u32,
    /// 3-byte manufacture and device ID.
    pub device_id: [u8; 3],
    /// Erase block size in bytes.
    pub erase_block_bytes: u32,
    /// Read manufacture and device ID.
    pub get_flash_id: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        buffer: *mut u8,
    ) -> EfiStatus,
    /// Read data from flash.
    pub read_data: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        flash_address: u32,
        length_in_bytes: u32,
        buffer: *mut u8,
    ) -> EfiStatus,
    /// Low frequency read data from flash.
    pub lf_read_data: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        flash_address: u32,
        length_in_bytes: u32,
        buffer: *mut u8,
    ) -> EfiStatus,
    /// Read status register.
    pub read_status: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        length_in_bytes: u32,
        flash_status: *mut u8,
    ) -> EfiStatus,
    /// Write status register.
    pub write_status: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        length_in_bytes: u32,
        flash_status: *const u8,
    ) -> EfiStatus,
    /// Write data to flash.
    pub write_data: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        flash_address: u32,
        length_in_bytes: u32,
        buffer: *const u8,
    ) -> EfiStatus,
    /// Erase 4 KiB blocks.
    pub erase: unsafe extern "efiapi" fn(
        this: *const EfiSpiNorFlashProtocol,
        flash_address: u32,
        block_count: u32,
    ) -> EfiStatus,
}

/// Extended SPI NOR Flash protocol.
#[repr(C)]
pub struct EfiSpiNorFlashProtocolEx {
    /// Version of this extended protocol (e.g. 0x0100).
    pub version: u16,
    /// Switch to 4-byte address mode.
    pub enter_4byte: unsafe extern "efiapi" fn(this: *const EfiSpiNorFlashProtocolEx) -> EfiStatus,
    /// Switch to 3-byte address mode.
    pub exit_4byte: unsafe extern "efiapi" fn(this: *const EfiSpiNorFlashProtocolEx) -> EfiStatus,
}

/// High-level safe wrapper around `EfiSpiNorFlashProtocol`.
pub struct SpiNorFlashProtocol<'a> {
    raw: &'a EfiSpiNorFlashProtocol,
}

impl<'a> SpiNorFlashProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = EFI_SPI_NOR_FLASH_PROTOCOL_GUID;

    /// Creates a safe wrapper from a raw protocol reference.
    #[must_use]
    pub const fn new(raw: &'a EfiSpiNorFlashProtocol) -> Self {
        Self { raw }
    }

    /// Flash size in bytes.
    #[must_use]
    pub const fn flash_size(&self) -> u32 {
        self.raw.flash_size
    }

    /// 3-byte manufacture and device ID.
    #[must_use]
    pub const fn device_id(&self) -> [u8; 3] {
        self.raw.device_id
    }

    /// Erase block size in bytes.
    #[must_use]
    pub const fn erase_block_bytes(&self) -> u32 {
        self.raw.erase_block_bytes
    }

    /// Read the 3-byte manufacture and device ID into `buffer`.
    ///
    /// # Errors
    /// Returns an error status if reading the device ID fails.
    pub fn get_flash_id(&self, buffer: &mut [u8; 3]) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.get_flash_id)(self.raw, buffer.as_mut_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Read data from flash into `buffer`.
    ///
    /// # Errors
    /// Returns `BAD_BUFFER_SIZE` if length exceeds 4GB, or error status on read failure.
    pub fn read_data(&self, flash_address: u32, buffer: &mut [u8]) -> Result<(), EfiStatus> {
        let Ok(len) = u32::try_from(buffer.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status =
            unsafe { (self.raw.read_data)(self.raw, flash_address, len, buffer.as_mut_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Low frequency read data from flash into `buffer`.
    ///
    /// # Errors
    /// Returns `BAD_BUFFER_SIZE` if length exceeds 4GB, or error status on read failure.
    pub fn lf_read_data(&self, flash_address: u32, buffer: &mut [u8]) -> Result<(), EfiStatus> {
        let Ok(len) = u32::try_from(buffer.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status =
            unsafe { (self.raw.lf_read_data)(self.raw, flash_address, len, buffer.as_mut_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Read the flash status register bytes.
    ///
    /// # Errors
    /// Returns an error if reading the status register fails or buffer length exceeds 4GB.
    pub fn read_status(&self, status_buf: &mut [u8]) -> Result<(), EfiStatus> {
        let Ok(len) = u32::try_from(status_buf.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status = unsafe { (self.raw.read_status)(self.raw, len, status_buf.as_mut_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Write to the flash status register.
    ///
    /// # Errors
    /// Returns an error if writing to the status register fails or buffer length exceeds 4GB.
    pub fn write_status(&self, status_buf: &[u8]) -> Result<(), EfiStatus> {
        let Ok(len) = u32::try_from(status_buf.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status = unsafe { (self.raw.write_status)(self.raw, len, status_buf.as_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Write data to SPI flash.
    ///
    /// # Errors
    /// Returns `BAD_BUFFER_SIZE` if length exceeds 4GB, or error status on write failure.
    pub fn write_data(&self, flash_address: u32, buffer: &[u8]) -> Result<(), EfiStatus> {
        let Ok(len) = u32::try_from(buffer.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status =
            unsafe { (self.raw.write_data)(self.raw, flash_address, len, buffer.as_ptr()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Erase 4 KiB blocks.
    ///
    /// # Errors
    /// Returns an error if block erase fails.
    pub fn erase(&self, flash_address: u32, block_count: u32) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.erase)(self.raw, flash_address, block_count) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// High-level safe wrapper around `EfiSpiNorFlashProtocolEx`.
pub struct SpiNorFlashProtocolEx<'a> {
    raw: &'a EfiSpiNorFlashProtocolEx,
}

impl<'a> SpiNorFlashProtocolEx<'a> {
    /// The extended protocol GUID.
    pub const GUID: EfiGuid = SPI_NOR_FLASH_PROTOCOL_EX_GUID;

    /// Creates a safe wrapper from a raw protocol reference.
    #[must_use]
    pub const fn new(raw: &'a EfiSpiNorFlashProtocolEx) -> Self {
        Self { raw }
    }

    /// Protocol version.
    #[must_use]
    pub const fn version(&self) -> u16 {
        self.raw.version
    }

    /// Switch to 4-byte address mode.
    ///
    /// # Errors
    /// Returns an error if switching to 4-byte mode fails.
    pub fn enter_4byte(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.enter_4byte)(self.raw) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Switch to 3-byte address mode.
    ///
    /// # Errors
    /// Returns an error if switching to 3-byte mode fails.
    pub fn exit_4byte(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.exit_4byte)(self.raw) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
