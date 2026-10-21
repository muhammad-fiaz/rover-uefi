//! # xHCI
//!
//! Native support for the eXtensible Host Controller Interface
//! (`xHCI`), the host controller interface defined by the USB 3.1
//! specification.
//!
//! The module covers the capability, runtime and operational registers,
//! the doorbell array, the command ring, the transfer rings, the event
//! ring and the device contexts.
//!
//! All register offsets are relative to the base of the controller's
//! memory mapped register space. The operational registers start at the
//! offset reported by `CAPLENGTH`, the runtime registers at the offset
//! reported by `RTSOFF` and the doorbell array at the offset reported
//! by `DBOFF`.

pub mod context;
pub mod ring;
pub mod trb;

pub use context::{
    CONTEXT_SIZE, CONTROL_CONTEXT_OFFSET, ControlEndpointContext, DeviceContext, EndpointContext,
    EndpointState, EndpointType, SLOT_CONTEXT_OFFSET, SlotContext, SlotState,
    endpoint_context_offset,
};
pub use ring::{CommandRing, EventRingSegment, TransferRing};
pub use trb::{
    CommandCompletionEvent, CommandTrb, CompletionCode, ControlTransfer, ControlTransferType,
    DataStageTrb, DisableSlotTrb, EnableSlotTrb, EventDataTrb, HostControllerNotificationEvent,
    LinkTrb, NoOpCommandTrb, NoOpTrb, NormalTrb, PortStatusChangeEvent, SetupStageTrb,
    StatusStageTrb, TRB_SIZE, TransferEvent, Trb, TrbType, UnknownTrb,
};

/// The offset of the capability register length byte.
pub const CAPAB_LENGTH_OFFSET: usize = 0x00;

/// The offset of the host controller interface version field.
pub const CAPAB_HCIVERSION_OFFSET: usize = 0x02;

/// The offset of the first host controller structural parameter word.
pub const CAPAB_HCSPARAMS1_OFFSET: usize = 0x04;

/// The offset of the second host controller structural parameter word.
pub const CAPAB_HCSPARAMS2_OFFSET: usize = 0x08;

/// The offset of the third host controller structural parameter word.
pub const CAPAB_HCSPARAMS3_OFFSET: usize = 0x0C;

/// The offset of the first host controller capability parameter word.
pub const CAPAB_HCCPARAMS1_OFFSET: usize = 0x10;

/// The offset of the doorbell array offset field.
pub const CAPAB_DBOFF_OFFSET: usize = 0x14;

/// The offset of the runtime register offset field.
pub const CAPAB_RTSOFF_OFFSET: usize = 0x18;

/// The offset of the second host controller capability parameter word.
pub const CAPAB_HCCPARAMS2_OFFSET: usize = 0x1C;

/// The offset of the microframe index register, relative to the runtime
/// registers.
pub const RTREGS_MFINDEX_OFFSET: usize = 0x000;

/// The offset of the interrupter register block, relative to the runtime
/// registers.
pub const RTREGS_INTERRUPTOR_OFFSET: usize = 0x020;

/// The size of one interrupter register block in bytes.
pub const RTREGS_INTERRUPTOR_STRIDE: usize = 0x020;

/// The offset of the event ring segment table size register.
pub const RTREGS_ERSTSZ_OFFSET: usize = 0x028;

/// The offset of the event ring segment table base address register.
pub const RTREGS_ERSTBA_OFFSET: usize = 0x030;

/// The offset of the event ring dequeue pointer register.
pub const RTREGS_ERDP_OFFSET: usize = 0x038;

/// The offset of the USB command register, relative to the operational
/// registers.
pub const OPREGS_USBCMD_OFFSET: usize = 0x00;

/// The offset of the USB status register, relative to the operational
/// registers.
pub const OPREGS_USBSTS_OFFSET: usize = 0x04;

/// The offset of the page size register, relative to the operational
/// registers.
pub const OPREGS_PAGESIZE_OFFSET: usize = 0x08;

/// The offset of the device notification control register.
pub const OPREGS_DNCTRL_OFFSET: usize = 0x14;

/// The offset of the command ring control register.
pub const OPREGS_CRCR_OFFSET: usize = 0x18;

/// The offset of the device context base address array pointer register.
pub const OPREGS_DCBAAP_OFFSET: usize = 0x30;

/// The offset of the configuration register.
pub const OPREGS_CONFIG_OFFSET: usize = 0x38;

/// The offset of the first port status and control register.
pub const OPREGS_PORTSC_OFFSET: usize = 0x400;

/// The size of one doorbell register in bytes.
pub const DOORBELL_SIZE: usize = 4;

/// The size of one entry of the event ring segment table in bytes.
pub const ERST_ENTRY_SIZE: usize = 16;

/// The capability registers of a host controller.
///
/// Several fields are single-bit hardware register flags, so the boolean
/// representation is deliberate.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HcCapabilities {
    /// The length of the capability register block in bytes.
    pub cap_length: u8,
    /// The host controller interface version, in binary coded decimal.
    pub hci_version: u16,
    /// The number of device slots the controller implements.
    pub max_slots: u8,
    /// The number of interrupters the controller implements.
    pub max_interrupter: u16,
    /// The number of root hub ports the controller implements.
    pub max_ports: u8,
    /// The interrupt scheduling threshold.
    pub interrupt_scheduling_threshold: u8,
    /// The maximum number of event ring segment table entries.
    pub erst_max: u8,
    /// The number of scratchpad buffers the controller implements.
    pub max_scratchpad_buffers: u16,
    /// Whether the controller implements scratchpad buffers.
    pub scratchpad_buffer_present: bool,
    /// Whether the controller implements 64-bit addressing.
    pub addressing_64_bit: bool,
    /// The size of a context in bytes, either 32 or 64.
    pub context_size: u32,
    /// Whether the controller implements port power control.
    pub port_power_control: bool,
    /// Whether the controller supports contiguous frame identifiers.
    pub contiguous_frame_id: bool,
    /// The maximum size of the primary stream context array.
    pub max_primary_stream_array_size: u8,
    /// The offset of the extended capability list, in double words.
    pub extended_capability_offset: u32,
    /// The offset of the doorbell array from the register base.
    pub doorbell_offset: u32,
    /// The offset of the runtime registers from the register base.
    pub runtime_offset: u32,
}

impl HcCapabilities {
    /// Parses the capability registers of a controller.
    ///
    /// Returns `None` when the register block is too short.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let cap_length = *bytes.get(CAPAB_LENGTH_OFFSET)?;
        let hci_version = u16::from_le_bytes([
            *bytes.get(CAPAB_HCIVERSION_OFFSET)?,
            *bytes.get(CAPAB_HCIVERSION_OFFSET + 1)?,
        ]);
        let params1 = read_word(bytes, CAPAB_HCSPARAMS1_OFFSET)?;
        let params2 = read_word(bytes, CAPAB_HCSPARAMS2_OFFSET)?;
        let capabilities = read_word(bytes, CAPAB_HCCPARAMS1_OFFSET)?;
        let doorbell = read_word(bytes, CAPAB_DBOFF_OFFSET)?;
        let runtime = read_word(bytes, CAPAB_RTSOFF_OFFSET)?;
        let scratchpad_high = (params2 >> 21) & 0x1F;
        let scratchpad_low = (params2 >> 27) & 0x1F;
        Some(Self {
            cap_length,
            hci_version,
            max_slots: u8::try_from(params1 & 0xFF).unwrap_or_default(),
            max_interrupter: u16::try_from((params1 >> 8) & 0x7FFF).unwrap_or_default(),
            max_ports: u8::try_from((params1 >> 24) & 0xFF).unwrap_or_default(),
            interrupt_scheduling_threshold: u8::try_from(params2 & 0x0F).unwrap_or_default(),
            erst_max: u8::try_from((params2 >> 4) & 0x0F).unwrap_or_default(),
            max_scratchpad_buffers: u16::try_from((scratchpad_high << 5) | scratchpad_low)
                .unwrap_or_default(),
            scratchpad_buffer_present: params2 & (1 << 26) != 0,
            addressing_64_bit: capabilities & 1 != 0,
            context_size: if capabilities & (1 << 2) != 0 { 64 } else { 32 },
            port_power_control: capabilities & (1 << 3) != 0,
            contiguous_frame_id: capabilities & (1 << 11) != 0,
            max_primary_stream_array_size: u8::try_from((capabilities >> 12) & 0x0F)
                .unwrap_or_default(),
            extended_capability_offset: (capabilities >> 16) & 0xFFFF,
            doorbell_offset: (doorbell >> 2) & 0x3FFF_FFFF,
            runtime_offset: (runtime >> 5) & 0x07FF_FFFF,
        })
    }

    /// Returns the offset of the operational registers.
    #[must_use]
    pub fn operational_offset(&self) -> usize {
        usize::from(self.cap_length)
    }

    /// Returns the number of event ring segment table entries the
    /// controller supports.
    #[must_use]
    pub fn erst_entry_count(&self) -> usize {
        usize::from(1_u8.checked_shl(u32::from(self.erst_max)).unwrap_or(0))
    }
}

/// A doorbell register of the doorbell array.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Doorbell {
    /// The doorbell target: zero for the host controller, otherwise the
    /// device slot identifier.
    pub target: u8,
    /// The stream identifier of the target endpoint.
    pub stream_id: u16,
}

impl Doorbell {
    /// Creates the doorbell that rings the host controller command ring.
    #[must_use]
    pub const fn host_controller() -> Self {
        Self {
            target: 0,
            stream_id: 0,
        }
    }

    /// Creates the doorbell that rings the transfer ring of a device
    /// slot.
    #[must_use]
    pub const fn slot(slot_id: u8) -> Self {
        Self {
            target: slot_id,
            stream_id: 0,
        }
    }

    /// Creates a doorbell that rings one stream of an endpoint.
    #[must_use]
    pub const fn stream(slot_id: u8, stream_id: u16) -> Self {
        Self {
            target: slot_id,
            stream_id,
        }
    }

    /// Encodes the doorbell into its 32-bit register value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        u32::from(self.target) | (u32::from(self.stream_id) << 16)
    }

    /// Decodes a doorbell from its 32-bit register value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            target: u8::try_from(value & 0xFF).unwrap_or_default(),
            stream_id: u16::try_from((value >> 16) & 0xFFFF).unwrap_or_default(),
        }
    }

    /// Returns the byte offset of the doorbell inside the doorbell array.
    #[must_use]
    pub fn array_offset(&self) -> usize {
        usize::from(self.target) * DOORBELL_SIZE
    }
}

/// The command ring control register.
///
/// The boolean fields are the individual single-bit register flags.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandRingControl {
    /// The ring cycle state of the command ring.
    pub ring_cycle_state: bool,
    /// Whether the command ring is stopped.
    pub command_ring_stop: bool,
    /// Whether the command ring is aborted.
    pub command_ring_abort: bool,
    /// Whether the command ring is running.
    pub command_ring_running: bool,
    /// The physical address of the command ring segment.
    pub ring_segment_base: u64,
}

impl CommandRingControl {
    /// Encodes the register into its 64-bit value.
    #[must_use]
    pub fn encode(&self) -> u64 {
        let base = self.ring_segment_base & 0x000F_FFFF_FFFF_FFC0;
        base | u64::from(self.ring_cycle_state)
            | (u64::from(self.command_ring_stop) << 1)
            | (u64::from(self.command_ring_abort) << 2)
            | (u64::from(self.command_ring_running) << 3)
    }

    /// Decodes the register from its 64-bit value.
    #[must_use]
    pub fn decode(value: u64) -> Self {
        Self {
            ring_cycle_state: value & 1 != 0,
            command_ring_stop: value & (1 << 1) != 0,
            command_ring_abort: value & (1 << 2) != 0,
            command_ring_running: value & (1 << 3) != 0,
            ring_segment_base: value & 0x000F_FFFF_FFFF_FFC0,
        }
    }
}

/// The device context base address array pointer register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceContextArrayAddress {
    /// The physical address of the device context base address array.
    pub base_address: u64,
}

impl DeviceContextArrayAddress {
    /// Encodes the register into its 64-bit value.
    #[must_use]
    pub fn encode(&self) -> u64 {
        self.base_address & 0x000F_FFFF_FFFF_FFC0
    }

    /// Decodes the register from its 64-bit value.
    #[must_use]
    pub fn decode(value: u64) -> Self {
        Self {
            base_address: value & 0x000F_FFFF_FFFF_FFC0,
        }
    }
}

/// An entry of the event ring segment table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventRingSegmentTableEntry {
    /// The physical address of the event ring segment.
    pub ring_segment_base: u64,
    /// The number of transfer request blocks in the segment.
    pub segment_size: u16,
}

impl EventRingSegmentTableEntry {
    /// Creates an entry for a segment of `segment_size` transfer request
    /// blocks.
    #[must_use]
    pub const fn new(ring_segment_base: u64, segment_size: u16) -> Self {
        Self {
            ring_segment_base,
            segment_size,
        }
    }

    /// Encodes the entry into its 16 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < ERST_ENTRY_SIZE {
            return false;
        }
        let base = self.ring_segment_base.to_le_bytes();
        let size = u32::from(self.segment_size).to_le_bytes();
        let Some(chunk) = out.get_mut(0..8) else {
            return false;
        };
        chunk.copy_from_slice(&base);
        let Some(chunk) = out.get_mut(8..12) else {
            return false;
        };
        chunk.copy_from_slice(&size);
        let Some(chunk) = out.get_mut(12..16) else {
            return false;
        };
        chunk.copy_from_slice(&[0; 4]);
        true
    }

    /// Decodes an entry from its 16 bytes.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let base = bytes.get(0..8)?;
        let mut raw = [0_u8; 8];
        raw.copy_from_slice(base);
        let size = bytes.get(8..12)?;
        let mut raw_size = [0_u8; 4];
        raw_size.copy_from_slice(size);
        Some(Self {
            ring_segment_base: u64::from_le_bytes(raw),
            segment_size: u16::try_from(u32::from_le_bytes(raw_size)).unwrap_or_default(),
        })
    }
}

/// The USB command register.
///
/// The boolean fields are the individual single-bit register flags.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsbCommand {
    /// Whether the controller runs the schedules.
    pub run: bool,
    /// Whether the controller is reset.
    pub host_controller_reset: bool,
    /// Whether the controller generates an interrupt when a command
    /// completes.
    pub interrupt_on_completion_event: bool,
    /// Whether the controller generates an interrupt when the host
    /// controller notification event is written.
    pub host_controller_notification_event: bool,
    /// Whether the controller asserts an interrupt when an event is
    /// posted to the event ring.
    pub host_system_error_event: bool,
    /// Whether the controller interrupts on the asynchronous schedule
    /// advance.
    pub asynchronous_schedule_event: bool,
    /// The number of microframes the controller waits before it raises
    /// an interrupt.
    pub interrupt_threshold: u8,
}

impl UsbCommand {
    /// Creates a command register that leaves the controller stopped.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            run: false,
            host_controller_reset: false,
            interrupt_on_completion_event: false,
            host_controller_notification_event: false,
            host_system_error_event: false,
            asynchronous_schedule_event: false,
            interrupt_threshold: 0,
        }
    }

    /// Encodes the register into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        u32::from(self.run)
            | (u32::from(self.host_controller_reset) << 1)
            | (u32::from(self.interrupt_on_completion_event) << 2)
            | (u32::from(self.host_controller_notification_event) << 3)
            | (u32::from(self.host_system_error_event) << 4)
            | (u32::from(self.asynchronous_schedule_event) << 5)
            | (u32::from(self.interrupt_threshold) << 16)
    }

    /// Decodes the register from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            run: value & 1 != 0,
            host_controller_reset: value & (1 << 1) != 0,
            interrupt_on_completion_event: value & (1 << 2) != 0,
            host_controller_notification_event: value & (1 << 3) != 0,
            host_system_error_event: value & (1 << 4) != 0,
            asynchronous_schedule_event: value & (1 << 5) != 0,
            interrupt_threshold: u8::try_from((value >> 16) & 0xFF).unwrap_or_default(),
        }
    }
}

impl Default for UsbCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// The USB status register.
///
/// The boolean fields are the individual single-bit register flags.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsbStatus {
    /// Whether the controller is halted.
    pub host_controller_halted: bool,
    /// Whether the controller is in the error state.
    pub host_controller_error: bool,
    /// Whether an asynchronous schedule event is pending.
    pub asynchronous_schedule_event: bool,
    /// Whether a port status change is pending.
    pub port_status_change: bool,
    /// Whether the controller is waiting for a doorbell.
    pub controller_not_ready: bool,
    /// Whether a host system error is pending.
    pub host_system_error: bool,
    /// Whether the event ring is full.
    pub event_ring_full: bool,
}

impl UsbStatus {
    /// Encodes the register into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        u32::from(self.host_controller_halted)
            | (u32::from(self.host_controller_error) << 1)
            | (u32::from(self.asynchronous_schedule_event) << 2)
            | (u32::from(self.port_status_change) << 3)
            | (u32::from(self.controller_not_ready) << 4)
            | (u32::from(self.host_system_error) << 5)
            | (u32::from(self.event_ring_full) << 6)
    }

    /// Decodes the register from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            host_controller_halted: value & 1 != 0,
            host_controller_error: value & (1 << 1) != 0,
            asynchronous_schedule_event: value & (1 << 2) != 0,
            port_status_change: value & (1 << 3) != 0,
            controller_not_ready: value & (1 << 4) != 0,
            host_system_error: value & (1 << 5) != 0,
            event_ring_full: value & (1 << 6) != 0,
        }
    }
}

/// Reads a little-endian double word at `offset`.
fn read_word(bytes: &[u8], offset: usize) -> Option<u32> {
    let chunk = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(chunk);
    Some(u32::from_le_bytes(raw))
}
