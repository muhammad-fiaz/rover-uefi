//! # Bus model
//!
//! Bus identifiers and architecture identifiers.

/// A bus identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BusId(pub u64);

impl BusId {
    /// Creates an identifier from a raw value.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u64 {
        self.0
    }
}

/// A CPU architecture identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArchId {
    /// 32-bit x86.
    X86 = 0,
    /// 64-bit x86.
    X86_64 = 1,
    /// 32-bit ARM.
    Arm = 2,
    /// 64-bit ARM.
    AArch64 = 3,
    /// RISC-V.
    RiscV = 4,
    /// An unknown architecture.
    Unknown = 255,
}

impl ArchId {
    /// Returns whether the architecture is 64-bit.
    #[must_use]
    pub const fn is_64_bit(self) -> bool {
        matches!(self, Self::X86_64 | Self::AArch64)
    }

    /// Creates an architecture from a raw value.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::X86,
            1 => Self::X86_64,
            2 => Self::Arm,
            3 => Self::AArch64,
            4 => Self::RiscV,
            _ => Self::Unknown,
        }
    }
}
