//! # Processor exception context
//!
//! The register image of a taken exception and the system
//! registers that describe it.
//!
//! When the processor takes an exception it stores a fixed set of
//! values in the exception link registers. The structure in this
//! module mirrors that set, so that a handler can read the state
//! of the interrupted context and restore it on return.
//!
//! The module also describes the exception vector table. The
//! table has sixteen entries of a fixed size, one for every
//! combination of an exception source and an exception class.

use super::SystemRegister;

/// The size of a single entry of the exception vector table.
pub const EXCEPTION_VECTOR_ENTRY_SIZE: u64 = 0x200;

/// The number of entries of the exception vector table.
pub const EXCEPTION_VECTOR_COUNT: u32 = 16;

/// The source of an exception.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExceptionSource {
    /// The exception was taken at the current exception level with
    /// the stack pointer of the lower level.
    CurrentLevelStackLower,
    /// The exception was taken at the current exception level with
    /// the stack pointer of the current level.
    CurrentLevelStackCurrent,
    /// The exception was taken at a lower exception level and
    /// reached the current level in the sixty-four bit state.
    LowerLevelSixtyFour,
    /// The exception was taken at a lower exception level and
    /// reached the current level in the thirty-two bit state.
    LowerLevelThirtyTwo,
}

impl ExceptionSource {
    /// Returns the index of the source inside the vector table.
    #[must_use]
    pub const fn index(self) -> u32 {
        match self {
            Self::CurrentLevelStackLower => 0,
            Self::CurrentLevelStackCurrent => 1,
            Self::LowerLevelSixtyFour => 2,
            Self::LowerLevelThirtyTwo => 3,
        }
    }
}

/// The class of an exception.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExceptionClass {
    /// A synchronous exception.
    Synchronous,
    /// An interrupt request.
    Interrupt,
    /// A fast interrupt request.
    FastInterrupt,
    /// An asynchronous error.
    AsynchronousError,
}

impl ExceptionClass {
    /// Returns the index of the class inside a group of four
    /// entries.
    #[must_use]
    pub const fn index(self) -> u32 {
        match self {
            Self::Synchronous => 0,
            Self::Interrupt => 1,
            Self::FastInterrupt => 2,
            Self::AsynchronousError => 3,
        }
    }
}

/// The system register that holds the saved program status.
pub const SPSR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 4,
    crm: 0,
    op2: 2,
};

/// The system register that holds the saved exception link.
pub const ELR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 4,
    crm: 0,
    op2: 1,
};

/// The system register that holds the syndrome of an exception.
pub const ESR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 5,
    crm: 2,
    op2: 0,
};

/// The system register that holds the faulting address.
pub const FAR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 6,
    crm: 0,
    op2: 0,
};

/// The system register that holds the translation table of a
/// faulting access.
pub const PAR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 7,
    crm: 0,
    op2: 0,
};

/// The system register that holds the base of the vector table.
pub const VBAR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 0,
    op2: 0,
};

/// The system register that holds the software thread pointer.
pub const TPIDR_EL0: SystemRegister = SystemRegister {
    op0: 3,
    op1: 3,
    crn: 13,
    crm: 0,
    op2: 2,
};

/// The system register that holds the reserved software thread
/// pointer.
pub const TPIDRRO_EL0: SystemRegister = SystemRegister {
    op0: 3,
    op1: 3,
    crn: 13,
    crm: 0,
    op2: 3,
};

/// The system register that holds the stack pointer of the lower
/// level.
pub const SP_EL0: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 13,
    crm: 0,
    op2: 0,
};

/// The system register that holds the stack pointer of the
/// current level.
pub const SP_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 13,
    crm: 0,
    op2: 4,
};

/// The system register that holds the stack pointer that a signal
/// handler uses.
pub const SP_ELX: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 13,
    crm: 0,
    op2: 6,
};

/// The register image of a taken exception.
///
/// The fields follow the order of the exception link registers
/// that the processor writes when it takes an exception.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ExceptionContext {
    /// The saved exception link of the current level.
    pub elr_el1: u64,
    /// The stack pointer of the lower level.
    pub sp_el0: u64,
    /// The stack pointer of the current level.
    pub sp_el1: u64,
    /// The saved exception link of the lower level.
    pub elr_el0: u64,
    /// The syndrome of the exception.
    pub esr_el1: u64,
    /// The address of the faulting access.
    pub far_el1: u64,
    /// The translation table of the faulting access.
    pub par_el1: u64,
    /// The software thread pointer.
    pub tpidr_el0: u64,
    /// The reserved software thread pointer.
    pub tpidrro_el0: u64,
    /// The stack pointer that a signal handler uses.
    pub sp_elx: u64,
}

/// Returns the offset of an entry of the exception vector table.
///
/// The table has sixteen entries. The first four belong to the
/// current exception level with the stack pointer of the lower
/// level, the next four to the current exception level with its
/// own stack pointer, the next four to a lower exception level in
/// the sixty-four bit state and the last four to a lower
/// exception level in the thirty-two bit state.
#[must_use]
pub fn exception_vector_offset(source: ExceptionSource, class: ExceptionClass) -> u64 {
    let index = source.index() * 4 + class.index();
    u64::from(index) * EXCEPTION_VECTOR_ENTRY_SIZE
}

/// Returns the index of an entry of the exception vector table.
#[must_use]
pub const fn exception_vector_index(source: ExceptionSource, class: ExceptionClass) -> u32 {
    source.index() * 4 + class.index()
}
