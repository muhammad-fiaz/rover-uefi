//! # 8254 Programmable Interval Timer (PIT)
//!
//! Provides hardware-level frequency divider calculations, operating modes,
//! and timer channel control for legacy PC/AT chipsets and emulators.

/// Base oscillator frequency of the 8254 PIT in Hertz (~1.193182 MHz).
pub const PIT_BASE_FREQUENCY_HZ: u32 = 1_193_182;

/// Channel 0 data I/O port (system timer IRQ0).
pub const PIT_CHANNEL0_DATA: u16 = 0x40;
/// Channel 1 data I/O port (legacy DRAM refresh).
pub const PIT_CHANNEL1_DATA: u16 = 0x41;
/// Channel 2 data I/O port (PC speaker tone generator).
pub const PIT_CHANNEL2_DATA: u16 = 0x42;
/// Mode/Command register I/O port.
pub const PIT_COMMAND_PORT: u16 = 0x43;

/// 8254 Operating Modes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitOperatingMode {
    /// Mode 0: Interrupt on terminal count.
    InterruptOnTerminalCount = 0,
    /// Mode 1: Hardware re-triggerable one-shot.
    HardwareOneShot = 1,
    /// Mode 2: Rate generator.
    RateGenerator = 2,
    /// Mode 3: Square wave generator.
    SquareWave = 3,
    /// Mode 4: Software triggered strobe.
    SoftwareStrobe = 4,
    /// Mode 5: Hardware triggered strobe.
    HardwareStrobe = 5,
}

/// 8254 Access Modes for divisor loading.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitAccessMode {
    /// Latch count value command.
    LatchCount = 0,
    /// Access low byte only.
    LowByteOnly = 1,
    /// Access high byte only.
    HighByteOnly = 2,
    /// Access low byte then high byte.
    LowHighByte = 3,
}

/// 8254 Timer Channel Selection.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitChannel {
    /// Channel 0 (System Timer IRQ0).
    Channel0 = 0,
    /// Channel 1 (Legacy DRAM refresh).
    Channel1 = 1,
    /// Channel 2 (PC speaker).
    Channel2 = 2,
    /// Read-back command.
    ReadBack = 3,
}

/// Construct the 8254 command byte from components.
#[must_use]
pub const fn build_pit_command(
    channel: PitChannel,
    access: PitAccessMode,
    mode: PitOperatingMode,
    bcd: bool,
) -> u8 {
    let ch = (channel as u8) << 6;
    let acc = (access as u8) << 4;
    let md = (mode as u8) << 1;
    let bcd_flag = if bcd { 1 } else { 0 };
    ch | acc | md | bcd_flag
}

/// Compute the 16-bit divisor for a desired target frequency in Hertz.
///
/// Divisor values range from 1 to 65535; a divisor of 0 encodes 65536 (~18.2 Hz).
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn compute_pit_divisor(target_hz: u32) -> u16 {
    if target_hz <= 18 {
        0
    } else if target_hz >= PIT_BASE_FREQUENCY_HZ {
        1
    } else {
        let divisor = PIT_BASE_FREQUENCY_HZ / target_hz;
        if divisor > 0xFFFF { 0 } else { divisor as u16 }
    }
}
