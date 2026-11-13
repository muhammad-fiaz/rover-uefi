//! # TCG Physical Presence Interface
//!
//! Provides constants, operation codes, return codes, and flags for managing
//! Trusted Platform Module (TPM 1.2 and TPM 2.0) physical presence operations
//! as defined by the TCG PC Client Platform Physical Presence Interface Specification.

use crate::base::EfiGuid;

/// TCG 1.2 Physical Presence Data variable GUID (`TCG_PHYSICAL_PRESENCE_DATA_GUID`).
pub const TCG_PHYSICAL_PRESENCE_DATA_GUID: EfiGuid = EfiGuid::new(
    0x0F64_99B1,
    0xE9AD,
    0x493D,
    [0xB9, 0xC2, 0x2F, 0x90, 0x81, 0x5C, 0x6C, 0xBC],
);

/// TCG 2.0 Physical Presence Data variable GUID (`TCG2_PHYSICAL_PRESENCE_DATA_GUID`).
pub const TCG2_PHYSICAL_PRESENCE_DATA_GUID: EfiGuid = EfiGuid::new(
    0xAEB9_C5C1,
    0x94F1,
    0x4D02,
    [0xBF, 0xD9, 0x46, 0x02, 0xDB, 0x2D, 0x3C, 0x54],
);

/// TCG Physical Presence ACPI Interface GUID (`TCG_PHYSICAL_PRESENCE_ACPI_GUID`).
pub const TCG_PHYSICAL_PRESENCE_ACPI_GUID: EfiGuid = EfiGuid::new(
    0x3DDD_FAA6,
    0x361B,
    0x4EB4,
    [0xA4, 0x24, 0x8D, 0x10, 0x08, 0x9D, 0x16, 0x53],
);

/// PEI Lock Physical Presence PPI GUID (`PEI_LOCK_PHYSICAL_PRESENCE_PPI_GUID`).
pub const PEI_LOCK_PHYSICAL_PRESENCE_PPI_GUID: EfiGuid = EfiGuid::new(
    0xEF9A_EFE5,
    0x2BD3,
    0x4031,
    [0xAF, 0x7D, 0x5E, 0xFE, 0x5A, 0xBB, 0x9A, 0x0D],
);

/// Variable name for TPM 1.2 physical presence request in UTF-16 (`"PhysicalPresence"`).
pub const PHYSICAL_PRESENCE_VARIABLE_NAME: &[u16] = &[
    'P' as u16, 'h' as u16, 'y' as u16, 's' as u16, 'i' as u16, 'c' as u16, 'a' as u16, 'l' as u16,
    'P' as u16, 'r' as u16, 'e' as u16, 's' as u16, 'e' as u16, 'n' as u16, 'c' as u16, 'e' as u16,
    0,
];

/// Variable name for TPM 2.0 physical presence request in UTF-16 (`"Tcg2PhysicalPresence"`).
pub const TCG2_PHYSICAL_PRESENCE_VARIABLE_NAME: &[u16] = &[
    'T' as u16, 'c' as u16, 'g' as u16, '2' as u16, 'P' as u16, 'h' as u16, 'y' as u16, 's' as u16,
    'i' as u16, 'c' as u16, 'a' as u16, 'l' as u16, 'P' as u16, 'r' as u16, 'e' as u16, 's' as u16,
    'e' as u16, 'n' as u16, 'c' as u16, 'e' as u16, 0,
];

/// Variable name for TPM 2.0 physical presence flags in UTF-16 (`"Tcg2PhysicalPresenceFlags"`).
pub const TCG2_PHYSICAL_PRESENCE_FLAGS_VARIABLE_NAME: &[u16] = &[
    'T' as u16, 'c' as u16, 'g' as u16, '2' as u16, 'P' as u16, 'h' as u16, 'y' as u16, 's' as u16,
    'i' as u16, 'c' as u16, 'a' as u16, 'l' as u16, 'P' as u16, 'r' as u16, 'e' as u16, 's' as u16,
    'e' as u16, 'n' as u16, 'c' as u16, 'e' as u16, 'F' as u16, 'l' as u16, 'a' as u16, 'g' as u16,
    's' as u16, 0,
];

// TPM 1.2 Physical Presence Operations
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

// TPM 2.0 Physical Presence Operations
/// TPM2 Physical Presence Operation: No action requested.
pub const TCG2_PHYSICAL_PRESENCE_NO_ACTION: u8 = 0;
/// TPM2 Physical Presence Operation: Enable TPM.
pub const TCG2_PHYSICAL_PRESENCE_ENABLE: u8 = 1;
/// TPM2 Physical Presence Operation: Disable TPM.
pub const TCG2_PHYSICAL_PRESENCE_DISABLE: u8 = 2;
/// TPM2 Physical Presence Operation: Clear TPM storage and endorsement hierarchy.
pub const TCG2_PHYSICAL_PRESENCE_CLEAR: u8 = 5;
/// TPM2 Physical Presence Operation: Enable and clear TPM.
pub const TCG2_PHYSICAL_PRESENCE_ENABLE_CLEAR: u8 = 14;
/// TPM2 Physical Presence Operation: Require PP for clear (true).
pub const TCG2_PHYSICAL_PRESENCE_SET_PP_REQUIRED_FOR_CLEAR_TRUE: u8 = 17;
/// TPM2 Physical Presence Operation: Require PP for clear (false).
pub const TCG2_PHYSICAL_PRESENCE_SET_PP_REQUIRED_FOR_CLEAR_FALSE: u8 = 18;
/// TPM2 Physical Presence Operation: Set PCR banks algorithm.
pub const TCG2_PHYSICAL_PRESENCE_SET_PCR_BANKS: u8 = 23;
/// TPM2 Physical Presence Operation: Change Endorsement Primary Seed (EPS).
pub const TCG2_PHYSICAL_PRESENCE_CHANGE_EPS: u8 = 24;
/// TPM2 Physical Presence Operation: Set PP required for change PCRs false.
pub const TCG2_PHYSICAL_PRESENCE_SET_PP_REQUIRED_FOR_CHANGE_PCRS_FALSE: u8 = 25;
/// TPM2 Physical Presence Operation: Set PP required for change PCRs true.
pub const TCG2_PHYSICAL_PRESENCE_SET_PP_REQUIRED_FOR_CHANGE_PCRS_TRUE: u8 = 26;
/// TPM2 Physical Presence Operation: Log all digests into event log.
pub const TCG2_PHYSICAL_PRESENCE_LOG_ALL_DIGESTS: u8 = 33;

// Storage Management Operations (TCG Opal / Block SID)
/// Storage Management: Enable Block SID.
pub const TCG2_PP_ENABLE_BLOCK_SID: u8 = 96;
/// Storage Management: Disable Block SID.
pub const TCG2_PP_DISABLE_BLOCK_SID: u8 = 97;
/// Storage Management: Require PP for enabling Block SID.
pub const TCG2_PP_SET_REQUIRED_ENABLE_BLOCK_SID_TRUE: u8 = 98;
/// Storage Management: Do not require PP for enabling Block SID.
pub const TCG2_PP_SET_REQUIRED_ENABLE_BLOCK_SID_FALSE: u8 = 99;
/// Storage Management: Require PP for disabling Block SID.
pub const TCG2_PP_SET_REQUIRED_DISABLE_BLOCK_SID_TRUE: u8 = 100;
/// Storage Management: Do not require PP for disabling Block SID.
pub const TCG2_PP_SET_REQUIRED_DISABLE_BLOCK_SID_FALSE: u8 = 101;

// ACPI Physical Presence Functions
/// ACPI Function: Get physical presence interface version.
pub const TCG_ACPI_FUNCTION_GET_PHYSICAL_PRESENCE_INTERFACE_VERSION: u32 = 1;
/// ACPI Function: Submit physical presence request to BIOS.
pub const TCG_ACPI_FUNCTION_SUBMIT_REQUEST_TO_BIOS: u32 = 2;
/// ACPI Function: Get pending physical presence request by OS.
pub const TCG_ACPI_FUNCTION_GET_PENDING_REQUEST_BY_OS: u32 = 3;
/// ACPI Function: Get platform action to transition to BIOS.
pub const TCG_ACPI_FUNCTION_GET_PLATFORM_ACTION_TO_TRANSITION_TO_BIOS: u32 = 4;
/// ACPI Function: Return physical presence request response to OS.
pub const TCG_ACPI_FUNCTION_RETURN_REQUEST_RESPONSE_TO_OS: u32 = 5;
/// ACPI Function: Submit request to BIOS version 2.
pub const TCG_ACPI_FUNCTION_SUBMIT_REQUEST_TO_BIOS_2: u32 = 7;
/// ACPI Function: Get user confirmation status for request.
pub const TCG_ACPI_FUNCTION_GET_USER_CONFIRMATION_STATUS_FOR_REQUEST: u32 = 8;

/// Operation Return Status: Success.
pub const TCG_PP_RETURN_TPM_SUCCESS: u32 = 0;
/// Operation Return Status: General failure.
pub const TCG_PP_RETURN_TPM_FAILURE: u32 = 1;
/// Operation Return Status: Blocked by BIOS settings.
pub const TCG_PP_RETURN_TPM_BLOCKED: u32 = 2;
/// Operation Return Status: User aborted prompt.
pub const TCG_PP_RETURN_USER_ABORT: u32 = 3;

/// Response code: User aborted the prompt.
pub const TCG_PP_OPERATION_RESPONSE_USER_ABORT: u32 = 0xFFFF_FFF0;
/// Response code: BIOS execution failure.
pub const TCG_PP_OPERATION_RESPONSE_BIOS_FAILURE: u32 = 0xFFFF_FFF1;

/// Physical Presence Platform Flags for TPM 1.2.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TcgPhysicalPresenceFlags {
    /// Physical presence lifetime lock bit.
    pub pp_lifetime_lock: bool,
    /// Physical presence command lock bit.
    pub pp_cmd_enable: bool,
}

/// TPM 1.2 physical presence variable payload.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TcgPhysicalPresenceData {
    /// Physical presence request opcode.
    pub pp_request: u8,
    /// Last executed physical presence request.
    pub last_pp_request: u8,
    /// Response status of the last request.
    pub pp_response: u32,
}

/// TPM 2.0 physical presence variable payload.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tcg2PhysicalPresenceData {
    /// Physical presence request opcode.
    pub pp_request: u8,
    /// Optional parameter for the request (e.g. PCR bank mask).
    pub pp_request_parameter: u32,
    /// Last executed physical presence request opcode.
    pub last_pp_request: u8,
    /// Response status of the last request.
    pub pp_response: u32,
}

/// TPM 2.0 physical presence management flags.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tcg2PhysicalPresenceFlagsData {
    /// Management flags bitmask.
    pub pp_flags: u32,
}

impl Tcg2PhysicalPresenceFlagsData {
    /// Management flag: PP required for Clear operation.
    pub const PP_REQUIRED_FOR_CLEAR: u32 = 1 << 1;
    /// Management flag: Reset tracking bit.
    pub const RESET_TRACK: u32 = 1 << 3;
    /// Management flag: PP required for Turn On.
    pub const PP_REQUIRED_FOR_TURN_ON: u32 = 1 << 4;
    /// Management flag: PP required for Turn Off.
    pub const PP_REQUIRED_FOR_TURN_OFF: u32 = 1 << 5;
    /// Management flag: PP required for Change EPS.
    pub const PP_REQUIRED_FOR_CHANGE_EPS: u32 = 1 << 6;
    /// Management flag: PP required for Change PCRs.
    pub const PP_REQUIRED_FOR_CHANGE_PCRS: u32 = 1 << 7;

    /// Checks if physical presence is required for clearing TPM.
    #[must_use]
    pub fn is_pp_required_for_clear(&self) -> bool {
        (self.pp_flags & Self::PP_REQUIRED_FOR_CLEAR) != 0
    }
}

/// Helper structure for inspecting TCG Physical Presence requests.
pub struct TcgPhysicalPresence;

impl TcgPhysicalPresence {
    /// Validate if an operation code corresponds to a known standard TPM 1.2 request.
    #[must_use]
    pub const fn is_valid_operation(op: u8) -> bool {
        op <= TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_FALSE
    }

    /// Validate if an operation code corresponds to a known standard TPM 2.0 request.
    #[must_use]
    pub const fn is_valid_tpm2_operation(op: u8) -> bool {
        op <= TCG2_PHYSICAL_PRESENCE_LOG_ALL_DIGESTS
            || (op >= TCG2_PP_ENABLE_BLOCK_SID
                && op <= TCG2_PP_SET_REQUIRED_DISABLE_BLOCK_SID_FALSE)
    }

    /// Check whether an operation requires physically clearing user data.
    #[must_use]
    pub const fn is_destructive(op: u8) -> bool {
        op == TCG_PHYSICAL_PRESENCE_CLEAR || op == TCG2_PHYSICAL_PRESENCE_CLEAR
    }

    /// Check whether an operation belongs to storage management (Block SID).
    #[must_use]
    pub const fn is_storage_management_operation(op: u8) -> bool {
        op >= TCG2_PP_ENABLE_BLOCK_SID && op <= TCG2_PP_SET_REQUIRED_DISABLE_BLOCK_SID_FALSE
    }
}
