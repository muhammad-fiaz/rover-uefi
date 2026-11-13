//! # PEI PPI Modules
//!
//! Pre-EFI Initialization (PEI) Program and Interface (PPI) definitions.

pub mod arm_mp_core;
pub mod core_ppis;

pub use arm_mp_core::{
    ARM_MP_CORE_INFO_GUID, ARM_MP_CORE_INFO_PPI_GUID, ArmCoreInfo, ArmMpCoreInfoGet,
    ArmMpCoreInfoPpi, MAX_CPUS_PER_MPCORE_SYSTEM, MPIDR_U_BIT_MASK, SCU_CONFIG_REG_OFFSET,
};
pub use core_ppis::*;
