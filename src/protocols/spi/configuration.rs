//! # SPI Configuration Protocol
//!
//! Defined in the UEFI PI Specification, this protocol describes the platform's
//! SPI buses, connected peripheral parts, chip select lines, and clock generation.

use crate::abi::types::Char16;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// SPI part attribute: supports 2-bit data bus width.
pub const SPI_PART_SUPPORTS_2_BIT_DATA_BUS_WIDTH: u32 = 1 << 0;
/// SPI part attribute: supports 4-bit data bus width.
pub const SPI_PART_SUPPORTS_4_BIT_DATA_BUS_WIDTH: u32 = 1 << 1;
/// SPI part attribute: supports 8-bit data bus width.
pub const SPI_PART_SUPPORTS_8_BIT_DATA_BUS_WIDTH: u32 = 1 << 2;

/// Description of a SPI chip independent of its usage on the board.
#[repr(C)]
#[derive(Debug)]
pub struct EfiSpiPart {
    /// SPI chip vendor name string.
    pub vendor: *const Char16,
    /// SPI chip part number string.
    pub part_number: *const Char16,
    /// Minimum operating clock frequency in Hertz.
    pub min_clock_hz: u32,
    /// Maximum operating clock frequency in Hertz.
    pub max_clock_hz: u32,
    /// True if high voltage asserts chip select; false if active low.
    pub chip_select_polarity: bool,
}

/// Description of a physical SPI bus and its attached peripherals.
#[repr(C)]
#[derive(Debug)]
pub struct EfiSpiBus {
    /// Human-readable friendly name of the SPI bus.
    pub friendly_name: *const Char16,
    /// Pointer to the first peripheral device on this bus.
    pub peripheral_list: *const EfiSpiPeripheral,
    /// Device path to the SPI host controller.
    pub controller_path: *const c_void,
    /// Board-level clock control routine (or null to use host controller clock).
    pub clock: Option<unsafe extern "efiapi" fn(*const EfiSpiPeripheral, *mut u32) -> EfiStatus>,
    /// Opaque clock parameter data.
    pub clock_parameter: *mut c_void,
}

/// Description of a SPI peripheral logic block connected to a SPI bus.
#[repr(C)]
#[derive(Debug)]
pub struct EfiSpiPeripheral {
    /// Pointer to the next peripheral on the same SPI bus, or null if last.
    pub next_spi_peripheral: *const EfiSpiPeripheral,
    /// Friendly name of this SPI peripheral function.
    pub friendly_name: *const Char16,
    /// GUID identifying the driver used for this SPI peripheral.
    pub spi_peripheral_driver_guid: *const EfiGuid,
    /// Chip datasheet description.
    pub spi_part: *const EfiSpiPart,
    /// Maximum clock frequency in Hertz override for this connection.
    pub max_clock_hz: u32,
    /// Clock polarity (CPOL): false = low idle, true = high idle.
    pub clock_polarity: bool,
    /// Clock phase (CPHA): false = leading edge sample, true = trailing edge.
    pub clock_phase: bool,
    /// Part wiring attributes (`SPI_PART_SUPPORTS_*`).
    pub attributes: u32,
    /// Board-specific configuration data.
    pub configuration_data: *const c_void,
    /// Pointer back to the parent SPI bus.
    pub spi_bus: *const EfiSpiBus,
    /// Board-level chip select routine (or null to use host controller CS).
    pub chip_select: Option<unsafe extern "efiapi" fn(*const EfiSpiPeripheral, bool) -> EfiStatus>,
    /// Opaque chip select parameter data.
    pub chip_select_parameter: *mut c_void,
}

/// Raw SPI Configuration Protocol structure (`EFI_SPI_CONFIGURATION_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSpiConfigurationProtocol {
    /// Total count of SPI buses on the board.
    pub bus_count: u32,
    /// Pointer to array of pointers to [`EfiSpiBus`] structures.
    pub bus_list: *const *const EfiSpiBus,
}

/// Safe wrapper around [`EfiSpiConfigurationProtocol`].
pub struct SpiConfiguration<'a> {
    proto: &'a EfiSpiConfigurationProtocol,
}

impl<'a> SpiConfiguration<'a> {
    /// Wraps a reference to [`EfiSpiConfigurationProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSpiConfigurationProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSpiConfigurationProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSpiConfigurationProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns the number of SPI buses configured on the platform.
    #[must_use]
    pub const fn bus_count(&self) -> u32 {
        self.proto.bus_count
    }

    /// Returns a reference to the [`EfiSpiBus`] at index `index`, if within bounds.
    #[must_use]
    pub fn bus(&self, index: usize) -> Option<&'a EfiSpiBus> {
        if index >= self.proto.bus_count as usize || self.proto.bus_list.is_null() {
            None
        } else {
            let bus_ptr = unsafe { *self.proto.bus_list.add(index) };
            if bus_ptr.is_null() {
                None
            } else {
                Some(unsafe { &*bus_ptr })
            }
        }
    }
}
