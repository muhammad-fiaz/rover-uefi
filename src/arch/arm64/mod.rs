//! # The 64-bit ARM architecture
//!
//! The register layouts of the generic interrupt controller and
//! the register images of a taken exception.
//!
//! * `gic` describes the distributor and the processor interface
//!   of the generic interrupt controller.
//! * `context` describes the exception context and the system
//!   registers that hold it.

pub mod cache;
pub mod context;
pub mod ffa;
pub mod gic;
pub mod mmu;
pub mod pl011;
pub mod psci;
pub mod smccc;
pub mod sysreg;

pub use ffa::{
    ARM_FFA_DESTINATION_ENDPOINT_ID, ARM_FFA_MAJOR_VERSION, ARM_FFA_MEM_PERM_CODE_X,
    ARM_FFA_MEM_PERM_CODE_XN, ARM_FFA_MEM_PERM_DATA_NO_ACCESS, ARM_FFA_MEM_PERM_DATA_RO,
    ARM_FFA_MEM_PERM_DATA_RW, ARM_FFA_MINOR_VERSION, ARM_FID_FFA_CONSOLE_LOG_64, ARM_FID_FFA_ERROR,
    ARM_FID_FFA_FEATURES, ARM_FID_FFA_ID_GET, ARM_FID_FFA_INTERRUPT, ARM_FID_FFA_MEM_DONATE_64,
    ARM_FID_FFA_MEM_LEND_64, ARM_FID_FFA_MEM_PERM_GET_64, ARM_FID_FFA_MEM_PERM_SET_64,
    ARM_FID_FFA_MEM_RETRIEVE_RECLAIM, ARM_FID_FFA_MEM_RETRIEVE_RELINQUISH,
    ARM_FID_FFA_MEM_RETRIEVE_REQ_64, ARM_FID_FFA_MEM_RETRIEVE_RESP, ARM_FID_FFA_MEM_SHARE_64,
    ARM_FID_FFA_MSG_SEND_DIRECT_REQ_32, ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64,
    ARM_FID_FFA_MSG_SEND_DIRECT_RESP_32, ARM_FID_FFA_MSG_SEND_DIRECT_RESP_64,
    ARM_FID_FFA_PARTITION_INFO_GET, ARM_FID_FFA_PARTITION_INFO_GET_REGS, ARM_FID_FFA_RX_ACQUIRE,
    ARM_FID_FFA_RX_RELEASE, ARM_FID_FFA_RXTX_MAP_64, ARM_FID_FFA_RXTX_UNMAP,
    ARM_FID_FFA_SPM_ID_GET, ARM_FID_FFA_SUCCESS_32, ARM_FID_FFA_SUCCESS_64, ARM_FID_FFA_VERSION,
    FfaStatus, ffa_dest_partition_id, ffa_hvc_call, ffa_id_get, ffa_is_compatible,
    ffa_is_secure_partition, ffa_make_mem_perm, ffa_make_version, ffa_mem_perm_get,
    ffa_mem_perm_set, ffa_msg_send_direct_req64, ffa_pack_partition_ids, ffa_smc_call,
    ffa_source_partition_id, ffa_version, ffa_version_major, ffa_version_minor,
};
pub use mmu::{
    ARM64_BLOCK_SIZE_1G, ARM64_BLOCK_SIZE_2M, ARM64_ENTRIES_PER_TABLE, ARM64_PAGE_SIZE_4K,
    ArmDescriptorFlags, ArmTableDescriptor, MAIR_ATTR_DEVICE_NGNRE, MAIR_ATTR_DEVICE_NGNRNE,
    MAIR_ATTR_NORMAL_NON_CACHEABLE, MAIR_ATTR_NORMAL_WRITE_BACK, MairEl1, TcrEl1, read_mair_el1,
    read_tcr_el1, read_ttbr0_el1, tlbi_vaae1, tlbi_vmalle1, write_mair_el1, write_tcr_el1,
    write_ttbr0_el1,
};
pub use sysreg::{
    CpacrEl1, CurrentEl, Daif, MidrEl1, MpidrEl1, SctlrEl1, enable_fp_simd, read_cpacr_el1,
    read_current_el, read_daif, read_midr_el1, read_mpidr_el1, read_sctlr_el1, read_vbar_el1,
    write_sctlr_el1, write_vbar_el1,
};

pub use pl011::{
    Pl011BaudDivisors, Pl011Uart, compute_pl011_divisors, cr as pl011_cr, fr as pl011_fr,
    lcr_h as pl011_lcr_h, reg as pl011_reg,
};
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
