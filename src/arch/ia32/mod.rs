//! # IA-32 (x86 32-bit) Architecture
//!
//! Architectural registers, segment registers, and exception context
//! for 32-bit x86 firmware environments.

/// EFLAGS Carry Flag.
pub const EFLAGS_CF: u32 = 0x0000_0001;
/// EFLAGS Parity Flag.
pub const EFLAGS_PF: u32 = 0x0000_0004;
/// EFLAGS Auxiliary Carry Flag.
pub const EFLAGS_AF: u32 = 0x0000_0010;
/// EFLAGS Zero Flag.
pub const EFLAGS_ZF: u32 = 0x0000_0040;
/// EFLAGS Sign Flag.
pub const EFLAGS_SF: u32 = 0x0000_0080;
/// EFLAGS Trap Flag.
pub const EFLAGS_TF: u32 = 0x0000_0100;
/// EFLAGS Interrupt Enable Flag.
pub const EFLAGS_IF: u32 = 0x0000_0200;
/// EFLAGS Direction Flag.
pub const EFLAGS_DF: u32 = 0x0000_0400;
/// EFLAGS Overflow Flag.
pub const EFLAGS_OF: u32 = 0x0000_0800;

/// IA-32 16-bit segment selectors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Ia32SegmentSelectors {
    /// Code segment selector.
    pub cs: u16,
    /// Data segment selector.
    pub ds: u16,
    /// Extra segment selector.
    pub es: u16,
    /// FS segment selector.
    pub fs: u16,
    /// GS segment selector.
    pub gs: u16,
    /// Stack segment selector.
    pub ss: u16,
}

/// IA-32 processor state recorded during an interrupt or exception.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Ia32ExceptionContext {
    /// General purpose register EAX.
    pub eax: u32,
    /// General purpose register EBX.
    pub ebx: u32,
    /// General purpose register ECX.
    pub ecx: u32,
    /// General purpose register EDX.
    pub edx: u32,
    /// Source Index register ESI.
    pub esi: u32,
    /// Destination Index register EDI.
    pub edi: u32,
    /// Base Pointer register EBP.
    pub ebp: u32,
    /// Stack Pointer register ESP.
    pub esp: u32,
    /// Instruction Pointer register EIP.
    pub eip: u32,
    /// Status register EFLAGS.
    pub eflags: u32,
    /// Segment selectors.
    pub segments: Ia32SegmentSelectors,
    /// Exception vector index.
    pub exception_vector: u32,
    /// Hardware error code.
    pub error_code: u32,
    /// Page fault linear address (CR2).
    pub cr2: u32,
}

impl Ia32ExceptionContext {
    /// Checks whether interrupts were enabled at exception entry.
    #[must_use]
    pub const fn are_interrupts_enabled(&self) -> bool {
        (self.eflags & EFLAGS_IF) != 0
    }
}
