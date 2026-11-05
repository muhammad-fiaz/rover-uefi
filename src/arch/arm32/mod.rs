//! # ARM 32-bit (ARMv7-A) Architecture
//!
//! Architectural registers, processor mode bits, and exception contexts
//! for 32-bit ARM firmware environments.

/// Processor execution mode bits in CPSR.
pub const CPSR_MODE_MASK: u32 = 0x0000_001F;
/// User execution mode.
pub const CPSR_MODE_USER: u32 = 0x10;
/// FIQ interrupt execution mode.
pub const CPSR_MODE_FIQ: u32 = 0x11;
/// IRQ interrupt execution mode.
pub const CPSR_MODE_IRQ: u32 = 0x12;
/// Supervisor (SVC) mode.
pub const CPSR_MODE_SUPERVISOR: u32 = 0x13;
/// Abort mode.
pub const CPSR_MODE_ABORT: u32 = 0x17;
/// Undefined instruction mode.
pub const CPSR_MODE_UNDEFINED: u32 = 0x1B;
/// System mode.
pub const CPSR_MODE_SYSTEM: u32 = 0x1F;

/// IRQ disable bit in CPSR (bit 7).
pub const CPSR_IRQ_DISABLED: u32 = 0x0000_0080;
/// FIQ disable bit in CPSR (bit 6).
pub const CPSR_FIQ_DISABLED: u32 = 0x0000_0040;
/// Thumb execution state bit in CPSR (bit 5).
pub const CPSR_THUMB_STATE: u32 = 0x0000_0020;

/// ARM 32-bit general register state at exception entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Arm32Registers {
    /// R0 through R12 general registers.
    pub r: [u32; 13],
    /// Stack Pointer (R13).
    pub sp: u32,
    /// Link Register (R14).
    pub lr: u32,
    /// Program Counter (R15).
    pub pc: u32,
    /// Current Program Status Register.
    pub cpsr: u32,
    /// Data Fault Status Register.
    pub dfsr: u32,
    /// Data Fault Address Register.
    pub dfar: u32,
    /// Instruction Fault Status Register.
    pub ifsr: u32,
    /// Instruction Fault Address Register.
    pub ifar: u32,
}

impl Arm32Registers {
    /// Checks whether the processor was in Thumb mode at exception time.
    #[must_use]
    pub const fn is_thumb(&self) -> bool {
        (self.cpsr & CPSR_THUMB_STATE) != 0
    }

    /// Checks whether IRQs were masked at exception time.
    #[must_use]
    pub const fn are_irqs_disabled(&self) -> bool {
        (self.cpsr & CPSR_IRQ_DISABLED) != 0
    }
}
