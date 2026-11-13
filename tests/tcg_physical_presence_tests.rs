//! Tests for TCG Physical Presence interface and variable data.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::protocols::security::tcg_pp::{
    PEI_LOCK_PHYSICAL_PRESENCE_PPI_GUID, TCG_PHYSICAL_PRESENCE_ACPI_GUID,
    TCG_PHYSICAL_PRESENCE_CLEAR, TCG_PHYSICAL_PRESENCE_DATA_GUID, TCG2_PHYSICAL_PRESENCE_CLEAR,
    TCG2_PHYSICAL_PRESENCE_DATA_GUID, TCG2_PHYSICAL_PRESENCE_ENABLE,
    TCG2_PHYSICAL_PRESENCE_NO_ACTION, TCG2_PP_ENABLE_BLOCK_SID, Tcg2PhysicalPresenceData,
    Tcg2PhysicalPresenceFlagsData, TcgPhysicalPresence, TcgPhysicalPresenceData,
};

#[test]
fn test_tcg_physical_presence_guids_and_ops() {
    assert_eq!(
        TCG2_PHYSICAL_PRESENCE_DATA_GUID.to_string(),
        "aeb9c5c1-94f1-4d02-bfd9-4602db2d3c54"
    );
    assert_eq!(
        TCG_PHYSICAL_PRESENCE_ACPI_GUID.to_string(),
        "3dddfaa6-361b-4eb4-a424-8d10089d1653"
    );
    assert_eq!(
        PEI_LOCK_PHYSICAL_PRESENCE_PPI_GUID.to_string(),
        "ef9aefe5-2bd3-4031-af7d-5efe5abb9a0d"
    );
    assert_eq!(
        TCG_PHYSICAL_PRESENCE_DATA_GUID.to_string(),
        "0f6499b1-e9ad-493d-b9c2-2f90815c6cbc"
    );

    assert!(TcgPhysicalPresence::is_valid_tpm2_operation(
        TCG2_PHYSICAL_PRESENCE_NO_ACTION
    ));
    assert!(TcgPhysicalPresence::is_valid_tpm2_operation(
        TCG2_PHYSICAL_PRESENCE_ENABLE
    ));
    assert!(TcgPhysicalPresence::is_valid_tpm2_operation(
        TCG2_PHYSICAL_PRESENCE_CLEAR
    ));
    assert!(TcgPhysicalPresence::is_valid_tpm2_operation(
        TCG2_PP_ENABLE_BLOCK_SID
    ));
    assert!(!TcgPhysicalPresence::is_valid_tpm2_operation(150));

    assert!(TcgPhysicalPresence::is_destructive(
        TCG_PHYSICAL_PRESENCE_CLEAR
    ));
    assert!(TcgPhysicalPresence::is_destructive(
        TCG2_PHYSICAL_PRESENCE_CLEAR
    ));
    assert!(!TcgPhysicalPresence::is_destructive(
        TCG2_PHYSICAL_PRESENCE_ENABLE
    ));

    assert!(TcgPhysicalPresence::is_storage_management_operation(
        TCG2_PP_ENABLE_BLOCK_SID
    ));
    assert!(!TcgPhysicalPresence::is_storage_management_operation(
        TCG2_PHYSICAL_PRESENCE_ENABLE
    ));

    let pp_data = Tcg2PhysicalPresenceData {
        pp_request: TCG2_PHYSICAL_PRESENCE_CLEAR,
        pp_request_parameter: 0,
        last_pp_request: 0,
        pp_response: 0,
    };
    assert_eq!(pp_data.pp_request, 5);

    let tpm1_data = TcgPhysicalPresenceData {
        pp_request: TCG_PHYSICAL_PRESENCE_CLEAR,
        last_pp_request: 0,
        pp_response: 0,
    };
    assert_eq!(tpm1_data.pp_request, 5);

    let flags = Tcg2PhysicalPresenceFlagsData {
        pp_flags: Tcg2PhysicalPresenceFlagsData::PP_REQUIRED_FOR_CLEAR,
    };
    assert!(flags.is_pp_required_for_clear());
}
