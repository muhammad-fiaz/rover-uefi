//! # ARM Multi-Core Info PPI and Data Structures
//!
//! Provides the early Multi-Processor Core Information PPI and mailbox descriptors
//! used during early PEI / SEC phases on multi-core ARM systems (e.g. Cortex-A, Neoverse,
//! Qualcomm Snapdragon, Apple Silicon, Rockchip, and QEMU virt).
//!
//! Defined in EDK II `ArmPkg/Include/Ppi/ArmMpCoreInfo.h` and `ArmPkg/Include/Guid/ArmMpCoreInfo.h`.

use crate::base::{EfiGuid, EfiStatus};

/// ARM MP Core Info PPI GUID.
pub const ARM_MP_CORE_INFO_PPI_GUID: EfiGuid = EfiGuid::new(
    0x6847_CC74,
    0xE9EC,
    0x4F8F,
    [0xA2, 0x9D, 0xAB, 0x44, 0xE7, 0x54, 0xA8, 0xFC],
);

/// ARM MP Core Info Descriptor GUID.
pub const ARM_MP_CORE_INFO_GUID: EfiGuid = EfiGuid::new(
    0xA4EE_0728,
    0xE5D7,
    0x4AC5,
    [0xB2, 0x1E, 0x65, 0x8E, 0xD8, 0x57, 0xE8, 0x34],
);

/// Maximum CPUs per `MPCore` cluster.
pub const MAX_CPUS_PER_MPCORE_SYSTEM: usize = 0x04;

/// SCU configuration register offset.
pub const SCU_CONFIG_REG_OFFSET: usize = 0x04;

/// Bitmask for MPIDR Uniprocessor (U) bit.
pub const MPIDR_U_BIT_MASK: u64 = 0x4000_0000;

/// Information for a single ARM processor core.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArmCoreInfo {
    /// Multiprocessor Affinity Register (MPIDR).
    pub mpidr: u64,
    /// Mailbox register to set/signal the secondary core.
    pub mailbox_set_address: u64,
    /// Mailbox register to read the status from the secondary core.
    pub mailbox_get_address: u64,
    /// Mailbox register to clear the signal.
    pub mailbox_clear_address: u64,
    /// Value to write when clearing mailbox.
    pub mailbox_clear_value: u64,
}

/// Function pointer to get MP core information.
pub type ArmMpCoreInfoGet = unsafe extern "efiapi" fn(
    arm_core_count: *mut usize,
    arm_core_table: *mut *const ArmCoreInfo,
) -> EfiStatus;

/// ARM MP Core Information PPI.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ArmMpCoreInfoPpi {
    /// Retrieves array of `ArmCoreInfo` entries for all cores in the `SoC`.
    pub get_mp_core_info: Option<ArmMpCoreInfoGet>,
}
