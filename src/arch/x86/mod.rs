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
pub mod context;
pub mod cpuid;

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
