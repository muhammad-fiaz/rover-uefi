//! # TCG Physical Presence Interface
//!
//! Provides constants, operation codes, return codes, and flags for managing
//! Trusted Platform Module (TPM 1.2 and TPM 2.0) physical presence operations
//! as defined by the TCG PC Client Platform Physical Presence Interface Specification.

use crate::base::EfiGuid;

/// TCG Physical Presence Data variable GUID (`TCG_PHYSICAL_PRESENCE_DATA_GUID`).
pub const TCG_PHYSICAL_PRESENCE_DATA_GUID: EfiGuid = EfiGuid::new(
    0xF646_1488,
    0x89A0,
    0x4305,
    [0xAC, 0x57, 0xAB, 0x19, 0x4F, 0x10, 0x88, 0xF5],
);

/// Variable name for physical presence request in UTF-16.
pub const PHYSICAL_PRESENCE_VARIABLE_NAME: &[u16] = &[
    'P' as u16, 'h' as u16, 'y' as u16, 's' as u16, 'i' as u16, 'c' as u16, 'a' as u16, 'l' as u16,
    'P' as u16, 'r' as u16, 'e' as u16, 's' as u16, 'e' as u16, 'n' as u16, 'c' as u16, 'e' as u16,
    0,
];

/// Physical Presence Operation: No action requested.
pub const TCG_PHYSICAL_PRESENCE_NO_ACTION: u8 = 0;
/// Physical Presence Operation: Enable TPM.
pub const TCG_PHYSICAL_PRESENCE_ENABLE: u8 = 1;
/// Physical Presence Operation: Disable TPM.
pub const TCG_PHYSICAL_PRESENCE_DISABLE: u8 = 2;
/// Physical Presence Operation: Activate TPM.
pub const TCG_PHYSICAL_PRESENCE_ACTIVATE: u8 = 3;
/// Physical Presence Operation: Deactivate TPM.
pub const TCG_PHYSICAL_PRESENCE_DEACTIVATE: u8 = 4;
/// Physical Presence Operation: Clear TPM ownership.
pub const TCG_PHYSICAL_PRESENCE_CLEAR: u8 = 5;
/// Physical Presence Operation: Enable and activate TPM.
pub const TCG_PHYSICAL_PRESENCE_ENABLE_ACTIVATE: u8 = 6;
/// Physical Presence Operation: Deactivate and disable TPM.
pub const TCG_PHYSICAL_PRESENCE_DEACTIVATE_DISABLE: u8 = 7;
/// Physical Presence Operation: Set owner install to true.
pub const TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_TRUE: u8 = 8;
/// Physical Presence Operation: Set owner install to false.
pub const TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_FALSE: u8 = 9;

/// Operation Return Status: Success.
pub const TCG_PP_RETURN_TPM_SUCCESS: u32 = 0;
/// Operation Return Status: General failure.
pub const TCG_PP_RETURN_TPM_FAILURE: u32 = 1;
/// Operation Return Status: Blocked by BIOS settings.
pub const TCG_PP_RETURN_TPM_BLOCKED: u32 = 2;
/// Operation Return Status: User aborted prompt.
pub const TCG_PP_RETURN_USER_ABORT: u32 = 3;

/// Physical Presence Platform Flags.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TcgPhysicalPresenceFlags {
    /// Physical presence lifetime lock bit.
    pub pp_lifetime_lock: bool,
    /// Physical presence command lock bit.
    pub pp_cmd_enable: bool,
}

/// Helper structure for inspecting TCG Physical Presence requests.
pub struct TcgPhysicalPresence;

impl TcgPhysicalPresence {
    /// Validate if an operation code corresponds to a known standard request.
    #[must_use]
    pub const fn is_valid_operation(op: u8) -> bool {
        op <= TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_FALSE
    }

    /// Check whether an operation requires physically clearing user data.
    #[must_use]
    pub const fn is_destructive(op: u8) -> bool {
        op == TCG_PHYSICAL_PRESENCE_CLEAR
    }
}
