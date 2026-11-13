//! # ACPI Power Management Timer (PM Timer)
//!
//! Provides precision timestamp calculations based on the standard 3.579545 MHz
//! ACPI PM Timer present across modern PC motherboards and chipsets.

/// Standard ACPI Power Management Timer frequency in Hz (3.579545 MHz).
pub const ACPI_PM_TIMER_FREQUENCY_HZ: u32 = 3_579_545;

/// 24-bit timer rollover mask.
pub const PM_TIMER_24BIT_MASK: u32 = 0x00FF_FFFF;

/// Computes the elapsed microseconds between two PM timer sample readings.
///
/// Handles 24-bit counter rollover gracefully.
#[must_use]
pub fn pm_timer_elapsed_microseconds(start_ticks: u32, end_ticks: u32) -> u64 {
    let diff = end_ticks.wrapping_sub(start_ticks) & PM_TIMER_24BIT_MASK;
    let ticks = u64::from(diff);
    // Microseconds = (ticks * 1_000_000) / 3_579_545
    (ticks * 1_000_000) / u64::from(ACPI_PM_TIMER_FREQUENCY_HZ)
}

/// Converts a microsecond duration to the corresponding count of PM Timer ticks.
#[must_use]
pub fn microseconds_to_pm_timer_ticks(us: u64) -> u32 {
    let ticks = (us * u64::from(ACPI_PM_TIMER_FREQUENCY_HZ)) / 1_000_000;
    let bounded = ticks.min(u64::from(PM_TIMER_24BIT_MASK));
    u32::try_from(bounded).unwrap_or(PM_TIMER_24BIT_MASK)
}
