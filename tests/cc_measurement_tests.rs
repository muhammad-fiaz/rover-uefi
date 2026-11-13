//! Tests for Confidential Computing (CC) Measurement Protocol.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::security::cc_measurement::{
    ARMCCA_MR_INDEX_0_RIM, ARMCCA_MR_INDEX_1_REM0, CC_BOOT_HASH_ALG_SHA256,
    CC_BOOT_HASH_ALG_SHA384, CC_EVENT_LOG_FORMAT_TCG_2, CC_FINAL_EVENTS_TABLE_GUID,
    CC_MEASUREMENT_PROTOCOL_GUID, CcMeasurement, EfiCcMeasurementBootServiceCapability,
    EfiCcMeasurementProtocol, EfiCcType, TDX_MR_INDEX_MRTD, TDX_MR_INDEX_RTMR0,
};

unsafe extern "efiapi" fn mock_get_capability(
    _this: *const EfiCcMeasurementProtocol,
    cap: *mut EfiCcMeasurementBootServiceCapability,
) -> EfiStatus {
    unsafe {
        (*cap).structure_version_major = 1;
        (*cap).structure_version_minor = 0;
        (*cap).hash_algorithm_bitmap = CC_BOOT_HASH_ALG_SHA256 | CC_BOOT_HASH_ALG_SHA384;
        (*cap).supported_event_logs = CC_EVENT_LOG_FORMAT_TCG_2;
        (*cap).cc_type = EfiCcType::Tdx as u8;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_event_log(
    _this: *const EfiCcMeasurementProtocol,
    _format: u32,
    location: *mut EfiPhysicalAddress,
    last_entry: *mut EfiPhysicalAddress,
    truncated: *mut Bool,
) -> EfiStatus {
    unsafe {
        *location = EfiPhysicalAddress::from_u64(0x1000_0000);
        *last_entry = EfiPhysicalAddress::from_u64(0x1000_0800);
        *truncated = 0;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_hash_log_extend(
    _this: *const EfiCcMeasurementProtocol,
    _flags: u64,
    _data_to_hash: EfiPhysicalAddress,
    _data_to_hash_len: u64,
    _event: *const rover_uefi::protocols::security::cc_measurement::EfiCcMeasurementEvent,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_map_pcr_to_mr_index(
    _this: *const EfiCcMeasurementProtocol,
    pcr_index: u32,
    mr_index: *mut u32,
) -> EfiStatus {
    unsafe {
        *mr_index = match pcr_index {
            0 => TDX_MR_INDEX_MRTD,
            1 | 7 => TDX_MR_INDEX_RTMR0,
            _ => 2,
        };
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_cc_measurement_protocol() {
    assert_eq!(
        CC_MEASUREMENT_PROTOCOL_GUID.to_string(),
        "96751a3d-72f4-41a6-a794-ed5d0e67ae6b"
    );
    assert_eq!(
        CC_FINAL_EVENTS_TABLE_GUID.to_string(),
        "dd4a4648-2de7-4665-964d-21d9ef5fb446"
    );
    assert_eq!(TDX_MR_INDEX_MRTD, 0);
    assert_eq!(TDX_MR_INDEX_RTMR0, 1);
    assert_eq!(ARMCCA_MR_INDEX_0_RIM, 0);
    assert_eq!(ARMCCA_MR_INDEX_1_REM0, 1);

    let raw = EfiCcMeasurementProtocol {
        get_capability: mock_get_capability,
        get_event_log: mock_get_event_log,
        hash_log_extend_event: mock_hash_log_extend,
        map_pcr_to_mr_index: mock_map_pcr_to_mr_index,
    };

    let client = CcMeasurement::new(&raw);
    let cap = client.get_capability().expect("capability should succeed");
    assert_eq!(cap.structure_version_major, 1);
    assert_eq!(cap.cc_type, EfiCcType::Tdx as u8);

    let (loc, last, trunc) = client
        .get_event_log(CC_EVENT_LOG_FORMAT_TCG_2)
        .expect("event log should succeed");
    assert_eq!(loc.to_u64(), 0x1000_0000);
    assert_eq!(last.to_u64(), 0x1000_0800);
    assert!(!trunc);

    let mr0 = client.map_pcr_to_mr_index(0).expect("map PCR 0");
    assert_eq!(mr0, TDX_MR_INDEX_MRTD);
    let mr1 = client.map_pcr_to_mr_index(1).expect("map PCR 1");
    assert_eq!(mr1, TDX_MR_INDEX_RTMR0);
}
