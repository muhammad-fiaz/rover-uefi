//! # Architecture abstraction
//!
//! The native description of the processor architectures that the
//! firmware interfaces can run on.
//!
//! The module is split in two layers.
//!
//! The `common` layer holds the primitives that every
//! architecture shares: the volatile accesses to device
//! registers, the monotonic time source and the description of
//! the buffers that a device transfers directly.
//!
//! The `x86`, `arm64` and `riscv` layers describe one
//! architecture each. They hold the register layouts of the
//! interrupt controllers, the processor identification and the
//! register images of a taken exception.
//!
//! The architecture layers are always compiled, so that their
//! descriptions can be inspected and tested on any host. Only
//! the part that executes a privileged instruction is restricted
//! to the architecture that defines it.

pub mod arm64;
pub mod common;
pub mod riscv;
pub mod x86;

pub use arm64::context::{ExceptionClass, ExceptionContext, ExceptionSource};
pub use arm64::gic::{GicCpuInterface, GicDistributor, GicVersion};
pub use common::dma::{
    DMA_MAX_SEGMENTS, DMA_MIN_ALIGNMENT, DMA_PAGE_ALIGNMENT, DmaBuffer, DmaDirection, DmaError,
    DmaSegment, DmaSegmentList,
};
pub use common::timer::{
    MILLIS_PER_SEC, MonotonicTimer, NANOS_PER_HOUR, NANOS_PER_MICRO, NANOS_PER_MILLI,
    NANOS_PER_MINUTE, NANOS_PER_SEC, TickAccumulator, Timer, TimerError,
};
pub use riscv::context::{TrapCause, TrapContext};
pub use riscv::sbi::{SbiEcall, SbiError, SbiRet};
pub use x86::apic::{
    DeliveryMode, DestinationMode, DestinationShorthand, IoApic, IoApicRedirection,
    IoInterruptVector, IpiRequest, IrqRoutingEntry, IrqSource, LocalApic, Polarity, TriggerMode,
};
pub use x86::context::{
    EXCEPTION_VECTOR_DOUBLE_FAULT, EXCEPTION_VECTOR_GENERAL_PROTECTION_FAULT,
    EXCEPTION_VECTOR_PAGE_FAULT, GeneralRegisters, InterruptContext, SegmentSelectors,
    SystemContext,
};
pub use x86::cpuid::{CpuInfo, Cpuid, CpuidResult, HostCpuid, StubCpuid};
