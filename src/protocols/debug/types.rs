//! # Debug types
//!
//! Processor and exception descriptions.

/// The divide-error exception.
pub const EXCEPT_DIVIDE_ERROR: i32 = 0;

/// The debug exception.
pub const EXCEPT_DEBUG: i32 = 1;

/// The non-maskable-interrupt exception.
pub const EXCEPT_NMI: i32 = 2;

/// The breakpoint exception.
pub const EXCEPT_BREAKPOINT: i32 = 3;

/// The overflow exception.
pub const EXCEPT_OVERFLOW: i32 = 4;

/// The invalid-opcode exception.
pub const EXCEPT_INVALID_OPCODE: i32 = 6;

/// The page-fault exception.
pub const EXCEPT_PAGE_FAULT: i32 = 14;

/// The processor instruction-set architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InstructionSetArchitecture(u16);

impl InstructionSetArchitecture {
    /// 32-bit x86.
    pub const IA32: Self = Self(0x014C);

    /// 64-bit x86.
    pub const X64: Self = Self(0x8664);

    /// Itanium.
    pub const IPF: Self = Self(0x0200);

    /// Byte-code.
    pub const EBC: Self = Self(0x0EBC);

    /// 64-bit `AArch`.
    pub const AARCH64: Self = Self(0xAA64);

    /// Creates a value from a raw machine type.
    #[must_use]
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns the raw machine type.
    #[must_use]
    pub const fn to_raw(self) -> u16 {
        self.0
    }
}

/// Returns whether an exception type is defined for x86-64.
#[must_use]
pub fn is_defined_x64_exception(exception_type: i32) -> bool {
    matches!(
        exception_type,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 11 | 12 | 13 | 14 | 16 | 17 | 18 | 19
    )
}

/// Returns whether a processor index is valid.
#[must_use]
pub fn validate_processor_index(index: usize, maximum: usize) -> bool {
    index <= maximum
}
