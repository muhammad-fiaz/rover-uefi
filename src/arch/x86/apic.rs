//! # Local and input/output advanced programmable interrupt
//! controllers
//!
//! The register layout of the local advanced programmable
//! interrupt controller and of the input/output advanced
//! programmable interrupt controller.
//!
//! The local controller is mapped into the physical address space
//! as a page of 32-bit registers. The input/output controller
//! has a much smaller register file that is reached through an
//! index and a data register, and it stores one redirection
//! entry per platform interrupt line.
//!
//! The view structures in this module own the base address of a
//! controller and provide the register accesses. They perform no
//! validation, so every accessor that touches the hardware is an
//! unsafe function.

use crate::arch::common::mmio::{read_u32, write_u32};

/// The offset of the local controller identification register.
pub const APIC_ID: u32 = 0x020;

/// The offset of the local controller version register.
pub const APIC_VERSION: u32 = 0x030;

/// The offset of the local controller task priority register.
pub const APIC_TPR: u32 = 0x080;

/// The offset of the local controller destination format
/// register.
pub const APIC_ICR_DFR: u32 = 0x0E0;

/// The offset of the local controller end of interrupt register.
pub const APIC_EOI: u32 = 0x0B0;

/// The offset of the local controller spurious interrupt vector
/// register.
pub const APIC_SVR: u32 = 0x0F0;

/// The offset of the in-service register.
pub const APIC_ISR: u32 = 0x100;

/// The offset of the trigger mode register.
pub const APIC_TMR: u32 = 0x180;

/// The offset of the interrupt request register.
pub const APIC_IRR: u32 = 0x200;

/// The offset of the error status register.
pub const APIC_ESR: u32 = 0x280;

/// The offset of the low half of the interrupt command register.
pub const APIC_ICR_LOW: u32 = 0x300;

/// The offset of the high half of the interrupt command register.
pub const APIC_ICR_HIGH: u32 = 0x310;

/// The offset of the timer local vector table register.
pub const APIC_LVT_TIMER: u32 = 0x320;

/// The offset of the thermal sensor local vector table register.
pub const APIC_LVT_THERMAL: u32 = 0x330;

/// The offset of the performance monitoring local vector table
/// register.
pub const APIC_LVT_PERF: u32 = 0x340;

/// The offset of the first local interrupt local vector table
/// register.
pub const APIC_LVT_LINT0: u32 = 0x350;

/// The offset of the second local interrupt local vector table
/// register.
pub const APIC_LVT_LINT1: u32 = 0x360;

/// The offset of the error local vector table register.
pub const APIC_LVT_ERROR: u32 = 0x370;

/// The offset of the timer initial count register.
pub const APIC_TIMER_INITCNT: u32 = 0x380;

/// The offset of the timer current count register.
pub const APIC_TIMER_CURCNT: u32 = 0x390;

/// The offset of the timer divide configuration register.
pub const APIC_TIMER_DIV: u32 = 0x3E0;

/// The mask of the interrupt vector inside the low half of the
/// interrupt command register.
pub const ICR_VECTOR_MASK: u32 = 0x0000_00FF;

/// The shift of the delivery mode inside the low half of the
/// interrupt command register.
pub const ICR_DELIVERY_MODE_SHIFT: u32 = 8;

/// The mask of the delivery mode inside the low half of the
/// interrupt command register.
pub const ICR_DELIVERY_MODE_MASK: u32 = 0x0000_0700;

/// The mask of the destination mode inside the low half of the
/// interrupt command register.
pub const ICR_DESTINATION_MODE_MASK: u32 = 0x0000_0800;

/// The mask of the delivery status inside the low half of the
/// interrupt command register.
pub const ICR_DELIVERY_STATUS_MASK: u32 = 0x0000_1000;

/// The mask of the level inside the low half of the interrupt
/// command register.
pub const ICR_LEVEL_MASK: u32 = 0x0001_0000;

/// The mask of the trigger mode inside the low half of the
/// interrupt command register.
pub const ICR_TRIGGER_MODE_MASK: u32 = 0x0002_0000;

/// The shift of the destination shorthand inside the low half of
/// the interrupt command register.
pub const ICR_SHORTHAND_SHIFT: u32 = 18;

/// The mask of the destination shorthand inside the low half of
/// the interrupt command register.
pub const ICR_SHORTHAND_MASK: u32 = 0x000C_0000;

/// The shift of the destination inside the high half of the
/// interrupt command register.
pub const ICR_DESTINATION_SHIFT: u32 = 24;

/// The mask of the destination inside the high half of the
/// interrupt command register.
pub const ICR_DESTINATION_MASK: u32 = 0xFF00_0000;

/// The mask of the software enable inside the spurious interrupt
/// vector register.
pub const SVR_SOFTWARE_ENABLE_MASK: u32 = 0x0000_0100;

/// The mask of the spurious vector inside the spurious interrupt
/// vector register.
pub const SVR_SPURIOUS_VECTOR_MASK: u32 = 0x0000_00FF;

/// The mask of the masked state inside the timer local vector
/// table register.
pub const LVT_TIMER_MASKED_MASK: u32 = 0x0001_0000;

/// The mask of the periodic mode inside the timer local vector
/// table register.
pub const LVT_TIMER_PERIODIC_MASK: u32 = 0x0002_0000;

/// The divide value that selects the unhalved bus frequency.
pub const APIC_TIMER_DIVIDE_FULL: u32 = 0x0B;

/// The divide value that selects the bus frequency divided by
/// two.
pub const APIC_TIMER_DIVIDE_HALF: u32 = 0x00;

/// The divide value that selects the bus frequency divided by
/// four.
pub const APIC_TIMER_DIVIDE_QUARTER: u32 = 0x01;

/// The divide value that selects the bus frequency divided by
/// eight.
pub const APIC_TIMER_DIVIDE_EIGHTH: u32 = 0x02;

/// The divide value that selects the bus frequency divided by
/// sixteen.
pub const APIC_TIMER_DIVIDE_SIXTEENTH: u32 = 0x03;

/// The divide value that selects the bus frequency divided by
/// thirty-two.
pub const APIC_TIMER_DIVIDE_THIRTY_SECOND: u32 = 0x08;

/// The divide value that selects the bus frequency divided by
/// sixty-four.
pub const APIC_TIMER_DIVIDE_SIXTY_FOURTH: u32 = 0x09;

/// The divide value that selects the bus frequency divided by
/// one hundred and twenty-eight.
pub const APIC_TIMER_DIVIDE_HUNDRED_TWENTY_EIGHTH: u32 = 0x0A;

/// The offset of the identification register of the input/output
/// controller.
pub const IO_APIC_ID_OFFSET: u32 = 0x000;

/// The offset of the version register of the input/output
/// controller.
pub const IO_APIC_VERSION_OFFSET: u32 = 0x001;

/// The offset of the index register of the input/output
/// controller.
pub const IO_APIC_INDEX_OFFSET: u32 = 0x010;

/// The offset of the data register of the input/output
/// controller.
pub const IO_APIC_DATA_OFFSET: u32 = 0x011;

/// The index of the first redirection entry of the input/output
/// controller.
pub const IO_APIC_REDIRECTION_INDEX: u32 = 0x010;

/// The number of redirection entries of the first input/output
/// controller.
pub const IO_APIC_MAX_REDIRECTION_ENTRIES: u32 = 24;

/// The mask of the interrupt vector inside a redirection entry.
pub const IO_APIC_VECTOR_MASK: u64 = 0x0000_00FF;

/// The shift of the delivery mode inside a redirection entry.
pub const IO_APIC_DELIVERY_MODE_SHIFT: u32 = 8;

/// The mask of the delivery mode inside a redirection entry.
pub const IO_APIC_DELIVERY_MODE_MASK: u64 = 0x0000_0700;

/// The mask of the destination mode inside a redirection entry.
pub const IO_APIC_DESTINATION_MODE_MASK: u64 = 0x0000_0800;

/// The mask of the delivery status inside a redirection entry.
pub const IO_APIC_DELIVERY_STATUS_MASK: u64 = 0x0000_1000;

/// The mask of the polarity inside a redirection entry.
pub const IO_APIC_POLARITY_MASK: u64 = 0x0000_2000;

/// The mask of the remote interrupt request register inside a
/// redirection entry.
pub const IO_APIC_REMOTE_IRR_MASK: u64 = 0x0000_4000;

/// The mask of the trigger mode inside a redirection entry.
pub const IO_APIC_TRIGGER_MODE_MASK: u64 = 0x0000_8000;

/// The mask of the masked state inside a redirection entry.
pub const IO_APIC_MASKED_MASK: u64 = 0x0001_0000;

/// The shift of the destination inside a redirection entry.
pub const IO_APIC_DESTINATION_SHIFT: u32 = 56;

/// The mask of the destination inside a redirection entry.
pub const IO_APIC_DESTINATION_MASK: u64 = 0xFF00_0000_0000_0000;

/// The delivery mode of an interrupt request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeliveryMode {
    /// The interrupt is delivered to the processor at once.
    Fixed,
    /// The interrupt is delivered when no other interrupt of
    /// higher priority is pending.
    LowestPriority,
    /// The interrupt is delivered as a system management
    /// interrupt.
    Smi,
    /// The interrupt is delivered as a non-maskable interrupt.
    Nmi,
    /// The interrupt is delivered as an initialisation
    /// interrupt.
    Init,
    /// The interrupt is delivered as a start-up interrupt.
    Startup,
}

impl DeliveryMode {
    /// Returns the numeric value of the delivery mode.
    #[must_use]
    pub const fn value(self) -> u32 {
        match self {
            Self::Fixed => 0b000,
            Self::LowestPriority => 0b001,
            Self::Smi => 0b010,
            Self::Nmi => 0b011,
            Self::Init => 0b100,
            Self::Startup => 0b101,
        }
    }

    /// Decodes a delivery mode from its numeric value.
    ///
    /// The two reserved encodings decode to `None`.
    #[must_use]
    pub const fn from_value(value: u32) -> Option<Self> {
        match value & 0b111 {
            0b000 => Some(Self::Fixed),
            0b001 => Some(Self::LowestPriority),
            0b010 => Some(Self::Smi),
            0b011 => Some(Self::Nmi),
            0b100 => Some(Self::Init),
            0b101 => Some(Self::Startup),
            _ => None,
        }
    }
}

/// The destination shorthand of an interrupt command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DestinationShorthand {
    /// The destination field selects the target.
    None,
    /// The target is the processor that writes the register.
    Physical,
    /// The target is every processor including the writer.
    AllIncludingSelf,
    /// The target is every processor except the writer.
    AllExcludingSelf,
}

impl DestinationShorthand {
    /// Returns the numeric value of the shorthand.
    #[must_use]
    pub const fn value(self) -> u32 {
        match self {
            Self::None => 0b00,
            Self::Physical => 0b01,
            Self::AllIncludingSelf => 0b10,
            Self::AllExcludingSelf => 0b11,
        }
    }

    /// Decodes a shorthand from its numeric value.
    #[must_use]
    pub const fn from_value(value: u32) -> Option<Self> {
        match value & 0b11 {
            0b00 => Some(Self::None),
            0b01 => Some(Self::Physical),
            0b10 => Some(Self::AllIncludingSelf),
            0b11 => Some(Self::AllExcludingSelf),
            _ => None,
        }
    }
}

/// The description of an inter-processor interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IpiRequest {
    /// The interrupt vector that the target processor handles.
    pub vector: u8,
    /// The delivery mode of the request.
    pub delivery_mode: DeliveryMode,
    /// The identifier of the target processor.
    pub destination: u8,
    /// The destination shorthand of the request.
    pub shorthand: DestinationShorthand,
}

impl IpiRequest {
    /// Creates a request for a single processor.
    #[must_use]
    pub const fn new(vector: u8, destination: u8, delivery_mode: DeliveryMode) -> Self {
        Self {
            vector,
            delivery_mode,
            destination,
            shorthand: DestinationShorthand::None,
        }
    }

    /// Encodes the low half of the interrupt command register.
    #[must_use]
    pub fn encode_low(&self) -> u32 {
        let mut value = u32::from(self.vector) & ICR_VECTOR_MASK;
        value |= (self.delivery_mode.value() << ICR_DELIVERY_MODE_SHIFT) & ICR_DELIVERY_MODE_MASK;
        value |= (self.shorthand.value() << ICR_SHORTHAND_SHIFT) & ICR_SHORTHAND_MASK;
        value
    }

    /// Encodes the high half of the interrupt command register.
    #[must_use]
    pub fn encode_high(&self) -> u32 {
        (u32::from(self.destination) << ICR_DESTINATION_SHIFT) & ICR_DESTINATION_MASK
    }
}

/// The polarity of a platform interrupt line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Polarity {
    /// The line is asserted with a high level.
    ActiveHigh,
    /// The line is asserted with a low level.
    ActiveLow,
}

impl Polarity {
    /// Returns the numeric value of the polarity.
    #[must_use]
    pub const fn value(self) -> u32 {
        match self {
            Self::ActiveHigh => 0,
            Self::ActiveLow => 1,
        }
    }

    /// Decodes a polarity from its numeric value.
    #[must_use]
    pub const fn from_value(value: u32) -> Option<Self> {
        match value & 1 {
            0 => Some(Self::ActiveHigh),
            _ => Some(Self::ActiveLow),
        }
    }
}

/// The trigger mode of a platform interrupt line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TriggerMode {
    /// The line is sampled on its rising edge.
    Edge,
    /// The line is sampled while it is asserted.
    Level,
}

impl TriggerMode {
    /// Returns the numeric value of the trigger mode.
    #[must_use]
    pub const fn value(self) -> u32 {
        match self {
            Self::Edge => 0,
            Self::Level => 1,
        }
    }

    /// Decodes a trigger mode from its numeric value.
    #[must_use]
    pub const fn from_value(value: u32) -> Option<Self> {
        match value & 1 {
            0 => Some(Self::Edge),
            _ => Some(Self::Level),
        }
    }
}

/// The way in which the destination field of a redirection entry
/// is interpreted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DestinationMode {
    /// The destination field holds the identifier of a single
    /// processor.
    Physical,
    /// The destination field holds a mask of processor
    /// identifiers.
    Logical,
}

impl DestinationMode {
    /// Returns the numeric value of the destination mode.
    #[must_use]
    pub const fn value(self) -> u32 {
        match self {
            Self::Physical => 0,
            Self::Logical => 1,
        }
    }

    /// Decodes a destination mode from its numeric value.
    #[must_use]
    pub const fn from_value(value: u32) -> Option<Self> {
        match value & 1 {
            0 => Some(Self::Physical),
            _ => Some(Self::Logical),
        }
    }
}

/// One redirection entry of the input/output controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IoApicRedirection {
    /// The interrupt vector that services the line.
    pub vector: u8,
    /// The delivery mode of the entry.
    pub delivery_mode: DeliveryMode,
    /// The way in which the destination field is interpreted.
    pub destination_mode: DestinationMode,
    /// The polarity of the line.
    pub polarity: Polarity,
    /// The trigger mode of the line.
    pub trigger_mode: TriggerMode,
    /// Whether the entry masks the line.
    pub masked: bool,
    /// The identifier or the mask of the target processors.
    pub destination: u8,
}

impl IoApicRedirection {
    /// Creates an unmasked, edge triggered entry.
    #[must_use]
    pub const fn new(vector: u8, destination: u8, delivery_mode: DeliveryMode) -> Self {
        Self {
            vector,
            delivery_mode,
            destination_mode: DestinationMode::Physical,
            polarity: Polarity::ActiveHigh,
            trigger_mode: TriggerMode::Edge,
            masked: false,
            destination,
        }
    }

    /// Encodes the entry into its 64-bit register image.
    #[must_use]
    pub fn encode(&self) -> u64 {
        let low = (u64::from(self.vector) & IO_APIC_VECTOR_MASK)
            | ((u64::from(self.delivery_mode.value()) << IO_APIC_DELIVERY_MODE_SHIFT)
                & IO_APIC_DELIVERY_MODE_MASK)
            | ((u64::from(self.destination_mode.value()) << 11) & IO_APIC_DESTINATION_MODE_MASK)
            | ((u64::from(self.polarity.value()) << 13) & IO_APIC_POLARITY_MASK)
            | ((u64::from(self.trigger_mode.value()) << 15) & IO_APIC_TRIGGER_MODE_MASK)
            | ((u64::from(u32::from(self.masked)) << 16) & IO_APIC_MASKED_MASK);
        let high =
            (u64::from(self.destination) << IO_APIC_DESTINATION_SHIFT) & IO_APIC_DESTINATION_MASK;
        low | high
    }

    /// Decodes an entry from its 64-bit register image.
    ///
    /// An unknown delivery mode decodes to
    /// [`DeliveryMode::Fixed`].
    #[must_use]
    pub fn decode(value: u64) -> Self {
        let delivery =
            u32::try_from((value & IO_APIC_DELIVERY_MODE_MASK) >> IO_APIC_DELIVERY_MODE_SHIFT)
                .unwrap_or_default();
        let destination_mode =
            u32::try_from((value & IO_APIC_DESTINATION_MODE_MASK) >> 11).unwrap_or_default();
        let polarity = u32::try_from((value & IO_APIC_POLARITY_MASK) >> 13).unwrap_or_default();
        let trigger = u32::try_from((value & IO_APIC_TRIGGER_MODE_MASK) >> 15).unwrap_or_default();
        let destination =
            u32::try_from((value & IO_APIC_DESTINATION_MASK) >> IO_APIC_DESTINATION_SHIFT)
                .unwrap_or_default();
        Self {
            vector: u8::try_from(value & IO_APIC_VECTOR_MASK).unwrap_or_default(),
            delivery_mode: DeliveryMode::from_value(delivery).unwrap_or(DeliveryMode::Fixed),
            destination_mode: DestinationMode::from_value(destination_mode)
                .unwrap_or(DestinationMode::Physical),
            polarity: Polarity::from_value(polarity).unwrap_or(Polarity::ActiveHigh),
            trigger_mode: TriggerMode::from_value(trigger).unwrap_or(TriggerMode::Edge),
            masked: value & IO_APIC_MASKED_MASK != 0,
            destination: u8::try_from(destination).unwrap_or_default(),
        }
    }
}

/// The bus, device and function that raise an interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IrqSource {
    /// The number of the bus.
    pub bus: u8,
    /// The number of the device on the bus.
    pub device: u8,
    /// The number of the function on the device.
    pub function: u8,
}

impl IrqSource {
    /// Creates a routing source.
    #[must_use]
    pub const fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
        }
    }
}

/// The mapping of a platform interrupt line to an architecture
/// interrupt vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IoInterruptVector {
    /// The platform interrupt line.
    pub irq: u8,
    /// The architecture interrupt vector that services the
    /// line.
    pub vector: u8,
    /// The device that raises the line.
    pub source: IrqSource,
}

impl IoInterruptVector {
    /// Creates a vector mapping.
    #[must_use]
    pub const fn new(irq: u8, vector: u8, source: IrqSource) -> Self {
        Self {
            irq,
            vector,
            source,
        }
    }
}

/// One entry of the interrupt routing table.
///
/// The routing table describes how the platform interrupt lines
/// are connected to the local controllers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IrqRoutingEntry {
    /// The platform interrupt line.
    pub irq: u8,
    /// The identifier of the bus that carries the line.
    pub bus_type: u8,
    /// The device that raises the line.
    pub source: IrqSource,
    /// The global system interrupt identifier of the line.
    pub global_system_interrupt: u32,
    /// The polarity of the line.
    pub polarity: Polarity,
    /// The trigger mode of the line.
    pub trigger_mode: TriggerMode,
}

impl IrqRoutingEntry {
    /// Creates a routing entry.
    #[must_use]
    pub const fn new(
        irq: u8,
        bus_type: u8,
        source: IrqSource,
        global_system_interrupt: u32,
    ) -> Self {
        Self {
            irq,
            bus_type,
            source,
            global_system_interrupt,
            polarity: Polarity::ActiveHigh,
            trigger_mode: TriggerMode::Edge,
        }
    }
}

/// A view of the local advanced programmable interrupt
/// controller.
///
/// The structure only holds the base address of the register
/// page. The register accesses are unsafe because the caller
/// must guarantee that the page is mapped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalApic {
    base: usize,
}

impl LocalApic {
    /// Creates a view of the controller at the given base
    /// address.
    #[must_use]
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    /// Returns the base address of the register page.
    #[must_use]
    pub const fn base(&self) -> usize {
        self.base
    }

    /// Returns the address of a register of the controller.
    #[must_use]
    pub const fn address(&self, offset: u32) -> usize {
        self.base.wrapping_add(offset as usize)
    }

    /// Reads the identification register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_id(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_ID)) }
    }

    /// Reads the version register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_version(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_VERSION)) }
    }

    /// Reads the task priority register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_tpr(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_TPR)) }
    }

    /// Writes the task priority register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_tpr(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_TPR), value) }
    }

    /// Signals the end of an interrupt.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn send_eoi(&self) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_EOI), 0) }
    }

    /// Reads the spurious interrupt vector register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_svr(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_SVR)) }
    }

    /// Writes the spurious interrupt vector register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_svr(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_SVR), value) }
    }

    /// Reads the low half of the interrupt command register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_icr_low(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_ICR_LOW)) }
    }

    /// Reads the high half of the interrupt command register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_icr_high(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_ICR_HIGH)) }
    }

    /// Writes the low half of the interrupt command register.
    ///
    /// The write starts the delivery of the interrupt.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_icr_low(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_ICR_LOW), value) }
    }

    /// Writes the high half of the interrupt command register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_icr_high(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_ICR_HIGH), value) }
    }

    /// Sends an inter-processor interrupt.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn send_ipi(&self, request: &IpiRequest) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe {
            write_u32(self.address(APIC_ICR_HIGH), request.encode_high());
            write_u32(self.address(APIC_ICR_LOW), request.encode_low());
        }
    }

    /// Reads the timer local vector table register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_lvt_timer(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_LVT_TIMER)) }
    }

    /// Writes the timer local vector table register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_lvt_timer(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_LVT_TIMER), value) }
    }

    /// Writes the timer initial count register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_timer_initial_count(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_TIMER_INITCNT), value) }
    }

    /// Reads the timer current count register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_timer_current_count(&self) -> u32 {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { read_u32(self.address(APIC_TIMER_CURCNT)) }
    }

    /// Writes the timer divide configuration register.
    ///
    /// # Safety
    ///
    /// The register page of the controller must be mapped.
    pub unsafe fn write_timer_divider(&self, value: u32) {
        // SAFETY: The caller guarantees that the register page
        // is mapped.
        unsafe { write_u32(self.address(APIC_TIMER_DIV), value) }
    }
}

/// A view of the input/output advanced programmable interrupt
/// controller.
///
/// The controller reaches its registers through an index and a
/// data register. The view owns the base address of the
/// register window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IoApic {
    base: usize,
}

impl IoApic {
    /// Creates a view of the controller at the given base
    /// address.
    #[must_use]
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    /// Returns the base address of the register window.
    #[must_use]
    pub const fn base(&self) -> usize {
        self.base
    }

    /// Returns the address of a register of the controller.
    #[must_use]
    pub const fn address(&self, offset: u32) -> usize {
        self.base.wrapping_add(offset as usize)
    }

    /// Selects the register that the data register addresses.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    pub unsafe fn write_index(&self, index: u32) {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { write_u32(self.address(IO_APIC_INDEX_OFFSET), index) }
    }

    /// Reads the currently selected register.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_data(&self) -> u32 {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { read_u32(self.address(IO_APIC_DATA_OFFSET)) }
    }

    /// Writes the currently selected register.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    pub unsafe fn write_data(&self, value: u32) {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { write_u32(self.address(IO_APIC_DATA_OFFSET), value) }
    }

    /// Reads a register of the controller by its index.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_register(&self, index: u32) -> u32 {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe {
            write_u32(self.address(IO_APIC_INDEX_OFFSET), index);
            read_u32(self.address(IO_APIC_DATA_OFFSET))
        }
    }

    /// Writes a register of the controller by its index.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    pub unsafe fn write_register(&self, index: u32, value: u32) {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe {
            write_u32(self.address(IO_APIC_INDEX_OFFSET), index);
            write_u32(self.address(IO_APIC_DATA_OFFSET), value);
        }
    }

    /// Reads the identification register of the controller.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_id(&self) -> u32 {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { self.read_register(IO_APIC_ID_OFFSET) }
    }

    /// Reads the version register of the controller.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_version(&self) -> u32 {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { self.read_register(IO_APIC_VERSION_OFFSET) }
    }

    /// Returns the number of redirection entries of the
    /// controller.
    ///
    /// The value is derived from the version register.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn max_redirection_entries(&self) -> u32 {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        let version = unsafe { self.read_version() };
        ((version >> 16) & 0xFF) + 1
    }

    /// Returns the index of the low half of a redirection entry.
    ///
    /// The low half of the entry for the given interrupt line
    /// uses the index `0x10 + 2 * irq`, the high half uses the
    /// next index.
    #[must_use]
    pub fn redirection_index(irq: u8) -> u32 {
        IO_APIC_REDIRECTION_INDEX + (u32::from(irq) * 2)
    }

    /// Reads the redirection entry of an interrupt line.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    #[must_use]
    pub unsafe fn read_redirection(&self, irq: u8) -> IoApicRedirection {
        let index = Self::redirection_index(irq);
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        let low = unsafe { self.read_register(index) };
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        let high = unsafe { self.read_register(index + 1) };
        let value = (u64::from(high) << 32) | u64::from(low);
        IoApicRedirection::decode(value)
    }

    /// Writes the redirection entry of an interrupt line.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    pub unsafe fn write_redirection(&self, irq: u8, entry: &IoApicRedirection) {
        let value = entry.encode();
        let index = Self::redirection_index(irq);
        let low = u32::try_from(value).unwrap_or_default();
        let high = u32::try_from(value >> 32).unwrap_or_default();
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe {
            self.write_register(index, low);
            self.write_register(index + 1, high);
        }
    }

    /// Programs the redirection entry of an interrupt line.
    ///
    /// # Safety
    ///
    /// The register window of the controller must be mapped.
    pub unsafe fn set_redirection(&self, irq: u8, entry: &IoApicRedirection) {
        // SAFETY: The caller guarantees that the register window
        // is mapped.
        unsafe { self.write_redirection(irq, entry) }
    }
}
