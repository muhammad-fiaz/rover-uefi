//! # The 64-bit x86 architecture
//!
//! The register layouts of the interrupt controllers, the
//! processor identification and the register images of a taken
//! exception.
//!
//! * `apic` describes the local and the input/output advanced
//!   programmable interrupt controllers.
//! * `context` describes the exception and the system register
//!   images that the processor saves.
//! * `cpuid` describes the processor identification.

pub mod apic;
pub mod cache;
pub mod cmos;
pub mod context;
pub mod control_regs;
pub mod cpuid;
pub mod gdt;
pub mod hpet;
pub mod idt;
pub mod io;
pub mod microcode;
pub mod msr;
pub mod paging;
pub mod pic;
pub mod pit;
pub mod pm_timer;
pub mod uart16550;

pub use gdt::{
    DescriptorTablePointer, SegmentDescriptor, SegmentSelector, TaskStateSegment64, halt, lgdt,
    ltr, read_cs, read_ds, read_es, read_fs, read_gs, read_ss, sgdt,
};
pub use idt::{
    GATE_PRESENT, GATE_TYPE_INTERRUPT64, GATE_TYPE_TRAP64, IDT_ENTRY_COUNT, Idt, IdtEntry, lidt,
    sidt,
};

pub use control_regs::{
    Cr0Flags, Cr3, Cr4Flags, Xcr0Flags, read_cr0, read_cr2, read_cr3, read_cr4, read_xcr0,
    write_cr0, write_cr3, write_cr4, write_xcr0,
};
pub use io::{Port8, Port16, Port32, in_u8, in_u16, in_u32, io_wait, out_u8, out_u16, out_u32};
pub use microcode::{
    CpuMicrocodeDate, CpuMicrocodeExtendedTableEntry, CpuMicrocodeExtendedTableHeader,
    CpuMicrocodeHeader, CpuMicrocodeProcessorSignature, calculate_microcode_checksum,
};
pub use msr::{
    ApicBase, EferFlags, IA32_APIC_BASE, IA32_EFER, IA32_FMASK, IA32_FS_BASE, IA32_GS_BASE,
    IA32_KERNEL_GS_BASE, IA32_LSTAR, IA32_MTRR_CAP, IA32_MTRR_DEF_TYPE, IA32_PAT, IA32_STAR,
    IA32_SYSENTER_CS, IA32_SYSENTER_EIP, IA32_SYSENTER_ESP, IA32_TSC, IA32_TSC_ADJUST,
    IA32_TSC_AUX, IA32_TSC_DEADLINE, PatMemoryType, PatRegister, rdmsr, rdtsc, rdtscp, wrmsr,
};
pub use paging::{
    PAGE_ENTRIES_PER_TABLE, PAGE_SIZE_1G, PAGE_SIZE_2M, PAGE_SIZE_4K, PageTable, PageTableEntry,
    PageTableFlags, PagingMode, canonicalize_48, canonicalize_57, flush_tlb, invlpg,
    page_offset_1g, page_offset_2m, page_offset_4k, pd_index, pdpt_index, pml4_index, pml5_index,
    pt_index,
};

pub use cmos::{
    CMOS_ADDRESS_PORT, CMOS_DATA_PORT, CMOS_NMI_DISABLE_BIT, CMOS_REG_CENTURY,
    CMOS_REG_DAY_OF_MONTH, CMOS_REG_DAY_OF_WEEK, CMOS_REG_HOURS, CMOS_REG_MINUTES, CMOS_REG_MONTH,
    CMOS_REG_SECONDS, CMOS_REG_STATUS_A, CMOS_REG_STATUS_B, CMOS_REG_STATUS_C, CMOS_REG_STATUS_D,
    CMOS_REG_YEAR, CmosDateTime, STATUS_A_UPDATE_IN_PROGRESS, STATUS_B_24_HOUR_MODE,
    STATUS_B_BINARY_MODE, bcd_to_binary, binary_to_bcd,
};
pub use hpet::{HPET_DEFAULT_BASE_ADDRESS, HpetController};
pub use pic::{
    ChainedPic, ICW1_ICW4, ICW1_INIT, ICW4_8086, PIC_EOI, PIC_MASK_ALL, PIC1_COMMAND, PIC1_DATA,
    PIC2_COMMAND, PIC2_DATA,
};
pub use pit::{
    PIT_BASE_FREQUENCY_HZ, PIT_CHANNEL0_DATA, PIT_CHANNEL1_DATA, PIT_CHANNEL2_DATA,
    PIT_COMMAND_PORT, PitAccessMode, PitChannel, PitOperatingMode, build_pit_command,
    compute_pit_divisor,
};
pub use pm_timer::{
    ACPI_PM_TIMER_FREQUENCY_HZ, PM_TIMER_24BIT_MASK, microseconds_to_pm_timer_ticks,
    pm_timer_elapsed_microseconds,
};
pub use uart16550::{
    UART_BASE_CLOCK_HZ, UART_MAX_BAUD, Uart16550, com_port as uart_com_port,
    compute_baud_divisor as compute_uart_baud_divisor, fcr as uart_fcr, lcr as uart_lcr,
    lsr as uart_lsr, mcr as uart_mcr, reg as uart_reg,
};

pub use apic::{
    APIC_EOI, APIC_ICR_HIGH, APIC_ICR_LOW, APIC_ID, APIC_LVT_TIMER, APIC_SVR, APIC_TIMER_CURCNT,
    APIC_TIMER_DIV, APIC_TIMER_INITCNT, APIC_TPR, DeliveryMode, DestinationMode,
    DestinationShorthand, IO_APIC_DATA_OFFSET, IO_APIC_INDEX_OFFSET, IO_APIC_REDIRECTION_INDEX,
    IoApic, IoApicRedirection, IoInterruptVector, IpiRequest, IrqRoutingEntry, IrqSource,
    LocalApic, Polarity, TriggerMode,
};
pub use context::{
    EXCEPTION_VECTOR_ALIGNMENT_CHECK, EXCEPTION_VECTOR_BREAKPOINT,
    EXCEPTION_VECTOR_CONTROL_PROTECTION, EXCEPTION_VECTOR_DEBUG,
    EXCEPTION_VECTOR_DEVICE_NOT_AVAILABLE, EXCEPTION_VECTOR_DIVIDE_ERROR,
    EXCEPTION_VECTOR_DOUBLE_FAULT, EXCEPTION_VECTOR_FLOATING_POINT,
    EXCEPTION_VECTOR_GENERAL_PROTECTION_FAULT, EXCEPTION_VECTOR_INVALID_OPCODE,
    EXCEPTION_VECTOR_MACHINE_CHECK, EXCEPTION_VECTOR_NON_MASKABLE_INTERRUPT,
    EXCEPTION_VECTOR_OVERFLOW, EXCEPTION_VECTOR_PAGE_FAULT, EXCEPTION_VECTOR_SIMD_FLOATING_POINT,
    EXCEPTION_VECTOR_SPURIOUS, EXCEPTION_VECTOR_VIRTUALIZATION, GeneralRegisters, InterruptContext,
    SegmentSelectors, SystemContext,
};
pub use cpuid::{
    AsciiString, CpuInfo, Cpuid, CpuidResult, ExtendedFeatures, HostCpuid, Leaf1Features,
    Leaf7Features, Leaf7Subleaf1Features, StubCpuid, XsaveFeatures,
};
