//! # High Precision Event Timer (HPET) Hardware
//!
//! MMIO register layouts, capability masks, and timer configuration for the
//! x86 High Precision Event Timer specification.

/// Standard default HPET base physical address on x86 platforms.
pub const HPET_DEFAULT_BASE_ADDRESS: u64 = 0xFED0_0000;

/// HPET MMIO Register Offsets.
pub mod reg {
    /// General Capabilities and ID Register (64-bit).
    pub const GENERAL_CAPABILITIES_ID: usize = 0x000;
    /// General Configuration Register (64-bit).
    pub const GENERAL_CONFIGURATION: usize = 0x010;
    /// General Interrupt Status Register (64-bit).
    pub const GENERAL_INTERRUPT_STATUS: usize = 0x020;
    /// Main Counter Value Register (64-bit).
    pub const MAIN_COUNTER_VALUE: usize = 0x0F0;
    /// Base offset for Timer N registers.
    pub const TIMER_BASE: usize = 0x100;
    /// Stride between successive Timer register blocks.
    pub const TIMER_STRIDE: usize = 0x020;
    /// Timer N Configuration and Capabilities Register (relative to timer base).
    pub const TIMER_CONFIG_CAP: usize = 0x000;
    /// Timer N Comparator Value Register (relative to timer base).
    pub const TIMER_COMPARATOR: usize = 0x008;
    /// Timer N FSB Interrupt Route Register (relative to timer base).
    pub const TIMER_FSB_ROUTE: usize = 0x010;
}

/// General Capabilities and ID bit masks.
pub mod cap {
    /// Revision ID mask (bits 7:0).
    pub const REV_ID_MASK: u64 = 0xFF;
    /// Number of timers minus 1 (bits 12:8).
    pub const NUM_TIM_MASK: u64 = 0x1F << 8;
    /// Main counter size: 0 = 32-bit, 1 = 64-bit (bit 13).
    pub const COUNT_SIZE_CAP_64: u64 = 1 << 13;
    /// Legacy replacement route capable (bit 15).
    pub const LEGACY_ROUTE_CAP: u64 = 1 << 15;
    /// Vendor ID mask (bits 31:16).
    pub const VENDOR_ID_MASK: u64 = 0xFFFF << 16;
    /// Counter clock period in femtoseconds (10^-15 s) (bits 63:32).
    pub const COUNTER_CLK_PERIOD_MASK: u64 = 0xFFFF_FFFF << 32;
}

/// General Configuration bit masks.
pub mod config {
    /// Overall Enable bit: starts the main counter.
    pub const ENABLE_CNF: u64 = 1 << 0;
    /// Legacy Replacement Route: maps Timer 0 to IRQ0/2 and Timer 1 to IRQ8.
    pub const LEG_RT_CNF: u64 = 1 << 1;
}

/// Timer N Configuration bit masks.
pub mod timer_config {
    /// Interrupt Type: 0 = edge-triggered, 1 = level-triggered.
    pub const LEVEL_TRIGGERED: u64 = 1 << 1;
    /// Interrupt Enable.
    pub const INTERRUPT_ENABLE: u64 = 1 << 2;
    /// Type: 0 = non-periodic, 1 = periodic.
    pub const PERIODIC_ENABLE: u64 = 1 << 3;
    /// Periodic Interrupt Capable (read-only).
    pub const PERIODIC_CAPABLE: u64 = 1 << 4;
    /// 64-bit Timer Capable (read-only).
    pub const SIZE_64_CAPABLE: u64 = 1 << 5;
    /// Set 32-bit Mode for a 64-bit timer.
    pub const FORCE_32BIT: u64 = 1 << 8;
    /// Interrupt Routing Mask (bits 13:9).
    pub const ROUTE_MASK: u64 = 0x1F << 9;
    /// FSB / MSI Interrupt Delivery Enable.
    pub const FSB_ENABLE: u64 = 1 << 14;
    /// FSB Delivery Capable (read-only).
    pub const FSB_CAPABLE: u64 = 1 << 15;
}

/// Hardware controller descriptor for an HPET memory-mapped peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HpetController {
    base_address: u64,
}

impl HpetController {
    /// Creates an HPET controller abstraction at the specified MMIO physical address.
    #[must_use]
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    /// Returns the physical MMIO base address.
    #[must_use]
    pub const fn base_address(&self) -> u64 {
        self.base_address
    }

    /// Extracts the clock tick period in femtoseconds from the capabilities register.
    #[must_use]
    pub const fn tick_period_fs(cap_reg: u64) -> u32 {
        (cap_reg >> 32) as u32
    }

    /// Extracts the total number of hardware comparator timers from capabilities.
    #[must_use]
    pub const fn timer_count(cap_reg: u64) -> u8 {
        let count_minus_one = (cap_reg >> 8) & 0x1F;
        (count_minus_one as u8).saturating_add(1)
    }

    /// Returns whether the main counter is 64-bit wide.
    #[must_use]
    pub const fn is_64bit_counter(cap_reg: u64) -> bool {
        (cap_reg & cap::COUNT_SIZE_CAP_64) != 0
    }
}
