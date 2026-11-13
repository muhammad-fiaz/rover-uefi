//! # Architecture independent primitives
//!
//! The parts of the hardware abstraction that do not depend on a
//! particular processor architecture.
//!
//! * `mmio` describes the volatile accesses to device
//!   registers.
//! * `timer` describes the monotonic time source and the
//!   arithmetic around a periodic tick.
//! * `dma` describes the buffers that a device transfers
//!   directly.
//! * `cache` describes the operations that maintain instruction
//!   and data caches.

pub mod cache;
pub mod dma;
pub mod mmio;
pub mod timer;

pub use cache::{
    CACHE_LINE_SIZE_DEFAULT, cache_line_align_down, cache_line_align_up, cache_line_span,
    invalidate_instruction_cache, invalidate_instruction_cache_range, writeback_data_cache,
    writeback_data_cache_range, writeback_invalidate_data_cache,
    writeback_invalidate_data_cache_range,
};

pub use dma::{
    DMA_MAX_SEGMENTS, DMA_MIN_ALIGNMENT, DMA_PAGE_ALIGNMENT, DmaBuffer, DmaDirection, DmaError,
    DmaSegment, DmaSegmentList, contiguous_buffer, contiguous_segment, is_aligned,
};
pub use mmio::{
    MmioReg, and_u8, and_u16, and_u32, and_u64, and_usize, compiler_barrier, memory_barrier,
    modify_u8, modify_u16, modify_u32, modify_u64, modify_usize, or_u8, or_u16, or_u32, or_u64,
    or_usize, read_memory_barrier, read_u8, read_u16, read_u32, read_u64, read_usize,
    write_memory_barrier, write_u8, write_u16, write_u32, write_u64, write_usize,
};
pub use timer::{
    MILLIS_PER_SEC, MonotonicTimer, NANOS_PER_HOUR, NANOS_PER_MICRO, NANOS_PER_MILLI,
    NANOS_PER_MINUTE, NANOS_PER_SEC, TickAccumulator, Timer, TimerError, millis_to_nanos,
    nanos_to_millis, nanos_to_secs, nanos_to_ticks, secs_to_nanos, ticks_to_nanos,
};
