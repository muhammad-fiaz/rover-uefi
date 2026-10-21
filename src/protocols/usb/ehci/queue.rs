//! # EHCI queues
//!
//! The queue head and the queue element transfer descriptor of the
//! Enhanced Host Controller Interface (`EHCI`).
//!
//! A queue head describes one endpoint and points at the first queue
//! element transfer descriptor of the endpoint. Every queue element
//! transfer descriptor describes one transfer and points at the next
//! one.

use crate::protocols::usb::ehci::{ScheduleElementType, ScheduleLink};

/// The size of a queue head in bytes.
pub const QUEUE_HEAD_SIZE: usize = 48;

/// The size of a queue element transfer descriptor in bytes.
pub const TRANSFER_DESCRIPTOR_SIZE: usize = 32;

/// The maximum number of bytes a queue element transfer descriptor can
/// transfer.
pub const MAX_TRANSFER_BYTES: u16 = 0x5000;

/// The speed of an endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndpointSpeed {
    /// Full speed, 12 megabits per second.
    FullSpeed,
    /// Low speed, 1.5 megabits per second.
    LowSpeed,
    /// High speed, 480 megabits per second.
    HighSpeed,
}

impl EndpointSpeed {
    /// Returns the numeric speed value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::FullSpeed => 0,
            Self::LowSpeed => 1,
            Self::HighSpeed => 2,
        }
    }

    /// Creates a speed from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::FullSpeed,
            1 => Self::LowSpeed,
            _ => Self::HighSpeed,
        }
    }
}

/// The token a queue element transfer descriptor uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PacketId {
    /// An OUT token.
    Out,
    /// An IN token.
    In,
    /// A SETUP token.
    Setup,
    /// The reserved encoding.
    Reserved,
}

impl PacketId {
    /// Returns the numeric packet identifier value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Out => 0,
            Self::In => 1,
            Self::Setup => 2,
            Self::Reserved => 3,
        }
    }

    /// Creates a packet identifier from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::Out,
            1 => Self::In,
            2 => Self::Setup,
            _ => Self::Reserved,
        }
    }
}

/// One entry of the buffer page pointer list of a queue element transfer
/// descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BufferPointer {
    /// The physical address of the page.
    pub page: u32,
    /// The offset into the page, valid for the first page only.
    pub offset: u16,
}

impl BufferPointer {
    /// Creates a buffer pointer for the given page.
    #[must_use]
    pub const fn new(page: u32) -> Self {
        Self { page, offset: 0 }
    }

    /// Creates a buffer pointer for the given page and offset.
    #[must_use]
    pub const fn with_offset(page: u32, offset: u16) -> Self {
        Self { page, offset }
    }

    /// Encodes the buffer pointer into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        self.encode_page() | u32::from(self.offset & 0x0FFF)
    }

    /// Encodes the buffer pointer without the current offset.
    #[must_use]
    pub fn encode_page(&self) -> u32 {
        self.page & 0xFFFF_F000
    }

    /// Decodes a buffer pointer from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            page: value & 0xFFFF_F000,
            offset: u16::try_from(value & 0x0FFF).unwrap_or_default(),
        }
    }
}

/// The status byte of a queue element transfer descriptor.
///
/// The fields mirror distinct hardware status bits, so keeping them as
/// named booleans is clearer than packing them into a bit mask.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransferStatus {
    /// Whether the controller may execute the transfer.
    pub active: bool,
    /// Whether the transfer was halted by an error.
    pub halted: bool,
    /// Whether the data buffer pointer was invalid.
    pub data_buffer_error: bool,
    /// Whether the device sent more data than the endpoint accepts.
    pub babble_detected: bool,
    /// Whether the transaction failed on the bus.
    pub transaction_error: bool,
    /// Whether a split transaction did not complete.
    pub incomplete_split_transaction: bool,
    /// Whether the split transaction state is valid.
    pub split_transaction_state: bool,
    /// Whether the controller should issue a ping.
    pub ping: bool,
}

impl TransferStatus {
    /// Decodes the status byte of a queue element transfer descriptor.
    #[must_use]
    pub fn decode(status: u8) -> Self {
        Self {
            active: status & (1 << 7) != 0,
            halted: status & (1 << 6) != 0,
            data_buffer_error: status & (1 << 5) != 0,
            babble_detected: status & (1 << 4) != 0,
            transaction_error: status & (1 << 3) != 0,
            incomplete_split_transaction: status & (1 << 2) != 0,
            split_transaction_state: status & (1 << 1) != 0,
            ping: status & 1 != 0,
        }
    }

    /// Encodes the status byte of a queue element transfer descriptor.
    #[must_use]
    pub fn encode(&self) -> u8 {
        let mut value = 0_u8;
        if self.active {
            value |= 1 << 7;
        }
        if self.halted {
            value |= 1 << 6;
        }
        if self.data_buffer_error {
            value |= 1 << 5;
        }
        if self.babble_detected {
            value |= 1 << 4;
        }
        if self.transaction_error {
            value |= 1 << 3;
        }
        if self.incomplete_split_transaction {
            value |= 1 << 2;
        }
        if self.split_transaction_state {
            value |= 1 << 1;
        }
        if self.ping {
            value |= 1;
        }
        value
    }
}

/// A queue element transfer descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransferDescriptor {
    /// The pointer to the next queue element transfer descriptor.
    pub next: ScheduleLink,
    /// The pointer to the next queue element transfer descriptor used
    /// after a short packet.
    pub alternate_next: ScheduleLink,
    /// The data toggle bit of the transaction.
    pub data_toggle: bool,
    /// The number of bytes the transfer moves.
    pub total_bytes: u16,
    /// Whether the controller raises an interrupt when the transfer
    /// completes.
    pub interrupt_on_complete: bool,
    /// The number of errors the controller tolerates.
    pub error_count: u8,
    /// The index of the active buffer page.
    pub current_page: u8,
    /// The token of the transaction.
    pub packet_id: PacketId,
    /// The status byte the controller writes back.
    pub status: u8,
    /// The buffer page pointer list.
    pub buffers: [BufferPointer; 5],
}

impl TransferDescriptor {
    /// Creates a transfer descriptor for an OUT or IN transfer.
    #[must_use]
    pub const fn new(packet_id: PacketId, total_bytes: u16) -> Self {
        Self {
            next: ScheduleLink::terminated(),
            alternate_next: ScheduleLink::terminated(),
            data_toggle: true,
            total_bytes,
            interrupt_on_complete: false,
            error_count: 3,
            current_page: 0,
            packet_id,
            status: 0,
            buffers: [BufferPointer::new(0); 5],
        }
    }

    /// Returns the decoded status of the transfer.
    #[must_use]
    pub fn transfer_status(&self) -> TransferStatus {
        TransferStatus::decode(self.status)
    }

    /// Encodes the transfer descriptor into `out`.
    ///
    /// Returns `false` when `out` is shorter than 32 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < TRANSFER_DESCRIPTOR_SIZE {
            return false;
        }
        let token = (u32::from(self.status & 0x7F))
            | (u32::from(self.packet_id.to_raw() & 0x3) << 8)
            | (u32::from(self.error_count & 0x3) << 10)
            | (u32::from(self.current_page & 0x7) << 12)
            | (u32::from(u8::from(self.interrupt_on_complete)) << 15)
            | (u32::from(self.total_bytes) << 16)
            | (u32::from(u8::from(self.data_toggle)) << 31);
        if !write_word(out, 0, self.next.encode())
            || !write_word(out, 4, self.alternate_next.encode())
            || !write_word(out, 8, token)
        {
            return false;
        }
        for (index, buffer) in self.buffers.iter().enumerate() {
            let value = if index == 0 {
                buffer.encode()
            } else {
                buffer.encode_page()
            };
            if !write_word(out, 12 + index * 4, value) {
                return false;
            }
        }
        true
    }

    /// Decodes a transfer descriptor from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let next = read_word(bytes, 0)?;
        let alternate_next = read_word(bytes, 4)?;
        let token = read_word(bytes, 8)?;
        let mut buffers = [BufferPointer::new(0); 5];
        for (index, buffer) in buffers.iter_mut().enumerate() {
            *buffer = BufferPointer::decode(read_word(bytes, 12 + index * 4)?);
        }
        Some(Self {
            next: ScheduleLink::decode(next),
            alternate_next: ScheduleLink::decode(alternate_next),
            data_toggle: token & (1 << 31) != 0,
            total_bytes: u16::try_from((token >> 16) & 0x7FFF).unwrap_or_default(),
            interrupt_on_complete: token & (1 << 15) != 0,
            current_page: u8::try_from((token >> 12) & 0x7).unwrap_or_default(),
            error_count: u8::try_from((token >> 10) & 0x3).unwrap_or_default(),
            packet_id: PacketId::from_raw(u8::try_from((token >> 8) & 0x3).unwrap_or_default()),
            status: u8::try_from(token & 0xFF).unwrap_or_default(),
            buffers,
        })
    }
}

/// The transfer overlay of a queue head.
///
/// The overlay is the controller's working copy of the queue element
/// transfer descriptor it is executing. The controller writes the
/// results of the transfer back into the descriptor when it retires it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransferOverlay {
    /// The pointer to the next queue element transfer descriptor.
    pub next_qtd: ScheduleLink,
    /// The pointer to the queue element transfer descriptor used after a
    /// short packet.
    pub alternate_next_qtd: ScheduleLink,
    /// The number of times the controller retries a transaction.
    pub nak_count: u8,
    /// The token of the transfer.
    pub token: u32,
    /// The buffer page pointer list.
    pub buffers: [BufferPointer; 5],
}

impl TransferOverlay {
    /// Encodes the overlay into `out`.
    ///
    /// Returns `false` when `out` is shorter than 32 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < TRANSFER_DESCRIPTOR_SIZE {
            return false;
        }
        let alternate = (self.alternate_next_qtd.encode() & 0xFFFF_FFE0)
            | (u32::from(self.nak_count & 0x7) << 12);
        if !write_word(out, 0, self.next_qtd.encode())
            || !write_word(out, 4, alternate)
            || !write_word(out, 8, self.token)
        {
            return false;
        }
        for (index, buffer) in self.buffers.iter().enumerate() {
            let value = if index == 0 {
                buffer.encode()
            } else {
                buffer.encode_page()
            };
            if !write_word(out, 12 + index * 4, value) {
                return false;
            }
        }
        true
    }

    /// Decodes the overlay from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let next_qtd = read_word(bytes, 0)?;
        let alternate = read_word(bytes, 4)?;
        let token = read_word(bytes, 8)?;
        let mut buffers = [BufferPointer::new(0); 5];
        for (index, buffer) in buffers.iter_mut().enumerate() {
            *buffer = BufferPointer::decode(read_word(bytes, 12 + index * 4)?);
        }
        Some(Self {
            next_qtd: ScheduleLink::decode(next_qtd),
            alternate_next_qtd: ScheduleLink::decode(alternate & 0xFFFF_FFE0),
            nak_count: u8::try_from((alternate >> 12) & 0x7).unwrap_or_default(),
            token,
            buffers,
        })
    }
}

/// A queue head, describing one endpoint of one device.
///
/// The boolean fields correspond to independent hardware control bits.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QueueHead {
    /// The pointer to the next queue head of the schedule.
    pub horizontal_link: ScheduleLink,
    /// The address of the device on the bus.
    pub device_address: u8,
    /// Whether the endpoint is inactivated after the next transaction.
    pub inactivate: bool,
    /// The number of the endpoint.
    pub endpoint_number: u8,
    /// The speed of the endpoint.
    pub endpoint_speed: EndpointSpeed,
    /// Whether the data toggle is taken from the transfer descriptor.
    pub data_toggle_control: bool,
    /// Whether the queue head is the head of the reclamation list.
    pub head_of_reclamation_list: bool,
    /// Whether the endpoint is a full or low speed control endpoint.
    pub control_endpoint: bool,
    /// The maximum packet size of the endpoint.
    pub max_packet_length: u16,
    /// The high bandwidth multiplier of the endpoint.
    pub multi: u8,
    /// The number of the port on the hub the device is attached to.
    pub port_number: u8,
    /// The address of the hub the device is attached to.
    pub hub_address: u8,
    /// The completion mask of a periodic endpoint.
    pub completion_mask: u8,
    /// The synchronisation mask of a periodic endpoint.
    pub sync_mask: u8,
    /// The queue element transfer descriptor the controller executes.
    pub current_qtd: u32,
    /// The transfer overlay of the queue head.
    pub overlay: TransferOverlay,
}

impl QueueHead {
    /// Creates a queue head for an endpoint of a device.
    #[must_use]
    pub const fn new(
        device_address: u8,
        endpoint_number: u8,
        endpoint_speed: EndpointSpeed,
        max_packet_length: u16,
    ) -> Self {
        Self {
            horizontal_link: ScheduleLink {
                address: 0,
                terminate: true,
                element_type: ScheduleElementType::QueueHead,
            },
            device_address,
            inactivate: false,
            endpoint_number,
            endpoint_speed,
            data_toggle_control: true,
            head_of_reclamation_list: false,
            control_endpoint: false,
            max_packet_length,
            multi: 0,
            port_number: 0,
            hub_address: 0,
            completion_mask: 0,
            sync_mask: 0,
            current_qtd: 0,
            overlay: TransferOverlay {
                next_qtd: ScheduleLink::terminated(),
                alternate_next_qtd: ScheduleLink::terminated(),
                nak_count: 0,
                token: 0,
                buffers: [BufferPointer::new(0); 5],
            },
        }
    }

    /// Encodes the queue head into `out`.
    ///
    /// Returns `false` when `out` is shorter than 48 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < QUEUE_HEAD_SIZE {
            return false;
        }
        let info1 = (u32::from(self.device_address & 0x7F))
            | (u32::from(u8::from(self.inactivate)) << 7)
            | (u32::from(self.endpoint_number & 0x0F) << 8)
            | (u32::from(self.endpoint_speed.to_raw() & 0x3) << 12)
            | (u32::from(u8::from(self.data_toggle_control)) << 14)
            | (u32::from(u8::from(self.head_of_reclamation_list)) << 15)
            | (u32::from(self.max_packet_length & 0x07FF) << 16)
            | (u32::from(u8::from(self.control_endpoint)) << 27);
        let info2 = u32::from(self.sync_mask)
            | (u32::from(self.completion_mask) << 8)
            | (u32::from(self.hub_address & 0x7F) << 17)
            | (u32::from(self.port_number & 0x3F) << 24)
            | (u32::from(self.multi & 0x3) << 30);
        if !write_word(out, 0, self.horizontal_link.encode())
            || !write_word(out, 4, info1)
            || !write_word(out, 8, info2)
            || !write_word(out, 12, self.current_qtd)
        {
            return false;
        }
        let Some(overlay) = out.get_mut(16..16 + TRANSFER_DESCRIPTOR_SIZE) else {
            return false;
        };
        self.overlay.encode(overlay)
    }

    /// Decodes a queue head from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let horizontal_link = read_word(bytes, 0)?;
        let info1 = read_word(bytes, 4)?;
        let info2 = read_word(bytes, 8)?;
        let current_qtd = read_word(bytes, 12)?;
        let overlay_bytes = bytes.get(16..16 + TRANSFER_DESCRIPTOR_SIZE)?;
        Some(Self {
            horizontal_link: ScheduleLink::decode(horizontal_link),
            device_address: u8::try_from(info1 & 0x7F).unwrap_or_default(),
            inactivate: info1 & (1 << 7) != 0,
            endpoint_number: u8::try_from((info1 >> 8) & 0x0F).unwrap_or_default(),
            endpoint_speed: EndpointSpeed::from_raw(
                u8::try_from((info1 >> 12) & 0x3).unwrap_or_default(),
            ),
            data_toggle_control: info1 & (1 << 14) != 0,
            head_of_reclamation_list: info1 & (1 << 15) != 0,
            control_endpoint: info1 & (1 << 27) != 0,
            max_packet_length: u16::try_from((info1 >> 16) & 0x07FF).unwrap_or_default(),
            multi: u8::try_from((info2 >> 30) & 0x3).unwrap_or_default(),
            port_number: u8::try_from((info2 >> 24) & 0x3F).unwrap_or_default(),
            hub_address: u8::try_from((info2 >> 17) & 0x7F).unwrap_or_default(),
            completion_mask: u8::try_from((info2 >> 8) & 0xFF).unwrap_or_default(),
            sync_mask: u8::try_from(info2 & 0xFF).unwrap_or_default(),
            current_qtd,
            overlay: TransferOverlay::decode(overlay_bytes)?,
        })
    }
}

/// Reads a little-endian double word at `offset`.
fn read_word(bytes: &[u8], offset: usize) -> Option<u32> {
    let chunk = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(chunk);
    Some(u32::from_le_bytes(raw))
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
