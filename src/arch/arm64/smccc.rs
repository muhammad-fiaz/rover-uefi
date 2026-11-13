//! # ARM Secure Monitor Call Calling Convention (SMCCC)
//!
//! Provides function ID encoding, service owner ranges, calling convention
//! types, and return value parsing compliant with the ARM SMCCC v1.2+ standard.

/// Fast call flag bit (bit 31).
pub const SMCCC_FAST_CALL_MASK: u32 = 0x8000_0000;
/// 64-bit call calling convention bit (bit 30: 1 = SMC64/HVC64, 0 = SMC32/HVC32).
pub const SMCCC_SMC64_MASK: u32 = 0x4000_0000;
/// Service Owner mask (bits 29:24).
pub const SMCCC_OWNER_MASK: u32 = 0x3F00_0000;
/// Function Number mask (bits 15:0).
pub const SMCCC_FUNC_NUM_MASK: u32 = 0x0000_FFFF;

/// Standard SMCCC Architecture Calls.
pub const SMCCC_VERSION: u32 = 0x8000_0000;
/// Architecture Features discovery.
pub const SMCCC_ARCH_FEATURES: u32 = 0x8000_0001;
/// Architecture `SoC` ID.
pub const SMCCC_ARCH_SOC_ID: u32 = 0x8000_0002;
/// Workaround 1 (Spectre variant 2 mitigation).
pub const SMCCC_ARCH_WORKAROUND_1: u32 = 0x8000_8000;
/// Workaround 2 (Speculative Store Bypass mitigation).
pub const SMCCC_ARCH_WORKAROUND_2: u32 = 0x8000_7FFF;

/// Standard return code: Call succeeded.
pub const SMCCC_RET_SUCCESS: i64 = 0;
/// Standard return code: Call not supported.
pub const SMCCC_RET_NOT_SUPPORTED: i64 = -1;
/// Standard return code: Call not required.
pub const SMCCC_RET_NOT_REQUIRED: i64 = -2;
/// Standard return code: Invalid parameters passed to call.
pub const SMCCC_RET_INVALID_PARAMETER: i64 = -3;

/// SMCCC Service Entity Owner range.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmcccOwner {
    /// ARM Architecture Calls (0).
    ArmArchitecture = 0,
    /// CPU Service Calls (1).
    CpuService = 1,
    /// Silicon Partner (`SiP`) Calls (2).
    SiP = 2,
    /// Original Equipment Manufacturer (OEM) Calls (3).
    Oem = 3,
    /// Standard Secure Service Calls (4, e.g. PSCI).
    StandardSecure = 4,
    /// Standard Hypervisor Service Calls (5).
    StandardHypervisor = 5,
    /// Vendor Specific Hypervisor Calls (6).
    VendorHypervisor = 6,
    /// Other or implementation-defined owner.
    Other(u8),
}

/// Parsed ARM SMCCC Function Identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmcccFunctionId {
    /// Whether this is a fast call (cannot be preempted).
    pub is_fast_call: bool,
    /// Whether this call uses the 64-bit calling convention (`SMC64`/`HVC64`).
    pub is_smc64: bool,
    /// The service owner entity.
    pub owner: SmcccOwner,
    /// The 16-bit function number within the service owner range.
    pub function_number: u16,
}

impl SmcccFunctionId {
    /// Parse a 32-bit raw SMCCC Function ID into its structured components.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        let is_fast_call = (raw & SMCCC_FAST_CALL_MASK) != 0;
        let is_smc64 = (raw & SMCCC_SMC64_MASK) != 0;
        let owner_num = ((raw & SMCCC_OWNER_MASK) >> 24) as u8;
        let owner = match owner_num {
            0 => SmcccOwner::ArmArchitecture,
            1 => SmcccOwner::CpuService,
            2 => SmcccOwner::SiP,
            3 => SmcccOwner::Oem,
            4 => SmcccOwner::StandardSecure,
            5 => SmcccOwner::StandardHypervisor,
            6 => SmcccOwner::VendorHypervisor,
            other => SmcccOwner::Other(other),
        };
        let function_number = (raw & SMCCC_FUNC_NUM_MASK) as u16;
        Self {
            is_fast_call,
            is_smc64,
            owner,
            function_number,
        }
    }

    /// Convert this structured Function ID into its 32-bit raw representation.
    #[must_use]
    pub const fn to_raw(&self) -> u32 {
        let fast = if self.is_fast_call {
            SMCCC_FAST_CALL_MASK
        } else {
            0
        };
        let smc64 = if self.is_smc64 { SMCCC_SMC64_MASK } else { 0 };
        let owner_num = match self.owner {
            SmcccOwner::ArmArchitecture => 0,
            SmcccOwner::CpuService => 1,
            SmcccOwner::SiP => 2,
            SmcccOwner::Oem => 3,
            SmcccOwner::StandardSecure => 4,
            SmcccOwner::StandardHypervisor => 5,
            SmcccOwner::VendorHypervisor => 6,
            SmcccOwner::Other(val) => val,
        };
        let owner = ((owner_num as u32) & 0x3F) << 24;
        let func = (self.function_number as u32) & SMCCC_FUNC_NUM_MASK;
        fast | smc64 | owner | func
    }
}

/// Standard return values from an SMCCC invocation (registers `X0..X3`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SmcccResult {
    /// Register X0 (primary status / result).
    pub x0: u64,
    /// Register X1 (secondary result).
    pub x1: u64,
    /// Register X2 (tertiary result).
    pub x2: u64,
    /// Register X3 (quaternary result).
    pub x3: u64,
}
