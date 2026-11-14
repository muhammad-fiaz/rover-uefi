//! Integration tests for Performance Measurement and Gauge protocols.

use core::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::diagnostic::performance_measurement::*;

static RECORD_ATTR: AtomicU32 = AtomicU32::new(999);
static RECORD_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
static RECORD_ID: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_create_perf_measurement(
    _caller_identifier: *const c_void,
    _guid: *const EfiGuid,
    _string: *const u8,
    timestamp: u64,
    _address: u64,
    identifier: u32,
    attribute: PerfMeasurementAttribute,
) -> EfiStatus {
    RECORD_ATTR.store(attribute as u32, Ordering::SeqCst);
    RECORD_TIMESTAMP.store(timestamp, Ordering::SeqCst);
    RECORD_ID.store(identifier, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

static GAUGE_START_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
static GAUGE_END_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

unsafe extern "efiapi" fn mock_start_gauge(
    _handle: *const c_void,
    _token: *const u8,
    _module: *const u8,
    timestamp: u64,
) -> EfiStatus {
    GAUGE_START_TIMESTAMP.store(timestamp, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_end_gauge(
    _handle: *const c_void,
    _token: *const u8,
    _module: *const u8,
    timestamp: u64,
) -> EfiStatus {
    GAUGE_END_TIMESTAMP.store(timestamp, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

static SAMPLE_ENTRY: GaugeDataEntry = GaugeDataEntry {
    handle: EfiPhysicalAddress::from_u64(0x1000),
    token: [b'T'; 32],
    module: [b'M'; 32],
    start_timestamp: 100,
    end_timestamp: 200,
};

unsafe extern "efiapi" fn mock_get_gauge(
    log_entry_key: UIntN,
    gauge_data_entry: *mut *const GaugeDataEntry,
) -> EfiStatus {
    if log_entry_key == 0 {
        if !gauge_data_entry.is_null() {
            unsafe {
                *gauge_data_entry = &raw const SAMPLE_ENTRY;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

static SAMPLE_ENTRY_EX: GaugeDataEntryEx = GaugeDataEntryEx {
    handle: EfiPhysicalAddress::from_u64(0x2000),
    token: [b'X'; 32],
    module: [b'Y'; 32],
    start_timestamp: 300,
    end_timestamp: 500,
    identifier: 42,
};

static GAUGE_EX_ID: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_start_gauge_ex(
    _handle: *const c_void,
    _token: *const u8,
    _module: *const u8,
    timestamp: u64,
    identifier: u32,
) -> EfiStatus {
    GAUGE_START_TIMESTAMP.store(timestamp, Ordering::SeqCst);
    GAUGE_EX_ID.store(identifier, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_end_gauge_ex(
    _handle: *const c_void,
    _token: *const u8,
    _module: *const u8,
    timestamp: u64,
    identifier: u32,
) -> EfiStatus {
    GAUGE_END_TIMESTAMP.store(timestamp, Ordering::SeqCst);
    GAUGE_EX_ID.store(identifier, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_gauge_ex(
    log_entry_key: UIntN,
    gauge_data_entry_ex: *mut *const GaugeDataEntryEx,
) -> EfiStatus {
    if log_entry_key == 0 {
        if !gauge_data_entry_ex.is_null() {
            unsafe {
                *gauge_data_entry_ex = &raw const SAMPLE_ENTRY_EX;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_performance_guids_and_constants() {
    assert_eq!(
        PERFORMANCE_MEASUREMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xc85d_06be,
            0x5f75,
            0x48ce,
            [0xa8, 0x0f, 0x12, 0x36, 0xba, 0x3b, 0x87, 0xb1]
        )
    );
    assert_eq!(
        SMM_PERFORMANCE_MEASUREMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xd56b_6d73,
            0x1a7b,
            0x4015,
            [0x9b, 0xb4, 0x7b, 0x07, 0x17, 0x29, 0xed, 0x24]
        )
    );
    assert_eq!(
        PERFORMANCE_PROTOCOL_GUID,
        EfiGuid::new(
            0x76b6_bdfa,
            0x2acd,
            0x4462,
            [0x9e, 0x3f, 0xcb, 0x58, 0xc9, 0x69, 0xd9, 0x37]
        )
    );
    assert_eq!(
        PERFORMANCE_EX_PROTOCOL_GUID,
        EfiGuid::new(
            0x1ea8_1bec,
            0xf01a,
            0x4d98,
            [0xa2, 0x01, 0x4a, 0x61, 0xce, 0x2f, 0xc0, 0x22]
        )
    );
    assert_eq!(
        SMM_PERFORMANCE_PROTOCOL_GUID,
        EfiGuid::new(
            0xf866_226a,
            0xeaa5,
            0x4f5a,
            [0xa9, 0x0a, 0x6c, 0xfb, 0xa5, 0x7c, 0x58, 0x8e]
        )
    );
    assert_eq!(
        SMM_PERFORMANCE_EX_PROTOCOL_GUID,
        EfiGuid::new(
            0x931f_c048,
            0xc71d,
            0x4455,
            [0x89, 0x30, 0x47, 0x06, 0x30, 0xe3, 0x0e, 0xe5]
        )
    );
    assert_eq!(PERFORMANCE_STRING_SIZE, 32);
}

#[test]
fn test_performance_measurement_wrapper() {
    let mock_proto = EfiPerformanceMeasurementProtocol {
        create_performance_measurement: mock_create_perf_measurement,
    };
    let client = unsafe { PerformanceMeasurement::from_raw(&raw const mock_proto) };

    assert!(
        client
            .create_measurement(
                None,
                None,
                Some(c"Entry1"),
                12345,
                0x8000,
                10,
                PerfMeasurementAttribute::PerfEntry,
            )
            .is_ok()
    );
    assert_eq!(
        RECORD_ATTR.load(Ordering::SeqCst),
        PerfMeasurementAttribute::PerfEntry as u32
    );
    assert_eq!(RECORD_TIMESTAMP.load(Ordering::SeqCst), 12345);
    assert_eq!(RECORD_ID.load(Ordering::SeqCst), 10);

    assert!(
        client
            .log_start(None, None, Some(c"StartEvent"), 5000, 20)
            .is_ok()
    );
    assert_eq!(
        RECORD_ATTR.load(Ordering::SeqCst),
        PerfMeasurementAttribute::PerfStartEntry as u32
    );
    assert_eq!(RECORD_TIMESTAMP.load(Ordering::SeqCst), 5000);
    assert_eq!(RECORD_ID.load(Ordering::SeqCst), 20);

    assert!(
        client
            .log_end(None, None, Some(c"EndEvent"), 6000, 20)
            .is_ok()
    );
    assert_eq!(
        RECORD_ATTR.load(Ordering::SeqCst),
        PerfMeasurementAttribute::PerfEndEntry as u32
    );
    assert_eq!(RECORD_TIMESTAMP.load(Ordering::SeqCst), 6000);
}

#[test]
fn test_performance_gauge_wrappers() {
    let mock_perf = EfiPerformanceProtocol {
        start_gauge: mock_start_gauge,
        end_gauge: mock_end_gauge,
        get_gauge: mock_get_gauge,
    };
    let client = unsafe { Performance::from_raw(&raw const mock_perf) };

    assert!(
        client
            .start_gauge(None, Some(c"TokenA"), Some(c"ModA"), 100)
            .is_ok()
    );
    assert_eq!(GAUGE_START_TIMESTAMP.load(Ordering::SeqCst), 100);

    assert!(
        client
            .end_gauge(None, Some(c"TokenA"), Some(c"ModA"), 200)
            .is_ok()
    );
    assert_eq!(GAUGE_END_TIMESTAMP.load(Ordering::SeqCst), 200);

    let gauge_entry = client.get_gauge(0);
    assert!(gauge_entry.is_ok());
    if let Ok(entry) = gauge_entry {
        assert_eq!(entry.handle, EfiPhysicalAddress::from_u64(0x1000));
        assert_eq!(entry.start_timestamp, 100);
        assert_eq!(entry.end_timestamp, 200);
    }

    assert_eq!(client.get_gauge(1).err(), Some(EfiStatus::NOT_FOUND));

    let mock_perf_ex = EfiPerformanceExProtocol {
        start_gauge_ex: mock_start_gauge_ex,
        end_gauge_ex: mock_end_gauge_ex,
        get_gauge_ex: mock_get_gauge_ex,
    };
    let client_ex = unsafe { PerformanceEx::from_raw(&raw const mock_perf_ex) };

    assert!(
        client_ex
            .start_gauge_ex(None, Some(c"TokenX"), Some(c"ModX"), 300, 77)
            .is_ok()
    );
    assert_eq!(GAUGE_START_TIMESTAMP.load(Ordering::SeqCst), 300);
    assert_eq!(GAUGE_EX_ID.load(Ordering::SeqCst), 77);

    assert!(
        client_ex
            .end_gauge_ex(None, Some(c"TokenX"), Some(c"ModX"), 500, 77)
            .is_ok()
    );
    assert_eq!(GAUGE_END_TIMESTAMP.load(Ordering::SeqCst), 500);

    let gauge_ex_entry = client_ex.get_gauge_ex(0);
    assert!(gauge_ex_entry.is_ok());
    if let Ok(entry) = gauge_ex_entry {
        assert_eq!(entry.handle, EfiPhysicalAddress::from_u64(0x2000));
        assert_eq!(entry.identifier, 42);
    }
}
