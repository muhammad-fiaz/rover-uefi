//! # Low-Level Synchronization and Concurrency Primitives
//!
//! Provides mutual exclusion locks, FIFO ticket locks, interrupt-safe locks,
//! hardware memory barriers, and multiprocessor atomic operations for firmware
//! systems running without an operating system scheduler.

pub mod atomic;
pub mod barrier;
pub mod intr_lock;
pub mod spin;
pub mod ticket;

pub use atomic::{
    atomic_add_u64, atomic_add_usize, atomic_and_u32, atomic_compare_exchange_u16,
    atomic_compare_exchange_u32, atomic_compare_exchange_u64, atomic_decrement_i32,
    atomic_decrement_i64, atomic_decrement_u32, atomic_decrement_u64, atomic_increment_i32,
    atomic_increment_i64, atomic_increment_u32, atomic_increment_u64, atomic_or_u32,
    atomic_sub_u64, atomic_xor_u32,
};
pub use barrier::{
    compiler_fence, cpu_pause, cpu_relax, instruction_barrier, memory_barrier, read_memory_barrier,
    write_memory_barrier,
};
pub use intr_lock::{
    InterruptState, IrqSpinLock, IrqSpinLockGuard, disable_interrupts, enable_interrupts,
    interrupts_enabled, restore_interrupts, without_interrupts,
};
pub use spin::{SpinLock, SpinLockGuard};
pub use ticket::{TicketLock, TicketLockGuard};
