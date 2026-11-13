//! # 16550 UART Controller
//!
//! Driver and register definitions for the National Semiconductor 16550A
//! (and compatible 16450 / 8250) Universal Asynchronous Receiver-Transmitter.
//!
//! Provides standard PC COM port I/O registers, baud rate divisor calculations,
//! FIFO control, and polling transmit/receive interfaces.

/// Standard COM port base addresses on PC/AT platforms.
pub mod com_port {
    /// COM1 standard I/O port base.
    pub const COM1: u16 = 0x03F8;
    /// COM2 standard I/O port base.
    pub const COM2: u16 = 0x02F8;
    /// COM3 standard I/O port base.
    pub const COM3: u16 = 0x03E8;
    /// COM4 standard I/O port base.
    pub const COM4: u16 = 0x02E8;
}

/// 16550 UART Register Offsets (relative to base I/O port).
pub mod reg {
    /// Receiver Buffer Register (read) / Transmitter Holding Register (write).
    pub const RBR_THR: u16 = 0;
    /// Divisor Latch Low Byte (when DLAB = 1).
    pub const DLL: u16 = 0;
    /// Interrupt Enable Register (when DLAB = 0).
    pub const IER: u16 = 1;
    /// Divisor Latch High Byte (when DLAB = 1).
    pub const DLM: u16 = 1;
    /// Interrupt Identification Register (read) / FIFO Control Register (write).
    pub const IIR_FCR: u16 = 2;
    /// Line Control Register.
    pub const LCR: u16 = 3;
    /// Modem Control Register.
    pub const MCR: u16 = 4;
    /// Line Status Register.
    pub const LSR: u16 = 5;
    /// Modem Status Register.
    pub const MSR: u16 = 6;
    /// Scratch Register.
    pub const SCR: u16 = 7;
}

/// Line Control Register (LCR) bit masks.
pub mod lcr {
    /// Word length 5 bits.
    pub const WORD_LEN_5: u8 = 0b00;
    /// Word length 6 bits.
    pub const WORD_LEN_6: u8 = 0b01;
    /// Word length 7 bits.
    pub const WORD_LEN_7: u8 = 0b10;
    /// Word length 8 bits.
    pub const WORD_LEN_8: u8 = 0b11;
    /// Two stop bits (or 1.5 if 5-bit words).
    pub const STOP_BITS_2: u8 = 1 << 2;
    /// Parity enable.
    pub const PARITY_ENABLE: u8 = 1 << 3;
    /// Even parity select.
    pub const PARITY_EVEN: u8 = 1 << 4;
    /// Sticky parity.
    pub const PARITY_STICKY: u8 = 1 << 5;
    /// Set break control.
    pub const SET_BREAK: u8 = 1 << 6;
    /// Divisor Latch Access Bit (DLAB).
    pub const DLAB: u8 = 1 << 7;
}

/// FIFO Control Register (FCR) bit masks.
pub mod fcr {
    /// Enable FIFOs.
    pub const FIFO_ENABLE: u8 = 1 << 0;
    /// Clear Receiver FIFO.
    pub const CLEAR_RX: u8 = 1 << 1;
    /// Clear Transmitter FIFO.
    pub const CLEAR_TX: u8 = 1 << 2;
    /// DMA mode select.
    pub const DMA_MODE: u8 = 1 << 3;
    /// Trigger level 1 byte.
    pub const TRIGGER_1_BYTE: u8 = 0b00 << 6;
    /// Trigger level 4 bytes.
    pub const TRIGGER_4_BYTES: u8 = 0b01 << 6;
    /// Trigger level 8 bytes.
    pub const TRIGGER_8_BYTES: u8 = 0b10 << 6;
    /// Trigger level 14 bytes.
    pub const TRIGGER_14_BYTES: u8 = 0b11 << 6;
}

/// Modem Control Register (MCR) bit masks.
pub mod mcr {
    /// Data Terminal Ready (DTR).
    pub const DTR: u8 = 1 << 0;
    /// Request to Send (RTS).
    pub const RTS: u8 = 1 << 1;
    /// Output 1 (auxiliary user-designated output).
    pub const OUT1: u8 = 1 << 2;
    /// Output 2 (enables UART interrupts on PC architecture).
    pub const OUT2: u8 = 1 << 3;
    /// Loopback test mode.
    pub const LOOPBACK: u8 = 1 << 4;
}

/// Line Status Register (LSR) bit masks.
pub mod lsr {
    /// Data Ready: at least one character in RBR or FIFO.
    pub const DATA_READY: u8 = 1 << 0;
    /// Overrun Error.
    pub const OVERRUN_ERROR: u8 = 1 << 1;
    /// Parity Error.
    pub const PARITY_ERROR: u8 = 1 << 2;
    /// Framing Error.
    pub const FRAMING_ERROR: u8 = 1 << 3;
    /// Break Indicator.
    pub const BREAK_INDICATOR: u8 = 1 << 4;
    /// Transmitter Holding Register Empty (ready for new character).
    pub const TX_HOLDING_EMPTY: u8 = 1 << 5;
    /// Transmitter Empty (both THR and shift register empty).
    pub const TX_EMPTY: u8 = 1 << 6;
    /// Error in Receiver FIFO.
    pub const FIFO_ERROR: u8 = 1 << 7;
}

/// Base clock frequency of standard PC UART hardware in Hz (1.8432 MHz).
pub const UART_BASE_CLOCK_HZ: u32 = 1_843_200;

/// Maximum baud rate of standard PC UART hardware (115,200 baud).
pub const UART_MAX_BAUD: u32 = 115_200;

/// Computes the 16-bit divisor latch for a desired baud rate.
///
/// Divisor = `UART_BASE_CLOCK_HZ / (16 * baud)`.
/// Returns 1 for rates at or above 115,200 baud.
#[must_use]
pub fn compute_baud_divisor(baud: u32) -> u16 {
    if baud == 0 {
        return 1;
    }
    let divisor = UART_BASE_CLOCK_HZ / (16 * baud);
    let bounded = divisor.clamp(1, u32::from(u16::MAX));
    u16::try_from(bounded).unwrap_or(1)
}

/// Hardware controller interface for a 16550 UART on standard I/O ports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uart16550 {
    base_port: u16,
}

impl Uart16550 {
    /// Creates a UART controller at the given base port address.
    #[must_use]
    pub const fn new(base_port: u16) -> Self {
        Self { base_port }
    }

    /// Returns the base I/O port address of this UART.
    #[must_use]
    pub const fn base_port(&self) -> u16 {
        self.base_port
    }

    /// Returns the port address for a given register offset.
    #[must_use]
    pub const fn port_for(&self, offset: u16) -> u16 {
        self.base_port.wrapping_add(offset)
    }

    /// Tests if the line status register flags indicate the transmitter is ready.
    #[must_use]
    pub const fn is_tx_ready(lsr_val: u8) -> bool {
        (lsr_val & lsr::TX_HOLDING_EMPTY) != 0
    }

    /// Tests if the line status register flags indicate data is waiting to be read.
    #[must_use]
    pub const fn is_rx_ready(lsr_val: u8) -> bool {
        (lsr_val & lsr::DATA_READY) != 0
    }
}
