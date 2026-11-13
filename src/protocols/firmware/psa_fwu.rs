//! # Platform Security Architecture (PSA) Firmware Update
//!
//! Provides data structures, command codes, and parsing helpers for the Arm
//! Platform Security Architecture Firmware Update for the A-profile specification
//! (ARM DEN0118). Used for out-of-band and Management Mode (MM) firmware updating.

use crate::base::EfiGuid;

/// PSA Firmware Update Agent GUID.
///
/// `{6823a838-1b06-470e-9774-0cce8bfb53fd}`
pub const PSA_FWU_UPDATE_AGENT_GUID: EfiGuid = EfiGuid::new(
    0x6823_A838,
    0x1B06,
    0x470E,
    [0x97, 0x74, 0x0C, 0xCE, 0x8B, 0xFB, 0x53, 0xFD],
);

/// PSA Firmware Update Image Directory GUID.
///
/// `{deee58d9-5147-4ad3-a290-77666e2341a5}`
pub const PSA_FWU_IMAGE_DIRECTORY_GUID: EfiGuid = EfiGuid::new(
    0xDEEE_58D9,
    0x5147,
    0x4AD3,
    [0xA2, 0x90, 0x77, 0x66, 0x6E, 0x23, 0x41, 0xA5],
);

/// PSA Firmware Update Metadata GUID.
///
/// `{8a7a84a0-8387-40f6-ab41-a8b9a5a60d23}`
pub const PSA_FWU_METADATA_GUID: EfiGuid = EfiGuid::new(
    0x8A7A_84A0,
    0x8387,
    0x40F6,
    [0xAB, 0x41, 0xA8, 0xB9, 0xA5, 0xA6, 0x0D, 0x23],
);

// --- Command IDs (DEN0118 section 3.4) ---

/// Discover firmware update service capabilities and image inventory.
pub const PSA_MM_FWU_COMMAND_DISCOVER: u32 = 0;
/// Begin firmware staging session.
pub const PSA_MM_FWU_COMMAND_BEGIN_STAGING: u32 = 16;
/// End firmware staging session.
pub const PSA_MM_FWU_COMMAND_END_STAGING: u32 = 17;
/// Cancel active staging session.
pub const PSA_MM_FWU_COMMAND_CANCEL_STAGING: u32 = 18;
/// Open a stream for writing or reading image payload.
pub const PSA_MM_FWU_COMMAND_OPEN: u32 = 19;
/// Write chunk to an open firmware stream.
pub const PSA_MM_FWU_COMMAND_WRITE_STREAM: u32 = 20;
/// Read chunk from an open firmware stream.
pub const PSA_MM_FWU_COMMAND_READ_STREAM: u32 = 21;
/// Commit staged firmware image.
pub const PSA_MM_FWU_COMMAND_COMMIT: u32 = 22;
/// Accept newly booted firmware image as permanent.
pub const PSA_MM_FWU_COMMAND_ACCEPT_IMAGE: u32 = 23;
/// Select previous firmware image bank.
pub const PSA_MM_FWU_COMMAND_SELECT_PREVIOUS: u32 = 24;

// --- Return Status Codes (DEN0118 section 3.4.2.11) ---

/// Command executed successfully.
pub const PSA_MM_FWU_SUCCESS: i32 = 0;
/// Unknown error occurred.
pub const PSA_MM_FWU_UNKNOWN: i32 = -1;
/// Service or agent is busy.
pub const PSA_MM_FWU_BUSY: i32 = -2;
/// Stream address or offset out of bounds.
pub const PSA_MM_FWU_OUT_OF_BOUNDS: i32 = -3;
/// Authentication failed for firmware payload.
pub const PSA_MM_FWU_AUTH_FAIL: i32 = -4;
/// Insufficient client permission.
pub const PSA_MM_FWU_NO_PERMISSION: i32 = -5;
/// Operation denied by security policy.
pub const PSA_MM_FWU_DENIED: i32 = -6;
/// Resume operation.
pub const PSA_MM_FWU_RESUME: i32 = -7;
/// Requested service not available.
pub const PSA_MM_FWU_NOT_AVAILABLE: i32 = -8;

// --- Service Status ---

/// Service is operative and ready to accept commands.
pub const SERVICE_STATUS_OPERATIVE: i32 = 0;
/// Initialization error.
pub const SERVICE_STATUS_ERR_INIT: i32 = -1;

/// Describes properties of an individual firmware image bank/component.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PsaFwuImageProperty {
    /// Unique image type identifier GUID.
    pub image_type_id: EfiGuid,
    /// Client permission mask.
    pub client_permissions: u32,
    /// Active firmware version.
    pub version: u32,
    /// Lowest accepted version for anti-rollback protection.
    pub lowest_accepted_version: u32,
}

/// Firmware stream transfer header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PsaFwuStreamHeader {
    /// Stream identifier assigned during open.
    pub stream_id: u32,
    /// Block size required for streaming in bytes.
    pub block_size: u32,
    /// Total stream size in bytes.
    pub stream_size: u64,
}

/// Result payload returned by `PSA_MM_FWU_COMMAND_DISCOVER`.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PsaFwuDiscoveryResult {
    /// Agent service status (`SERVICE_STATUS_OPERATIVE`).
    pub service_status: i32,
    /// Specification version (major << 16 | minor).
    pub version: u32,
    /// Number of distinct firmware images managed by this agent.
    pub num_images: u32,
}

/// Validates whether a given status code indicates success.
#[must_use]
pub const fn psa_fwu_is_success(status: i32) -> bool {
    status == PSA_MM_FWU_SUCCESS
}

/// Formats a 32-bit PSA specification version into (major, minor).
#[must_use]
pub const fn parse_psa_version(version: u32) -> (u16, u16) {
    let major = (version >> 16) as u16;
    let minor = (version & 0xFFFF) as u16;
    (major, minor)
}
