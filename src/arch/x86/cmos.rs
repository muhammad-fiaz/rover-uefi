//! # MC146818 Real-Time Clock and CMOS RAM
//!
//! Provides hardware-level I/O port constants, register offsets, BCD conversion,
//! and datetime formatting for x86 platform real-time clocks and CMOS NVRAM.

/// CMOS Address / Index I/O port.
pub const CMOS_ADDRESS_PORT: u16 = 0x70;
/// CMOS Data I/O port.
pub const CMOS_DATA_PORT: u16 = 0x71;

/// NMI disable bitmask applied to index port writes.
pub const CMOS_NMI_DISABLE_BIT: u8 = 0x80;

/// Seconds register index (00-59).
pub const CMOS_REG_SECONDS: u8 = 0x00;
/// Minutes register index (00-59).
pub const CMOS_REG_MINUTES: u8 = 0x02;
/// Hours register index (00-23 in 24-hr mode).
pub const CMOS_REG_HOURS: u8 = 0x04;
/// Day of week register index (01-07, Sunday = 1).
pub const CMOS_REG_DAY_OF_WEEK: u8 = 0x06;
/// Day of month register index (01-31).
pub const CMOS_REG_DAY_OF_MONTH: u8 = 0x07;
/// Month register index (01-12).
pub const CMOS_REG_MONTH: u8 = 0x08;
/// Year register index (00-99).
pub const CMOS_REG_YEAR: u8 = 0x09;
/// Century register index (typically from ACPI FADT century field, commonly 0x32).
pub const CMOS_REG_CENTURY: u8 = 0x32;

/// Status Register A: Update-In-Progress (`UIP`) flag bit (bit 7).
pub const CMOS_REG_STATUS_A: u8 = 0x0A;
/// Status Register B: 24-hour mode (bit 1) and Binary mode (bit 2).
pub const CMOS_REG_STATUS_B: u8 = 0x0B;
/// Status Register C: Interrupt flag status.
pub const CMOS_REG_STATUS_C: u8 = 0x0C;
/// Status Register D: CMOS battery power valid bit (bit 7).
pub const CMOS_REG_STATUS_D: u8 = 0x0D;

/// Status Register A Update-In-Progress bit.
pub const STATUS_A_UPDATE_IN_PROGRESS: u8 = 0x80;
/// Status Register B 24-hour format bit (1 = 24-hr, 0 = 12-hr).
pub const STATUS_B_24_HOUR_MODE: u8 = 0x02;
/// Status Register B Binary mode bit (1 = binary, 0 = BCD).
pub const STATUS_B_BINARY_MODE: u8 = 0x04;

/// Convert a Binary-Coded Decimal (`BCD`) byte to a binary integer.
#[must_use]
pub const fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

/// Convert a binary integer (0-99) to a Binary-Coded Decimal (`BCD`) byte.
#[must_use]
pub const fn binary_to_bcd(binary: u8) -> u8 {
    (((binary / 10) & 0x0F) << 4) | (binary % 10)
}

/// Real-time clock date and time snapshot decoded from CMOS.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CmosDateTime {
    /// Full four-digit calendar year (e.g. 2026).
    pub year: u16,
    /// Calendar month (1-12).
    pub month: u8,
    /// Day of the month (1-31).
    pub day: u8,
    /// Hour of the day (0-23).
    pub hour: u8,
    /// Minute of the hour (0-59).
    pub minute: u8,
    /// Second of the minute (0-59).
    pub second: u8,
}

impl CmosDateTime {
    /// Check whether this datetime representation contains valid ranges.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.month >= 1
            && self.month <= 12
            && self.day >= 1
            && self.day <= 31
            && self.hour < 24
            && self.minute < 60
            && self.second < 60
    }
}
