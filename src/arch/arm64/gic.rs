//! # Generic interrupt controller
//!
//! The register layout of the generic interrupt controller for
//! the 64-bit architecture.
//!
//! The controller has three parts. The *distributor* holds the
//! state of the shared peripheral interrupts. The *processor
//! interface* accepts and completes the interrupts of a single
//! processor. The *redistributor* belongs to the third
//! generation of the controller and serves the private
//! peripheral interrupts of one processor.
//!
//! The first generation of the controller reaches its processor
//! interface through memory mapped registers. The third
//! generation reaches it through system registers, so the
//! module describes both: the memory mapped offsets of the
//! second generation and the encodings of the third generation
//! system registers.
//!
//! The view structures own the base address of a register frame
//! and provide the accesses. Every accessor that touches the
//! hardware is an unsafe function because the caller must
//! guarantee that the frame is mapped.

use crate::arch::common::mmio::{read_u32, write_u32};

use super::SystemRegister;

/// The offset of the distributor control register.
pub const GICD_CTLR: u32 = 0x000;

/// The offset of the distributor type register.
pub const GICD_TYPER: u32 = 0x004;

/// The offset of the distributor implementer identification
/// register.
pub const GICD_IIDR: u32 = 0x008;

/// The offset of the distributor interrupt group register.
pub const GICD_IGROUPR: u32 = 0x080;

/// The offset of the distributor set-enable register.
pub const GICD_ISENABLER: u32 = 0x100;

/// The offset of the distributor clear-enable register.
pub const GICD_ICENABLER: u32 = 0x180;

/// The offset of the distributor set-pending register.
pub const GICD_ISPENDR: u32 = 0x200;

/// The offset of the distributor clear-pending register.
pub const GICD_ICPENDR: u32 = 0x280;

/// The offset of the distributor set-active register.
pub const GICD_ISACTIVER: u32 = 0x300;

/// The offset of the distributor clear-active register.
pub const GICD_ICACTIVER: u32 = 0x380;

/// The offset of the distributor interrupt priority register.
pub const GICD_IPRIORITYR: u32 = 0x400;

/// The offset of the distributor interrupt processor target
/// register of the second generation.
pub const GICD_ITARGETSR: u32 = 0x800;

/// The offset of the distributor interrupt configuration
/// register of the second generation.
pub const GICD_ICFGR_V2: u32 = 0xC00;

/// The offset of the distributor interrupt configuration
/// register of the third generation.
pub const GICD_ICFGR_V3: u32 = 0x800;

/// The offset of the distributor interrupt group modifier
/// register.
pub const GICD_IGRPMODR: u32 = 0xD00;

/// The offset of the distributor non-secure access control
/// register.
pub const GICD_NSACR: u32 = 0xE00;

/// The offset of the distributor software generated interrupt
/// register.
pub const GICD_SGIR: u32 = 0xF00;

/// The offset of the distributor clear pending software generated
/// interrupt register.
pub const GICD_CPENDSGIR: u32 = 0xF10;

/// The offset of the distributor set pending software generated
/// interrupt register.
pub const GICD_SPENDSGIR: u32 = 0xF20;

/// The offset of the distributor status register of the third
/// generation.
pub const GICD_STATUSR: u32 = 0x010;

/// The offset of the distributor type register of the third
/// generation.
pub const GICD_TYPER2: u32 = 0x00C;

/// The offset of the distributor interrupt routing register of
/// the third generation.
pub const GICD_IROUTER: u32 = 0x6000;

/// The offset of the processor interface control register.
pub const GICC_CTLR: u32 = 0x000;

/// The offset of the processor interface priority mask register.
pub const GICC_PMR: u32 = 0x004;

/// The offset of the processor interface binary point register.
pub const GICC_BPR: u32 = 0x008;

/// The offset of the processor interface interrupt acknowledge
/// register.
pub const GICC_IAR: u32 = 0x00C;

/// The offset of the processor interface end of interrupt
/// register.
pub const GICC_EOIR: u32 = 0x010;

/// The offset of the processor interface running priority
/// register.
pub const GICC_RPR: u32 = 0x014;

/// The offset of the processor interface highest priority pending
/// interrupt register.
pub const GICC_HPPIR: u32 = 0x018;

/// The offset of the processor interface active priority
/// register.
pub const GICC_APR: u32 = 0x0D0;

/// The offset of the processor interface non-secure active
/// priority register.
pub const GICC_NSAPR: u32 = 0x0E0;

/// The offset of the processor interface identification
/// register.
pub const GICC_IIDR: u32 = 0x0FC;

/// The offset of the processor interface deactivate interrupt
/// register.
pub const GICC_DIR: u32 = 0x1000;

/// The offset of the legacy memory mapped system register enable
/// register of the first generation processor interface.
///
/// The second generation uses the offset for the running priority
/// register and the third generation replaces the register with
/// the `ICC_SRE_EL1` system register.
pub const GICC_SRE_INT: u32 = 0x014;

/// The offset of the legacy memory mapped software generated
/// interrupt register of the first generation processor
/// interface.
///
/// The third generation replaces the register with the
/// `ICC_SGI1R_EL1` system register.
pub const GICC_SGI_INT: u32 = 0x018;

/// The number of interrupts that a distributor register covers.
pub const GIC_INTERRUPTS_PER_REGISTER: u32 = 32;

/// The number of interrupts that a configuration register covers.
pub const GIC_INTERRUPTS_PER_CONFIGURATION: u32 = 16;

/// The number of interrupt priorities that a priority register
/// covers.
pub const GIC_PRIORITIES_PER_REGISTER: u32 = 4;

/// The group of an interrupt that the secure state handles.
pub const GIC_GROUP_0: u32 = 0;

/// The group of an interrupt that the non-secure state handles.
pub const GIC_GROUP_1: u32 = 1;

/// The interrupt identifier that the controller reports when no
/// interrupt is pending.
pub const GIC_SPURIOUS_INTERRUPT: u32 = 1023;

/// The identifier of the group zero interrupt.
pub const GIC_INTERRUPT_GROUP_0: u32 = 1020;

/// The identifier of the group one interrupt.
pub const GIC_INTERRUPT_GROUP_1: u32 = 1021;

/// The identifier of the non-maskable interrupt.
pub const GIC_INTERRUPT_NMI: u32 = 1022;

/// The generation of the generic interrupt controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GicVersion {
    /// The second generation of the controller.
    Second,
    /// The third generation of the controller.
    Third,
}

/// The priority mask system register of the third generation
/// processor interface.
pub const ICC_PRIO_MASK_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 4,
    crm: 0,
    op2: 2,
};

/// The end of interrupt system register of the third generation
/// processor interface.
pub const ICC_EOIR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 0,
};

/// The deactivate interrupt system register of the third
/// generation processor interface.
pub const ICC_DIR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 1,
};

/// The system register enable system register of the secure
/// state of the third generation processor interface.
pub const ICC_SRE_EL2: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 2,
};

/// The system register enable system register of the third
/// generation processor interface.
pub const ICC_SRE_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 3,
};

/// The group zero software generated interrupt system register.
pub const ICC_SGI0R_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 4,
};

/// The group one software generated interrupt system register.
pub const ICC_SGI1R_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 1,
    op2: 5,
};

/// The highest priority pending interrupt system register of the
/// third generation processor interface.
pub const ICC_HPPIR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 2,
    op2: 0,
};

/// The running priority system register of the third generation
/// processor interface.
pub const ICC_RPR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 2,
    op2: 1,
};

/// The binary point system register of the third generation
/// processor interface.
pub const ICC_BPR1_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 3,
    op2: 3,
};

/// The group one interrupt enable system register of the third
/// generation processor interface.
pub const ICC_IGRPEN1_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 12,
    op2: 7,
};

/// The control system register of the third generation processor
/// interface.
pub const ICC_CTLR_EL1: SystemRegister = SystemRegister {
    op0: 3,
    op1: 0,
    crn: 12,
    crm: 12,
    op2: 12,
};

/// A view of the distributor of the generic interrupt
/// controller.
///
/// The structure owns the base address of the distributor frame
/// and the generation of the controller, because the two
/// generations disagree about the position of the configuration
/// registers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GicDistributor {
    base: usize,
    version: GicVersion,
}

impl GicDistributor {
    /// Creates a view of the distributor at the given base
    /// address.
    #[must_use]
    pub const fn new(base: usize, version: GicVersion) -> Self {
        Self { base, version }
    }

    /// Returns the base address of the distributor frame.
    #[must_use]
    pub const fn base(&self) -> usize {
        self.base
    }

    /// Returns the generation of the controller.
    #[must_use]
    pub const fn version(&self) -> GicVersion {
        self.version
    }

    /// Returns the address of a register of the distributor.
    #[must_use]
    pub const fn address(&self, offset: u32) -> usize {
        self.base.wrapping_add(offset as usize)
    }

    /// Returns the offset of the configuration registers.
    #[must_use]
    pub const fn configuration_offset(&self) -> u32 {
        match self.version {
            GicVersion::Second => GICD_ICFGR_V2,
            GicVersion::Third => GICD_ICFGR_V3,
        }
    }

    /// Reads the control register.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_control(&self) -> u32 {
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { read_u32(self.address(GICD_CTLR)) }
    }

    /// Writes the control register.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn write_control(&self, value: u32) {
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(GICD_CTLR), value) }
    }

    /// Reads the type register.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_type(&self) -> u32 {
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { read_u32(self.address(GICD_TYPER)) }
    }

    /// Reads the type register of the third generation.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_type_extended(&self) -> u32 {
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { read_u32(self.address(GICD_TYPER2)) }
    }

    /// Reads the status register.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_status(&self) -> u32 {
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { read_u32(self.address(GICD_STATUSR)) }
    }

    /// Enables an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn enable_interrupt(&self, interrupt: u32) {
        let (offset, bit) = Self::register_and_bit(GICD_ISENABLER, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), 1 << bit) }
    }

    /// Disables an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn disable_interrupt(&self, interrupt: u32) {
        let (offset, bit) = Self::register_and_bit(GICD_ICENABLER, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), 1 << bit) }
    }

    /// Marks an interrupt as pending.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn set_pending(&self, interrupt: u32) {
        let (offset, bit) = Self::register_and_bit(GICD_ISPENDR, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), 1 << bit) }
    }

    /// Clears the pending state of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn clear_pending(&self, interrupt: u32) {
        let (offset, bit) = Self::register_and_bit(GICD_ICPENDR, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), 1 << bit) }
    }

    /// Reads the priority of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_priority(&self, interrupt: u32) -> u8 {
        let index = interrupt / GIC_PRIORITIES_PER_REGISTER;
        let shift = (interrupt % GIC_PRIORITIES_PER_REGISTER) * 8;
        let offset = GICD_IPRIORITYR + index * 4;
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        u8::try_from((value >> shift) & 0xFF).unwrap_or_default()
    }

    /// Writes the priority of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn write_priority(&self, interrupt: u32, priority: u8) {
        let index = interrupt / GIC_PRIORITIES_PER_REGISTER;
        let shift = (interrupt % GIC_PRIORITIES_PER_REGISTER) * 8;
        let offset = GICD_IPRIORITYR + index * 4;
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        let mask = 0xFFu32 << shift;
        let updated = (value & !mask) | ((u32::from(priority) << shift) & mask);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), updated) }
    }

    /// Reads the group of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_group(&self, interrupt: u32) -> u32 {
        let (offset, bit) = Self::register_and_bit(GICD_IGROUPR, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        (value >> bit) & 1
    }

    /// Writes the group of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn write_group(&self, interrupt: u32, group: u32) {
        let (offset, bit) = Self::register_and_bit(GICD_IGROUPR, interrupt);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        let mask = 1u32 << bit;
        let updated = (value & !mask) | ((group & 1) << bit);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), updated) }
    }

    /// Reads the configuration of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    #[must_use]
    pub unsafe fn read_configuration(&self, interrupt: u32) -> u32 {
        let index = interrupt / GIC_INTERRUPTS_PER_CONFIGURATION;
        let shift = (interrupt % GIC_INTERRUPTS_PER_CONFIGURATION) * 2;
        let offset = self.configuration_offset() + index * 4;
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        (value >> shift) & 0x03
    }

    /// Writes the configuration of an interrupt.
    ///
    /// # Safety
    ///
    /// The distributor frame must be mapped.
    pub unsafe fn write_configuration(&self, interrupt: u32, configuration: u32) {
        let index = interrupt / GIC_INTERRUPTS_PER_CONFIGURATION;
        let shift = (interrupt % GIC_INTERRUPTS_PER_CONFIGURATION) * 2;
        let offset = self.configuration_offset() + index * 4;
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        let value = unsafe { read_u32(self.address(offset)) };
        let mask = 0x03u32 << shift;
        let updated = (value & !mask) | ((configuration << shift) & mask);
        // SAFETY: The caller guarantees that the distributor frame
        // is mapped.
        unsafe { write_u32(self.address(offset), updated) }
    }

    /// Returns the offset of a register and the bit of an
    /// interrupt inside that register.
    #[must_use]
    pub const fn register_and_bit(base: u32, interrupt: u32) -> (u32, u32) {
        let index = interrupt / GIC_INTERRUPTS_PER_REGISTER;
        let bit = interrupt % GIC_INTERRUPTS_PER_REGISTER;
        (base + index * 4, bit)
    }
}

/// A view of the memory mapped processor interface of the second
/// generation generic interrupt controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GicCpuInterface {
    base: usize,
}

impl GicCpuInterface {
    /// Creates a view of the processor interface at the given base
    /// address.
    #[must_use]
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    /// Returns the base address of the processor interface frame.
    #[must_use]
    pub const fn base(&self) -> usize {
        self.base
    }

    /// Returns the address of a register of the processor
    /// interface.
    #[must_use]
    pub const fn address(&self, offset: u32) -> usize {
        self.base.wrapping_add(offset as usize)
    }

    /// Reads the control register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_control(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_CTLR)) }
    }

    /// Writes the control register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn write_control(&self, value: u32) {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(GICC_CTLR), value) }
    }

    /// Reads the priority mask register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_priority_mask(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_PMR)) }
    }

    /// Writes the priority mask register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn write_priority_mask(&self, value: u32) {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(GICC_PMR), value) }
    }

    /// Reads the binary point register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_binary_point(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_BPR)) }
    }

    /// Writes the binary point register.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn write_binary_point(&self, value: u32) {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(GICC_BPR), value) }
    }

    /// Acknowledges the highest priority pending interrupt.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn acknowledge_interrupt(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_IAR)) }
    }

    /// Signals the end of an interrupt.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn end_of_interrupt(&self, interrupt: u32) {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(GICC_EOIR), interrupt) }
    }

    /// Deactivates an interrupt.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn deactivate_interrupt(&self, interrupt: u32) {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(GICC_DIR), interrupt) }
    }

    /// Reads the running priority.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_running_priority(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_RPR)) }
    }

    /// Reads the highest priority pending interrupt.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_highest_pending(&self) -> u32 {
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(GICC_HPPIR)) }
    }

    /// Reads the active priority of a priority level.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_active_priority(&self, level: u32) -> u32 {
        let offset = GICC_APR + level * 4;
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(offset)) }
    }

    /// Writes the active priority of a priority level.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    pub unsafe fn write_active_priority(&self, level: u32, value: u32) {
        let offset = GICC_APR + level * 4;
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { write_u32(self.address(offset), value) }
    }

    /// Reads the non-secure active priority of a priority level.
    ///
    /// # Safety
    ///
    /// The processor interface frame must be mapped.
    #[must_use]
    pub unsafe fn read_non_secure_active_priority(&self, level: u32) -> u32 {
        let offset = GICC_NSAPR + level * 4;
        // SAFETY: The caller guarantees that the frame is mapped.
        unsafe { read_u32(self.address(offset)) }
    }
}
