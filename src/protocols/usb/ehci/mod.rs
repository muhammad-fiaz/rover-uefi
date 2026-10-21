//! # EHCI
//!
//! Native support for the Enhanced Host Controller Interface (`EHCI`),
//! the USB 2.0 host controller interface defined by the USB 2.0
//! specification.
//!
//! The module covers the capability registers, the operational
//! registers, the asynchronous and periodic schedules and the periodic
//! frame list.

pub mod queue;

pub use queue::{
    BufferPointer, EndpointSpeed, PacketId, QueueHead, TransferDescriptor, TransferOverlay,
    TransferStatus,
};

/// The offset of the capability register length byte.
pub const CAPLENGTH_OFFSET: usize = 0x00;

/// The offset of the host controller interface version field.
pub const HCIVERSION_OFFSET: usize = 0x02;

/// The offset of the host controller structural parameters word.
pub const HCSPARAMS_OFFSET: usize = 0x04;

/// The offset of the host controller capability parameters word.
pub const HCCPARAMS_OFFSET: usize = 0x08;

/// The offset of the companion port route description.
pub const HCSP_PORTROUTE_OFFSET: usize = 0x0C;

/// The offset of the USB command register, relative to the operational
/// register base.
pub const USBCMD_OFFSET: usize = 0x00;

/// The offset of the USB status register, relative to the operational
/// register base.
pub const USBSTS_OFFSET: usize = 0x04;

/// The offset of the USB interrupt enable register, relative to the
/// operational register base.
pub const USBINTR_OFFSET: usize = 0x08;

/// The offset of the frame index register, relative to the operational
/// register base.
pub const FRINDEX_OFFSET: usize = 0x0C;

/// The offset of the control data structure segment register.
pub const CTRLDSSEGMENT_OFFSET: usize = 0x10;

/// The offset of the periodic list base address register.
pub const PERIODICLISTBASE_OFFSET: usize = 0x14;

/// The offset of the asynchronous list address register.
pub const ASYNCLISTADDR_OFFSET: usize = 0x18;

/// The offset of the configuration flag register.
pub const CONFIGFLAG_OFFSET: usize = 0x40;

/// The offset of the first port status and control register.
pub const PORTSC_OFFSET: usize = 0x44;

/// The number of entries of a full periodic frame list.
pub const PERIODIC_FRAME_LIST_ENTRIES: usize = 1024;

macro_rules! flag_type {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$flag_meta:meta])*
                $flag:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name(u32);

        impl $name {
            $(
                $(#[$flag_meta])*
                pub const $flag: Self = Self($value);
            )*

            /// Creates a flag set from its raw bits.
            #[must_use]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                Self(bits)
            }

            /// Returns the raw bits of the flag set.
            #[must_use]
            pub const fn bits(self) -> u32 {
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

            /// Returns whether no flag is set.
            #[must_use]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
        }
    };
}

flag_type! {
    /// The capability flags of a host controller.
    HcCapabilityFlags {
        /// The ports have power switches.
        PORT_POWER_CONTROL = 1 << 0,
        /// The port routing is enumerated explicitly.
        PORT_ROUTING_RULES = 1 << 1,
        /// The ports support a port indicator.
        PORT_INDICATOR = 1 << 2,
        /// The controller implements 64-bit addressing.
        ADDRESSING_64_BIT = 1 << 3,
        /// The size of the periodic frame list is programmable.
        PROGRAMMABLE_FRAME_LIST = 1 << 4,
        /// The controller implements asynchronous schedule park mode.
        ASYNC_SCHEDULE_PARK_CAPABILITY = 1 << 5,
        /// The isochronous scheduling threshold is counted in whole
        /// frames.
        ISOCHRONOUS_THRESHOLD_FULL_FRAME = 1 << 6,
    }
}

flag_type! {
    /// The flags of the USB command register.
    UsbCommandFlags {
        /// The controller runs the schedules.
        RUN_STOP = 1 << 0,
        /// The controller is reset.
        HOST_CONTROLLER_RESET = 1 << 1,
        /// The periodic schedule is processed.
        PERIODIC_SCHEDULE_ENABLE = 1 << 2,
        /// The asynchronous schedule is processed.
        ASYNC_SCHEDULE_ENABLE = 1 << 3,
        /// The controller interrupts when it advances the asynchronous
        /// schedule.
        INTERRUPT_ON_ASYNC_ADVANCE = 1 << 4,
        /// The controller performs a light host controller reset.
        LIGHT_HOST_CONTROLLER_RESET = 1 << 5,
        /// Asynchronous schedule park mode is enabled.
        ASYNC_SCHEDULE_PARK_ENABLE = 1 << 6,
    }
}

flag_type! {
    /// The flags of the USB status register.
    UsbStatusFlags {
        /// A transfer completed.
        USB_INTERRUPT = 1 << 0,
        /// A USB error occurred.
        USB_ERROR_INTERRUPT = 1 << 1,
        /// A port status changed.
        PORT_CHANGE_DETECT = 1 << 2,
        /// The frame list rolled over.
        FRAME_LIST_ROLLOVER = 1 << 3,
        /// A host system error occurred.
        HOST_SYSTEM_ERROR = 1 << 4,
        /// The asynchronous schedule advanced.
        INTERRUPT_ON_ASYNC_ADVANCE = 1 << 5,
        /// The controller is halted.
        HOST_CONTROLLER_HALTED = 1 << 6,
        /// The asynchronous schedule is empty.
        RECLAMATION = 1 << 7,
        /// The periodic schedule is enabled.
        PERIODIC_SCHEDULE_STATUS = 1 << 8,
        /// The asynchronous schedule is enabled.
        ASYNC_SCHEDULE_STATUS = 1 << 9,
    }
}

flag_type! {
    /// The flags of the USB interrupt enable register.
    UsbInterruptFlags {
        /// A completed transfer raises an interrupt.
        USB_INTERRUPT_ENABLE = 1 << 0,
        /// A USB error raises an interrupt.
        USB_ERROR_INTERRUPT_ENABLE = 1 << 1,
        /// A port change raises an interrupt.
        PORT_CHANGE_INTERRUPT_ENABLE = 1 << 2,
        /// A frame list rollover raises an interrupt.
        FRAME_LIST_ROLLOVER_ENABLE = 1 << 3,
        /// A host system error raises an interrupt.
        HOST_SYSTEM_ERROR_ENABLE = 1 << 4,
        /// An asynchronous schedule advance raises an interrupt.
        INTERRUPT_ON_ASYNC_ADVANCE_ENABLE = 1 << 5,
    }
}

/// The type of the schedule data structure a link pointer refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScheduleElementType {
    /// An isochronous transfer descriptor.
    IsochronousTransferDescriptor,
    /// A queue head.
    QueueHead,
    /// A split transaction isochronous transfer descriptor.
    SplitTransactionIsochronousTransferDescriptor,
    /// A frame span traversal node.
    FrameSpanTraversalNode,
}

impl ScheduleElementType {
    /// Returns the numeric type value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::IsochronousTransferDescriptor => 0,
            Self::QueueHead => 1,
            Self::SplitTransactionIsochronousTransferDescriptor => 2,
            Self::FrameSpanTraversalNode => 3,
        }
    }

    /// Creates a type from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::IsochronousTransferDescriptor,
            1 => Self::QueueHead,
            2 => Self::SplitTransactionIsochronousTransferDescriptor,
            _ => Self::FrameSpanTraversalNode,
        }
    }
}

/// A link pointer of the periodic or the asynchronous schedule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScheduleLink {
    /// The physical address of the referenced schedule data structure.
    pub address: u32,
    /// Whether the pointer is invalid and the list ends here.
    pub terminate: bool,
    /// The type of the referenced schedule data structure.
    pub element_type: ScheduleElementType,
}

impl ScheduleLink {
    /// Creates a link pointer to a queue head.
    #[must_use]
    pub const fn queue_head(address: u32) -> Self {
        Self {
            address,
            terminate: false,
            element_type: ScheduleElementType::QueueHead,
        }
    }

    /// Creates a terminated link pointer.
    #[must_use]
    pub const fn terminated() -> Self {
        Self {
            address: 0,
            terminate: true,
            element_type: ScheduleElementType::QueueHead,
        }
    }

    /// Encodes the link pointer into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        (self.address & 0xFFFF_FFE0)
            | (u32::from(self.terminate))
            | (u32::from(self.element_type.to_raw() & 0x3) << 1)
    }

    /// Decodes the link pointer from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            address: value & 0xFFFF_FFE0,
            terminate: value & 1 != 0,
            element_type: ScheduleElementType::from_raw(
                u8::try_from((value >> 1) & 0x3).unwrap_or_default(),
            ),
        }
    }
}

/// An entry of the periodic frame list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FrameListEntry {
    /// The physical address of the referenced schedule data structure.
    pub address: u32,
    /// Whether the frame is empty.
    pub terminate: bool,
    /// The type of the referenced schedule data structure.
    pub element_type: ScheduleElementType,
}

impl FrameListEntry {
    /// Creates an entry that references a queue head.
    #[must_use]
    pub const fn queue_head(address: u32) -> Self {
        Self {
            address,
            terminate: false,
            element_type: ScheduleElementType::QueueHead,
        }
    }

    /// Creates an entry that marks the frame as empty.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            address: 0,
            terminate: true,
            element_type: ScheduleElementType::QueueHead,
        }
    }

    /// Encodes the entry into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        (self.address & 0xFFFF_FFE0)
            | (u32::from(self.terminate))
            | (u32::from(self.element_type.to_raw() & 0x3) << 1)
    }

    /// Decodes the entry from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        Self {
            address: value & 0xFFFF_FFE0,
            terminate: value & 1 != 0,
            element_type: ScheduleElementType::from_raw(
                u8::try_from((value >> 1) & 0x3).unwrap_or_default(),
            ),
        }
    }
}

/// The number of entries of the periodic frame list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FrameListSize {
    /// The frame list holds 1024 entries.
    Entries1024,
    /// The frame list holds 512 entries.
    Entries512,
    /// The frame list holds 256 entries.
    Entries256,
    /// The reserved encoding, which the controller treats as 1024
    /// entries.
    Reserved,
}

impl FrameListSize {
    /// Returns the number of entries of the frame list.
    #[must_use]
    pub fn entry_count(self) -> usize {
        match self {
            Self::Entries1024 | Self::Reserved => 1024,
            Self::Entries512 => 512,
            Self::Entries256 => 256,
        }
    }

    /// Returns the number of bits of the frame index that select an
    /// entry of the frame list.
    #[must_use]
    pub fn index_bits(self) -> u32 {
        match self {
            Self::Entries1024 | Self::Reserved => 12,
            Self::Entries512 => 11,
            Self::Entries256 => 10,
        }
    }

    /// Returns the numeric encoding of the frame list size.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Entries1024 => 0,
            Self::Entries512 => 1,
            Self::Entries256 => 2,
            Self::Reserved => 3,
        }
    }

    /// Creates a frame list size from its numeric encoding.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::Entries1024,
            1 => Self::Entries512,
            2 => Self::Entries256,
            _ => Self::Reserved,
        }
    }
}

/// The capability registers of a host controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HcCapabilities {
    /// The length of the capability register block in bytes.
    pub cap_length: u8,
    /// The host controller interface version, in binary coded decimal.
    pub hci_version: u16,
    /// The number of physical downstream ports.
    pub port_count: u8,
    /// The number of ports of each companion controller.
    pub ports_per_companion: u8,
    /// The number of companion controllers.
    pub companion_controller_count: u8,
    /// The one-based number of the debug port, zero when absent.
    pub debug_port: u8,
    /// The isochronous scheduling threshold.
    pub isochronous_scheduling_threshold: u8,
    /// The capability flags of the controller.
    pub flags: HcCapabilityFlags,
}

impl HcCapabilities {
    /// Parses the capability registers of a controller.
    ///
    /// Returns `None` when the register block is too short.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let cap_length = *bytes.get(CAPLENGTH_OFFSET)?;
        let version_bytes = bytes.get(HCIVERSION_OFFSET..HCIVERSION_OFFSET + 2)?;
        let hci_version = u16::from_le_bytes([version_bytes[0], version_bytes[1]]);
        let params = read_word(bytes, HCSPARAMS_OFFSET)?;
        let capabilities = read_word(bytes, HCCPARAMS_OFFSET)?;
        let mut flags = HcCapabilityFlags::from_bits_truncate(0);
        if params & (1 << 4) != 0 {
            flags = flags.union(HcCapabilityFlags::PORT_POWER_CONTROL);
        }
        if params & (1 << 7) != 0 {
            flags = flags.union(HcCapabilityFlags::PORT_ROUTING_RULES);
        }
        if params & (1 << 16) != 0 {
            flags = flags.union(HcCapabilityFlags::PORT_INDICATOR);
        }
        if capabilities & 1 != 0 {
            flags = flags.union(HcCapabilityFlags::ADDRESSING_64_BIT);
        }
        if capabilities & (1 << 1) != 0 {
            flags = flags.union(HcCapabilityFlags::PROGRAMMABLE_FRAME_LIST);
        }
        if capabilities & (1 << 2) != 0 {
            flags = flags.union(HcCapabilityFlags::ASYNC_SCHEDULE_PARK_CAPABILITY);
        }
        if capabilities & (1 << 7) != 0 {
            flags = flags.union(HcCapabilityFlags::ISOCHRONOUS_THRESHOLD_FULL_FRAME);
        }
        Some(Self {
            cap_length,
            hci_version,
            port_count: u8::try_from(params & 0x0F).unwrap_or_default(),
            ports_per_companion: u8::try_from((params >> 8) & 0x0F).unwrap_or_default(),
            companion_controller_count: u8::try_from((params >> 12) & 0x0F).unwrap_or_default(),
            debug_port: u8::try_from((params >> 20) & 0x0F).unwrap_or_default(),
            isochronous_scheduling_threshold: u8::try_from((capabilities >> 4) & 0x7)
                .unwrap_or_default(),
            flags,
        })
    }
}

/// The USB command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsbCommand {
    /// The number of transactions the controller may execute from one
    /// queue head before it advances the asynchronous schedule.
    pub async_schedule_park_count: u8,
    /// The number of microframes between two interrupts.
    pub interrupt_threshold: u8,
    /// The size of the periodic frame list.
    pub frame_list_size: FrameListSize,
    /// The flags of the register.
    pub flags: UsbCommandFlags,
}

impl UsbCommand {
    /// Creates a command register that leaves the controller stopped.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            async_schedule_park_count: 0,
            interrupt_threshold: 8,
            frame_list_size: FrameListSize::Entries1024,
            flags: UsbCommandFlags::from_bits_truncate(0),
        }
    }

    /// Returns whether the controller runs the schedules.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.flags.contains(UsbCommandFlags::RUN_STOP)
    }

    /// Encodes the register into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        let flags = self.flags;
        let mut value = 0_u32;
        if flags.contains(UsbCommandFlags::RUN_STOP) {
            value |= 1 << 0;
        }
        if flags.contains(UsbCommandFlags::HOST_CONTROLLER_RESET) {
            value |= 1 << 1;
        }
        value |= (u32::from(self.frame_list_size.to_raw()) & 0x3) << 2;
        if flags.contains(UsbCommandFlags::PERIODIC_SCHEDULE_ENABLE) {
            value |= 1 << 4;
        }
        if flags.contains(UsbCommandFlags::ASYNC_SCHEDULE_ENABLE) {
            value |= 1 << 5;
        }
        if flags.contains(UsbCommandFlags::INTERRUPT_ON_ASYNC_ADVANCE) {
            value |= 1 << 6;
        }
        if flags.contains(UsbCommandFlags::LIGHT_HOST_CONTROLLER_RESET) {
            value |= 1 << 7;
        }
        value |= u32::from(self.async_schedule_park_count & 0x3) << 8;
        if flags.contains(UsbCommandFlags::ASYNC_SCHEDULE_PARK_ENABLE) {
            value |= 1 << 11;
        }
        value | (u32::from(self.interrupt_threshold) << 16)
    }

    /// Decodes the register from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        let mut flags = UsbCommandFlags::from_bits_truncate(0);
        if value & 1 != 0 {
            flags = flags.union(UsbCommandFlags::RUN_STOP);
        }
        if value & (1 << 1) != 0 {
            flags = flags.union(UsbCommandFlags::HOST_CONTROLLER_RESET);
        }
        if value & (1 << 4) != 0 {
            flags = flags.union(UsbCommandFlags::PERIODIC_SCHEDULE_ENABLE);
        }
        if value & (1 << 5) != 0 {
            flags = flags.union(UsbCommandFlags::ASYNC_SCHEDULE_ENABLE);
        }
        if value & (1 << 6) != 0 {
            flags = flags.union(UsbCommandFlags::INTERRUPT_ON_ASYNC_ADVANCE);
        }
        if value & (1 << 7) != 0 {
            flags = flags.union(UsbCommandFlags::LIGHT_HOST_CONTROLLER_RESET);
        }
        if value & (1 << 11) != 0 {
            flags = flags.union(UsbCommandFlags::ASYNC_SCHEDULE_PARK_ENABLE);
        }
        Self {
            async_schedule_park_count: u8::try_from((value >> 8) & 0x3).unwrap_or_default(),
            interrupt_threshold: u8::try_from((value >> 16) & 0xFF).unwrap_or_default(),
            frame_list_size: FrameListSize::from_raw(
                u8::try_from((value >> 2) & 0x3).unwrap_or_default(),
            ),
            flags,
        }
    }
}

impl Default for UsbCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// The USB status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsbStatus {
    /// The flags of the register.
    pub flags: UsbStatusFlags,
}

impl UsbStatus {
    /// Encodes the register into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        let flags = self.flags;
        let mut value = 0_u32;
        if flags.contains(UsbStatusFlags::USB_INTERRUPT) {
            value |= 1 << 0;
        }
        if flags.contains(UsbStatusFlags::USB_ERROR_INTERRUPT) {
            value |= 1 << 1;
        }
        if flags.contains(UsbStatusFlags::PORT_CHANGE_DETECT) {
            value |= 1 << 2;
        }
        if flags.contains(UsbStatusFlags::FRAME_LIST_ROLLOVER) {
            value |= 1 << 3;
        }
        if flags.contains(UsbStatusFlags::HOST_SYSTEM_ERROR) {
            value |= 1 << 4;
        }
        if flags.contains(UsbStatusFlags::INTERRUPT_ON_ASYNC_ADVANCE) {
            value |= 1 << 5;
        }
        if flags.contains(UsbStatusFlags::HOST_CONTROLLER_HALTED) {
            value |= 1 << 12;
        }
        if flags.contains(UsbStatusFlags::RECLAMATION) {
            value |= 1 << 13;
        }
        if flags.contains(UsbStatusFlags::PERIODIC_SCHEDULE_STATUS) {
            value |= 1 << 14;
        }
        if flags.contains(UsbStatusFlags::ASYNC_SCHEDULE_STATUS) {
            value |= 1 << 15;
        }
        value
    }

    /// Decodes the register from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        let mut flags = UsbStatusFlags::from_bits_truncate(0);
        if value & 1 != 0 {
            flags = flags.union(UsbStatusFlags::USB_INTERRUPT);
        }
        if value & (1 << 1) != 0 {
            flags = flags.union(UsbStatusFlags::USB_ERROR_INTERRUPT);
        }
        if value & (1 << 2) != 0 {
            flags = flags.union(UsbStatusFlags::PORT_CHANGE_DETECT);
        }
        if value & (1 << 3) != 0 {
            flags = flags.union(UsbStatusFlags::FRAME_LIST_ROLLOVER);
        }
        if value & (1 << 4) != 0 {
            flags = flags.union(UsbStatusFlags::HOST_SYSTEM_ERROR);
        }
        if value & (1 << 5) != 0 {
            flags = flags.union(UsbStatusFlags::INTERRUPT_ON_ASYNC_ADVANCE);
        }
        if value & (1 << 12) != 0 {
            flags = flags.union(UsbStatusFlags::HOST_CONTROLLER_HALTED);
        }
        if value & (1 << 13) != 0 {
            flags = flags.union(UsbStatusFlags::RECLAMATION);
        }
        if value & (1 << 14) != 0 {
            flags = flags.union(UsbStatusFlags::PERIODIC_SCHEDULE_STATUS);
        }
        if value & (1 << 15) != 0 {
            flags = flags.union(UsbStatusFlags::ASYNC_SCHEDULE_STATUS);
        }
        Self { flags }
    }
}

/// The USB interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsbInterrupt {
    /// The flags of the register.
    pub flags: UsbInterruptFlags,
}

impl UsbInterrupt {
    /// Creates an interrupt register that enables no interrupt.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            flags: UsbInterruptFlags::from_bits_truncate(0),
        }
    }

    /// Encodes the register into its 32-bit value.
    #[must_use]
    pub fn encode(&self) -> u32 {
        let flags = self.flags;
        let mut value = 0_u32;
        if flags.contains(UsbInterruptFlags::USB_INTERRUPT_ENABLE) {
            value |= 1 << 0;
        }
        if flags.contains(UsbInterruptFlags::USB_ERROR_INTERRUPT_ENABLE) {
            value |= 1 << 1;
        }
        if flags.contains(UsbInterruptFlags::PORT_CHANGE_INTERRUPT_ENABLE) {
            value |= 1 << 2;
        }
        if flags.contains(UsbInterruptFlags::FRAME_LIST_ROLLOVER_ENABLE) {
            value |= 1 << 3;
        }
        if flags.contains(UsbInterruptFlags::HOST_SYSTEM_ERROR_ENABLE) {
            value |= 1 << 4;
        }
        if flags.contains(UsbInterruptFlags::INTERRUPT_ON_ASYNC_ADVANCE_ENABLE) {
            value |= 1 << 5;
        }
        value
    }

    /// Decodes the register from its 32-bit value.
    #[must_use]
    pub fn decode(value: u32) -> Self {
        let mut flags = UsbInterruptFlags::from_bits_truncate(0);
        if value & 1 != 0 {
            flags = flags.union(UsbInterruptFlags::USB_INTERRUPT_ENABLE);
        }
        if value & (1 << 1) != 0 {
            flags = flags.union(UsbInterruptFlags::USB_ERROR_INTERRUPT_ENABLE);
        }
        if value & (1 << 2) != 0 {
            flags = flags.union(UsbInterruptFlags::PORT_CHANGE_INTERRUPT_ENABLE);
        }
        if value & (1 << 3) != 0 {
            flags = flags.union(UsbInterruptFlags::FRAME_LIST_ROLLOVER_ENABLE);
        }
        if value & (1 << 4) != 0 {
            flags = flags.union(UsbInterruptFlags::HOST_SYSTEM_ERROR_ENABLE);
        }
        if value & (1 << 5) != 0 {
            flags = flags.union(UsbInterruptFlags::INTERRUPT_ON_ASYNC_ADVANCE_ENABLE);
        }
        Self { flags }
    }
}

impl Default for UsbInterrupt {
    fn default() -> Self {
        Self::new()
    }
}

/// Reads a little-endian double word at `offset`.
fn read_word(bytes: &[u8], offset: usize) -> Option<u32> {
    let chunk = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(chunk);
    Some(u32::from_le_bytes(raw))
}
