//! # Variable Policy Protocol and Client API
//!
//! Enforces hardware and platform security policies on UEFI variables,
//! restricting variable creation, modification, attribute constraints,
//! and state-based locking for Secure Boot key stores and critical settings.

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};

/// Variable Policy Protocol GUID: `{81d1675c-86f6-48df-bd95-9a6e4f0925c3}`.
pub const VARIABLE_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x81D1_675C,
    0x86F6,
    0x48DF,
    [0xBD, 0x95, 0x9A, 0x6E, 0x4F, 0x09, 0x25, 0xC3],
);

/// Current revision of the Variable Policy Protocol interface.
pub const VARIABLE_POLICY_PROTOCOL_REVISION: u64 = 0x0002_0000;

/// Version number for individual variable policy entries.
pub const VARIABLE_POLICY_ENTRY_REVISION: u32 = 0x0001_0000;

/// Wildcard value indicating no minimum size requirement.
pub const VARIABLE_POLICY_NO_MIN_SIZE: u32 = 0;
/// Wildcard value indicating no maximum size requirement.
pub const VARIABLE_POLICY_NO_MAX_SIZE: u32 = u32::MAX;
/// Wildcard value indicating no mandatory attribute bits.
pub const VARIABLE_POLICY_NO_MUST_ATTR: u32 = 0;
/// Wildcard value indicating no prohibited attribute bits.
pub const VARIABLE_POLICY_NO_CANT_ATTR: u32 = 0;

/// Variable policy lock type: No lock.
pub const VARIABLE_POLICY_TYPE_NO_LOCK: u8 = 0;
/// Variable policy lock type: Lock immediately on policy registration.
pub const VARIABLE_POLICY_TYPE_LOCK_NOW: u8 = 1;
/// Variable policy lock type: Lock on broad platform state change.
pub const VARIABLE_POLICY_TYPE_LOCK_ON_CREATE: u8 = 2;
/// Variable policy lock type: Lock when a target variable reaches a specific value.
pub const VARIABLE_POLICY_TYPE_LOCK_ON_VAR_STATE: u8 = 3;

/// Structure representing a registered variable policy constraint.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariablePolicyEntry {
    /// Structure revision (`VARIABLE_POLICY_ENTRY_REVISION`).
    pub version: u32,
    /// Total byte size of this policy entry header plus trailing name data.
    pub size: u16,
    /// Offset from start of structure to variable name string.
    pub offset_to_name: u16,
    /// Target variable namespace GUID.
    pub namespace: EfiGuid,
    /// Minimum allowed size for the variable payload.
    pub min_size: u32,
    /// Maximum allowed size for the variable payload.
    pub max_size: u32,
    /// Attribute bits that the variable must have.
    pub attributes_must_have: u32,
    /// Attribute bits that the variable cannot have.
    pub attributes_cant_have: u32,
    /// Lock policy trigger type (`VARIABLE_POLICY_TYPE_*`).
    pub lock_policy_type: u8,
    /// Reserved alignment padding.
    pub padding: [u8; 3],
}

/// Policy payload for state-dependent variable locking.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariableLockOnVarStatePolicy {
    /// Namespace GUID of the trigger variable.
    pub namespace: EfiGuid,
    /// Byte value indicating lock trigger state.
    pub value: u8,
    /// Reserved alignment padding.
    pub padding: u8,
}

/// Disables policy enforcement if allowed by platform configuration.
pub type DisableVariablePolicy = unsafe extern "efiapi" fn() -> EfiStatus;

/// Queries whether the policy enforcement engine is active.
pub type IsVariablePolicyEnabled = unsafe extern "efiapi" fn(state: *mut Bool) -> EfiStatus;

/// Registers a new variable policy entry.
pub type RegisterVariablePolicy =
    unsafe extern "efiapi" fn(policy_entry: *const VariablePolicyEntry) -> EfiStatus;

/// Dumps registered variable policy entries to a caller buffer.
pub type DumpVariablePolicy =
    unsafe extern "efiapi" fn(policy: *mut u8, size: *mut u32) -> EfiStatus;

/// Permanently locks the variable policy interface until next reset.
pub type LockVariablePolicy = unsafe extern "efiapi" fn() -> EfiStatus;

/// Queries policy information for a specific variable.
pub type GetVariablePolicyInfo = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    name_buffer_size: *mut UIntN,
    variable_policy: *mut VariablePolicyEntry,
    variable_policy_variable_name: *mut Char16,
) -> EfiStatus;

/// Queries lock-on-state policy information for a specific variable.
pub type GetLockOnVariableStateVariablePolicyInfo = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    name_buffer_size: *mut UIntN,
    variable_policy: *mut VariableLockOnVarStatePolicy,
    lock_policy_variable_name: *mut Char16,
) -> EfiStatus;

/// Variable Policy Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariablePolicyProtocol {
    /// Interface revision.
    pub revision: u64,
    /// Disables policy engine.
    pub disable_variable_policy: DisableVariablePolicy,
    /// Checks if engine is enabled.
    pub is_variable_policy_enabled: IsVariablePolicyEnabled,
    /// Registers a policy.
    pub register_variable_policy: RegisterVariablePolicy,
    /// Dumps registered policies.
    pub dump_variable_policy: DumpVariablePolicy,
    /// Locks interface until reboot.
    pub lock_variable_policy: LockVariablePolicy,
    /// Gets policy for a variable.
    pub get_variable_policy_info: GetVariablePolicyInfo,
    /// Gets lock-on-state policy.
    pub get_lock_on_variable_state_variable_policy_info: GetLockOnVariableStateVariablePolicyInfo,
}

/// Safe client wrapper for Variable Policy operations.
#[derive(Debug)]
pub struct VariablePolicy<'a> {
    protocol: &'a VariablePolicyProtocol,
}

impl<'a> VariablePolicy<'a> {
    /// Create a new safe view of the variable policy protocol.
    #[must_use]
    pub fn new(protocol: &'a VariablePolicyProtocol) -> Self {
        Self { protocol }
    }

    /// Check whether variable policy enforcement is enabled.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware variable policy protocol.
    pub fn is_enabled(&self) -> Result<bool, EfiStatus> {
        let mut enabled: Bool = 0;
        let status = unsafe { (self.protocol.is_variable_policy_enabled)(&raw mut enabled) };
        if status == EfiStatus::SUCCESS {
            Ok(enabled != 0)
        } else {
            Err(status)
        }
    }

    /// Permanently lock the variable policy interface until system reboot.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware variable policy protocol.
    pub fn lock(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.lock_variable_policy)() };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
