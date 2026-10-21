//! # Supervisor trap context
//!
//! The register image that the processor saves when it traps to
//! the supervisor mode, and the encoding of the cause of the
//! trap.
//!
//! The structure holds the integer registers that the supervisor
//! can modify, followed by the control and status registers that
//! describe the trap. The zero register is not part of the
//! structure because it always reads as zero.

/// The cause of an instruction address misalignment.
pub const SCAUSE_INSTRUCTION_ADDRESS_MISALIGNED: usize = 0;

/// The cause of an illegal instruction.
pub const SCAUSE_ILLEGAL_INSTRUCTION: usize = 1;

/// The cause of a breakpoint.
pub const SCAUSE_BREAKPOINT: usize = 2;

/// The cause of a load address misalignment.
pub const SCAUSE_LOAD_ADDRESS_MISALIGNED: usize = 3;

/// The cause of a load access fault.
pub const SCAUSE_LOAD_ACCESS_FAULT: usize = 4;

/// The cause of a store address misalignment.
pub const SCAUSE_STORE_ADDRESS_MISALIGNED: usize = 5;

/// The cause of a store access fault.
pub const SCAUSE_STORE_ACCESS_FAULT: usize = 6;

/// The cause of an environment call from the user mode.
pub const SCAUSE_USER_ECALL: usize = 7;

/// The cause of an environment call from the supervisor mode.
pub const SCAUSE_SUPERVISOR_ECALL: usize = 8;

/// The cause of an environment call from the hypervisor mode.
pub const SCAUSE_HYPERVISOR_ECALL: usize = 9;

/// The cause of an environment call from the virtual supervisor
/// mode.
pub const SCAUSE_VIRTUAL_SUPERVISOR_ECALL: usize = 10;

/// The cause of an environment call from the machine mode.
pub const SCAUSE_MACHINE_ECALL: usize = 11;

/// The cause of an instruction page fault.
pub const SCAUSE_INSTRUCTION_PAGE_FAULT: usize = 12;

/// The cause of a load page fault.
pub const SCAUSE_LOAD_PAGE_FAULT: usize = 13;

/// The cause of a store page fault.
pub const SCAUSE_STORE_PAGE_FAULT: usize = 14;

/// The bit of the cause register that separates an interrupt from
/// a synchronous exception.
///
/// The bit is the most significant bit of the register, so it
/// depends on the width of the register.
pub const SCAUSE_INTERRUPT_BIT: usize = 1 << (usize::BITS - 1);

/// The bit of the supervisor status register that enables the
/// supervisor interrupts.
pub const SSTATUS_SIE: usize = 1 << 1;

/// The bit of the supervisor status register that records whether
/// the supervisor interrupts were enabled before the trap.
pub const SSTATUS_SPIE: usize = 1 << 5;

/// The bit of the supervisor status register that records the
/// privilege mode before the trap.
pub const SSTATUS_SPP: usize = 1 << 8;

/// The mask of the privilege mode inside the supervisor status
/// register.
pub const SSTATUS_SPP_MASK: usize = 3 << 8;

/// The bit of the supervisor status register that permits the
/// user mode to access the supervisor memory.
pub const SSTATUS_SUM: usize = 1 << 18;

/// The bit of the supervisor status register that permits the
/// user mode to execute the memory that is marked as
/// executable.
pub const SSTATUS_MXR: usize = 1 << 19;

/// The cause of a trap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrapCause {
    /// The address of an instruction was not aligned.
    InstructionAddressMisaligned,
    /// The instruction was not defined.
    IllegalInstruction,
    /// The processor reached a breakpoint.
    Breakpoint,
    /// The address of a load was not aligned.
    LoadAddressMisaligned,
    /// A load could not access its memory.
    LoadAccessFault,
    /// The address of a store was not aligned.
    StoreAddressMisaligned,
    /// A store could not access its memory.
    StoreAccessFault,
    /// The user mode executed an environment call.
    EnvironmentCallFromUser,
    /// The supervisor mode executed an environment call.
    EnvironmentCallFromSupervisor,
    /// The hypervisor mode executed an environment call.
    EnvironmentCallFromHypervisor,
    /// The virtual supervisor mode executed an environment call.
    EnvironmentCallFromVirtualSupervisor,
    /// The machine mode executed an environment call.
    EnvironmentCallFromMachine,
    /// An instruction could not access its memory.
    InstructionPageFault,
    /// A load could not access its memory because of a
    /// translation fault.
    LoadPageFault,
    /// A store could not access its memory because of a
    /// translation fault.
    StorePageFault,
    /// The cause register held a synchronous cause that the
    /// architecture does not define.
    Unknown(u64),
    /// The cause register described an interrupt request.
    Interrupt(u64),
}

impl TrapCause {
    /// Decodes the cause of a trap.
    ///
    /// The most significant bit of the cause register separates
    /// an interrupt request from a synchronous exception.
    #[must_use]
    pub fn decode(scause: usize) -> Self {
        if scause & SCAUSE_INTERRUPT_BIT != 0 {
            let number = u64::try_from(scause & !SCAUSE_INTERRUPT_BIT).unwrap_or_default();
            return Self::Interrupt(number);
        }
        match scause {
            SCAUSE_INSTRUCTION_ADDRESS_MISALIGNED => Self::InstructionAddressMisaligned,
            SCAUSE_ILLEGAL_INSTRUCTION => Self::IllegalInstruction,
            SCAUSE_BREAKPOINT => Self::Breakpoint,
            SCAUSE_LOAD_ADDRESS_MISALIGNED => Self::LoadAddressMisaligned,
            SCAUSE_LOAD_ACCESS_FAULT => Self::LoadAccessFault,
            SCAUSE_STORE_ADDRESS_MISALIGNED => Self::StoreAddressMisaligned,
            SCAUSE_STORE_ACCESS_FAULT => Self::StoreAccessFault,
            SCAUSE_USER_ECALL => Self::EnvironmentCallFromUser,
            SCAUSE_SUPERVISOR_ECALL => Self::EnvironmentCallFromSupervisor,
            SCAUSE_HYPERVISOR_ECALL => Self::EnvironmentCallFromHypervisor,
            SCAUSE_VIRTUAL_SUPERVISOR_ECALL => Self::EnvironmentCallFromVirtualSupervisor,
            SCAUSE_MACHINE_ECALL => Self::EnvironmentCallFromMachine,
            SCAUSE_INSTRUCTION_PAGE_FAULT => Self::InstructionPageFault,
            SCAUSE_LOAD_PAGE_FAULT => Self::LoadPageFault,
            SCAUSE_STORE_PAGE_FAULT => Self::StorePageFault,
            _ => Self::Unknown(u64::try_from(scause).unwrap_or_default()),
        }
    }

    /// Returns whether the cause describes an interrupt request.
    #[must_use]
    pub const fn is_interrupt(&self) -> bool {
        matches!(self, Self::Interrupt(_))
    }
}

/// Returns whether the cause register describes an interrupt
/// request.
#[must_use]
pub const fn is_interrupt(scause: usize) -> bool {
    scause & SCAUSE_INTERRUPT_BIT != 0
}

/// Returns the cause of a trap without the interrupt bit.
#[must_use]
pub fn exception_code(scause: usize) -> u64 {
    let raw = scause & !SCAUSE_INTERRUPT_BIT;
    u64::try_from(raw).unwrap_or_default()
}

/// The register image of a trap to the supervisor mode.
///
/// The integer registers come first in the order of their numbers,
/// except for the zero register, which is never stored. The
/// control and status registers follow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TrapContext {
    /// The return address register.
    pub ra: usize,
    /// The stack pointer.
    pub sp: usize,
    /// The global pointer.
    pub gp: usize,
    /// The thread pointer.
    pub tp: usize,
    /// The temporary register zero.
    pub t0: usize,
    /// The temporary register one.
    pub t1: usize,
    /// The temporary register two.
    pub t2: usize,
    /// The saved register zero.
    pub s0: usize,
    /// The saved register one.
    pub s1: usize,
    /// The argument register zero.
    pub a0: usize,
    /// The argument register one.
    pub a1: usize,
    /// The argument register two.
    pub a2: usize,
    /// The argument register three.
    pub a3: usize,
    /// The argument register four.
    pub a4: usize,
    /// The argument register five.
    pub a5: usize,
    /// The argument register six.
    pub a6: usize,
    /// The argument register seven.
    pub a7: usize,
    /// The saved register two.
    pub s2: usize,
    /// The saved register three.
    pub s3: usize,
    /// The saved register four.
    pub s4: usize,
    /// The saved register five.
    pub s5: usize,
    /// The saved register six.
    pub s6: usize,
    /// The saved register seven.
    pub s7: usize,
    /// The saved register eight.
    pub s8: usize,
    /// The saved register nine.
    pub s9: usize,
    /// The saved register ten.
    pub s10: usize,
    /// The saved register eleven.
    pub s11: usize,
    /// The temporary register three.
    pub t3: usize,
    /// The temporary register four.
    pub t4: usize,
    /// The temporary register five.
    pub t5: usize,
    /// The temporary register six.
    pub t6: usize,
    /// The supervisor status register.
    pub sstatus: usize,
    /// The supervisor exception program counter.
    pub sepc: usize,
    /// The supervisor cause register.
    pub scause: usize,
    /// The supervisor trap value register.
    pub stval: usize,
    /// The supervisor trap handler base address.
    pub stvec: usize,
    /// The supervisor interrupt enable register.
    pub sie: usize,
    /// The supervisor interrupt pending register.
    pub sip: usize,
}
