//! # `LoongArch` 64-bit Architecture
//!
//! Architectural general registers, control status registers (CSRs),
//! and exception context for `LoongArch64` firmware environments.

pub mod csr;

pub use csr::{
    CSR_ASID, CSR_BADI, CSR_BADV, CSR_CRMD, CSR_ECFG, CSR_EENTRY, CSR_ERA, CSR_ESTAT, CSR_EUEN,
    CSR_PGD, CSR_PGDH, CSR_PGDL, CSR_PRMD, CSR_TLBEHI, CSR_TLBELO0, CSR_TLBELO1, CSR_TLBIDX, Crmd,
    csr_read, csr_write,
};

/// `LoongArch` 64-bit general purpose and control registers at exception entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LoongArch64ExceptionContext {
    /// R0 (constant zero) through R31 general-purpose registers.
    pub r: [u64; 32],
    /// Current Mode information (CRMD).
    pub crmd: u64,
    /// Previous Mode information (PRMD).
    pub prmd: u64,
    /// Extended User Enable (EUEN).
    pub euen: u64,
    /// Exception Configuration (ECFG).
    pub ecfg: u64,
    /// Exception Status (ESTAT).
    pub estat: u64,
    /// Exception Return Address (ERA).
    pub era: u64,
    /// Bad Virtual Address (BADV).
    pub badv: u64,
    /// Bad Instruction (BADI).
    pub badi: u64,
}

impl LoongArch64ExceptionContext {
    /// Returns the exception code (ECODE) from the ESTAT register.
    #[must_use]
    pub const fn exception_code(&self) -> u32 {
        ((self.estat >> 16) & 0x3F) as u32
    }

    /// Returns the sub-exception code (ESUBCODE) from the ESTAT register.
    #[must_use]
    pub const fn sub_exception_code(&self) -> u32 {
        ((self.estat >> 22) & 0x1FF) as u32
    }
}
