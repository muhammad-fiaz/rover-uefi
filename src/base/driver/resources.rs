//! # Device resources
//!
//! Memory-mapped, port I/O, DMA, and interrupt resource types.
//!
//! Resources are plain descriptors. Mapping or transferring data
//! requires a native driver implementation behind these types.

/// A memory-mapped region descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MmioRegion {
    /// The base physical address.
    pub base: u64,
    /// The length in bytes.
    pub length: u64,
}

impl MmioRegion {
    /// Creates a region, or `None` when the length is zero or the
    /// address range overflows.
    #[must_use]
    pub const fn new(base: u64, length: u64) -> Option<Self> {
        if length == 0 {
            return None;
        }
        let end = base.checked_add(length);
        match end {
            None => None,
            Some(_) => Some(Self { base, length }),
        }
    }

    /// Returns the end address (exclusive), or `None` on overflow.
    #[must_use]
    pub const fn end(&self) -> Option<u64> {
        self.base.checked_add(self.length)
    }

    /// Returns whether the region contains `address`.
    #[must_use]
    pub const fn contains(&self, address: u64) -> bool {
        if self.length == 0 {
            return false;
        }
        match address.checked_sub(self.base) {
            Some(offset) => offset < self.length,
            None => false,
        }
    }
}

/// A port I/O region descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PortIoRegion {
    /// The base port.
    pub base: u16,
    /// The length in ports.
    pub length: u16,
}

impl PortIoRegion {
    /// Creates a region, or `None` when the length is zero or the
    /// port range overflows `u16`.
    #[must_use]
    pub const fn new(base: u16, length: u16) -> Option<Self> {
        if length == 0 {
            return None;
        }
        let end = (base as u32) + (length as u32);
        if end > 0x1_0000 {
            return None;
        }
        Some(Self { base, length })
    }

    /// Returns whether the region contains `port`.
    #[must_use]
    pub const fn contains(&self, port: u16) -> bool {
        match (port as u32).checked_sub(self.base as u32) {
            Some(offset) => offset < self.length as u32,
            None => false,
        }
    }
}

/// A DMA buffer descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaBuffer {
    /// The physical address.
    pub address: u64,
    /// The length in bytes.
    pub length: u64,
    /// The required alignment in bytes (power of two).
    pub alignment: u64,
}

impl DmaBuffer {
    /// Creates a descriptor, or `None` when the length is zero, the
    /// alignment is not a power of two, or the address is misaligned.
    #[must_use]
    pub const fn new(address: u64, length: u64, alignment: u64) -> Option<Self> {
        if length == 0 || alignment == 0 || !alignment.is_power_of_two() {
            return None;
        }
        if alignment > 1 && address % alignment != 0 {
            return None;
        }
        Some(Self {
            address,
            length,
            alignment,
        })
    }
}

/// An interrupt descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Interrupt {
    /// The interrupt line.
    pub line: u32,
    /// The interrupt pin.
    pub pin: u8,
}

impl Interrupt {
    /// Creates an interrupt descriptor.
    #[must_use]
    pub const fn new(line: u32, pin: u8) -> Self {
        Self { line, pin }
    }
}

/// A device resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Resource {
    /// A memory-mapped region.
    Mmio(MmioRegion),
    /// A port I/O region.
    Port(PortIoRegion),
    /// A DMA buffer.
    Dma(DmaBuffer),
    /// An interrupt.
    Interrupt(Interrupt),
}

impl Resource {
    /// Returns whether the resource is memory-mapped.
    #[must_use]
    pub const fn is_mmio(&self) -> bool {
        matches!(self, Self::Mmio(_))
    }
}
