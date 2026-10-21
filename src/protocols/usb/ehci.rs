//! # Enhanced Host Controller Interface
//!
//! Register definitions and transfer-structure encodings for the USB
//! 2.0 Enhanced Host Controller Interface (`EHCI`), as described by the
//! `EHCI` specification.
//!
//! The module covers the capability registers, the operational
//! registers, the port status bits, the queue head and the queue
//! transfer descriptor. It deliberately does not implement a complete
//! host-controller driver: there is no periodic or asynchronous
//! schedule manager, no interrupt handling loop and no enumeration
//! state machine. Those belong to a controller driver that uses the
//! structures defined here.
//!
//! Register access goes through [`EhciMmio`], which is constructed from
//! a mapped register base. Every accessor is unsafe because the caller
//! must guarantee that the base address names a mapped `EHCI` register
//! window and that exclusive ownership of the controller has been
//! established.

use crate::arch::common::mmio::{read_u32, write_u32};

/// The size of the capability register block in bytes.
pub const CAPABILITY_BLOCK_SIZE: usize = 0x0C;

/// Offset of `CAPLENGTH` inside the capability block.
pub const CAPLENGTH: usize = 0x00;

/// Offset of `HCIVERSION` inside the capability block.
pub const HCIVERSION: usize = 0x02;

/// Offset of `USBCMD` from the operational base.
pub const USBCMD: usize = 0x00;

/// Offset of `USBSTS` from the operational base.
pub const USBSTS: usize = 0x04;

/// Offset of `USBINTR` from the operational base.
pub const USBINTR: usize = 0x08;

/// Offset of `FRINDEX` from the operational base.
pub const FRINDEX: usize = 0x0C;

/// Offset of `CTRLDSSEGMENT` from the operational base.
pub const CTRLDSSEGMENT: usize = 0x10;

/// Offset of `PERIODICLISTBASE` from the operational base.
pub const PERIODICLISTBASE: usize = 0x14;

/// Offset of `ASYNCLISTADDR` from the operational base.
pub const ASYNCLISTADDR: usize = 0x18;

/// Offset of `CONFIGFLAG` from the operational base.
pub const CONFIGFLAG: usize = 0x40;

/// Offset of the first port status register from the operational base.
pub const PORTSC_BASE: usize = 0x44;

/// `USBCMD` run/stop.
pub const USBCMD_RUN_STOP: u32 = 1 << 0;

/// `USBCMD` host controller reset.
pub const USBCMD_HC_RESET: u32 = 1 << 1;

/// `USBCMD` frame list size bit.
pub const USBCMD_FRAME_LIST_SIZE: u32 = 1 << 2;

/// `USBCMD` periodic schedule enable.
pub const USBCMD_PERIODIC_ENABLE: u32 = 1 << 4;

/// `USBCMD` asynchronous schedule enable.
pub const USBCMD_ASYNC_ENABLE: u32 = 1 << 5;

/// `USBCMD` interrupt on async advance doorbell.
pub const USBCMD_ASYNC_ADVANCE: u32 = 1 << 6;

/// Shift of the interrupt threshold control field.
pub const USBCMD_INTERRUPT_THRESHOLD_SHIFT: u32 = 16;

/// Mask of the interrupt threshold control field.
pub const USBCMD_INTERRUPT_THRESHOLD_MASK: u32 = 0xFF << USBCMD_INTERRUPT_THRESHOLD_SHIFT;

/// Shift of the frame list size encoding.
pub const USBCMD_FRAME_LIST_SIZE_SHIFT: u32 = 2;

/// Mask of the frame list size encoding.
pub const USBCMD_FRAME_LIST_SIZE_MASK: u32 = 0x03 << USBCMD_FRAME_LIST_SIZE_SHIFT;

/// `USBSTS` USB interrupt.
pub const USBSTS_USB_INTERRUPT: u32 = 1 << 0;

/// `USBSTS` USB error interrupt.
pub const USBSTS_USB_ERROR: u32 = 1 << 1;

/// `USBSTS` port change detect.
pub const USBSTS_PORT_CHANGE: u32 = 1 << 2;

/// `USBSTS` frame list rollover.
pub const USBSTS_FRAME_LIST_ROLLOVER: u32 = 1 << 3;

/// `USBSTS` host system error.
pub const USBSTS_HOST_SYSTEM_ERROR: u32 = 1 << 4;

/// `USBSTS` interrupt on async advance.
pub const USBSTS_ASYNC_ADVANCE: u32 = 1 << 5;

/// `USBSTS` host controller halted.
pub const USBSTS_HALTED: u32 = 1 << 12;

/// `USBSTS` reclamation.
pub const USBSTS_RECLAMATION: u32 = 1 << 13;

/// `USBSTS` periodic schedule status.
pub const USBSTS_PERIODIC_STATUS: u32 = 1 << 14;

/// `USBSTS` asynchronous schedule status.
pub const USBSTS_ASYNC_STATUS: u32 = 1 << 15;

/// All writable status bits cleared by writing ones.
pub const USBSTS_WRITE_ONE_TO_CLEAR: u32 = USBSTS_USB_INTERRUPT
    | USBSTS_USB_ERROR
    | USBSTS_PORT_CHANGE
    | USBSTS_FRAME_LIST_ROLLOVER
    | USBSTS_HOST_SYSTEM_ERROR
    | USBSTS_ASYNC_ADVANCE;

/// `PORTSC` current connect status.
pub const PORTSC_CONNECT: u32 = 1 << 0;

/// `PORTSC` connect status change.
pub const PORTSC_CONNECT_CHANGE: u32 = 1 << 1;

/// `PORTSC` port enabled or disabled.
pub const PORTSC_ENABLE: u32 = 1 << 2;

/// `PORTSC` port enable change.
pub const PORTSC_ENABLE_CHANGE: u32 = 1 << 3;

/// `PORTSC` over-current active.
pub const PORTSC_OVER_CURRENT: u32 = 1 << 4;

/// `PORTSC` over-current change.
pub const PORTSC_OVER_CURRENT_CHANGE: u32 = 1 << 5;

/// `PORTSC` force port resume.
pub const PORTSC_FORCE_RESUME: u32 = 1 << 6;

/// `PORTSC` suspend.
pub const PORTSC_SUSPEND: u32 = 1 << 7;

/// `PORTSC` port reset.
pub const PORTSC_RESET: u32 = 1 << 8;

/// `PORTSC` line status change.
pub const PORTSC_LINE_STATUS_CHANGE: u32 = 1 << 11;

/// `PORTSC` port power.
pub const PORTSC_POWER: u32 = 1 << 12;

/// `PORTSC` port owner.
pub const PORTSC_OWNER: u32 = 1 << 13;

/// Shift of the port indicator control field.
pub const PORTSC_INDICATOR_SHIFT: u32 = 14;

/// `PORTSC` port test control shift.
pub const PORTSC_TEST_SHIFT: u32 = 16;

/// `PORTSC` wake on connect enable.
pub const PORTSC_WAKE_CONNECT: u32 = 1 << 20;

/// `PORTSC` wake on disconnect enable.
pub const PORTSC_WAKE_DISCONNECT: u32 = 1 << 21;

/// `PORTSC` wake on over-current enable.
pub const PORTSC_WAKE_OVER_CURRENT: u32 = 1 << 22;

/// The bits of `PORTSC` that are write-one-to-clear.
pub const PORTSC_WRITE_ONE_TO_CLEAR: u32 = PORTSC_CONNECT_CHANGE
    | PORTSC_ENABLE_CHANGE
    | PORTSC_OVER_CURRENT_CHANGE
    | PORTSC_LINE_STATUS_CHANGE;

/// The size of a queue head in bytes.
pub const QUEUE_HEAD_SIZE: usize = 48;

/// The alignment required for a queue head.
pub const QUEUE_HEAD_ALIGNMENT: usize = 32;

/// The size of a queue transfer descriptor in bytes.
pub const QUEUE_TD_SIZE: usize = 32;

/// The alignment required for a queue transfer descriptor.
pub const QUEUE_TD_ALIGNMENT: usize = 32;

/// Queue head token `ACTIVE`.
pub const QTD_ACTIVE: u32 = 1 << 7;

/// Queue head token `HALTED`.
pub const QTD_HALTED: u32 = 1 << 6;

/// Queue head token `DATA_BUFFER_ERROR`.
pub const QTD_DATA_BUFFER_ERROR: u32 = 1 << 5;

/// Queue head token `BABBLE`.
pub const QTD_BABBLE: u32 = 1 << 4;

/// Queue head token `TRANSACTION_ERROR`.
pub const QTD_TRANSACTION_ERROR: u32 = 1 << 3;

/// Queue head token `MISSED_MICROFRAME`.
pub const QTD_MISSED_MICROFRAME: u32 = 1 << 2;

/// Queue head token `SPLIT_STATE` shift.
pub const QTD_SPLIT_STATE_SHIFT: u32 = 0;

/// Queue head token `PING_STATE` shift.
pub const QTD_PING_STATE_SHIFT: u32 = 1;

/// Queue head token `PID` shift.
pub const QTD_PID_SHIFT: u32 = 8;

/// Queue head token `ERROR_COUNTER` shift.
pub const QTD_ERROR_COUNTER_SHIFT: u32 = 10;

/// Queue head token `CURRENT_PAGE` shift.
pub const QTD_CURRENT_PAGE_SHIFT: u32 = 12;

/// Queue head token `INTERRUPT_ON_COMPLETE` bit.
pub const QTD_INTERRUPT_ON_COMPLETE: u32 = 1 << 15;

/// Queue head token `TOTAL_BYTES` shift.
pub const QTD_TOTAL_BYTES_SHIFT: u32 = 16;

/// Queue head token `DATA_TOGGLE` bit.
pub const QTD_DATA_TOGGLE: u32 = 1 << 31;

/// A queue transaction descriptor.
///
/// The encoding follows the `EHCI` queue element transfer descriptor:
/// a next-pointer double word, an alternate next-pointer double word,
/// a token double word and five buffer pointer double words.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueTd {
    /// Physical address of the next descriptor, or the terminate bit.
    pub next: u32,
    /// Alternate next descriptor and status flags.
    pub alternate: u32,
    /// Transfer token.
    pub token: u32,
    /// Buffer pointer page 0.
    pub buffer0: u32,
    /// Buffer pointer page 1.
    pub buffer1: u32,
    /// Buffer pointer page 2.
    pub buffer2: u32,
    /// Buffer pointer page 3.
    pub buffer3: u32,
    /// Buffer pointer page 4.
    pub buffer4: u32,
}

impl QueueTd {
    /// The terminate bit in a link pointer.
    pub const TERMINATE: u32 = 1;

    /// Creates an empty, terminated descriptor.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next: Self::TERMINATE,
            alternate: 0,
            token: 0,
            buffer0: 0,
            buffer1: 0,
            buffer2: 0,
            buffer3: 0,
            buffer4: 0,
        }
    }

    /// Encodes the descriptor into its 32-byte little-endian image.
    #[must_use]
    pub fn encode(&self) -> [u8; QUEUE_TD_SIZE] {
        let mut bytes = [0_u8; QUEUE_TD_SIZE];
        for (index, value) in [
            self.next,
            self.alternate,
            self.token,
            self.buffer0,
            self.buffer1,
            self.buffer2,
            self.buffer3,
            self.buffer4,
        ]
        .iter()
        .enumerate()
        {
            let start = index * 4;
            bytes[start..start + 4].copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    /// Decodes a descriptor from its 32-byte little-endian image.
    #[must_use]
    pub fn decode(bytes: &[u8; QUEUE_TD_SIZE]) -> Self {
        let word = |index: usize| {
            let start = index * 4;
            u32::from_le_bytes([
                bytes[start],
                bytes[start + 1],
                bytes[start + 2],
                bytes[start + 3],
            ])
        };
        Self {
            next: word(0),
            alternate: word(1),
            token: word(2),
            buffer0: word(3),
            buffer1: word(4),
            buffer2: word(5),
            buffer3: word(6),
            buffer4: word(7),
        }
    }

    /// Returns whether the descriptor is still active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.token & QTD_ACTIVE != 0
    }

    /// Returns whether the controller halted the descriptor.
    #[must_use]
    pub const fn is_halted(&self) -> bool {
        self.token & QTD_HALTED != 0
    }

    /// Returns whether any error status bit is set.
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.token & (QTD_DATA_BUFFER_ERROR | QTD_BABBLE | QTD_TRANSACTION_ERROR) != 0
    }

    /// Returns the number of bytes remaining in the transfer.
    #[must_use]
    pub const fn bytes_remaining(&self) -> u32 {
        (self.token >> QTD_TOTAL_BYTES_SHIFT) & 0x7FFF
    }

    /// Returns whether the next-pointer terminates the list.
    #[must_use]
    pub const fn next_is_terminated(&self) -> bool {
        self.next & Self::TERMINATE != 0
    }
}

/// A capability register snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EhciCapabilities {
    /// The length of the capability block in bytes.
    pub capability_length: u8,
    /// The interface version.
    pub interface_version: u16,
    /// The structural parameters (`HCSPARAMS`).
    pub structural_parameters: u32,
    /// The capability parameters (`HCCPARAMS`).
    pub capability_parameters: u32,
}

impl EhciCapabilities {
    /// Returns the number of ports from the structural parameters.
    #[must_use]
    pub const fn port_count(&self) -> u8 {
        (self.structural_parameters & 0x0F) as u8
    }

    /// Returns the number of port power control switches.
    #[must_use]
    pub const fn power_control_switches(&self) -> bool {
        self.structural_parameters & (1 << 4) != 0
    }

    /// Returns the number of companion controllers.
    #[must_use]
    pub const fn companion_controllers(&self) -> u8 {
        ((self.structural_parameters >> 12) & 0x0F) as u8
    }

    /// Returns whether the extended capability pointer is available.
    #[must_use]
    pub const fn extended_capabilities(&self) -> bool {
        self.capability_parameters & 1 != 0
    }

    /// Returns whether 64-bit addressing is supported.
    #[must_use]
    pub const fn address_64_bit(&self) -> bool {
        self.capability_parameters & (1 << 0) != 0 && self.capability_parameters & (1 << 1) != 0
    }
}

/// A decoded port status and control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortStatus(u32);

impl PortStatus {
    /// Creates a port status from the raw register value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw register value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }

    /// Returns whether a device is connected.
    #[must_use]
    pub const fn connected(self) -> bool {
        self.0 & PORTSC_CONNECT != 0
    }

    /// Returns whether the port is enabled.
    #[must_use]
    pub const fn enabled(self) -> bool {
        self.0 & PORTSC_ENABLE != 0
    }

    /// Returns whether the port is suspended.
    #[must_use]
    pub const fn suspended(self) -> bool {
        self.0 & PORTSC_SUSPEND != 0
    }

    /// Returns whether the port is in reset.
    #[must_use]
    pub const fn in_reset(self) -> bool {
        self.0 & PORTSC_RESET != 0
    }

    /// Returns whether the port is powered.
    #[must_use]
    pub const fn powered(self) -> bool {
        self.0 & PORTSC_POWER != 0
    }

    /// Returns whether the port is owned by a companion controller.
    #[must_use]
    pub const fn owned_by_companion(self) -> bool {
        self.0 & PORTSC_OWNER != 0
    }

    /// Returns whether an over-current condition is active.
    #[must_use]
    pub const fn over_current(self) -> bool {
        self.0 & PORTSC_OVER_CURRENT != 0
    }

    /// Returns the raw port speed encoding.
    ///
    /// The `EHCI` register does not carry the speed directly; a driver
    /// derives the speed from the line status and the reset result.
    #[must_use]
    pub const fn line_status(self) -> u8 {
        ((self.0 >> 10) & 0x03) as u8
    }
}

/// A mapped `EHCI` register window.
///
/// The type only performs volatile register accesses. It does not own
/// the controller and does not synchronise with other users.
#[derive(Clone, Copy, Debug)]
pub struct EhciMmio {
    capability_base: usize,
    operational_base: usize,
    capability_length: u8,
}

impl EhciMmio {
    /// Creates a register window from a mapped capability base.
    ///
    /// # Safety
    ///
    /// `capability_base` must denote a mapped `EHCI` capability register
    /// block that remains valid for the lifetime of every access made
    /// through the returned value. The caller must own the controller
    /// exclusively.
    #[must_use]
    pub unsafe fn new(capability_base: usize) -> Option<Self> {
        // SAFETY: The caller guarantees that the capability block is
        // mapped, so the length byte can be read.
        let capability_length = unsafe { crate::arch::common::mmio::read_u8(capability_base) };
        if capability_length < CAPABILITY_BLOCK_SIZE as u8 {
            return None;
        }
        Some(Self {
            capability_base,
            operational_base: capability_base.checked_add(usize::from(capability_length))?,
            capability_length,
        })
    }

    /// Returns the length of the capability block.
    #[must_use]
    pub const fn capability_length(&self) -> u8 {
        self.capability_length
    }

    /// Reads the capability registers.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped.
    #[must_use]
    pub unsafe fn capabilities(&self) -> EhciCapabilities {
        // SAFETY: The caller guarantees that the window is mapped.
        unsafe {
            EhciCapabilities {
                capability_length: crate::arch::common::mmio::read_u8(self.capability_base),
                interface_version: crate::arch::common::mmio::read_u16(
                    self.capability_base + HCIVERSION,
                ),
                structural_parameters: read_u32(self.capability_base + 0x04),
                capability_parameters: read_u32(self.capability_base + 0x08),
            }
        }
    }

    /// Reads an operational register.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and `offset` must name a
    /// valid 32-bit operational register.
    #[must_use]
    pub unsafe fn read_operational(&self, offset: usize) -> u32 {
        // SAFETY: The caller guarantees the window and register.
        unsafe { read_u32(self.operational_base + offset) }
    }

    /// Writes an operational register.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and `offset` must name a
    /// valid writable 32-bit operational register.
    pub unsafe fn write_operational(&self, offset: usize, value: u32) {
        // SAFETY: The caller guarantees the window and register.
        unsafe { write_u32(self.operational_base + offset, value) }
    }

    /// Reads a port status and control register.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and `port` must be a
    /// valid zero-based port index.
    #[must_use]
    pub unsafe fn port_status(&self, port: usize) -> PortStatus {
        // SAFETY: The caller guarantees the window and port index.
        let raw = unsafe { read_u32(self.operational_base + PORTSC_BASE + port * 4) };
        PortStatus::from_raw(raw)
    }

    /// Writes a port status and control register.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and `port` must be a
    /// valid zero-based port index.
    pub unsafe fn write_port_status(&self, port: usize, value: u32) {
        // SAFETY: The caller guarantees the window and port index.
        unsafe { write_u32(self.operational_base + PORTSC_BASE + port * 4, value) }
    }

    /// Requests a host controller reset.
    ///
    /// The caller must poll [`EhciMmio::is_reset_complete`] afterwards.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and the caller must own
    /// the controller exclusively.
    pub unsafe fn begin_reset(&self) {
        // SAFETY: The caller guarantees the window and ownership.
        let command = unsafe { self.read_operational(USBCMD) };
        // SAFETY: Same guarantee.
        unsafe { self.write_operational(USBCMD, command | USBCMD_HC_RESET) };
    }

    /// Returns whether the host controller reset has completed.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped.
    #[must_use]
    pub unsafe fn is_reset_complete(&self) -> bool {
        // SAFETY: The caller guarantees the window is mapped.
        unsafe { self.read_operational(USBCMD) & USBCMD_HC_RESET == 0 }
    }

    /// Enables the asynchronous schedule.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped, the schedule must have
    /// been programmed, and the caller must own the controller.
    pub unsafe fn enable_async_schedule(&self) {
        // SAFETY: The caller guarantees the window and ownership.
        unsafe {
            let value = self.read_operational(USBCMD);
            self.write_operational(USBCMD, value | USBCMD_ASYNC_ENABLE);
        }
    }

    /// Starts the controller.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped, the schedules must have
    /// been programmed, and the caller must own the controller.
    pub unsafe fn start(&self) {
        // SAFETY: The caller guarantees the window and ownership.
        unsafe {
            let value = self.read_operational(USBCMD);
            self.write_operational(USBCMD, value | USBCMD_RUN_STOP);
        }
    }

    /// Stops the controller.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped and the caller must own
    /// the controller.
    pub unsafe fn stop(&self) {
        // SAFETY: The caller guarantees the window and ownership.
        unsafe {
            let value = self.read_operational(USBCMD);
            self.write_operational(USBCMD, value & !USBCMD_RUN_STOP);
        }
    }

    /// Clears write-one-to-clear status bits.
    ///
    /// # Safety
    ///
    /// The register window must still be mapped.
    pub unsafe fn clear_status(&self, bits: u32) {
        // SAFETY: The caller guarantees the window is mapped.
        unsafe { self.write_operational(USBSTS, bits & USBSTS_WRITE_ONE_TO_CLEAR) }
    }
}

#[cfg(test)]
mod tests {
    use super::{EhciCapabilities, PortStatus, QueueTd};
    use super::{
        PORTSC_CONNECT, PORTSC_ENABLE, PORTSC_OWNER, QUEUE_TD_SIZE, USBSTS_HALTED, USBSTS_PERIODIC_STATUS,
    };

    #[test]
    fn port_status_decoding() {
        let status = PortStatus::from_raw(
            PORTSC_CONNECT | PORTSC_ENABLE | (3 << 10) | USBSTS_PERIODIC_STATUS,
        );
        assert!(status.connected());
        assert!(status.enabled());
        assert!(!status.suspended());
        assert_eq!(status.line_status(), 3);
        assert!(!status.owned_by_companion());
        assert_eq!(status.to_raw() & USBSTS_HALTED, 0);
        assert_eq!(PortStatus::from_raw(PORTSC_OWNER).owned_by_companion(), true);
    }

    #[test]
    fn queue_td_round_trip() {
        let mut td = QueueTd::new();
        td.token = super::QTD_ACTIVE | (0x40 << super::QTD_TOTAL_BYTES_SHIFT);
        td.buffer0 = 0x1000;
        let encoded = td.encode();
        assert_eq!(encoded.len(), QUEUE_TD_SIZE);
        let decoded = QueueTd::decode(&encoded);
        assert_eq!(decoded, td);
        assert!(decoded.is_active());
        assert_eq!(decoded.bytes_remaining(), 0x40);
        assert!(decoded.next_is_terminated());
    }

    #[test]
    fn capability_decoding() {
        let capabilities = EhciCapabilities {
            capability_length: 0x20,
            interface_version: 0x0100,
            structural_parameters: 0x0000_0006 | (2 << 12),
            capability_parameters: 0x0000_0003,
        };
        assert_eq!(capabilities.port_count(), 6);
        assert_eq!(capabilities.companion_controllers(), 2);
        assert!(capabilities.address_64_bit());
        assert!(capabilities.extended_capabilities());
    }
}
