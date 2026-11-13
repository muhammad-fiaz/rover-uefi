//! # Variable Policy Protocol
//!
//! Defined in EDK II `MdeModulePkg`.
//!
//! Enforces hardware and platform security policies on UEFI variables,
//! restricting variable creation, modification, attribute constraints,
//! and state-based locking for Secure Boot key stores and critical settings.

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};

/// EDK II Variable Policy Protocol GUID: `{81d1675c-86f6-48df-bd95-9a6e4f0925c3}`.
pub const EDKII_VARIABLE_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x81D1_675C,
    0x86F6,
    0x48DF,
    [0xBD, 0x95, 0x9A, 0x6E, 0x4F, 0x09, 0x25, 0xC3],
);

/// Current revision of the Variable Policy Protocol interface.
pub const EDKII_VARIABLE_POLICY_PROTOCOL_REVISION: u64 = 0x0002_0000;

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
/// Variable policy lock type: Lock immediately upon registration.
pub const VARIABLE_POLICY_TYPE_LOCK_NOW: u8 = 1;
/// Variable policy lock type: Lock on variable creation.
pub const VARIABLE_POLICY_TYPE_LOCK_ON_CREATE: u8 = 2;
/// Variable policy lock type: Lock when another variable reaches a target state.
pub const VARIABLE_POLICY_TYPE_LOCK_ON_VAR_STATE: u8 = 3;

/// Entry header describing constraints on a UEFI variable.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariablePolicyEntry {
    /// Entry structure revision (`VARIABLE_POLICY_ENTRY_REVISION`).
    pub version: u32,
    /// Total byte size of this entry including variable-length fields.
    pub size: u16,
    /// Offset in bytes from entry start to the variable name string.
    pub offset_to_name: u16,
    /// Vendor namespace GUID of the protected variable.
    pub namespace: EfiGuid,
    /// Minimum allowed payload size in bytes.
    pub min_size: u32,
    /// Maximum allowed payload size in bytes.
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
pub type EdkiiDisableVariablePolicy = unsafe extern "efiapi" fn() -> EfiStatus;

/// Queries whether the policy enforcement engine is active.
pub type EdkiiIsVariablePolicyEnabled = unsafe extern "efiapi" fn(state: *mut Bool) -> EfiStatus;

/// Registers a new variable policy entry.
pub type EdkiiRegisterVariablePolicy =
    unsafe extern "efiapi" fn(policy_entry: *const VariablePolicyEntry) -> EfiStatus;

/// Dumps registered variable policy entries to a caller buffer.
pub type EdkiiDumpVariablePolicy =
    unsafe extern "efiapi" fn(policy: *mut u8, size: *mut u32) -> EfiStatus;

/// Permanently locks the variable policy interface until next reset.
pub type EdkiiLockVariablePolicy = unsafe extern "efiapi" fn() -> EfiStatus;

/// Queries policy information for a specific variable.
pub type EdkiiGetVariablePolicyInfo = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    name_buffer_size: *mut UIntN,
    variable_policy: *mut VariablePolicyEntry,
    variable_policy_variable_name: *mut Char16,
) -> EfiStatus;

/// Queries lock-on-state policy information for a specific variable.
pub type EdkiiGetLockOnVariableStateVariablePolicyInfo = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    name_buffer_size: *mut UIntN,
    variable_policy: *mut VariableLockOnVarStatePolicy,
    lock_policy_variable_name: *mut Char16,
) -> EfiStatus;

/// EDK II Variable Policy Protocol interface (`EDKII_VARIABLE_POLICY_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EdkiiVariablePolicyProtocol {
    /// Interface revision.
    pub revision: u64,
    /// Disables policy engine.
    pub disable_variable_policy: EdkiiDisableVariablePolicy,
    /// Checks if engine is enabled.
    pub is_variable_policy_enabled: EdkiiIsVariablePolicyEnabled,
    /// Registers a policy.
    pub register_variable_policy: EdkiiRegisterVariablePolicy,
    /// Dumps registered policies.
    pub dump_variable_policy: EdkiiDumpVariablePolicy,
    /// Locks interface until reboot.
    pub lock_variable_policy: EdkiiLockVariablePolicy,
    /// Gets policy for a variable.
    pub get_variable_policy_info: EdkiiGetVariablePolicyInfo,
    /// Gets lock-on-state policy.
    pub get_lock_on_variable_state_variable_policy_info:
        EdkiiGetLockOnVariableStateVariablePolicyInfo,
}
