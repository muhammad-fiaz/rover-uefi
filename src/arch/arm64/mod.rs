//! # The 64-bit ARM architecture
//!
//! The register layouts of the generic interrupt controller and
//! the register images of a taken exception.
//!
//! * `gic` describes the distributor and the processor interface
//!   of the generic interrupt controller.
//! * `context` describes the exception context and the system
//!   registers that hold it.

pub mod context;
pub mod gic;
pub mod psci;
pub mod smccc;

pub use psci::{
    PSCI_AFFINITY_INFO_SMC32, PSCI_AFFINITY_INFO_SMC64, PSCI_AFFINITY_OFF, PSCI_AFFINITY_ON,
    PSCI_AFFINITY_ON_PENDING, PSCI_CPU_OFF, PSCI_CPU_ON_SMC32, PSCI_CPU_ON_SMC64,
    PSCI_CPU_SUSPEND_SMC32, PSCI_CPU_SUSPEND_SMC64, PSCI_FEATURES, PSCI_MIGRATE_SMC32,
    PSCI_MIGRATE_SMC64, PSCI_SYSTEM_OFF, PSCI_SYSTEM_RESET, PSCI_SYSTEM_RESET2_SMC64, PSCI_VERSION,
    PsciStatus, parse_psci_version, psci_target_cpu64,
};
pub use smccc::{
    SMCCC_ARCH_FEATURES, SMCCC_ARCH_SOC_ID, SMCCC_ARCH_WORKAROUND_1, SMCCC_ARCH_WORKAROUND_2,
    SMCCC_FAST_CALL_MASK, SMCCC_FUNC_NUM_MASK, SMCCC_OWNER_MASK, SMCCC_RET_INVALID_PARAMETER,
    SMCCC_RET_NOT_REQUIRED, SMCCC_RET_NOT_SUPPORTED, SMCCC_RET_SUCCESS, SMCCC_SMC64_MASK,
    SMCCC_VERSION, SmcccFunctionId, SmcccOwner, SmcccResult,
};

pub use context::{
    ELR_EL1, ESR_EL1, EXCEPTION_VECTOR_COUNT, EXCEPTION_VECTOR_ENTRY_SIZE, ExceptionClass,
    ExceptionContext, ExceptionSource, FAR_EL1, PAR_EL1, SP_EL0, SP_EL1, SP_ELX, SPSR_EL1,
    TPIDR_EL0, TPIDRRO_EL0, VBAR_EL1, exception_vector_index, exception_vector_offset,
};
pub use gic::{
    GICC_BPR, GICC_CTLR, GICC_DIR, GICC_EOIR, GICC_HPPIR, GICC_IAR, GICC_IIDR, GICC_NSAPR,
    GICC_PMR, GICC_RPR, GICD_CTLR, GICD_ICENABLER, GICD_ICFGR_V2, GICD_ICFGR_V3, GICD_ICPENDR,
    GICD_IGROUPR, GICD_IPRIORITYR, GICD_ISENABLER, GICD_ISPENDR, GICD_ITARGETSR, GICD_SGIR,
    GICD_TYPER, GicCpuInterface, GicDistributor, GicVersion, ICC_BPR1_EL1, ICC_CTLR_EL1,
    ICC_DIR_EL1, ICC_EOIR_EL1, ICC_HPPIR_EL1, ICC_IGRPEN1_EL1, ICC_PRIO_MASK_EL1, ICC_RPR_EL1,
    ICC_SGI0R_EL1, ICC_SGI1R_EL1, ICC_SRE_EL1, ICC_SRE_EL2,
};

/// The encoding of a system register.
///
/// The fields select the register inside the system register
/// space of the architecture. The same encoding describes the
/// exception registers and the registers of the interrupt
/// controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemRegister {
    /// The opcode field of the encoding.
    pub op0: u32,
    /// The first operand field of the encoding.
    pub op1: u32,
    /// The number of the system register file.
    pub crn: u32,
    /// The number of the register inside the file.
    pub crm: u32,
    /// The second operand field of the encoding.
    pub op2: u32,
}

impl SystemRegister {
    /// Creates an encoding from its five fields.
    #[must_use]
    pub fn new(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> Self {
        Self {
            op0: u32::from(op0),
            op1: u32::from(op1),
            crn: u32::from(crn),
            crm: u32::from(crm),
            op2: u32::from(op2),
        }
    }

    /// Returns the numeric encoding of the system register.
    #[must_use]
    pub const fn encode(self) -> u32 {
        (self.op0 << 19) | (self.op1 << 16) | (self.crn << 12) | (self.crm << 8) | (self.op2 << 5)
    }
}
