//! # xHCI device contexts
//!
//! The device context structures used by the eXtensible Host Controller
//! Interface (`xHCI`): the slot context, the default control endpoint
//! context and the general endpoint contexts.
//!
//! A device context is an array of 32-byte contexts. Index 0 holds the
//! slot context, index 1 holds the default control endpoint context and
//! index `n` holds the endpoint context of the endpoint with the device
//! context identifier `n - 1`.

use crate::base::bytes::read_u32_le;

/// The size of a single context in bytes.
pub const CONTEXT_SIZE: usize = 32;

/// The offset of the slot context inside a device context.
pub const SLOT_CONTEXT_OFFSET: usize = 0;

/// The offset of the default control endpoint context.
pub const CONTROL_CONTEXT_OFFSET: usize = CONTEXT_SIZE;

/// The state of an endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndpointState {
    /// The endpoint is disabled.
    Disabled,
    /// The endpoint is running.
    Running,
    /// The endpoint is halted by an error.
    Halted,
    /// The endpoint is stopped by the host.
    Stopped,
    /// The endpoint is in the error state.
    Error,
}

impl EndpointState {
    /// Returns the numeric state value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Disabled => 0,
            Self::Running => 1,
            Self::Halted => 2,
            Self::Stopped => 3,
            Self::Error => 4,
        }
    }

    /// Creates a state from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x7 {
            0 => Self::Disabled,
            1 => Self::Running,
            2 => Self::Halted,
            3 => Self::Stopped,
            _ => Self::Error,
        }
    }
}

/// The type of an endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// An invalid endpoint type.
    Invalid,
    /// An isochronous OUT endpoint.
    IsochronousOut,
    /// A bulk OUT endpoint.
    BulkOut,
    /// An interrupt OUT endpoint.
    InterruptOut,
    /// A control endpoint.
    Control,
    /// An isochronous IN endpoint.
    IsochronousIn,
    /// A bulk IN endpoint.
    BulkIn,
    /// An interrupt IN endpoint.
    InterruptIn,
}

impl EndpointType {
    /// Returns the numeric endpoint type value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Invalid => 0,
            Self::IsochronousOut => 1,
            Self::BulkOut => 2,
            Self::InterruptOut => 3,
            Self::Control => 4,
            Self::IsochronousIn => 5,
            Self::BulkIn => 6,
            Self::InterruptIn => 7,
        }
    }

    /// Creates an endpoint type from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x7 {
            1 => Self::IsochronousOut,
            2 => Self::BulkOut,
            3 => Self::InterruptOut,
            4 => Self::Control,
            5 => Self::IsochronousIn,
            6 => Self::BulkIn,
            7 => Self::InterruptIn,
            _ => Self::Invalid,
        }
    }

    /// Returns whether the endpoint transfers data from the device.
    #[must_use]
    pub fn is_in(self) -> bool {
        matches!(self, Self::IsochronousIn | Self::BulkIn | Self::InterruptIn)
    }
}

/// The state of a device slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlotState {
    /// The slot is disabled or enabled but not yet addressed.
    Disabled,
    /// The slot is in the default state.
    Default,
    /// The slot is addressed.
    Addressed,
    /// The slot is configured.
    Configured,
}

impl SlotState {
    /// Returns the numeric slot state value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Disabled => 0,
            Self::Default => 1,
            Self::Addressed => 2,
            Self::Configured => 3,
        }
    }

    /// Creates a slot state from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x7 {
            1 => Self::Default,
            2 => Self::Addressed,
            3 => Self::Configured,
            _ => Self::Disabled,
        }
    }
}

/// The slot context of a device context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotContext {
    /// The route string of the device.
    pub route_string: u32,
    /// The speed of the device.
    pub speed: u8,
    /// Whether the device hangs off a multi transaction translator hub.
    pub multi_transaction_translator: bool,
    /// Whether the device is a hub.
    pub hub: bool,
    /// The index of the last valid endpoint context.
    pub context_entries: u8,
    /// The maximum exit latency of the device.
    pub max_exit_latency: u16,
    /// The one-based root hub port the device is attached to.
    pub root_hub_port: u8,
    /// The number of ports, valid only for hubs.
    pub number_of_ports: u8,
    /// The slot identifier of the parent hub.
    pub parent_hub_slot: u8,
    /// The port number on the parent hub.
    pub parent_port_number: u8,
    /// The transaction translator think time of a high speed hub.
    pub transaction_translator_think_time: u8,
    /// The interrupter that receives the events of the device.
    pub interrupter_target: u16,
    /// The address the controller assigned to the device.
    pub usb_device_address: u8,
    /// The state of the slot.
    pub slot_state: SlotState,
}

impl SlotContext {
    /// Creates an empty slot context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            route_string: 0,
            speed: 0,
            multi_transaction_translator: false,
            hub: false,
            context_entries: 1,
            max_exit_latency: 0,
            root_hub_port: 0,
            number_of_ports: 0,
            parent_hub_slot: 0,
            parent_port_number: 0,
            transaction_translator_think_time: 0,
            interrupter_target: 0,
            usb_device_address: 0,
            slot_state: SlotState::Disabled,
        }
    }

    /// Encodes the slot context into `out`.
    ///
    /// Returns `false` when `out` is shorter than 32 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < CONTEXT_SIZE {
            return false;
        }
        let word0 = (self.route_string & 0x000F_FFFF)
            | (u32::from(self.speed & 0x0F) << 20)
            | (if self.multi_transaction_translator {
                1 << 25
            } else {
                0
            })
            | (if self.hub { 1 << 26 } else { 0 })
            | (u32::from(self.context_entries & 0x1F) << 27);
        let word1 = u32::from(self.max_exit_latency)
            | (u32::from(self.root_hub_port) << 16)
            | (u32::from(self.number_of_ports) << 24);
        let word2 = u32::from(self.parent_hub_slot)
            | (u32::from(self.parent_port_number) << 8)
            | (u32::from(self.transaction_translator_think_time & 0x3) << 16)
            | (u32::from(self.interrupter_target & 0x3FF) << 22);
        let word3 =
            u32::from(self.usb_device_address) | (u32::from(self.slot_state.to_raw() & 0x1F) << 27);
        write_word(out, 0, word0)
            && write_word(out, 4, word1)
            && write_word(out, 8, word2)
            && write_word(out, 12, word3)
    }

    /// Decodes a slot context from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let word0 = read_u32_le(bytes, 0)?;
        let word1 = read_u32_le(bytes, 4)?;
        let word2 = read_u32_le(bytes, 8)?;
        let word3 = read_u32_le(bytes, 12)?;
        Some(Self {
            route_string: word0 & 0x000F_FFFF,
            speed: u8::try_from((word0 >> 20) & 0x0F).unwrap_or_default(),
            multi_transaction_translator: word0 & (1 << 25) != 0,
            hub: word0 & (1 << 26) != 0,
            context_entries: u8::try_from((word0 >> 27) & 0x1F).unwrap_or_default(),
            max_exit_latency: u16::try_from(word1 & 0xFFFF).unwrap_or_default(),
            root_hub_port: u8::try_from((word1 >> 16) & 0xFF).unwrap_or_default(),
            number_of_ports: u8::try_from((word1 >> 24) & 0xFF).unwrap_or_default(),
            parent_hub_slot: u8::try_from(word2 & 0xFF).unwrap_or_default(),
            parent_port_number: u8::try_from((word2 >> 8) & 0xFF).unwrap_or_default(),
            transaction_translator_think_time: u8::try_from((word2 >> 16) & 0x3)
                .unwrap_or_default(),
            interrupter_target: u16::try_from((word2 >> 22) & 0x3FF).unwrap_or_default(),
            usb_device_address: u8::try_from(word3 & 0xFF).unwrap_or_default(),
            slot_state: SlotState::from_raw(u8::try_from((word3 >> 27) & 0x1F).unwrap_or_default()),
        })
    }
}

impl Default for SlotContext {
    fn default() -> Self {
        Self::new()
    }
}

/// The default control endpoint context of a device context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ControlEndpointContext {
    /// The state of the endpoint.
    pub ep_state: EndpointState,
    /// The error counter of the endpoint.
    pub error_count: u8,
    /// The maximum packet size of the default control pipe.
    pub max_packet_size: u16,
    /// The maximum burst size, zero for the default control pipe.
    pub max_burst_size: u8,
    /// The cycle state of the transfer ring.
    pub dequeue_cycle_state: bool,
    /// The physical address of the transfer ring.
    pub tr_dequeue_pointer: u64,
    /// The average transfer request block length.
    pub average_trb_length: u16,
}

impl ControlEndpointContext {
    /// Creates a control endpoint context with the given packet size.
    #[must_use]
    pub const fn new(max_packet_size: u16) -> Self {
        Self {
            ep_state: EndpointState::Disabled,
            error_count: 3,
            max_packet_size,
            max_burst_size: 0,
            dequeue_cycle_state: true,
            tr_dequeue_pointer: 0,
            average_trb_length: 8,
        }
    }

    /// Encodes the context into `out`.
    ///
    /// Returns `false` when `out` is shorter than 32 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        let word0 = u32::from(self.ep_state.to_raw() & 0x0F)
            | (u32::from(self.error_count & 0x3) << 4)
            | (u32::from(EndpointType::Control.to_raw() & 0x07) << 8);
        let word1 = (u32::from(self.max_burst_size) << 8) | (u32::from(self.max_packet_size) << 16);
        let (dequeue_low, dequeue_high) = split_dequeue_pointer(self.tr_dequeue_pointer);
        let word2 = u32::from(self.dequeue_cycle_state) | dequeue_low;
        let word3 = dequeue_high;
        let word4 = u32::from(self.average_trb_length);
        write_word(out, 0, word0)
            && write_word(out, 4, word1)
            && write_word(out, 8, word2)
            && write_word(out, 12, word3)
            && write_word(out, 16, word4)
    }

    /// Decodes a control endpoint context from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let word0 = read_u32_le(bytes, 0)?;
        let word1 = read_u32_le(bytes, 4)?;
        let word2 = read_u32_le(bytes, 8)?;
        let word3 = read_u32_le(bytes, 12)?;
        let word4 = read_u32_le(bytes, 16)?;
        Some(Self {
            ep_state: EndpointState::from_raw(u8::try_from(word0 & 0x0F).unwrap_or_default()),
            error_count: u8::try_from((word0 >> 4) & 0x3).unwrap_or_default(),
            max_packet_size: u16::try_from((word1 >> 16) & 0xFFFF).unwrap_or_default(),
            max_burst_size: u8::try_from((word1 >> 8) & 0xFF).unwrap_or_default(),
            dequeue_cycle_state: word2 & 1 != 0,
            tr_dequeue_pointer: join_dequeue_pointer(word2, word3),
            average_trb_length: u16::try_from(word4 & 0xFFFF).unwrap_or_default(),
        })
    }
}

impl Default for ControlEndpointContext {
    fn default() -> Self {
        Self::new(64)
    }
}

/// The context of a non-control endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EndpointContext {
    /// The state of the endpoint.
    pub ep_state: EndpointState,
    /// The error counter of the endpoint.
    pub error_count: u8,
    /// The type of the endpoint.
    pub endpoint_type: EndpointType,
    /// The high bandwidth multiplier of the endpoint.
    pub mult: u8,
    /// The number of primary streams, zero when streams are not used.
    pub max_primary_streams: u8,
    /// Whether the stream context array is linear.
    pub linear_stream_array: bool,
    /// The service interval of a periodic endpoint.
    pub interval: u8,
    /// The maximum burst size of the endpoint.
    pub max_burst_size: u8,
    /// The maximum packet size of the endpoint.
    pub max_packet_size: u16,
    /// The cycle state of the transfer ring.
    pub dequeue_cycle_state: bool,
    /// The physical address of the transfer ring.
    pub tr_dequeue_pointer: u64,
    /// The average transfer request block length.
    pub average_trb_length: u16,
    /// The maximum payload per service interval, periodic endpoints only.
    pub max_esit_payload: u16,
}

impl EndpointContext {
    /// Creates an endpoint context of the given type.
    #[must_use]
    pub const fn new(endpoint_type: EndpointType, max_packet_size: u16) -> Self {
        Self {
            ep_state: EndpointState::Disabled,
            error_count: 3,
            endpoint_type,
            mult: 0,
            max_primary_streams: 0,
            linear_stream_array: false,
            interval: 0,
            max_burst_size: 0,
            max_packet_size,
            dequeue_cycle_state: true,
            tr_dequeue_pointer: 0,
            average_trb_length: 8,
            max_esit_payload: 0,
        }
    }

    /// Encodes the context into `out`.
    ///
    /// Returns `false` when `out` is shorter than 32 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        let word0 = u32::from(self.ep_state.to_raw() & 0x0F)
            | (u32::from(self.mult & 0x3) << 8)
            | (u32::from(self.max_primary_streams & 0x1F) << 10)
            | (if self.linear_stream_array { 1 << 15 } else { 0 })
            | (u32::from(self.interval) << 16);
        let word1 = (u32::from(self.error_count & 0x3) << 1)
            | (u32::from(self.endpoint_type.to_raw() & 0x07) << 3)
            | (u32::from(self.max_burst_size) << 8)
            | (u32::from(self.max_packet_size) << 16);
        let (dequeue_low, dequeue_high) = split_dequeue_pointer(self.tr_dequeue_pointer);
        let word2 = u32::from(self.dequeue_cycle_state) | dequeue_low;
        let word3 = dequeue_high;
        let word4 = u32::from(self.average_trb_length) | (u32::from(self.max_esit_payload) << 16);
        write_word(out, 0, word0)
            && write_word(out, 4, word1)
            && write_word(out, 8, word2)
            && write_word(out, 12, word3)
            && write_word(out, 16, word4)
    }

    /// Decodes an endpoint context from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let word0 = read_u32_le(bytes, 0)?;
        let word1 = read_u32_le(bytes, 4)?;
        let word2 = read_u32_le(bytes, 8)?;
        let word3 = read_u32_le(bytes, 12)?;
        let word4 = read_u32_le(bytes, 16)?;
        Some(Self {
            ep_state: EndpointState::from_raw(u8::try_from(word0 & 0x0F).unwrap_or_default()),
            error_count: u8::try_from((word1 >> 1) & 0x3).unwrap_or_default(),
            endpoint_type: EndpointType::from_raw(
                u8::try_from((word1 >> 3) & 0x07).unwrap_or_default(),
            ),
            mult: u8::try_from((word0 >> 8) & 0x3).unwrap_or_default(),
            max_primary_streams: u8::try_from((word0 >> 10) & 0x1F).unwrap_or_default(),
            linear_stream_array: word0 & (1 << 15) != 0,
            interval: u8::try_from((word0 >> 16) & 0xFF).unwrap_or_default(),
            max_burst_size: u8::try_from((word1 >> 8) & 0xFF).unwrap_or_default(),
            max_packet_size: u16::try_from((word1 >> 16) & 0xFFFF).unwrap_or_default(),
            dequeue_cycle_state: word2 & 1 != 0,
            tr_dequeue_pointer: join_dequeue_pointer(word2, word3),
            average_trb_length: u16::try_from(word4 & 0xFFFF).unwrap_or_default(),
            max_esit_payload: u16::try_from((word4 >> 16) & 0xFFFF).unwrap_or_default(),
        })
    }
}

/// A device context: the slot context and the default control endpoint
/// context of a device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceContext {
    /// The slot context.
    pub slot: SlotContext,
    /// The default control endpoint context.
    pub control_endpoint: ControlEndpointContext,
}

impl DeviceContext {
    /// Creates a device context.
    #[must_use]
    pub const fn new(slot: SlotContext, control_endpoint: ControlEndpointContext) -> Self {
        Self {
            slot,
            control_endpoint,
        }
    }

    /// Encodes the device context into `out`.
    ///
    /// Returns `false` when `out` is shorter than 64 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        let Some(slot) = out.get_mut(0..CONTEXT_SIZE) else {
            return false;
        };
        if !self.slot.encode(slot) {
            return false;
        }
        let Some(control) =
            out.get_mut(CONTROL_CONTEXT_OFFSET..CONTROL_CONTEXT_OFFSET + CONTEXT_SIZE)
        else {
            return false;
        };
        self.control_endpoint.encode(control)
    }

    /// Decodes a device context from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let slot_bytes = bytes.get(0..CONTEXT_SIZE)?;
        let control_bytes =
            bytes.get(CONTROL_CONTEXT_OFFSET..CONTROL_CONTEXT_OFFSET + CONTEXT_SIZE)?;
        Some(Self {
            slot: SlotContext::decode(slot_bytes)?,
            control_endpoint: ControlEndpointContext::decode(control_bytes)?,
        })
    }
}

/// Returns the byte offset of the endpoint context of the endpoint with
/// the given device context identifier.
#[must_use]
pub fn endpoint_context_offset(dci: u8) -> Option<usize> {
    if dci == 0 {
        return None;
    }
    usize::from(dci).checked_add(1)?.checked_mul(CONTEXT_SIZE)
}

/// Splits a 64-bit transfer ring dequeue pointer into the two double
/// words of an endpoint context.
///
/// The low double word holds the dequeue cycle state in bit 0 and the
/// pointer bits 31:4 in bits 31:4. The high double word holds the
/// pointer bits 63:32.
fn split_dequeue_pointer(pointer: u64) -> (u32, u32) {
    let low = u32::try_from(pointer & 0x0FFF_FFFF).unwrap_or_default() << 4;
    let high = u32::try_from((pointer >> 32) & 0xFFFF_FFFF).unwrap_or_default();
    (low, high)
}

/// Joins the two double words of a transfer ring dequeue pointer.
fn join_dequeue_pointer(low: u32, high: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low >> 4)
}

/// Writes a little-endian double word at `offset`.
fn write_word(out: &mut [u8], offset: usize, value: u32) -> bool {
    let Some(end) = offset.checked_add(4) else {
        return false;
    };
    let Some(chunk) = out.get_mut(offset..end) else {
        return false;
    };
    chunk.copy_from_slice(&value.to_le_bytes());
    true
}
