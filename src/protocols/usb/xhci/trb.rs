//! # xHCI Transfer Request Blocks
//!
//! Encoding and decoding of the Transfer Request Blocks (TRBs) used by
//! the eXtensible Host Controller Interface (`xHCI`), as described by the
//! USB 3.1 and `xHCI` specifications.
//!
//! Every TRB occupies exactly 16 bytes and is laid out as four
//! little-endian double words:
//!
//! | Double word | Contents                                          |
//! |-------------|---------------------------------------------------|
//! | 0 and 1     | Parameter: a 64-bit pointer or immediate data    |
//! | 2           | Status: transfer length and interrupter target    |
//! | 3           | Control: cycle bit, flags and the TRB type        |
//!
//! The TRB type occupies bits 15:10 of the control double word and
//! the cycle bit occupies bit 0.

use crate::base::bytes::{read_u32_le, read_u64_le};

/// The size of a Transfer Request Block in bytes.
pub const TRB_SIZE: usize = 16;

/// The mask of the TRB type field inside the control double word.
const TRB_TYPE_MASK: u32 = 0x3F;

/// The shift of the TRB type field inside the control double word.
const TRB_TYPE_SHIFT: u32 = 10;

/// The mask of the transfer length field inside the status double word.
const TRB_LENGTH_MASK: u32 = 0x1_FFFF;

/// The shift of the transfer descriptor size field.
const TRB_TD_SIZE_SHIFT: u32 = 17;

/// The mask of the transfer descriptor size field.
const TRB_TD_SIZE_MASK: u32 = 0x1F;

/// The shift of the interrupter target field.
const TRB_INTERRUPTER_SHIFT: u32 = 22;

/// The shift of the completion code field inside an event status word.
const EVENT_COMPLETION_SHIFT: u32 = 24;

/// The mask of the completion code field inside an event status word.
const EVENT_COMPLETION_MASK: u32 = 0xFF;

/// The shift of the slot identifier field inside a command control word.
const COMMAND_SLOT_SHIFT: u32 = 24;

/// The shift of the endpoint identifier field inside a command control word.
const COMMAND_ENDPOINT_SHIFT: u32 = 16;

/// The mask of the endpoint identifier field.
const COMMAND_ENDPOINT_MASK: u32 = 0x1F;

/// The bit that requests the block transfer split transaction flag.
const COMMAND_BSR: u32 = 1 << 9;

/// The bit that requests a deconfigure operation.
const COMMAND_DC: u32 = 1 << 9;

/// The bit that requests the transfer state protocol.
const COMMAND_TSP: u32 = 1 << 9;

/// The shift of the stream identifier field inside a command status word.
const COMMAND_STREAM_SHIFT: u32 = 16;

/// Narrows a double word to a 16-bit value.
fn narrow_u16(value: u32) -> u16 {
    u16::try_from(value & 0xFFFF).unwrap_or_default()
}

/// Narrows a double word to an 8-bit value.
fn narrow_u8(value: u32) -> u8 {
    u8::try_from(value & 0xFF).unwrap_or_default()
}

/// Writes a little-endian double word at `offset`.
fn write_u32(out: &mut [u8], offset: usize, value: u32) -> bool {
    let Some(end) = offset.checked_add(4) else {
        return false;
    };
    let Some(chunk) = out.get_mut(offset..end) else {
        return false;
    };
    chunk.copy_from_slice(&value.to_le_bytes());
    true
}

/// Writes a little-endian 64-bit value at `offset`.
fn write_u64(out: &mut [u8], offset: usize, value: u64) -> bool {
    let Some(end) = offset.checked_add(8) else {
        return false;
    };
    let Some(chunk) = out.get_mut(offset..end) else {
        return false;
    };
    chunk.copy_from_slice(&value.to_le_bytes());
    true
}

/// The control flags of a Transfer Request Block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrbFlags(u16);

impl TrbFlags {
    /// The cycle bit of the TRB.
    pub const CYCLE: Self = Self(0x0001);
    /// The Evaluate Next TRB flag.
    pub const EVALUATE_NEXT: Self = Self(0x0002);
    /// The Interrupt On Short Packet flag.
    pub const INTERRUPT_ON_SHORT_PACKET: Self = Self(0x0004);
    /// The No Snoop flag.
    pub const NO_SNOOP: Self = Self(0x0008);
    /// The Chain flag.
    pub const CHAIN: Self = Self(0x0010);
    /// The Interrupt On Completion flag.
    pub const INTERRUPT_ON_COMPLETION: Self = Self(0x0020);
    /// The Immediate Data flag.
    pub const IMMEDIATE_DATA: Self = Self(0x0040);
    /// The Toggle Cycle flag of a link TRB.
    pub const TOGGLE_CYCLE: Self = Self(0x0080);
    /// The Direction flag of a data or a status stage TRB.
    pub const DIRECTION_IN: Self = Self(0x0100);
    /// The Event Data flag of a transfer event.
    pub const EVENT_DATA: Self = Self(0x0200);
    /// The Block Transfer Split Request flag of an address device
    /// command.
    pub const BLOCK_SPLIT_REQUEST: Self = Self(0x0400);
    /// The Deconfigure flag of a configure endpoint command.
    pub const DECONFIGURE: Self = Self(0x0800);
    /// The Transfer State Protocol flag of a reset endpoint command.
    pub const TRANSFER_STATE_PROTOCOL: Self = Self(0x1000);

    /// Creates a flag set from its raw bits.
    #[must_use]
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self(bits)
    }

    /// Returns the raw bits of the flag set.
    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns whether every flag of `other` is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns the union of two flag sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the difference of two flag sets.
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Returns whether no flag is set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// The type identifier of a Transfer Request Block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrbType {
    /// Normal TRB, type 1.
    Normal,
    /// Setup stage TRB, type 2.
    SetupStage,
    /// Data stage TRB, type 3.
    DataStage,
    /// Status stage TRB, type 4.
    StatusStage,
    /// Isochronous TRB, type 5.
    Isochronous,
    /// Link TRB, type 6.
    Link,
    /// Event data TRB, type 7.
    EventData,
    /// No-op TRB, type 8.
    NoOp,
    /// Enable slot command, type 9.
    EnableSlot,
    /// Disable slot command, type 10.
    DisableSlot,
    /// Address device command, type 11.
    AddressDevice,
    /// Configure endpoint command, type 12.
    ConfigureEndpoint,
    /// Evaluate context command, type 13.
    EvaluateContext,
    /// Reset endpoint command, type 14.
    ResetEndpoint,
    /// Stop endpoint command, type 15.
    StopEndpoint,
    /// Set transfer ring dequeue pointer command, type 16.
    SetTrDequeuePointer,
    /// Reset device command, type 17.
    ResetDevice,
    /// No-op command, type 23.
    NoOpCommand,
    /// Transfer event, type 32.
    TransferEvent,
    /// Command completion event, type 33.
    CommandCompletionEvent,
    /// Host controller notification event, type 34.
    HostControllerNotificationEvent,
    /// Port status change event, type 35.
    PortStatusChangeEvent,
    /// Doorbell event, type 36.
    DoorbellEvent,
    /// Host controller event, type 37.
    HostControllerEvent,
    /// Device notification event, type 38.
    DeviceNotificationEvent,
    /// `MFINDEX` wrap event, type 39.
    MfindexWrapEvent,
    /// A type identifier that the specification has not assigned.
    Reserved(u16),
}

impl TrbType {
    /// Returns the numeric type identifier.
    #[must_use]
    pub fn to_raw(self) -> u16 {
        match self {
            Self::Normal => 1,
            Self::SetupStage => 2,
            Self::DataStage => 3,
            Self::StatusStage => 4,
            Self::Isochronous => 5,
            Self::Link => 6,
            Self::EventData => 7,
            Self::NoOp => 8,
            Self::EnableSlot => 9,
            Self::DisableSlot => 10,
            Self::AddressDevice => 11,
            Self::ConfigureEndpoint => 12,
            Self::EvaluateContext => 13,
            Self::ResetEndpoint => 14,
            Self::StopEndpoint => 15,
            Self::SetTrDequeuePointer => 16,
            Self::ResetDevice => 17,
            Self::NoOpCommand => 23,
            Self::TransferEvent => 32,
            Self::CommandCompletionEvent => 33,
            Self::HostControllerNotificationEvent => 34,
            Self::PortStatusChangeEvent => 35,
            Self::DoorbellEvent => 36,
            Self::HostControllerEvent => 37,
            Self::DeviceNotificationEvent => 38,
            Self::MfindexWrapEvent => 39,
            Self::Reserved(raw) => raw,
        }
    }

    /// Creates a type from its numeric identifier.
    #[must_use]
    pub fn from_raw(raw: u16) -> Self {
        match raw {
            1 => Self::Normal,
            2 => Self::SetupStage,
            3 => Self::DataStage,
            4 => Self::StatusStage,
            5 => Self::Isochronous,
            6 => Self::Link,
            7 => Self::EventData,
            8 => Self::NoOp,
            9 => Self::EnableSlot,
            10 => Self::DisableSlot,
            11 => Self::AddressDevice,
            12 => Self::ConfigureEndpoint,
            13 => Self::EvaluateContext,
            14 => Self::ResetEndpoint,
            15 => Self::StopEndpoint,
            16 => Self::SetTrDequeuePointer,
            17 => Self::ResetDevice,
            23 => Self::NoOpCommand,
            32 => Self::TransferEvent,
            33 => Self::CommandCompletionEvent,
            34 => Self::HostControllerNotificationEvent,
            35 => Self::PortStatusChangeEvent,
            36 => Self::DoorbellEvent,
            37 => Self::HostControllerEvent,
            38 => Self::DeviceNotificationEvent,
            39 => Self::MfindexWrapEvent,
            other => Self::Reserved(other),
        }
    }

    /// Returns whether the identifier belongs to an event ring type.
    #[must_use]
    pub fn is_event(self) -> bool {
        matches!(
            self,
            Self::TransferEvent
                | Self::CommandCompletionEvent
                | Self::HostControllerNotificationEvent
                | Self::PortStatusChangeEvent
                | Self::DoorbellEvent
                | Self::HostControllerEvent
                | Self::DeviceNotificationEvent
                | Self::MfindexWrapEvent
        )
    }

    /// Returns whether the identifier belongs to a command ring type.
    #[must_use]
    pub fn is_command(self) -> bool {
        matches!(
            self,
            Self::EnableSlot
                | Self::DisableSlot
                | Self::AddressDevice
                | Self::ConfigureEndpoint
                | Self::EvaluateContext
                | Self::ResetEndpoint
                | Self::StopEndpoint
                | Self::SetTrDequeuePointer
                | Self::ResetDevice
                | Self::NoOpCommand
        )
    }

    /// Returns whether the identifier is one the specification assigns.
    #[must_use]
    pub fn is_assigned(self) -> bool {
        !matches!(self, Self::Reserved(_))
    }
}

/// The data stage type of a control transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControlTransferType {
    /// The transfer has no data stage.
    NoData,
    /// Reserved by the specification.
    Reserved,
    /// The transfer has an OUT data stage.
    OutData,
    /// The transfer has an IN data stage.
    InData,
}

impl ControlTransferType {
    /// Returns the numeric transfer type value.
    #[must_use]
    pub fn to_raw(self) -> u16 {
        match self {
            Self::NoData => 0,
            Self::Reserved => 1,
            Self::OutData => 2,
            Self::InData => 3,
        }
    }

    /// Creates a transfer type from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u16) -> Self {
        match raw & 0x3 {
            0 => Self::NoData,
            1 => Self::Reserved,
            2 => Self::OutData,
            _ => Self::InData,
        }
    }
}

/// The completion status reported by an event TRB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CompletionCode {
    /// The event is invalid.
    Invalid,
    /// The operation completed successfully.
    Success,
    /// The data buffer pointer was invalid.
    DataBufferError,
    /// The device produced more data than the endpoint accepts.
    BabbleDetected,
    /// The transaction failed on the bus.
    UsbTransactionError,
    /// The TRB was invalid.
    TrbError,
    /// The endpoint stalled.
    StallError,
    /// The controller could not allocate resources.
    ResourceError,
    /// Not enough bandwidth was available.
    BandwidthError,
    /// No device slot was available.
    NoSlotsAvailable,
    /// The stream type was invalid.
    InvalidStreamType,
    /// The device slot was not enabled.
    SlotNotEnabled,
    /// The endpoint was not enabled.
    EndpointNotEnabled,
    /// A short packet ended the transfer.
    ShortPacket,
    /// The isochronous transfer ring underran.
    RingUnderrun,
    /// The isochronous transfer ring overran.
    RingOverrun,
    /// The virtual function event ring filled up.
    VfEventRingFull,
    /// A command parameter was invalid.
    ParameterError,
    /// The isochronous bandwidth was exceeded.
    BandwidthOverrun,
    /// The device context was in the wrong state.
    ContextStateError,
    /// The device did not answer a ping.
    NoPingResponse,
    /// The event ring filled up.
    EventRingFull,
    /// The device is incompatible with the controller.
    IncompatibleDevice,
    /// An isochronous service interval was missed.
    MissedService,
    /// The command ring stopped.
    CommandRingStopped,
    /// The command was aborted.
    CommandAborted,
    /// The transfer ring stopped.
    Stopped,
    /// The transfer ring stopped with an invalid length.
    StoppedLengthInvalid,
    /// The transfer ring stopped with a short packet.
    StoppedShortPacket,
    /// The maximum exit latency was too large.
    MaxExitLatencyTooLarge,
    /// Reserved by the specification.
    Reserved,
    /// The isochronous buffer overran.
    IsochBufferOverrun,
    /// An event was lost.
    EventLost,
    /// The controller reported an undefined error.
    UndefinedError,
    /// The stream identifier was invalid.
    InvalidStreamId,
    /// The secondary bandwidth was exceeded.
    SecondaryBandwidthError,
    /// A split transaction failed.
    SplitTransactionError,
    /// A vendor defined completion code.
    Vendor(u8),
}

impl CompletionCode {
    /// Returns the numeric completion code.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Invalid => 0,
            Self::Success => 1,
            Self::DataBufferError => 2,
            Self::BabbleDetected => 3,
            Self::UsbTransactionError => 4,
            Self::TrbError => 5,
            Self::StallError => 6,
            Self::ResourceError => 7,
            Self::BandwidthError => 8,
            Self::NoSlotsAvailable => 9,
            Self::InvalidStreamType => 10,
            Self::SlotNotEnabled => 11,
            Self::EndpointNotEnabled => 12,
            Self::ShortPacket => 13,
            Self::RingUnderrun => 14,
            Self::RingOverrun => 15,
            Self::VfEventRingFull => 16,
            Self::ParameterError => 17,
            Self::BandwidthOverrun => 18,
            Self::ContextStateError => 19,
            Self::NoPingResponse => 20,
            Self::EventRingFull => 21,
            Self::IncompatibleDevice => 22,
            Self::MissedService => 23,
            Self::CommandRingStopped => 24,
            Self::CommandAborted => 25,
            Self::Stopped => 26,
            Self::StoppedLengthInvalid => 27,
            Self::StoppedShortPacket => 28,
            Self::MaxExitLatencyTooLarge => 29,
            Self::Reserved => 30,
            Self::IsochBufferOverrun => 31,
            Self::EventLost => 32,
            Self::UndefinedError => 33,
            Self::InvalidStreamId => 34,
            Self::SecondaryBandwidthError => 35,
            Self::SplitTransactionError => 36,
            Self::Vendor(raw) => raw,
        }
    }

    /// Creates a completion code from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Invalid,
            1 => Self::Success,
            2 => Self::DataBufferError,
            3 => Self::BabbleDetected,
            4 => Self::UsbTransactionError,
            5 => Self::TrbError,
            6 => Self::StallError,
            7 => Self::ResourceError,
            8 => Self::BandwidthError,
            9 => Self::NoSlotsAvailable,
            10 => Self::InvalidStreamType,
            11 => Self::SlotNotEnabled,
            12 => Self::EndpointNotEnabled,
            13 => Self::ShortPacket,
            14 => Self::RingUnderrun,
            15 => Self::RingOverrun,
            16 => Self::VfEventRingFull,
            17 => Self::ParameterError,
            18 => Self::BandwidthOverrun,
            19 => Self::ContextStateError,
            20 => Self::NoPingResponse,
            21 => Self::EventRingFull,
            22 => Self::IncompatibleDevice,
            23 => Self::MissedService,
            24 => Self::CommandRingStopped,
            25 => Self::CommandAborted,
            26 => Self::Stopped,
            27 => Self::StoppedLengthInvalid,
            28 => Self::StoppedShortPacket,
            29 => Self::MaxExitLatencyTooLarge,
            30 => Self::Reserved,
            31 => Self::IsochBufferOverrun,
            32 => Self::EventLost,
            33 => Self::UndefinedError,
            34 => Self::InvalidStreamId,
            35 => Self::SecondaryBandwidthError,
            36 => Self::SplitTransactionError,
            other => Self::Vendor(other),
        }
    }

    /// Returns whether the completion status reports a successful transfer.
    #[must_use]
    pub fn is_success(self) -> bool {
        matches!(self, Self::Success | Self::ShortPacket)
    }
}

/// A Normal data TRB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NormalTrb {
    /// The physical address of the data buffer.
    pub buffer: u64,
    /// The number of bytes to transfer.
    pub length: u32,
    /// The number of packets remaining in the transfer descriptor.
    pub td_size: u8,
    /// The interrupter that receives the completion event.
    pub interrupter: u16,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

impl NormalTrb {
    /// Creates a Normal TRB that transfers `length` bytes from `buffer`.
    #[must_use]
    pub const fn new(buffer: u64, length: u32, interrupter: u16) -> Self {
        Self {
            buffer,
            length,
            td_size: 0,
            interrupter,
            flags: TrbFlags::CYCLE.union(TrbFlags::INTERRUPT_ON_COMPLETION),
        }
    }
}

/// A setup stage TRB, carrying the eight bytes of a USB setup packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SetupStageTrb {
    /// The eight bytes of the setup packet.
    pub setup: [u8; 8],
    /// The interrupter that receives the completion event.
    pub interrupter: u16,
    /// The data stage type of the control transfer.
    pub transfer_type: ControlTransferType,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

impl SetupStageTrb {
    /// Creates a setup stage TRB for the given setup packet.
    #[must_use]
    pub const fn new(setup: [u8; 8], interrupter: u16, transfer_type: ControlTransferType) -> Self {
        Self {
            setup,
            interrupter,
            transfer_type,
            flags: TrbFlags::CYCLE.union(TrbFlags::IMMEDIATE_DATA),
        }
    }
}

/// A data stage TRB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DataStageTrb {
    /// The physical address of the data buffer.
    pub buffer: u64,
    /// The number of bytes to transfer.
    pub length: u32,
    /// The number of packets remaining in the transfer descriptor.
    pub td_size: u8,
    /// The interrupter that receives the completion event.
    pub interrupter: u16,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

impl DataStageTrb {
    /// Creates a data stage TRB.
    #[must_use]
    pub const fn new(buffer: u64, length: u32, direction_in: bool, interrupter: u16) -> Self {
        let flags = TrbFlags::CYCLE;
        let flags = if direction_in {
            flags.union(TrbFlags::DIRECTION_IN)
        } else {
            flags
        };
        Self {
            buffer,
            length,
            td_size: 0,
            interrupter,
            flags,
        }
    }
}

/// A status stage TRB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StatusStageTrb {
    /// The interrupter that receives the completion event.
    pub interrupter: u16,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

impl StatusStageTrb {
    /// Creates a status stage TRB.
    #[must_use]
    pub const fn new(direction_in: bool, interrupter: u16) -> Self {
        let flags = TrbFlags::CYCLE.union(TrbFlags::INTERRUPT_ON_COMPLETION);
        let flags = if direction_in {
            flags.union(TrbFlags::DIRECTION_IN)
        } else {
            flags
        };
        Self { interrupter, flags }
    }
}

/// A link TRB, pointing at the next segment of a ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LinkTrb {
    /// The physical address of the next ring segment.
    pub next_segment: u64,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

impl LinkTrb {
    /// Creates a link TRB pointing at `next_segment`.
    #[must_use]
    pub const fn new(next_segment: u64) -> Self {
        Self {
            next_segment,
            flags: TrbFlags::CYCLE,
        }
    }
}

/// An event data TRB, carrying 64 bits of data for a transfer event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventDataTrb {
    /// The event data value.
    pub data: u64,
    /// The interrupter that receives the completion event.
    pub interrupter: u16,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A no-op TRB placed in a transfer ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NoOpTrb {
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// An enable slot command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnableSlotTrb {
    /// The slot type taken from the supported protocol capability.
    pub slot_type: u8,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A disable slot command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DisableSlotTrb {
    /// The device slot to disable.
    pub slot_id: u8,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A no-op command placed in the command ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NoOpCommandTrb {
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A command that carries an input context pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandTrb {
    /// The command type.
    pub command_type: TrbType,
    /// The physical address of the input context.
    pub input_context: u64,
    /// The device slot the command applies to.
    pub slot_id: u8,
    /// The endpoint context identifier the command applies to.
    pub endpoint_id: u8,
    /// The stream identifier for a set dequeue pointer command.
    pub stream_id: u16,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A transfer event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransferEvent {
    /// The physical address of the TRB that completed.
    pub trb_pointer: u64,
    /// The number of bytes that were not transferred.
    pub residual: u32,
    /// The completion status.
    pub completion: CompletionCode,
    /// The endpoint context identifier.
    pub endpoint_id: u8,
    /// The device slot identifier.
    pub slot_id: u8,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A command completion event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandCompletionEvent {
    /// The physical address of the command TRB that completed.
    pub command_trb_pointer: u64,
    /// The command specific completion parameter.
    pub parameter: u32,
    /// The completion status.
    pub completion: CompletionCode,
    /// The device slot identifier.
    pub slot_id: u8,
    /// The virtual function identifier.
    pub vf_id: u8,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A host controller notification event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HostControllerNotificationEvent {
    /// The raw status double word.
    pub status: u32,
    /// The completion status.
    pub completion: CompletionCode,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A port status change event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PortStatusChangeEvent {
    /// The one-based root hub port identifier.
    pub port_id: u8,
    /// The completion status.
    pub completion: CompletionCode,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A TRB whose type is not known to this implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnknownTrb {
    /// The raw type identifier.
    pub trb_type: u16,
    /// The raw parameter double words.
    pub parameter: u64,
    /// The raw status double word.
    pub status: u32,
    /// The raw control double word.
    pub control: u32,
    /// The control flags of the TRB.
    pub flags: TrbFlags,
}

/// A Transfer Request Block.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Trb {
    /// A Normal data TRB.
    Normal(NormalTrb),
    /// A setup stage TRB.
    SetupStage(SetupStageTrb),
    /// A data stage TRB.
    DataStage(DataStageTrb),
    /// A status stage TRB.
    StatusStage(StatusStageTrb),
    /// A link TRB.
    Link(LinkTrb),
    /// An event data TRB.
    EventData(EventDataTrb),
    /// A no-op TRB.
    NoOp(NoOpTrb),
    /// An enable slot command.
    EnableSlot(EnableSlotTrb),
    /// A disable slot command.
    DisableSlot(DisableSlotTrb),
    /// A no-op command.
    NoOpCommand(NoOpCommandTrb),
    /// A command carrying an input context pointer.
    Command(CommandTrb),
    /// A transfer event.
    TransferEvent(TransferEvent),
    /// A command completion event.
    CommandCompletion(CommandCompletionEvent),
    /// A host controller notification event.
    HostControllerNotification(HostControllerNotificationEvent),
    /// A port status change event.
    PortStatusChange(PortStatusChangeEvent),
    /// A TRB of an unassigned type.
    Unknown(UnknownTrb),
}

impl Trb {
    /// Returns the type identifier of the TRB.
    #[must_use]
    pub fn trb_type(&self) -> TrbType {
        match self {
            Self::Normal(_) => TrbType::Normal,
            Self::SetupStage(_) => TrbType::SetupStage,
            Self::DataStage(_) => TrbType::DataStage,
            Self::StatusStage(_) => TrbType::StatusStage,
            Self::Link(_) => TrbType::Link,
            Self::EventData(_) => TrbType::EventData,
            Self::NoOp(_) => TrbType::NoOp,
            Self::EnableSlot(_) => TrbType::EnableSlot,
            Self::DisableSlot(_) => TrbType::DisableSlot,
            Self::NoOpCommand(_) => TrbType::NoOpCommand,
            Self::Command(command) => command.command_type,
            Self::TransferEvent(_) => TrbType::TransferEvent,
            Self::CommandCompletion(_) => TrbType::CommandCompletionEvent,
            Self::HostControllerNotification(_) => TrbType::HostControllerNotificationEvent,
            Self::PortStatusChange(_) => TrbType::PortStatusChangeEvent,
            Self::Unknown(unknown) => TrbType::Reserved(unknown.trb_type),
        }
    }

    /// Returns the control flags of the TRB.
    #[must_use]
    pub fn flags(&self) -> TrbFlags {
        match self {
            Self::Normal(trb) => trb.flags,
            Self::SetupStage(trb) => trb.flags,
            Self::DataStage(trb) => trb.flags,
            Self::StatusStage(trb) => trb.flags,
            Self::Link(trb) => trb.flags,
            Self::EventData(trb) => trb.flags,
            Self::NoOp(trb) => trb.flags,
            Self::EnableSlot(trb) => trb.flags,
            Self::DisableSlot(trb) => trb.flags,
            Self::NoOpCommand(trb) => trb.flags,
            Self::Command(command) => command.flags,
            Self::TransferEvent(event) => event.flags,
            Self::CommandCompletion(event) => event.flags,
            Self::HostControllerNotification(event) => event.flags,
            Self::PortStatusChange(event) => event.flags,
            Self::Unknown(unknown) => unknown.flags,
        }
    }

    /// Returns the cycle bit of the TRB.
    #[must_use]
    pub fn cycle(&self) -> bool {
        self.flags().contains(TrbFlags::CYCLE)
    }

    /// Sets the cycle bit of the TRB.
    pub fn set_cycle(&mut self, cycle: bool) {
        let flags = self.flags();
        let flags = if cycle {
            flags.union(TrbFlags::CYCLE)
        } else {
            flags.difference(TrbFlags::CYCLE)
        };
        match self {
            Self::Normal(trb) => trb.flags = flags,
            Self::SetupStage(trb) => trb.flags = flags,
            Self::DataStage(trb) => trb.flags = flags,
            Self::StatusStage(trb) => trb.flags = flags,
            Self::Link(trb) => trb.flags = flags,
            Self::EventData(trb) => trb.flags = flags,
            Self::NoOp(trb) => trb.flags = flags,
            Self::EnableSlot(trb) => trb.flags = flags,
            Self::DisableSlot(trb) => trb.flags = flags,
            Self::NoOpCommand(trb) => trb.flags = flags,
            Self::Command(command) => command.flags = flags,
            Self::TransferEvent(event) => event.flags = flags,
            Self::CommandCompletion(event) => event.flags = flags,
            Self::HostControllerNotification(event) => event.flags = flags,
            Self::PortStatusChange(event) => event.flags = flags,
            Self::Unknown(unknown) => unknown.flags = flags,
        }
    }

    /// Returns the completion status carried by an event TRB.
    #[must_use]
    pub fn completion_code(&self) -> Option<CompletionCode> {
        match self {
            Self::TransferEvent(event) => Some(event.completion),
            Self::CommandCompletion(event) => Some(event.completion),
            Self::HostControllerNotification(event) => Some(event.completion),
            Self::PortStatusChange(event) => Some(event.completion),
            _ => None,
        }
    }

    /// Encodes the TRB into `out`.
    ///
    /// Returns `false` when `out` is too small to hold the 16-byte TRB.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < TRB_SIZE {
            return false;
        }
        let (parameter, status, control) = self.words();
        write_u64(out, 0, parameter) && write_u32(out, 8, status) && write_u32(out, 12, control)
    }

    /// Builds the parameter, status and control double words of the TRB.
    fn words(&self) -> (u64, u32, u32) {
        let trb_type = self.trb_type();
        let flags = self.flags();
        let base = u32::from(trb_type.to_raw()) << TRB_TYPE_SHIFT;
        let control = base
            | control_flag(TrbFlags::CYCLE, flags)
            | control_flag(TrbFlags::EVALUATE_NEXT, flags)
            | control_flag(TrbFlags::INTERRUPT_ON_SHORT_PACKET, flags)
            | control_flag(TrbFlags::NO_SNOOP, flags)
            | control_flag(TrbFlags::CHAIN, flags)
            | control_flag(TrbFlags::INTERRUPT_ON_COMPLETION, flags)
            | control_flag(TrbFlags::IMMEDIATE_DATA, flags)
            | control_flag(TrbFlags::TOGGLE_CYCLE, flags)
            | control_flag(TrbFlags::EVENT_DATA, flags);
        match self {
            Self::Normal(trb) => {
                let status = transfer_status(trb.length, trb.td_size, trb.interrupter);
                (trb.buffer, status, control)
            }
            Self::SetupStage(trb) => {
                let status = transfer_status(8, 0, trb.interrupter);
                let control = control | (u32::from(trb.transfer_type.to_raw() & 0x3) << 16);
                (u64::from_le_bytes(trb.setup), status, control)
            }
            Self::DataStage(trb) => {
                let status = transfer_status(trb.length, trb.td_size, trb.interrupter);
                let control = control | direction_bit(flags);
                (trb.buffer, status, control)
            }
            Self::StatusStage(trb) => {
                let status = transfer_status(0, 0, trb.interrupter);
                let control = control | direction_bit(flags);
                (0, status, control)
            }
            Self::Link(trb) => (trb.next_segment, 0, control),
            Self::EventData(trb) => {
                let status = transfer_status(0, 0, trb.interrupter);
                (trb.data, status, control)
            }
            Self::NoOp(_) | Self::NoOpCommand(_) => (0, 0, control),
            Self::EnableSlot(trb) => {
                let control = control | (u32::from(trb.slot_type & 0x1F) << 16);
                (0, 0, control)
            }
            Self::DisableSlot(trb) => {
                let control = control | (u32::from(trb.slot_id) << COMMAND_SLOT_SHIFT);
                (0, 0, control)
            }
            Self::Command(command) => {
                let status = u32::from(command.stream_id) << COMMAND_STREAM_SHIFT;
                let control = command_control(command, base, flags);
                (command.input_context, status, control)
            }
            Self::TransferEvent(event) => {
                let status = event_status(event.residual, event.completion);
                let control = control
                    | (u32::from(event.endpoint_id & 0x1F) << COMMAND_ENDPOINT_SHIFT)
                    | (u32::from(event.slot_id) << COMMAND_SLOT_SHIFT);
                (event.trb_pointer, status, control)
            }
            Self::CommandCompletion(event) => {
                let status = event_status(event.parameter, event.completion);
                let control = control
                    | (u32::from(event.vf_id) << COMMAND_ENDPOINT_SHIFT)
                    | (u32::from(event.slot_id) << COMMAND_SLOT_SHIFT);
                (event.command_trb_pointer, status, control)
            }
            Self::HostControllerNotification(event) => {
                let status = event_status(0, event.completion);
                (0, status, control)
            }
            Self::PortStatusChange(event) => {
                let status = event_status(0, event.completion);
                let control = control | (u32::from(event.port_id) << COMMAND_SLOT_SHIFT);
                (0, status, control)
            }
            Self::Unknown(unknown) => {
                let control = (unknown.control & !(TRB_TYPE_MASK << TRB_TYPE_SHIFT))
                    | base
                    | control_flag(TrbFlags::CYCLE, flags);
                (unknown.parameter, unknown.status, control)
            }
        }
    }

    /// Decodes a TRB from the first 16 bytes of `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let parameter = read_u64_le(bytes, 0)?;
        let status = read_u32_le(bytes, 8)?;
        let control = read_u32_le(bytes, 12)?;
        let raw_type = narrow_u16((control >> TRB_TYPE_SHIFT) & TRB_TYPE_MASK);
        let trb_type = TrbType::from_raw(raw_type);
        let flags = decode_flags(trb_type, control);
        Some(Self::from_words(&TrbWords {
            trb_type,
            parameter,
            status,
            control,
            flags,
        }))
    }

    /// Builds a TRB from its double words.
    fn from_words(words: &TrbWords) -> Self {
        let TrbWords {
            trb_type,
            parameter,
            status,
            control,
            flags,
        } = *words;
        let length = status & TRB_LENGTH_MASK;
        let td_size = narrow_u8((status >> TRB_TD_SIZE_SHIFT) & TRB_TD_SIZE_MASK);
        let interrupter = narrow_u16((status >> TRB_INTERRUPTER_SHIFT) & 0x3FF);
        match trb_type {
            TrbType::Normal => Self::Normal(NormalTrb {
                buffer: parameter,
                length,
                td_size,
                interrupter,
                flags,
            }),
            TrbType::SetupStage => Self::SetupStage(SetupStageTrb {
                setup: parameter.to_le_bytes(),
                interrupter,
                transfer_type: ControlTransferType::from_raw(narrow_u16((control >> 16) & 0x3)),
                flags,
            }),
            TrbType::DataStage => Self::DataStage(DataStageTrb {
                buffer: parameter,
                length,
                td_size,
                interrupter,
                flags,
            }),
            TrbType::StatusStage => Self::StatusStage(StatusStageTrb { interrupter, flags }),
            TrbType::Link => Self::Link(LinkTrb {
                next_segment: parameter,
                flags,
            }),
            TrbType::EventData => Self::EventData(EventDataTrb {
                data: parameter,
                interrupter,
                flags,
            }),
            TrbType::NoOp => Self::NoOp(NoOpTrb { flags }),
            TrbType::EnableSlot => Self::EnableSlot(EnableSlotTrb {
                slot_type: narrow_u8((control >> 16) & 0x1F),
                flags,
            }),
            TrbType::DisableSlot => Self::DisableSlot(DisableSlotTrb {
                slot_id: narrow_u8((control >> COMMAND_SLOT_SHIFT) & 0xFF),
                flags,
            }),
            TrbType::NoOpCommand => Self::NoOpCommand(NoOpCommandTrb { flags }),
            TrbType::AddressDevice
            | TrbType::ConfigureEndpoint
            | TrbType::EvaluateContext
            | TrbType::ResetEndpoint
            | TrbType::StopEndpoint
            | TrbType::SetTrDequeuePointer
            | TrbType::ResetDevice => Self::Command(CommandTrb {
                command_type: trb_type,
                input_context: parameter,
                slot_id: narrow_u8((control >> COMMAND_SLOT_SHIFT) & 0xFF),
                endpoint_id: narrow_u8((control >> COMMAND_ENDPOINT_SHIFT) & COMMAND_ENDPOINT_MASK),
                stream_id: narrow_u16((status >> COMMAND_STREAM_SHIFT) & 0xFFFF),
                flags,
            }),
            _ => Self::from_event_words(words),
        }
    }

    /// Builds an event TRB from its double words.
    fn from_event_words(words: &TrbWords) -> Self {
        let TrbWords {
            trb_type,
            parameter,
            status,
            control,
            flags,
        } = *words;
        let completion = CompletionCode::from_raw(narrow_u8(
            (status >> EVENT_COMPLETION_SHIFT) & EVENT_COMPLETION_MASK,
        ));
        match trb_type {
            TrbType::TransferEvent => Self::TransferEvent(TransferEvent {
                trb_pointer: parameter,
                residual: status & 0x00FF_FFFF,
                completion,
                endpoint_id: narrow_u8((control >> COMMAND_ENDPOINT_SHIFT) & COMMAND_ENDPOINT_MASK),
                slot_id: narrow_u8((control >> COMMAND_SLOT_SHIFT) & 0xFF),
                flags,
            }),
            TrbType::CommandCompletionEvent => Self::CommandCompletion(CommandCompletionEvent {
                command_trb_pointer: parameter,
                parameter: status & 0x00FF_FFFF,
                completion,
                slot_id: narrow_u8((control >> COMMAND_SLOT_SHIFT) & 0xFF),
                vf_id: narrow_u8((control >> COMMAND_ENDPOINT_SHIFT) & 0xFF),
                flags,
            }),
            TrbType::HostControllerNotificationEvent => {
                Self::HostControllerNotification(HostControllerNotificationEvent {
                    status,
                    completion,
                    flags,
                })
            }
            TrbType::PortStatusChangeEvent => Self::PortStatusChange(PortStatusChangeEvent {
                port_id: narrow_u8((control >> COMMAND_SLOT_SHIFT) & 0xFF),
                completion,
                flags,
            }),
            other => Self::Unknown(UnknownTrb {
                trb_type: other.to_raw(),
                parameter,
                status,
                control,
                flags,
            }),
        }
    }
}

/// The double words of a Transfer Request Block.
#[derive(Clone, Copy)]
struct TrbWords {
    trb_type: TrbType,
    parameter: u64,
    status: u32,
    control: u32,
    flags: TrbFlags,
}

/// Returns the control word bit of a flag.
fn control_flag(flag: TrbFlags, flags: TrbFlags) -> u32 {
    if flags.contains(flag) {
        control_bit(flag)
    } else {
        0
    }
}

/// Returns the control word bit of a flag.
const fn control_bit(flag: TrbFlags) -> u32 {
    match flag {
        TrbFlags::CYCLE => 1 << 0,
        TrbFlags::EVALUATE_NEXT | TrbFlags::TOGGLE_CYCLE => 1 << 1,
        TrbFlags::INTERRUPT_ON_SHORT_PACKET | TrbFlags::EVENT_DATA => 1 << 2,
        TrbFlags::NO_SNOOP => 1 << 3,
        TrbFlags::CHAIN => 1 << 4,
        TrbFlags::INTERRUPT_ON_COMPLETION => 1 << 5,
        TrbFlags::IMMEDIATE_DATA => 1 << 6,
        TrbFlags::DIRECTION_IN => 1 << 16,
        _ => 0,
    }
}

/// Returns the direction bit of a data or status stage TRB.
fn direction_bit(flags: TrbFlags) -> u32 {
    control_flag(TrbFlags::DIRECTION_IN, flags)
}

/// Builds the status word of a transfer TRB.
fn transfer_status(length: u32, td_size: u8, interrupter: u16) -> u32 {
    (length & TRB_LENGTH_MASK)
        | (u32::from(td_size & 0x1F) << TRB_TD_SIZE_SHIFT)
        | (u32::from(interrupter) << TRB_INTERRUPTER_SHIFT)
}

/// Builds the status word of an event TRB.
fn event_status(value: u32, completion: CompletionCode) -> u32 {
    (value & 0x00FF_FFFF) | (u32::from(completion.to_raw()) << EVENT_COMPLETION_SHIFT)
}

/// Builds the control word of a command TRB.
fn command_control(command: &CommandTrb, base: u32, flags: TrbFlags) -> u32 {
    let mut control = base
        | control_flag(TrbFlags::CYCLE, flags)
        | (u32::from(command.slot_id) << COMMAND_SLOT_SHIFT)
        | ((u32::from(command.endpoint_id) & COMMAND_ENDPOINT_MASK) << COMMAND_ENDPOINT_SHIFT);
    match command.command_type {
        TrbType::AddressDevice if flags.contains(TrbFlags::BLOCK_SPLIT_REQUEST) => {
            control |= COMMAND_BSR;
        }
        TrbType::ConfigureEndpoint if flags.contains(TrbFlags::DECONFIGURE) => {
            control |= COMMAND_DC;
        }
        TrbType::ResetEndpoint if flags.contains(TrbFlags::TRANSFER_STATE_PROTOCOL) => {
            control |= COMMAND_TSP;
        }
        _ => {}
    }
    control
}

/// Decodes the control flags of a TRB.
fn decode_flags(trb_type: TrbType, control: u32) -> TrbFlags {
    let mut flags = TrbFlags::from_bits_truncate(0);
    if control & (1 << 0) != 0 {
        flags = flags.union(TrbFlags::CYCLE);
    }
    if control & (1 << 1) != 0 {
        flags = if trb_type == TrbType::Link {
            flags.union(TrbFlags::TOGGLE_CYCLE)
        } else {
            flags.union(TrbFlags::EVALUATE_NEXT)
        };
    }
    if control & (1 << 2) != 0 {
        flags = if trb_type == TrbType::TransferEvent {
            flags.union(TrbFlags::EVENT_DATA)
        } else {
            flags.union(TrbFlags::INTERRUPT_ON_SHORT_PACKET)
        };
    }
    if control & (1 << 3) != 0 {
        flags = flags.union(TrbFlags::NO_SNOOP);
    }
    if control & (1 << 4) != 0 {
        flags = flags.union(TrbFlags::CHAIN);
    }
    if control & (1 << 5) != 0 {
        flags = flags.union(TrbFlags::INTERRUPT_ON_COMPLETION);
    }
    if control & (1 << 6) != 0 {
        flags = flags.union(TrbFlags::IMMEDIATE_DATA);
    }
    flags
}

/// The TRB sequence that performs a USB control transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ControlTransfer {
    /// The setup stage TRB.
    pub setup: SetupStageTrb,
    /// The optional data stage TRB.
    pub data: Option<DataStageTrb>,
    /// The status stage TRB.
    pub status: StatusStageTrb,
}

impl ControlTransfer {
    /// Creates a control transfer for the given setup packet.
    #[must_use]
    pub const fn new(setup: [u8; 8], interrupter: u16, transfer_type: ControlTransferType) -> Self {
        Self {
            setup: SetupStageTrb::new(setup, interrupter, transfer_type),
            data: None,
            status: StatusStageTrb::new(true, interrupter),
        }
    }

    /// Adds a data stage to the transfer.
    #[must_use]
    pub fn with_data(mut self, buffer: u64, length: u32, direction_in: bool) -> Self {
        let interrupter = self.setup.interrupter;
        self.data = Some(DataStageTrb::new(buffer, length, direction_in, interrupter));
        self
    }

    /// Returns the TRBs of the transfer in ring order.
    #[must_use]
    pub fn trbs(&self) -> [Option<Trb>; 3] {
        [
            Some(Trb::SetupStage(self.setup)),
            self.data.map(Trb::DataStage),
            Some(Trb::StatusStage(self.status)),
        ]
    }
}
