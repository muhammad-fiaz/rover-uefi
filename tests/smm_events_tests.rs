//! Integration tests for SMM event notification protocols.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::firmware::smm_events::{
    SMM_EXIT_BOOT_SERVICES_PROTOCOL_GUID, SMM_LEGACY_BOOT_PROTOCOL_GUID,
    SMM_READY_TO_BOOT_PROTOCOL_GUID,
};

#[test]
fn test_smm_event_guids() {
    assert_eq!(
        SMM_EXIT_BOOT_SERVICES_PROTOCOL_GUID,
        EfiGuid::new(
            0x296E_B418,
            0xC4C8,
            0x4E05,
            [0xAB, 0x59, 0x39, 0xE8, 0xAF, 0x56, 0xF0, 0x0A],
        )
    );
    assert_eq!(
        SMM_READY_TO_BOOT_PROTOCOL_GUID,
        EfiGuid::new(
            0x6E05_7ECF,
            0xFA99,
            0x4F39,
            [0x95, 0xBC, 0x59, 0xF9, 0x92, 0x1D, 0x17, 0xE4],
        )
    );
    assert_eq!(
        SMM_LEGACY_BOOT_PROTOCOL_GUID,
        EfiGuid::new(
            0x85A8_AB57,
            0x0644,
            0x4110,
            [0x85, 0x0F, 0x98, 0x13, 0x22, 0x04, 0x70, 0x70],
        )
    );
}
