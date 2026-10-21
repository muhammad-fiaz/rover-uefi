//! # PCI types
//!
//! Widths, operations, and addresses.

/// An I/O width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PciWidth(u32);

impl PciWidth {
    /// 8-bit width.
    pub const UINT8: Self = Self(0);

    /// 16-bit width.
    pub const UINT16: Self = Self(1);

    /// 32-bit width.
    pub const UINT32: Self = Self(2);

    /// 64-bit width.
    pub const UINT64: Self = Self(3);

    /// FIFO 8-bit width.
    pub const FIFO_UINT8: Self = Self(4);

    /// FIFO 16-bit width.
    pub const FIFO_UINT16: Self = Self(5);

    /// FIFO 32-bit width.
    pub const FIFO_UINT32: Self = Self(6);

    /// FIFO 64-bit width.
    pub const FIFO_UINT64: Self = Self(7);

    /// Fill 8-bit width.
    pub const FILL_UINT8: Self = Self(8);

    /// Fill 16-bit width.
    pub const FILL_UINT16: Self = Self(9);

    /// Fill 32-bit width.
    pub const FILL_UINT32: Self = Self(10);

    /// Fill 64-bit width.
    pub const FILL_UINT64: Self = Self(11);

    /// Maximum width.
    pub const MAX: Self = Self(12);

    /// Creates a value from a raw integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }

    /// Returns the byte width.
    #[must_use]
    pub const fn byte_width(self) -> Option<usize> {
        match self.0 {
            0 | 4 | 8 => Some(1),
            1 | 5 | 9 => Some(2),
            2 | 6 | 10 => Some(4),
            3 | 7 | 11 => Some(8),
            _ => None,
        }
    }
}

/// A root bridge DMA operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RootBridgeOperation(u32);

impl RootBridgeOperation {
    /// Bus master read.
    pub const BUS_MASTER_READ: Self = Self(0);

    /// Bus master write.
    pub const BUS_MASTER_WRITE: Self = Self(1);

    /// Common buffer.
    pub const BUS_MASTER_COMMON_BUFFER: Self = Self(2);

    /// Bus master read 64.
    pub const BUS_MASTER_READ64: Self = Self(3);

    /// Bus master write 64.
    pub const BUS_MASTER_WRITE64: Self = Self(4);

    /// Common buffer 64.
    pub const BUS_MASTER_COMMON_BUFFER64: Self = Self(5);

    /// Maximum operation.
    pub const MAX: Self = Self(6);

    /// Creates a value from a raw integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

/// A PCI I/O DMA operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PciIoOperation(u32);

impl PciIoOperation {
    /// Bus master read.
    pub const BUS_MASTER_READ: Self = Self(0);

    /// Bus master write.
    pub const BUS_MASTER_WRITE: Self = Self(1);

    /// Common buffer.
    pub const BUS_MASTER_COMMON_BUFFER: Self = Self(2);

    /// Maximum operation.
    pub const MAX: Self = Self(3);

    /// Creates a value from a raw integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

/// A PCI attribute operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AttributeOperation(u32);

impl AttributeOperation {
    /// Get attributes.
    pub const GET: Self = Self(0);

    /// Set attributes.
    pub const SET: Self = Self(1);

    /// Enable attributes.
    pub const ENABLE: Self = Self(2);

    /// Disable attributes.
    pub const DISABLE: Self = Self(3);

    /// Supported attributes.
    pub const SUPPORTED: Self = Self(4);

    /// Maximum operation.
    pub const MAX: Self = Self(5);

    /// Creates a value from a raw integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

/// A PCI address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PciAddress {
    /// The register.
    pub register: u8,
    /// The function.
    pub function: u8,
    /// The device.
    pub device: u8,
    /// The bus.
    pub bus: u8,
    /// The extended register.
    pub extended_register: u32,
}

/// Returns the encoded root bridge address.
///
/// The encoding follows the PCI address macro.
#[must_use]
pub fn pci_address(bus: u8, device: u8, function: u8, register: u16) -> u64 {
    let bus_part = u64::from(bus) << 24;
    let device_part = u64::from(device) << 16;
    let function_part = u64::from(function) << 8;
    let register_part = u64::from(register);
    bus_part | device_part | function_part | register_part
}

/// Returns bus, device, and function from an
/// encoded address.
#[must_use]
pub fn decode_bdf(address: u64) -> (u8, u8, u8) {
    (
        ((address >> 24) & 0xFF) as u8,
        ((address >> 16) & 0xFF) as u8,
        ((address >> 8) & 0xFF) as u8,
    )
}
