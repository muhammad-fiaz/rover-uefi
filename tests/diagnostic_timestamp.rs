// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI 2.4 Timestamp Protocol.

use core::mem::size_of;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::diagnostic::{
    EfiTimestampProperties, EfiTimestampProtocol, TIMESTAMP_PROTOCOL_GUID, TimestampProtocol,
};

static mut TICK_COUNTER: u64 = 1_000_000;

unsafe extern "efiapi" fn mock_get_timestamp() -> u64 {
    unsafe {
        TICK_COUNTER += 100;
        TICK_COUNTER
    }
}

unsafe extern "efiapi" fn mock_get_properties(
    properties: *mut EfiTimestampProperties,
) -> EfiStatus {
    if properties.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*properties).frequency = 10_000_000; // 10 MHz
        (*properties).end_value = u64::MAX;
    }
    EfiStatus::SUCCESS
}

#[test]
fn guid_and_layout() {
    assert_eq!(
        TIMESTAMP_PROTOCOL_GUID.to_bytes(),
        [
            0x41, 0xde, 0xbf, 0xaf, 0x6e, 0x2e, 0x62, 0x42, 0xba, 0x65, 0x62, 0xb9, 0x23, 0x6e,
            0x54, 0x95
        ]
    );
    assert_eq!(size_of::<EfiTimestampProperties>(), 16);
    assert_eq!(size_of::<EfiTimestampProtocol>(), size_of::<usize>() * 2);
}

#[test]
fn protocol_wrapper() {
    let mut raw = EfiTimestampProtocol {
        get_timestamp: mock_get_timestamp,
        get_properties: mock_get_properties,
    };

    let Some(mut proto) = (unsafe { TimestampProtocol::from_raw(&raw mut raw) }) else {
        panic!("expected protocol wrapper");
    };

    let t1 = proto.get_timestamp();
    let t2 = proto.get_timestamp();
    assert!(t2 > t1);

    let Ok(props) = proto.get_properties() else {
        panic!("properties valid");
    };
    assert_eq!(props.frequency, 10_000_000);
    assert_eq!(props.end_value, u64::MAX);
}
