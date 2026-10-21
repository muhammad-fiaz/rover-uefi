//! TCG2 tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::TCG2_GUID;
use rover_uefi::protocols::security::tcg2::capability::{
    EXTEND_ONLY, EfiTcg2BootServiceCapability, EfiTcg2FinalEventsTable, FINAL_EVENTS_TABLE_VERSION,
    HASH_ALG_SHA1, HASH_ALG_SHA256, HASH_ALG_SHA384, HASH_ALG_SHA512, HASH_ALG_SM3_256,
    MAX_PCR_INDEX, PE_COFF_IMAGE_FLAG,
};
use rover_uefi::protocols::security::tcg2::protocol::EfiTcg2Protocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        TCG2_GUID.to_bytes(),
        [
            0x6C, 0x76, 0x7F, 0x60, 0x55, 0x74, 0xBE, 0x42, 0x93, 0x0B, 0xE4, 0xD7, 0x6D, 0xB2,
            0x72, 0x0F
        ]
    );
    assert_eq!(EfiGuid::from_bytes(TCG2_GUID.to_bytes()), TCG2_GUID);
}

#[test]
fn protocol_has_seven_entries() {
    assert_eq!(
        size_of::<EfiTcg2Protocol>(),
        7 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, submit_command),
        3 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, get_result_of_set_active_pcr_banks),
        6 * core::mem::size_of::<usize>()
    );
}

#[test]
fn capability_layout() {
    assert_eq!(size_of::<EfiTcg2BootServiceCapability>(), 36);
    assert_eq!(offset_of!(EfiTcg2BootServiceCapability, size), 0);
    assert_eq!(
        offset_of!(EfiTcg2BootServiceCapability, hash_algorithm_bitmap),
        8
    );
    assert_eq!(
        offset_of!(EfiTcg2BootServiceCapability, supported_event_logs),
        12
    );
    assert_eq!(
        offset_of!(EfiTcg2BootServiceCapability, tpm_present_flag),
        16
    );
}

#[test]
fn capability_constants() {
    assert_eq!(HASH_ALG_SHA1, 1);
    assert_eq!(HASH_ALG_SHA256, 2);
    assert_eq!(HASH_ALG_SHA384, 4);
    assert_eq!(HASH_ALG_SHA512, 8);
    assert_eq!(HASH_ALG_SM3_256, 0x10);
    assert_eq!(EXTEND_ONLY, 1);
    assert_eq!(PE_COFF_IMAGE_FLAG, 0x10);
    assert_eq!(MAX_PCR_INDEX, 23);
    assert_eq!(FINAL_EVENTS_TABLE_VERSION, 1);
    assert_eq!(size_of::<EfiTcg2FinalEventsTable>(), 16);
}
