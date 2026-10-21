//! # ARM System Control and Management Interface (SCMI) Protocols
//!
//! Provides SCMI Base, Clock, and Performance protocol definitions
//! defined in the ARM SCMI Specification.

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

/// SCMI Performance Protocol `GUID`.
pub const ARM_SCMI_PERFORMANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x09B8_BA84,
    0x3DD3,
    0x49A6,
    [0xA0, 0x5A, 0x31, 0x34, 0xA5, 0xF0, 0x7B, 0xAD],
);

/// Returns the protocol version supported by SCP firmware.
pub type EfiScmiGetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiScmiBaseProtocol, version: *mut u32) -> EfiStatus;

/// Returns total number of protocols supported.
pub type EfiScmiGetTotalNumProtocols =
    unsafe extern "efiapi" fn(this: *mut EfiScmiBaseProtocol, num: *mut u32) -> EfiStatus;

/// SCMI Base Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiBaseProtocol {
    /// Gets protocol version.
    pub get_version: EfiScmiGetVersion,
    /// Gets total number of protocols.
    pub get_total_num_protocols: EfiScmiGetTotalNumProtocols,
}

/// Returns clock protocol version.
pub type EfiScmiClockGetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiScmiClockProtocol, version: *mut u32) -> EfiStatus;

/// Returns total number of clocks.
pub type EfiScmiClockGetTotalNumClocks =
    unsafe extern "efiapi" fn(this: *mut EfiScmiClockProtocol, num: *mut u32) -> EfiStatus;

/// Returns clock rate in Hz.
pub type EfiScmiClockGetRate = unsafe extern "efiapi" fn(
    this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    rate: *mut u64,
) -> EfiStatus;

/// SCMI Clock Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiClockProtocol {
    /// Gets clock protocol version.
    pub get_version: EfiScmiClockGetVersion,
    /// Gets total number of clocks.
    pub get_total_num_clocks: EfiScmiClockGetTotalNumClocks,
    /// Gets clock rate in Hz.
    pub get_rate: EfiScmiClockGetRate,
}

/// SCMI Performance Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiScmiPerformanceProtocol {
    /// Protocol version.
    pub version: u32,
}
