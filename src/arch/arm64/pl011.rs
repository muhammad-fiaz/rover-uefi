//! # ARM `PrimeCell` PL011 UART Controller
//!
//! Register definitions and calculation helpers for the ARM `PrimeCell`
//! UART (PL011) memory-mapped hardware peripheral.

/// PL011 MMIO register offsets (in bytes).
pub mod reg {
    /// Data Register.
    pub const UARTDR: u32 = 0x000;
    /// Receive Status Register / Error Clear Register.
    pub const UARTRSR: u32 = 0x004;
    /// Flag Register.
    pub const UARTFR: u32 = 0x018;
    /// Low-power Counter Register.
    pub const UARTILPR: u32 = 0x020;
    /// Integer Baud Rate Register.
    pub const UARTIBRD: u32 = 0x024;
    /// Fractional Baud Rate Register.
    pub const UARTFBRD: u32 = 0x028;
    /// Line Control Register.
    pub const UARTLCR_H: u32 = 0x02C;
    /// Control Register.
    pub const UARTCR: u32 = 0x030;
    /// Interrupt FIFO Level Select Register.
    pub const UARTIFLS: u32 = 0x034;
    /// Interrupt Mask Set/Clear Register.
    pub const UARTIMSC: u32 = 0x038;
    /// Raw Interrupt Status Register.
    pub const UARTRIS: u32 = 0x03C;
    /// Masked Interrupt Status Register.
    pub const UARTMIS: u32 = 0x040;
    /// Interrupt Clear Register.
    pub const UARTICR: u32 = 0x044;
    /// DMA Control Register.
    pub const UARTDMACR: u32 = 0x048;
}

/// Flag Register (UARTFR) bit masks.
pub mod fr {
    /// Clear to Send.
    pub const CTS: u32 = 1 << 0;
    /// Data Set Ready.
    pub const DSR: u32 = 1 << 1;
    /// Data Carrier Detect.
    pub const DCD: u32 = 1 << 2;
    /// UART Busy.
    pub const BUSY: u32 = 1 << 3;
    /// Receive FIFO Empty.
    pub const RXFE: u32 = 1 << 4;
    /// Transmit FIFO Full.
    pub const TXFF: u32 = 1 << 5;
    /// Receive FIFO Full.
    pub const RXFF: u32 = 1 << 6;
    /// Transmit FIFO Empty.
    pub const TXFE: u32 = 1 << 7;
    /// Ring Indicator.
    pub const RI: u32 = 1 << 8;
}

/// Line Control Register (`UARTLCR_H`) bit masks.
pub mod lcr_h {
    /// Send Break.
    pub const BRK: u32 = 1 << 0;
    /// Parity Enable.
    pub const PEN: u32 = 1 << 1;
    /// Even Parity Select.
    pub const EPS: u32 = 1 << 2;
    /// Two Stop Bits Select.
    pub const STP2: u32 = 1 << 3;
    /// Enable FIFOs.
    pub const FEN: u32 = 1 << 4;
    /// Word length 5 bits.
    pub const WLEN_5: u32 = 0b00 << 5;
    /// Word length 6 bits.
    pub const WLEN_6: u32 = 0b01 << 5;
    /// Word length 7 bits.
    pub const WLEN_7: u32 = 0b10 << 5;
    /// Word length 8 bits.
    pub const WLEN_8: u32 = 0b11 << 5;
    /// Stick Parity Select.
    pub const SPS: u32 = 1 << 7;
}

/// Control Register (UARTCR) bit masks.
pub mod cr {
    /// UART Enable.
    pub const UARTEN: u32 = 1 << 0;
    /// SIREN (SIR enable).
    pub const SIREN: u32 = 1 << 1;
    /// SIRLP (SIR low power).
    pub const SIRLP: u32 = 1 << 2;
    /// Transmit Enable.
    pub const TXE: u32 = 1 << 8;
    /// Receive Enable.
    pub const RXE: u32 = 1 << 9;
    /// Data Terminal Ready.
    pub const DTR: u32 = 1 << 10;
    /// Request to Send.
    pub const RTS: u32 = 1 << 11;
    /// Out1.
    pub const OUT1: u32 = 1 << 12;
    /// Out2.
    pub const OUT2: u32 = 1 << 13;
    /// RTS Hardware Flow Control Enable.
    pub const RTSEN: u32 = 1 << 14;
    /// CTS Hardware Flow Control Enable.
    pub const CTSEN: u32 = 1 << 15;
}

/// Baud rate divisors for PL011.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pl011BaudDivisors {
    /// Integer baud rate divisor (16 bits, written to UARTIBRD).
    pub integer_divisor: u16,
    /// Fractional baud rate divisor (6 bits, written to UARTFBRD).
    pub fractional_divisor: u8,
}

/// Calculates the integer and fractional baud rate divisors for PL011.
///
/// Formula: `Divisor = uart_clk_hz / (16 * baud)`.
/// `fractional_divisor = round((remainder * 64) / divisor_denominator)`.
#[must_use]
pub fn compute_pl011_divisors(uart_clk_hz: u32, baud: u32) -> Pl011BaudDivisors {
    if uart_clk_hz == 0 || baud == 0 {
        return Pl011BaudDivisors {
            integer_divisor: 1,
            fractional_divisor: 0,
        };
    }

    let denom = 16_u64 * u64::from(baud);
    let clk = u64::from(uart_clk_hz);

    let int_part = clk / denom;
    let remainder = clk % denom;

    // Fractional part: round(remainder * 64 / denom)
    let frac_scaled = (remainder * 64) + (denom / 2);
    let frac_part = (frac_scaled / denom) & 0x3F;

    let clamped_int = int_part.clamp(1, u64::from(u16::MAX));
    let integer_divisor = u16::try_from(clamped_int).unwrap_or(1);
    let fractional_divisor = u8::try_from(frac_part).unwrap_or(0);

    Pl011BaudDivisors {
        integer_divisor,
        fractional_divisor,
    }
}

/// Hardware controller abstraction for an ARM `PrimeCell` PL011 UART at a physical MMIO address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pl011Uart {
    base_address: u64,
}

impl Pl011Uart {
    /// Creates a PL011 controller descriptor at the specified MMIO base physical address.
    #[must_use]
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    /// Returns the MMIO base address.
    #[must_use]
    pub const fn base_address(&self) -> u64 {
        self.base_address
    }

    /// Computes the register offset physical address.
    #[must_use]
    pub const fn reg_address(&self, offset: u32) -> u64 {
        self.base_address.wrapping_add(offset as u64)
    }

    /// Returns whether the transmit FIFO is full according to the flag register.
    #[must_use]
    pub const fn is_tx_full(flag_reg: u32) -> bool {
        (flag_reg & fr::TXFF) != 0
    }

    /// Returns whether the receive FIFO is empty according to the flag register.
    #[must_use]
    pub const fn is_rx_empty(flag_reg: u32) -> bool {
        (flag_reg & fr::RXFE) != 0
    }
}
