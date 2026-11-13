//! # OP-TEE Trusted Execution Environment Call Protocol
//!
//! Provides data structures and interfaces for interacting with OP-TEE OS
//! via ARM Secure Monitor Calls (SMC) or Hypervisor Calls (HVC).

use crate::base::EfiGuid;

/// OP-TEE OS 128-bit UID returned by the Trusted OS Call UID inquiry.
pub const OPTEE_OS_UID: [u32; 4] = [0x384F_B3E0, 0xE7F8_11E3, 0xAF63_0002, 0xA5D5_C51B];

/// OP-TEE OS UUID as an [`EfiGuid`].
///
/// `{384fb3e0-e7f8-11e3-af63-0002a5d5c51b}`
pub const OPTEE_OS_GUID: EfiGuid = EfiGuid::new(
    0x384F_B3E0,
    0xE7F8,
    0x11E3,
    [0xAF, 0x63, 0x00, 0x02, 0xA5, 0xD5, 0xC5, 0x1B],
);

/// Checks whether the 128-bit UID matches OP-TEE OS.
#[must_use]
pub const fn is_optee_uid(uid: &[u32; 4]) -> bool {
    uid[0] == OPTEE_OS_UID[0]
        && uid[1] == OPTEE_OS_UID[1]
        && uid[2] == OPTEE_OS_UID[2]
        && uid[3] == OPTEE_OS_UID[3]
}

// --- Parameter Attribute Types ---

/// No parameter.
pub const OPTEE_MSG_ATTR_TYPE_NONE: u64 = 0x0;
/// Value input parameter.
pub const OPTEE_MSG_ATTR_TYPE_VALUE_INPUT: u64 = 0x1;
/// Value output parameter.
pub const OPTEE_MSG_ATTR_TYPE_VALUE_OUTPUT: u64 = 0x2;
/// Value in-out parameter.
pub const OPTEE_MSG_ATTR_TYPE_VALUE_INOUT: u64 = 0x3;
/// Shared memory input buffer.
pub const OPTEE_MSG_ATTR_TYPE_MEMORY_INPUT: u64 = 0x9;
/// Shared memory output buffer.
pub const OPTEE_MSG_ATTR_TYPE_MEMORY_OUTPUT: u64 = 0xA;
/// Shared memory in-out buffer.
pub const OPTEE_MSG_ATTR_TYPE_MEMORY_INOUT: u64 = 0xB;

/// Mask for parameter attribute type.
pub const OPTEE_MSG_ATTR_TYPE_MASK: u64 = 0xFF;

// --- Return Status Codes ---

/// Operation completed successfully.
pub const OPTEE_SUCCESS: u32 = 0x0000_0000;
/// Error origin: communication layer.
pub const OPTEE_ORIGIN_COMMUNICATION: u32 = 0x0000_0002;
/// Communication error.
pub const OPTEE_ERROR_COMMUNICATION: u32 = 0xFFFF_000E;
/// Security violation.
pub const OPTEE_ERROR_SECURITY: u32 = 0xFFFF_000F;
/// Provided buffer is too small.
pub const OPTEE_ERROR_SHORT_BUFFER: u32 = 0xFFFF_0010;

/// Shared memory parameter descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpteeMsgParamMemory {
    /// Physical or intermediate physical address of shared memory buffer.
    pub buffer_address: u64,
    /// Size of the memory buffer in bytes.
    pub size: u64,
    /// Shared memory reference cookie.
    pub shared_memory_reference: u64,
}

/// Value-based parameter descriptor (registers A, B, C).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpteeMsgParamValue {
    /// Parameter A.
    pub a: u64,
    /// Parameter B.
    pub b: u64,
    /// Parameter C.
    pub c: u64,
}

/// Union of parameter contents.
#[repr(C)]
#[derive(Clone, Copy)]
pub union OpteeMsgParamData {
    /// Memory descriptor.
    pub memory: OpteeMsgParamMemory,
    /// Value descriptor.
    pub value: OpteeMsgParamValue,
}

impl core::fmt::Debug for OpteeMsgParamData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // SAFETY: Both union members are plain data of identical size (24 bytes).
        let val = unsafe { self.value };
        write!(
            f,
            "ParamData(a={:#x}, b={:#x}, c={:#x})",
            val.a, val.b, val.c
        )
    }
}

/// Parameter passed to OP-TEE command or session.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OpteeMsgParam {
    /// Parameter attribute flags and type.
    pub attribute: u64,
    /// Parameter data payload.
    pub data: OpteeMsgParamData,
}

impl Default for OpteeMsgParam {
    fn default() -> Self {
        Self {
            attribute: OPTEE_MSG_ATTR_TYPE_NONE,
            data: OpteeMsgParamData {
                value: OpteeMsgParamValue::default(),
            },
        }
    }
}

/// Maximum parameters passed in a standard OP-TEE message.
pub const OPTEE_MAX_CALL_PARAMS: usize = 4;

/// OP-TEE message header.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpteeMsgHeader {
    /// Command identifier.
    pub command: u32,
    /// Function identifier.
    pub function: u32,
    /// Session identifier.
    pub session: u32,
    /// Cancel request identifier.
    pub cancel_id: u32,
    /// Padding for 64-bit alignment.
    pub pad: u32,
    /// Return code.
    pub ret: u32,
    /// Return origin code.
    pub ret_origin: u32,
    /// Number of actual parameter elements following this header.
    pub num_params: u32,
}

impl OpteeMsgHeader {
    /// Creates a new message header with specified command, function, and session.
    #[must_use]
    pub const fn new(command: u32, function: u32, session: u32, num_params: u32) -> Self {
        Self {
            command,
            function,
            session,
            cancel_id: 0,
            pad: 0,
            ret: 0,
            ret_origin: 0,
            num_params,
        }
    }

    /// Checks if the message returned success.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.ret == OPTEE_SUCCESS
    }
}
