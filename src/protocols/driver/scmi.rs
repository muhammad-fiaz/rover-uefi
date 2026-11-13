//! # ARM System Control and Management Interface (SCMI) Protocols
//!
//! Provides native Rust protocol definitions and safe client wrappers for the
//! ARM System Control and Management Interface (SCMI) Base, Clock, and Performance
//! protocols as defined in the ARM SCMI Specification (DEN0056).

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// SCMI Base Protocol `GUID`.
pub const ARM_SCMI_BASE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD7E5_ABE9,
    0x33AB,
    0x418E,
    [0x9F, 0x91, 0x72, 0xDA, 0xE2, 0xBA, 0x8E, 0x2F],
);

/// SCMI Clock Protocol `GUID`.
pub const ARM_SCMI_CLOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x91CE_67A8,
    0xE0AA,
    0x4012,
    [0xB9, 0x9F, 0xB6, 0xFC, 0xF3, 0x04, 0x8E, 0xAA],
);

/// SCMI Clock2 Protocol `GUID`.
pub const ARM_SCMI_CLOCK2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB8D8_CAF2,
    0x9E94,
    0x462C,
    [0xA8, 0x34, 0x6C, 0x99, 0xFC, 0x05, 0xEF, 0xCF],
);

/// SCMI Performance Protocol `GUID`.
pub const ARM_SCMI_PERFORMANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x09B8_BA84,
    0x3DD3,
    0x49A6,
    [0xA0, 0x5A, 0x31, 0x34, 0xA5, 0xF0, 0x7B, 0xAD],
);

/// Maximum length of vendor or clock name ASCII strings in bytes.
pub const SCMI_MAX_STR_LEN: usize = 16;

/// SCMI Base Protocol Version 1.0.
pub const SCMI_BASE_PROTOCOL_VERSION_V1: u32 = 0x10000;
/// SCMI Base Protocol Version 2.0.
pub const SCMI_BASE_PROTOCOL_VERSION_V2: u32 = 0x20000;

/// SCMI Clock Protocol Version 1.0.
pub const SCMI_CLOCK_PROTOCOL_VERSION: u32 = 0x10000;
/// SCMI Clock2 Protocol Version 1.
pub const SCMI_CLOCK2_PROTOCOL_VERSION: u32 = 1;

/// Clock rate representation format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ScmiClockRateFormat {
    /// Non-linear, discrete list of frequencies.
    Discrete = 0,
    /// Linear range with minimum, maximum, and step size.
    Linear = 1,
}

/// Linear clock rate range triplet (min, max, step).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ScmiClockRateRange {
    /// Minimum supported rate in Hz.
    pub min: u64,
    /// Maximum supported rate in Hz.
    pub max: u64,
    /// Frequency step size in Hz.
    pub step: u64,
}

/// SCMI clock rate union, representing either a discrete rate or a linear range.
#[repr(C)]
#[derive(Clone, Copy)]
pub union ScmiClockRate {
    /// Discrete rate in Hz.
    pub rate: u64,
    /// Linear rate range.
    pub range: ScmiClockRateRange,
}

impl core::fmt::Debug for ScmiClockRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ScmiClockRate")
            .field("rate", &unsafe { self.rate })
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Base Protocol Function Pointers and ABI
// ---------------------------------------------------------------------------

/// Returns the protocol version supported by SCP firmware.
pub type EfiScmiBaseGetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiScmiBaseProtocol, version: *mut u32) -> EfiStatus;

/// Returns total number of protocols supported.
pub type EfiScmiBaseGetTotalProtocols =
    unsafe extern "efiapi" fn(this: *mut EfiScmiBaseProtocol, num: *mut u32) -> EfiStatus;

/// Discovers vendor identifier string.
pub type EfiScmiBaseDiscoverVendor = unsafe extern "efiapi" fn(
    this: *mut EfiScmiBaseProtocol,
    vendor_identifier: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus;

/// Discovers sub-vendor identifier string.
pub type EfiScmiBaseDiscoverSubVendor = unsafe extern "efiapi" fn(
    this: *mut EfiScmiBaseProtocol,
    vendor_identifier: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus;

/// Discovers vendor-specific implementation version.
pub type EfiScmiBaseDiscoverImplementationVersion = unsafe extern "efiapi" fn(
    this: *mut EfiScmiBaseProtocol,
    implementation_version: *mut u32,
) -> EfiStatus;

/// Discovers list of supported protocol IDs.
pub type EfiScmiBaseDiscoverListProtocols = unsafe extern "efiapi" fn(
    this: *mut EfiScmiBaseProtocol,
    protocol_list_size: *mut u32,
    protocol_list: *mut u8,
) -> EfiStatus;

/// Interface structure for SCMI Base Protocol (`SCMI_BASE_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiBaseProtocol {
    /// Returns the SCMI base protocol version.
    pub get_version: EfiScmiBaseGetVersion,
    /// Returns total number of SCMI protocols supported.
    pub get_total_protocols: EfiScmiBaseGetTotalProtocols,
    /// Returns vendor identifier string.
    pub discover_vendor: EfiScmiBaseDiscoverVendor,
    /// Returns sub-vendor identifier string.
    pub discover_sub_vendor: EfiScmiBaseDiscoverSubVendor,
    /// Returns vendor implementation version.
    pub discover_implementation_version: EfiScmiBaseDiscoverImplementationVersion,
    /// Returns list of supported protocol identifiers.
    pub discover_list_protocols: EfiScmiBaseDiscoverListProtocols,
}

// ---------------------------------------------------------------------------
// Clock Protocol Function Pointers and ABI
// ---------------------------------------------------------------------------

/// Returns clock protocol version.
pub type EfiScmiClockGetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiScmiClockProtocol, version: *mut u32) -> EfiStatus;

/// Returns total number of clocks.
pub type EfiScmiClockGetTotalClocks =
    unsafe extern "efiapi" fn(this: *mut EfiScmiClockProtocol, total_clocks: *mut u32) -> EfiStatus;

/// Returns attributes of a clock device.
pub type EfiScmiClockGetClockAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    enabled: *mut bool,
    ascii_name: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus;

/// Returns list of supported rates for a clock device.
pub type EfiScmiClockDescribeRates = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    format: *mut ScmiClockRateFormat,
    total_rates: *mut u32,
    rate_array_size: *mut u32,
    rate_array: *mut ScmiClockRate,
) -> EfiStatus;

/// Returns clock rate in Hz.
pub type EfiScmiClockRateGet = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    rate: *mut u64,
) -> EfiStatus;

/// Sets clock rate in Hz.
pub type EfiScmiClockRateSet = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    rate: u64,
) -> EfiStatus;

/// Interface structure for SCMI Clock Protocol (`SCMI_CLOCK_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiClockProtocol {
    /// Returns clock protocol version.
    pub get_version: EfiScmiClockGetVersion,
    /// Returns total number of clock devices.
    pub get_total_clocks: EfiScmiClockGetTotalClocks,
    /// Returns clock device attributes.
    pub get_clock_attributes: EfiScmiClockGetClockAttributes,
    /// Returns supported clock rate ranges or frequencies.
    pub describe_rates: EfiScmiClockDescribeRates,
    /// Returns current clock rate in Hz.
    pub rate_get: EfiScmiClockRateGet,
    /// Sets current clock rate in Hz.
    pub rate_set: EfiScmiClockRateSet,
}

// ---------------------------------------------------------------------------
// Clock2 Protocol Function Pointers and ABI
// ---------------------------------------------------------------------------

/// Returns clock2 protocol version.
pub type EfiScmiClock2GetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiScmiClock2Protocol, version: *mut u32) -> EfiStatus;

/// Returns total number of clocks for clock2.
pub type EfiScmiClock2GetTotalClocks = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    total_clocks: *mut u32,
) -> EfiStatus;

/// Returns attributes of a clock device for clock2.
pub type EfiScmiClock2GetClockAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    enabled: *mut bool,
    ascii_name: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus;

/// Returns list of supported rates for a clock device for clock2.
pub type EfiScmiClock2DescribeRates = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    format: *mut ScmiClockRateFormat,
    total_rates: *mut u32,
    rate_array_size: *mut u32,
    rate_array: *mut ScmiClockRate,
) -> EfiStatus;

/// Returns clock rate in Hz for clock2.
pub type EfiScmiClock2RateGet = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    rate: *mut u64,
) -> EfiStatus;

/// Sets clock rate in Hz for clock2.
pub type EfiScmiClock2RateSet = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    rate: u64,
) -> EfiStatus;

/// Enables or disables specified clock device in clock2.
pub type EfiScmiClock2Enable = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    enable: bool,
) -> EfiStatus;

/// Interface structure for SCMI Clock2 Protocol (`SCMI_CLOCK2_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiClock2Protocol {
    /// Returns clock protocol version.
    pub get_version: EfiScmiClock2GetVersion,
    /// Returns total number of clock devices.
    pub get_total_clocks: EfiScmiClock2GetTotalClocks,
    /// Returns clock device attributes.
    pub get_clock_attributes: EfiScmiClock2GetClockAttributes,
    /// Returns supported clock rate ranges or frequencies.
    pub describe_rates: EfiScmiClock2DescribeRates,
    /// Returns current clock rate in Hz.
    pub rate_get: EfiScmiClock2RateGet,
    /// Sets current clock rate in Hz.
    pub rate_set: EfiScmiClock2RateSet,
    /// Extension version field.
    pub version: UIntN,
    /// Enables or disables the clock device.
    pub enable: EfiScmiClock2Enable,
}

// ---------------------------------------------------------------------------
// Performance Protocol ABI
// ---------------------------------------------------------------------------

/// SCMI Performance Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiPerformanceProtocol {
    /// Protocol version.
    pub version: u32,
}

// ---------------------------------------------------------------------------
// Safe Native Rust Client Wrappers
// ---------------------------------------------------------------------------

/// Safe wrapper around [`EfiScmiBaseProtocol`].
pub struct ScmiBase<'a> {
    proto: &'a mut EfiScmiBaseProtocol,
}

impl<'a> ScmiBase<'a> {
    /// Wraps a mutable reference to [`EfiScmiBaseProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiScmiBaseProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiScmiBaseProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiScmiBaseProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Retrieves the SCMI Base protocol version.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn version(&mut self) -> Result<u32, EfiStatus> {
        let mut ver = 0u32;
        let status = unsafe { (self.proto.get_version)(self.proto, core::ptr::from_mut(&mut ver)) };
        if status == EfiStatus::SUCCESS {
            Ok(ver)
        } else {
            Err(status)
        }
    }

    /// Retrieves total number of SCMI protocols supported by SCP firmware.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn total_protocols(&mut self) -> Result<u32, EfiStatus> {
        let mut total = 0u32;
        let status = unsafe {
            (self.proto.get_total_protocols)(self.proto, core::ptr::from_mut(&mut total))
        };
        if status == EfiStatus::SUCCESS {
            Ok(total)
        } else {
            Err(status)
        }
    }

    /// Discovers vendor name identifier string.
    ///
    /// # Errors
    ///
    /// Returns an error status if vendor discovery fails.
    pub fn discover_vendor(&mut self) -> Result<[u8; SCMI_MAX_STR_LEN], EfiStatus> {
        let mut vendor = [0u8; SCMI_MAX_STR_LEN];
        let status =
            unsafe { (self.proto.discover_vendor)(self.proto, core::ptr::from_mut(&mut vendor)) };
        if status == EfiStatus::SUCCESS {
            Ok(vendor)
        } else {
            Err(status)
        }
    }

    /// Discovers sub-vendor name identifier string.
    ///
    /// # Errors
    ///
    /// Returns an error status if sub-vendor discovery fails.
    pub fn discover_sub_vendor(&mut self) -> Result<[u8; SCMI_MAX_STR_LEN], EfiStatus> {
        let mut sub_vendor = [0u8; SCMI_MAX_STR_LEN];
        let status = unsafe {
            (self.proto.discover_sub_vendor)(self.proto, core::ptr::from_mut(&mut sub_vendor))
        };
        if status == EfiStatus::SUCCESS {
            Ok(sub_vendor)
        } else {
            Err(status)
        }
    }

    /// Discovers implementation version.
    ///
    /// # Errors
    ///
    /// Returns an error status if implementation version query fails.
    pub fn discover_implementation_version(&mut self) -> Result<u32, EfiStatus> {
        let mut impl_ver = 0u32;
        let status = unsafe {
            (self.proto.discover_implementation_version)(
                self.proto,
                core::ptr::from_mut(&mut impl_ver),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(impl_ver)
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around [`EfiScmiClockProtocol`].
pub struct ScmiClock<'a> {
    proto: &'a mut EfiScmiClockProtocol,
}

impl<'a> ScmiClock<'a> {
    /// Wraps a mutable reference to [`EfiScmiClockProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiScmiClockProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiScmiClockProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiScmiClockProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Retrieves the Clock protocol version.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn version(&mut self) -> Result<u32, EfiStatus> {
        let mut ver = 0u32;
        let status = unsafe { (self.proto.get_version)(self.proto, core::ptr::from_mut(&mut ver)) };
        if status == EfiStatus::SUCCESS {
            Ok(ver)
        } else {
            Err(status)
        }
    }

    /// Retrieves the total number of clocks supported by firmware.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn total_clocks(&mut self) -> Result<u32, EfiStatus> {
        let mut total = 0u32;
        let status =
            unsafe { (self.proto.get_total_clocks)(self.proto, core::ptr::from_mut(&mut total)) };
        if status == EfiStatus::SUCCESS {
            Ok(total)
        } else {
            Err(status)
        }
    }

    /// Retrieves clock attributes for `clock_id`.
    ///
    /// Returns `(enabled, ascii_name)`.
    ///
    /// # Errors
    ///
    /// Returns an error status if `clock_id` is invalid or cannot be queried.
    pub fn get_clock_attributes(
        &mut self,
        clock_id: u32,
    ) -> Result<(bool, [u8; SCMI_MAX_STR_LEN]), EfiStatus> {
        let mut enabled = false;
        let mut name = [0u8; SCMI_MAX_STR_LEN];
        let status = unsafe {
            (self.proto.get_clock_attributes)(
                self.proto,
                clock_id,
                core::ptr::from_mut(&mut enabled),
                core::ptr::from_mut(&mut name),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((enabled, name))
        } else {
            Err(status)
        }
    }

    /// Retrieves current clock frequency in Hz for `clock_id`.
    ///
    /// # Errors
    ///
    /// Returns an error status if rate query fails.
    pub fn get_rate(&mut self, clock_id: u32) -> Result<u64, EfiStatus> {
        let mut rate = 0u64;
        let status =
            unsafe { (self.proto.rate_get)(self.proto, clock_id, core::ptr::from_mut(&mut rate)) };
        if status == EfiStatus::SUCCESS {
            Ok(rate)
        } else {
            Err(status)
        }
    }

    /// Sets clock frequency in Hz for `clock_id`.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting rate fails.
    pub fn set_rate(&mut self, clock_id: u32, rate: u64) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.rate_set)(self.proto, clock_id, rate) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around [`EfiScmiClock2Protocol`].
pub struct ScmiClock2<'a> {
    proto: &'a mut EfiScmiClock2Protocol,
}

impl<'a> ScmiClock2<'a> {
    /// Wraps a mutable reference to [`EfiScmiClock2Protocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiScmiClock2Protocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiScmiClock2Protocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiScmiClock2Protocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Retrieves the Clock2 protocol version.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn version(&mut self) -> Result<u32, EfiStatus> {
        let mut ver = 0u32;
        let status = unsafe { (self.proto.get_version)(self.proto, core::ptr::from_mut(&mut ver)) };
        if status == EfiStatus::SUCCESS {
            Ok(ver)
        } else {
            Err(status)
        }
    }

    /// Retrieves the total number of clocks supported by firmware.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware communication fails.
    pub fn total_clocks(&mut self) -> Result<u32, EfiStatus> {
        let mut total = 0u32;
        let status =
            unsafe { (self.proto.get_total_clocks)(self.proto, core::ptr::from_mut(&mut total)) };
        if status == EfiStatus::SUCCESS {
            Ok(total)
        } else {
            Err(status)
        }
    }

    /// Retrieves current clock frequency in Hz for `clock_id`.
    ///
    /// # Errors
    ///
    /// Returns an error status if rate query fails.
    pub fn get_rate(&mut self, clock_id: u32) -> Result<u64, EfiStatus> {
        let mut rate = 0u64;
        let status =
            unsafe { (self.proto.rate_get)(self.proto, clock_id, core::ptr::from_mut(&mut rate)) };
        if status == EfiStatus::SUCCESS {
            Ok(rate)
        } else {
            Err(status)
        }
    }

    /// Sets clock frequency in Hz for `clock_id`.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting rate fails.
    pub fn set_rate(&mut self, clock_id: u32, rate: u64) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.rate_set)(self.proto, clock_id, rate) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Enables or disables the specified clock device.
    ///
    /// # Errors
    ///
    /// Returns an error status if enable/disable fails.
    pub fn enable(&mut self, clock_id: u32, enable: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.enable)(self.proto, clock_id, enable) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
