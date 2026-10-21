//! # IPMI Protocol
//!
//! Provides the Intelligent Platform Management Interface (IPMI) protocol
//! used to communicate with Baseboard Management Controllers (BMCs).
//!
//! Defined in the IPMI Specification and UEFI Manageability specifications.

use crate::base::{EfiGuid, EfiStatus};

/// The IPMI protocol `GUID`.
pub const IPMI_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xDBC6_381F,
    0x5554,
    0x4D14,
    [0x8F, 0xFD, 0x76, 0xD7, 0x87, 0xB8, 0xAC, 0xBF],
);

/// The SMM IPMI protocol `GUID`.
pub const SMM_IPMI_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5169_AF60,
    0x8C5A,
    0x4243,
    [0xB3, 0xE9, 0x56, 0xC5, 0x6D, 0x18, 0xEE, 0x26],
);

/// IPMI Net Function codes.
pub mod net_fn {
    /// Chassis commands.
    pub const CHASSIS: u8 = 0x00;
    /// Bridge commands.
    pub const BRIDGE: u8 = 0x02;
    /// Sensor/Event commands.
    pub const SENSOR_EVENT: u8 = 0x04;
    /// Application commands.
    pub const APP: u8 = 0x06;
    /// Firmware commands.
    pub const FIRMWARE: u8 = 0x08;
    /// Storage commands.
    pub const STORAGE: u8 = 0x0A;
    /// Transport commands.
    pub const TRANSPORT: u8 = 0x0C;
    /// Group extension commands.
    pub const GROUP_EXTENSION: u8 = 0x2C;
    /// OEM commands.
    pub const OEM: u8 = 0x2E;
}

/// IPMI Application Commands.
pub mod app_cmd {
    /// Get Device ID.
    pub const GET_DEVICE_ID: u8 = 0x01;
    /// Get Cold Reset.
    pub const COLD_RESET: u8 = 0x02;
    /// Get Warm Reset.
    pub const WARM_RESET: u8 = 0x03;
    /// Get Self Test Results.
    pub const GET_SELF_TEST_RESULTS: u8 = 0x04;
    /// Get Device GUID.
    pub const GET_DEVICE_GUID: u8 = 0x08;
    /// Reset Watchdog Timer.
    pub const RESET_WATCHDOG_TIMER: u8 = 0x22;
    /// Set Watchdog Timer.
    pub const SET_WATCHDOG_TIMER: u8 = 0x24;
    /// Get Watchdog Timer.
    pub const GET_WATCHDOG_TIMER: u8 = 0x25;
    /// Set BMC Global Enables.
    pub const SET_BMC_GLOBAL_ENABLES: u8 = 0x2E;
    /// Get BMC Global Enables.
    pub const GET_BMC_GLOBAL_ENABLES: u8 = 0x2F;
    /// Clear Message Flags.
    pub const CLEAR_MSG_FLAGS: u8 = 0x30;
    /// Get Message Flags.
    pub const GET_MSG_FLAGS: u8 = 0x31;
    /// Get Message.
    pub const GET_MSG: u8 = 0x33;
    /// Send Message.
    pub const SEND_MSG: u8 = 0x34;
    /// Read Event Message Buffer.
    pub const READ_EVENT_MSG_BUFFER: u8 = 0x35;
    /// Get System GUID.
    pub const GET_SYSTEM_GUID: u8 = 0x37;
}

/// IPMI Completion Codes.
pub mod completion_code {
    /// Command completed normally.
    pub const SUCCESS: u8 = 0x00;
    /// Node busy.
    pub const NODE_BUSY: u8 = 0xC0;
    /// Invalid command.
    pub const INVALID_COMMAND: u8 = 0xC1;
    /// Command invalid for given LUN.
    pub const INVALID_CMD_FOR_LUN: u8 = 0xC2;
    /// Timeout while processing command.
    pub const TIMEOUT: u8 = 0xC3;
    /// Out of storage space.
    pub const OUT_OF_SPACE: u8 = 0xC4;
    /// Reservation canceled or invalid reservation ID.
    pub const RESERVATION_CANCELED: u8 = 0xC5;
    /// Request data truncated.
    pub const REQ_DATA_TRUNCATED: u8 = 0xC6;
    /// Request data length invalid.
    pub const REQ_DATA_LEN_INVALID: u8 = 0xC7;
    /// Request data field length limit exceeded.
    pub const REQ_DATA_LEN_EXCEEDED: u8 = 0xC8;
    /// Parameter out of range.
    pub const PARAM_OUT_OF_RANGE: u8 = 0xC9;
    /// Cannot return number of requested data bytes.
    pub const CANNOT_RETURN_NUM_BYTES: u8 = 0xCA;
    /// Requested sensor, data, or record not present.
    pub const REQ_SENSOR_NOT_PRESENT: u8 = 0xCB;
    /// Invalid data field in request.
    pub const INVALID_DATA_FIELD: u8 = 0xCC;
    /// Command illegal for specified sensor/record type.
    pub const CMD_ILLEGAL_FOR_SENSOR: u8 = 0xCD;
    /// Unspecified error.
    pub const UNSPECIFIED_ERROR: u8 = 0xFF;
}

/// IPMI Keyboard Controller Style (KCS) Interface constants.
pub mod kcs {
    /// Output Buffer Full (status register bit 0).
    pub const STATUS_OBF: u8 = 1 << 0;
    /// Input Buffer Full (status register bit 1).
    pub const STATUS_IBF: u8 = 1 << 1;
    /// SMS Attention (status register bit 2).
    pub const STATUS_SMS_ATN: u8 = 1 << 2;
    /// Command / Data (status register bit 3): 1 = Command, 0 = Data.
    pub const STATUS_CD: u8 = 1 << 3;
    /// State mask (status register bits 6-7).
    pub const STATUS_STATE_MASK: u8 = 0xC0;

    /// KCS state: Idle state.
    pub const STATE_IDLE: u8 = 0x00;
    /// KCS state: Read state.
    pub const STATE_READ: u8 = 0x40;
    /// KCS state: Write state.
    pub const STATE_WRITE: u8 = 0x80;
    /// KCS state: Error state.
    pub const STATE_ERROR: u8 = 0xC0;

    /// KCS Control Code: Get Status / Abort.
    pub const CMD_GET_STATUS: u8 = 0x60;
    /// KCS Control Code: Write Start.
    pub const CMD_WRITE_START: u8 = 0x61;
    /// KCS Control Code: Write End.
    pub const CMD_WRITE_END: u8 = 0x62;
    /// KCS Control Code: Read.
    pub const CMD_READ: u8 = 0x68;
}

/// Submits an IPMI command to the BMC.
pub type EfiIpmiSubmitCommand = unsafe extern "efiapi" fn(
    this: *mut EfiIpmiProtocol,
    net_function: u8,
    command: u8,
    request_data: *const u8,
    request_data_size: u32,
    response_data: *mut u8,
    response_data_size: *mut u32,
) -> EfiStatus;

/// The EFI IPMI Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiIpmiProtocol {
    /// Submits a command byte stream via IPMI and receives response.
    pub submit_command: EfiIpmiSubmitCommand,
}
