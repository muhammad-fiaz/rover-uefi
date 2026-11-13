//! # ARM Power State Coordination Interface (PSCI)
//!
//! Provides hardware-level CPU core power management, multi-core initialization,
//! affinity state queries, and platform shutdown/reset services compliant with
//! the ARM PSCI v0.2, v1.0, and v1.1 specifications.

/// Query PSCI interface version (`PSCI_VERSION`).
pub const PSCI_VERSION: u32 = 0x8400_0000;

/// Suspend CPU core in 32-bit mode (`CPU_SUSPEND`).
pub const PSCI_CPU_SUSPEND_SMC32: u32 = 0x8400_0001;
/// Suspend CPU core in 64-bit mode (`CPU_SUSPEND`).
pub const PSCI_CPU_SUSPEND_SMC64: u32 = 0xC400_0001;

/// Power off current calling CPU core (`CPU_OFF`).
pub const PSCI_CPU_OFF: u32 = 0x8400_0002;

/// Power on target CPU core in 32-bit mode (`CPU_ON`).
pub const PSCI_CPU_ON_SMC32: u32 = 0x8400_0003;
/// Power on target CPU core in 64-bit mode (`CPU_ON`).
pub const PSCI_CPU_ON_SMC64: u32 = 0xC400_0003;

/// Query affinity state of target core in 32-bit mode (`AFFINITY_INFO`).
pub const PSCI_AFFINITY_INFO_SMC32: u32 = 0x8400_0004;
/// Query affinity state of target core in 64-bit mode (`AFFINITY_INFO`).
pub const PSCI_AFFINITY_INFO_SMC64: u32 = 0xC400_0004;

/// Migrate OS context to target core in 32-bit mode (`MIGRATE`).
pub const PSCI_MIGRATE_SMC32: u32 = 0x8400_0005;
/// Migrate OS context to target core in 64-bit mode (`MIGRATE`).
pub const PSCI_MIGRATE_SMC64: u32 = 0xC400_0005;

/// Power down system (`SYSTEM_OFF`).
pub const PSCI_SYSTEM_OFF: u32 = 0x8400_0008;

/// Cold system reset (`SYSTEM_RESET`).
pub const PSCI_SYSTEM_RESET: u32 = 0x8400_0009;

/// Query supported PSCI features (`PSCI_FEATURES`).
pub const PSCI_FEATURES: u32 = 0x8400_000A;

/// Warm/architectural system reset in 64-bit mode (`SYSTEM_RESET2`).
pub const PSCI_SYSTEM_RESET2_SMC64: u32 = 0xC400_0012;

/// Core affinity state: core is online and powered on.
pub const PSCI_AFFINITY_ON: u32 = 0;
/// Core affinity state: core is powered off.
pub const PSCI_AFFINITY_OFF: u32 = 1;
/// Core affinity state: core power-on transition is pending.
pub const PSCI_AFFINITY_ON_PENDING: u32 = 2;

/// PSCI return error codes.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PsciStatus {
    /// Operation completed successfully.
    Success = 0,
    /// Function or feature not supported by firmware.
    NotSupported = -1,
    /// Invalid parameters passed to function.
    InvalidParameters = -2,
    /// Access denied or insufficient privilege.
    Denied = -3,
    /// Target CPU is already powered on.
    AlreadyOn = -4,
    /// Target CPU power-on is already pending.
    OnPending = -5,
    /// Internal firmware failure occurred.
    InternalFailure = -6,
    /// Target CPU is not present or valid on this platform.
    NotPresent = -7,
    /// Target CPU is disabled.
    Disabled = -8,
    /// Invalid physical or virtual address supplied.
    InvalidAddress = -9,
}

impl PsciStatus {
    /// Convert an integer return code into a `PsciStatus`.
    #[must_use]
    pub const fn from_raw(code: i64) -> Self {
        match code {
            0 => Self::Success,
            -1 => Self::NotSupported,
            -2 => Self::InvalidParameters,
            -3 => Self::Denied,
            -4 => Self::AlreadyOn,
            -5 => Self::OnPending,
            -6 => Self::InternalFailure,
            -7 => Self::NotPresent,
            -8 => Self::Disabled,
            _ => Self::InvalidAddress,
        }
    }
}

/// Pack 64-bit ARM MPIDR affinity fields into a PSCI target CPU identifier.
#[must_use]
pub const fn psci_target_cpu64(aff3: u8, aff2: u8, aff1: u8, aff0: u8) -> u64 {
    ((aff3 as u64) << 32) | ((aff2 as u64) << 16) | ((aff1 as u64) << 8) | (aff0 as u64)
}

/// Decode raw PSCI version register into `(major, minor)`.
#[must_use]
pub const fn parse_psci_version(raw: u32) -> (u16, u16) {
    let major = (raw >> 16) as u16;
    let minor = (raw & 0xFFFF) as u16;
    (major, minor)
}
