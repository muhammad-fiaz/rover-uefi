//! # Processor exception contexts
//!
//! The register images that the processor saves when it takes an
//! exception, and the register images that it restores when the
//! handler returns.
//!
//! The structures describe the 32-bit and the 64-bit form of the
//! architecture. They are laid out with the `C` representation so
//! that the field offsets and the sizes are predictable and can be
//! checked by a test.
//!
//! The exception vector numbers are defined as constants so that
//! an interrupt descriptor table can be built without repeating
//! the magic numbers.

/// The vector of the divide error exception.
pub const EXCEPTION_VECTOR_DIVIDE_ERROR: u8 = 0;

/// The vector of the debug exception.
pub const EXCEPTION_VECTOR_DEBUG: u8 = 1;

/// The vector of the non-maskable interrupt.
pub const EXCEPTION_VECTOR_NON_MASKABLE_INTERRUPT: u8 = 2;

/// The vector of the breakpoint exception.
pub const EXCEPTION_VECTOR_BREAKPOINT: u8 = 3;

/// The vector of the overflow exception.
pub const EXCEPTION_VECTOR_OVERFLOW: u8 = 4;

/// The vector of the bound range exceeded exception.
pub const EXCEPTION_VECTOR_BOUNDS_RANGE_EXCEEDED: u8 = 5;

/// The vector of the invalid opcode exception.
pub const EXCEPTION_VECTOR_INVALID_OPCODE: u8 = 6;

/// The vector of the device not available exception.
pub const EXCEPTION_VECTOR_DEVICE_NOT_AVAILABLE: u8 = 7;

/// The vector of the double fault exception.
pub const EXCEPTION_VECTOR_DOUBLE_FAULT: u8 = 8;

/// The vector of the coprocessor segment overrun exception.
pub const EXCEPTION_VECTOR_COPROCESSOR_SEGMENT_OVERRUN: u8 = 9;

/// The vector of the invalid task state segment exception.
pub const EXCEPTION_VECTOR_INVALID_TASK_STATE_SEGMENT: u8 = 10;

/// The vector of the segment not present exception.
pub const EXCEPTION_VECTOR_SEGMENT_NOT_PRESENT: u8 = 11;

/// The vector of the stack segment fault exception.
pub const EXCEPTION_VECTOR_STACK_SEGMENT_FAULT: u8 = 12;

/// The vector of the general protection fault exception.
pub const EXCEPTION_VECTOR_GENERAL_PROTECTION_FAULT: u8 = 13;

/// The vector of the page fault exception.
pub const EXCEPTION_VECTOR_PAGE_FAULT: u8 = 14;

/// The vector that the architecture reserves.
pub const EXCEPTION_VECTOR_RESERVED: u8 = 15;

/// The vector of the floating point exception.
pub const EXCEPTION_VECTOR_FLOATING_POINT: u8 = 16;

/// The vector of the alignment check exception.
pub const EXCEPTION_VECTOR_ALIGNMENT_CHECK: u8 = 17;

/// The vector of the machine check exception.
pub const EXCEPTION_VECTOR_MACHINE_CHECK: u8 = 18;

/// The vector of the vector floating point exception.
pub const EXCEPTION_VECTOR_SIMD_FLOATING_POINT: u8 = 19;

/// The vector of the virtualization exception.
pub const EXCEPTION_VECTOR_VIRTUALIZATION: u8 = 20;

/// The vector of the control protection exception.
pub const EXCEPTION_VECTOR_CONTROL_PROTECTION: u8 = 21;

/// The first vector that the architecture assigns to a maskable
/// interrupt request.
pub const EXCEPTION_VECTOR_FIRST_IRQ: u8 = 32;

/// The last vector that the architecture assigns to a maskable
/// interrupt request.
pub const EXCEPTION_VECTOR_LAST_IRQ: u8 = 47;

/// The vector that the local controller uses for a spurious
/// interrupt.
pub const EXCEPTION_VECTOR_SPURIOUS: u8 = 255;

/// The number of vectors that the architecture reserves for
/// exceptions.
pub const EXCEPTION_VECTOR_COUNT: u8 = 32;

/// Returns whether a vector describes an exception rather than a
/// maskable interrupt request.
#[must_use]
pub const fn is_exception(vector: u8) -> bool {
    vector < EXCEPTION_VECTOR_COUNT
}

/// Returns whether the exception pushes an error code onto the
/// stack.
#[must_use]
pub const fn has_error_code(vector: u8) -> bool {
    matches!(
        vector,
        EXCEPTION_VECTOR_DIVIDE_ERROR
            | EXCEPTION_VECTOR_BOUNDS_RANGE_EXCEEDED
            | EXCEPTION_VECTOR_INVALID_OPCODE
            | EXCEPTION_VECTOR_DEVICE_NOT_AVAILABLE
            | EXCEPTION_VECTOR_DOUBLE_FAULT
            | EXCEPTION_VECTOR_INVALID_TASK_STATE_SEGMENT
            | EXCEPTION_VECTOR_SEGMENT_NOT_PRESENT
            | EXCEPTION_VECTOR_STACK_SEGMENT_FAULT
            | EXCEPTION_VECTOR_GENERAL_PROTECTION_FAULT
            | EXCEPTION_VECTOR_PAGE_FAULT
            | EXCEPTION_VECTOR_ALIGNMENT_CHECK
            | EXCEPTION_VECTOR_CONTROL_PROTECTION
    )
}

/// The segment selectors that the processor saves with an
/// exception.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SegmentSelectors {
    /// The code segment selector.
    pub cs: u16,
    /// The data segment selector.
    pub ds: u16,
    /// The extra segment selector.
    pub es: u16,
    /// The segment selector of the additional segment register.
    pub fs: u16,
    /// The segment selector of the general segment register.
    pub gs: u16,
    /// The stack segment selector.
    pub ss: u16,
}

/// The general purpose registers that the processor saves with an
/// exception.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct GeneralRegisters {
    /// The accumulator register.
    pub eax: u32,
    /// The count register.
    pub ecx: u32,
    /// The data register.
    pub edx: u32,
    /// The base register.
    pub ebx: u32,
    /// The stack pointer.
    pub esp: u32,
    /// The frame pointer.
    pub ebp: u32,
    /// The source index.
    pub esi: u32,
    /// The destination index.
    pub edi: u32,
}

/// The 32-bit register image of a taken exception.
///
/// The structure holds the state that the processor saves before
/// it transfers control to the handler. The fields follow the
/// order in which the processor pushes them onto the stack.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct InterruptContext {
    /// The segment selectors.
    pub segments: SegmentSelectors,
    /// The general purpose registers.
    pub general: GeneralRegisters,
    /// The instruction pointer of the interrupted context.
    pub eip: u32,
    /// The status register of the interrupted context.
    pub eflags: u32,
    /// The error code that the exception pushed, or zero.
    pub error_code: u32,
    /// The vector of the exception.
    pub trap_number: u32,
    /// The address that caused a page fault.
    pub cr2: u32,
}

/// The 64-bit register image of a taken exception.
///
/// The structure holds the state that the processor saves before
/// it transfers control to the handler, in the layout that the
/// architecture prescribes for the exception stack frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SystemContext {
    /// The stack pointer of the interrupted context.
    pub rsp: u64,
    /// The instruction pointer of the interrupted context.
    pub rip: u64,
    /// The status register of the interrupted context.
    pub rflags: u64,
    /// The frame pointer of the interrupted context.
    pub rbp: u64,
    /// The general purpose register eight.
    pub r8: u64,
    /// The general purpose register nine.
    pub r9: u64,
    /// The general purpose register ten.
    pub r10: u64,
    /// The general purpose register eleven.
    pub r11: u64,
    /// The general purpose register twelve.
    pub r12: u64,
    /// The general purpose register thirteen.
    pub r13: u64,
    /// The general purpose register fourteen.
    pub r14: u64,
    /// The general purpose register fifteen.
    pub r15: u64,
    /// The address that caused a page fault.
    pub cr2: u64,
    /// The base of the general segment register.
    pub gs_base: u64,
    /// The base of the additional segment register.
    pub fs_base: u64,
    /// The code segment selector.
    pub cs: u16,
    /// The data segment selector.
    pub ds: u16,
    /// The extra segment selector.
    pub es: u16,
    /// The segment selector of the additional segment register.
    pub fs: u16,
    /// The segment selector of the general segment register.
    pub gs: u16,
    /// The stack segment selector.
    pub ss: u16,
    /// The error code that the exception pushed, or zero.
    pub error_code: u32,
    /// The vector of the exception.
    pub vector: u32,
}
