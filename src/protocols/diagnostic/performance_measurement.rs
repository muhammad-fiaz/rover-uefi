//! # Performance Measurement and Gauge Protocols
//!
//! Provides logging and retrieval of performance timestamp records
//! during firmware execution phases (PEI, DXE, SMM).

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// Performance measurement protocol GUID.
pub const PERFORMANCE_MEASUREMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xc85d_06be,
    0x5f75,
    0x48ce,
    [0xa8, 0x0f, 0x12, 0x36, 0xba, 0x3b, 0x87, 0xb1],
);

/// SMM performance measurement protocol GUID.
pub const SMM_PERFORMANCE_MEASUREMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xd56b_6d73,
    0x1a7b,
    0x4015,
    [0x9b, 0xb4, 0x7b, 0x07, 0x17, 0x29, 0xed, 0x24],
);

/// Performance protocol GUID.
pub const PERFORMANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x76b6_bdfa,
    0x2acd,
    0x4462,
    [0x9e, 0x3f, 0xcb, 0x58, 0xc9, 0x69, 0xd9, 0x37],
);

/// Extended performance protocol GUID.
pub const PERFORMANCE_EX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1ea8_1bec,
    0xf01a,
    0x4d98,
    [0xa2, 0x01, 0x4a, 0x61, 0xce, 0x2f, 0xc0, 0x22],
);

/// SMM performance protocol GUID.
pub const SMM_PERFORMANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xf866_226a,
    0xeaa5,
    0x4f5a,
    [0xa9, 0x0a, 0x6c, 0xfb, 0xa5, 0x7c, 0x58, 0x8e],
);

/// SMM extended performance protocol GUID.
pub const SMM_PERFORMANCE_EX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x931f_c048,
    0xc71d,
    0x4455,
    [0x89, 0x30, 0x47, 0x06, 0x30, 0xe3, 0x0e, 0xe5],
);

/// Maximum token string length for DXE/SMM performance records.
pub const PERFORMANCE_STRING_SIZE: usize = 32;

/// Performance measurement record attribute.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerfMeasurementAttribute {
    /// Start marker for a timed measurement.
    PerfStartEntry = 0,
    /// End marker for a timed measurement.
    PerfEndEntry = 1,
    /// General point-in-time measurement record.
    PerfEntry = 2,
}

/// Creates a performance record with event description and timestamp.
pub type CreatePerformanceMeasurement = unsafe extern "efiapi" fn(
    caller_identifier: *const c_void,
    guid: *const EfiGuid,
    string: *const u8,
    timestamp: u64,
    address: u64,
    identifier: u32,
    attribute: PerfMeasurementAttribute,
) -> EfiStatus;

/// Low-level ABI structure for the performance measurement protocol.
#[repr(C)]
pub struct EfiPerformanceMeasurementProtocol {
    /// Creates a performance record.
    pub create_performance_measurement: CreatePerformanceMeasurement,
}

/// Gauge data entry for basic performance protocol.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GaugeDataEntry {
    /// Context or image handle.
    pub handle: EfiPhysicalAddress,
    /// Measured token string name (ASCII, null-terminated).
    pub token: [u8; PERFORMANCE_STRING_SIZE],
    /// Module string name (ASCII, null-terminated).
    pub module: [u8; PERFORMANCE_STRING_SIZE],
    /// Start timestamp count.
    pub start_timestamp: u64,
    /// End timestamp count.
    pub end_timestamp: u64,
}

/// Gauge data entry for extended performance protocol.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GaugeDataEntryEx {
    /// Context or image handle.
    pub handle: EfiPhysicalAddress,
    /// Measured token string name (ASCII, null-terminated).
    pub token: [u8; PERFORMANCE_STRING_SIZE],
    /// Module string name (ASCII, null-terminated).
    pub module: [u8; PERFORMANCE_STRING_SIZE],
    /// Start timestamp count.
    pub start_timestamp: u64,
    /// End timestamp count.
    pub end_timestamp: u64,
    /// 32-bit measurement identifier.
    pub identifier: u32,
}

/// Function pointer to start a basic gauge measurement.
pub type PerformanceStartGauge = unsafe extern "efiapi" fn(
    handle: *const c_void,
    token: *const u8,
    module: *const u8,
    timestamp: u64,
) -> EfiStatus;

/// Function pointer to end a basic gauge measurement.
pub type PerformanceEndGauge = unsafe extern "efiapi" fn(
    handle: *const c_void,
    token: *const u8,
    module: *const u8,
    timestamp: u64,
) -> EfiStatus;

/// Function pointer to retrieve a basic gauge record.
pub type PerformanceGetGauge = unsafe extern "efiapi" fn(
    log_entry_key: UIntN,
    gauge_data_entry: *mut *const GaugeDataEntry,
) -> EfiStatus;

/// Function pointer to start an extended gauge measurement.
pub type PerformanceStartGaugeEx = unsafe extern "efiapi" fn(
    handle: *const c_void,
    token: *const u8,
    module: *const u8,
    timestamp: u64,
    identifier: u32,
) -> EfiStatus;

/// Function pointer to end an extended gauge measurement.
pub type PerformanceEndGaugeEx = unsafe extern "efiapi" fn(
    handle: *const c_void,
    token: *const u8,
    module: *const u8,
    timestamp: u64,
    identifier: u32,
) -> EfiStatus;

/// Function pointer to retrieve an extended gauge record.
pub type PerformanceGetGaugeEx = unsafe extern "efiapi" fn(
    log_entry_key: UIntN,
    gauge_data_entry_ex: *mut *const GaugeDataEntryEx,
) -> EfiStatus;

/// Low-level ABI structure for the basic performance protocol.
#[repr(C)]
pub struct EfiPerformanceProtocol {
    /// Starts gauge measurement.
    pub start_gauge: PerformanceStartGauge,
    /// Ends gauge measurement.
    pub end_gauge: PerformanceEndGauge,
    /// Retrieves gauge measurement record.
    pub get_gauge: PerformanceGetGauge,
}

/// Low-level ABI structure for the extended performance protocol.
#[repr(C)]
pub struct EfiPerformanceExProtocol {
    /// Starts extended gauge measurement.
    pub start_gauge_ex: PerformanceStartGaugeEx,
    /// Ends extended gauge measurement.
    pub end_gauge_ex: PerformanceEndGaugeEx,
    /// Retrieves extended gauge measurement record.
    pub get_gauge_ex: PerformanceGetGaugeEx,
}

/// Safe client wrapper around `EfiPerformanceMeasurementProtocol`.
pub struct PerformanceMeasurement<'a> {
    proto: &'a EfiPerformanceMeasurementProtocol,
}

impl PerformanceMeasurement<'_> {
    /// Wraps a raw pointer to an `EfiPerformanceMeasurementProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `proto` points to a valid, live
    /// `EfiPerformanceMeasurementProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPerformanceMeasurementProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Creates a performance measurement record.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if record creation failed or parameters were invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn create_measurement(
        &self,
        caller_identifier: Option<*const c_void>,
        guid: Option<&EfiGuid>,
        string: Option<&core::ffi::CStr>,
        timestamp: u64,
        address: u64,
        identifier: u32,
        attribute: PerfMeasurementAttribute,
    ) -> Result<(), EfiStatus> {
        let caller_ptr = caller_identifier.unwrap_or(core::ptr::null());
        let guid_ptr = guid.map_or(core::ptr::null(), core::ptr::from_ref::<EfiGuid>);
        let str_ptr = string.map_or(core::ptr::null(), |s| s.as_ptr().cast::<u8>());

        let status = unsafe {
            (self.proto.create_performance_measurement)(
                caller_ptr, guid_ptr, str_ptr, timestamp, address, identifier, attribute,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Logs the start of an event or module execution.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if logging the start record failed.
    pub fn log_start(
        &self,
        caller_identifier: Option<*const c_void>,
        guid: Option<&EfiGuid>,
        string: Option<&core::ffi::CStr>,
        timestamp: u64,
        identifier: u32,
    ) -> Result<(), EfiStatus> {
        self.create_measurement(
            caller_identifier,
            guid,
            string,
            timestamp,
            0,
            identifier,
            PerfMeasurementAttribute::PerfStartEntry,
        )
    }

    /// Logs the end of an event or module execution.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if logging the end record failed.
    pub fn log_end(
        &self,
        caller_identifier: Option<*const c_void>,
        guid: Option<&EfiGuid>,
        string: Option<&core::ffi::CStr>,
        timestamp: u64,
        identifier: u32,
    ) -> Result<(), EfiStatus> {
        self.create_measurement(
            caller_identifier,
            guid,
            string,
            timestamp,
            0,
            identifier,
            PerfMeasurementAttribute::PerfEndEntry,
        )
    }
}

/// Safe client wrapper around `EfiPerformanceProtocol`.
pub struct Performance<'a> {
    proto: &'a EfiPerformanceProtocol,
}

impl<'a> Performance<'a> {
    /// Wraps a raw pointer to an `EfiPerformanceProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `proto` points to a valid, live
    /// `EfiPerformanceProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPerformanceProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Starts a gauge performance measurement.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if recording could not be started.
    pub fn start_gauge(
        &self,
        handle: Option<*const c_void>,
        token: Option<&core::ffi::CStr>,
        module: Option<&core::ffi::CStr>,
        timestamp: u64,
    ) -> Result<(), EfiStatus> {
        let handle_ptr = handle.unwrap_or(core::ptr::null());
        let token_ptr = token.map_or(core::ptr::null(), |t| t.as_ptr().cast::<u8>());
        let module_ptr = module.map_or(core::ptr::null(), |m| m.as_ptr().cast::<u8>());

        let status =
            unsafe { (self.proto.start_gauge)(handle_ptr, token_ptr, module_ptr, timestamp) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Ends a gauge performance measurement.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if recording could not be ended.
    pub fn end_gauge(
        &self,
        handle: Option<*const c_void>,
        token: Option<&core::ffi::CStr>,
        module: Option<&core::ffi::CStr>,
        timestamp: u64,
    ) -> Result<(), EfiStatus> {
        let handle_ptr = handle.unwrap_or(core::ptr::null());
        let token_ptr = token.map_or(core::ptr::null(), |t| t.as_ptr().cast::<u8>());
        let module_ptr = module.map_or(core::ptr::null(), |m| m.as_ptr().cast::<u8>());

        let status =
            unsafe { (self.proto.end_gauge)(handle_ptr, token_ptr, module_ptr, timestamp) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves a gauge data entry by key.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if retrieval failed or entry key was not found.
    pub fn get_gauge(&self, log_entry_key: UIntN) -> Result<&'a GaugeDataEntry, EfiStatus> {
        let mut entry_ptr: *const GaugeDataEntry = core::ptr::null();
        let status = unsafe { (self.proto.get_gauge)(log_entry_key, &raw mut entry_ptr) };
        if status == EfiStatus::SUCCESS && !entry_ptr.is_null() {
            unsafe { Ok(&*entry_ptr) }
        } else if status == EfiStatus::SUCCESS {
            Err(EfiStatus::NOT_FOUND)
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiPerformanceExProtocol`.
pub struct PerformanceEx<'a> {
    proto: &'a EfiPerformanceExProtocol,
}

impl<'a> PerformanceEx<'a> {
    /// Wraps a raw pointer to an `EfiPerformanceExProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `proto` points to a valid, live
    /// `EfiPerformanceExProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPerformanceExProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Starts an extended gauge performance measurement.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if recording could not be started.
    pub fn start_gauge_ex(
        &self,
        handle: Option<*const c_void>,
        token: Option<&core::ffi::CStr>,
        module: Option<&core::ffi::CStr>,
        timestamp: u64,
        identifier: u32,
    ) -> Result<(), EfiStatus> {
        let handle_ptr = handle.unwrap_or(core::ptr::null());
        let token_ptr = token.map_or(core::ptr::null(), |t| t.as_ptr().cast::<u8>());
        let module_ptr = module.map_or(core::ptr::null(), |m| m.as_ptr().cast::<u8>());

        let status = unsafe {
            (self.proto.start_gauge_ex)(handle_ptr, token_ptr, module_ptr, timestamp, identifier)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Ends an extended gauge performance measurement.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if recording could not be ended.
    pub fn end_gauge_ex(
        &self,
        handle: Option<*const c_void>,
        token: Option<&core::ffi::CStr>,
        module: Option<&core::ffi::CStr>,
        timestamp: u64,
        identifier: u32,
    ) -> Result<(), EfiStatus> {
        let handle_ptr = handle.unwrap_or(core::ptr::null());
        let token_ptr = token.map_or(core::ptr::null(), |t| t.as_ptr().cast::<u8>());
        let module_ptr = module.map_or(core::ptr::null(), |m| m.as_ptr().cast::<u8>());

        let status = unsafe {
            (self.proto.end_gauge_ex)(handle_ptr, token_ptr, module_ptr, timestamp, identifier)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves an extended gauge data entry by key.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if retrieval failed or entry key was not found.
    pub fn get_gauge_ex(&self, log_entry_key: UIntN) -> Result<&'a GaugeDataEntryEx, EfiStatus> {
        let mut entry_ptr: *const GaugeDataEntryEx = core::ptr::null();
        let status = unsafe { (self.proto.get_gauge_ex)(log_entry_key, &raw mut entry_ptr) };
        if status == EfiStatus::SUCCESS && !entry_ptr.is_null() {
            unsafe { Ok(&*entry_ptr) }
        } else if status == EfiStatus::SUCCESS {
            Err(EfiStatus::NOT_FOUND)
        } else {
            Err(status)
        }
    }
}
