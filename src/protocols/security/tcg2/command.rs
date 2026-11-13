//! # TPM 2.0 Command Framing and Serialization
//!
//! Provides TPM 2.0 command and response headers, standard command and response codes,
//! and zero-allocation command builders for early-boot cryptographic platform initialization.

/// Standard TPM 2.0 tag: Command without authorization sessions.
pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
/// Standard TPM 2.0 tag: Command with authorization sessions.
pub const TPM_ST_SESSIONS: u16 = 0x8002;

/// TPM 2.0 Startup type: Clear startup.
pub const TPM_SU_CLEAR: u16 = 0x0000;
/// TPM 2.0 Startup type: State restore (e.g. S3 resume).
pub const TPM_SU_STATE: u16 = 0x0001;

// TPM 2.0 Command Codes
/// `TPM2_Startup` command code.
pub const TPM_CC_STARTUP: u32 = 0x0000_0144;
/// `TPM2_Shutdown` command code.
pub const TPM_CC_SHUTDOWN: u32 = 0x0000_0145;
/// `TPM2_SelfTest` command code.
pub const TPM_CC_SELF_TEST: u32 = 0x0000_0143;
/// `TPM2_GetRandom` command code.
pub const TPM_CC_GET_RANDOM: u32 = 0x0000_017B;
/// `TPM2_PCR_Read` command code.
pub const TPM_CC_PCR_READ: u32 = 0x0000_017E;
/// `TPM2_PCR_Extend` command code.
pub const TPM_CC_PCR_EXTEND: u32 = 0x0000_0182;
/// `TPM2_HashSequenceStart` command code.
pub const TPM_CC_HASH_SEQUENCE_START: u32 = 0x0000_0186;
/// `TPM2_SequenceUpdate` command code.
pub const TPM_CC_SEQUENCE_UPDATE: u32 = 0x0000_015C;
/// `TPM2_SequenceComplete` command code.
pub const TPM_CC_SEQUENCE_COMPLETE: u32 = 0x0000_013E;

// TPM 2.0 Response Codes
/// Response: Success.
pub const TPM_RC_SUCCESS: u32 = 0x000;
/// Response: Bad tag.
pub const TPM_RC_BAD_TAG: u32 = 0x01E;
/// Response: TPM not initialized.
pub const TPM_RC_INITIALIZE: u32 = 0x100;
/// Response: General TPM failure.
pub const TPM_RC_FAILURE: u32 = 0x101;
/// Response: Bad locality.
pub const TPM_RC_LOCALITY: u32 = 0x907;

/// TPM 2.0 standard command header.
///
/// NOTE: All fields are encoded in **big-endian** on wire per TCG specification.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TpmCommandHeader {
    /// Command tag (`TPM_ST_NO_SESSIONS` or `TPM_ST_SESSIONS`).
    pub tag: u16,
    /// Total command size in bytes including header.
    pub param_size: u32,
    /// Command code (`TPM_CC_*`).
    pub command_code: u32,
}

/// TPM 2.0 standard response header.
///
/// NOTE: All fields are encoded in **big-endian** on wire per TCG specification.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TpmResponseHeader {
    /// Response tag.
    pub tag: u16,
    /// Total response size in bytes including header.
    pub param_size: u32,
    /// Response return code (`TPM_RC_*`).
    pub response_code: u32,
}

/// Errors occurring during TPM command generation or response parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpmCommandError {
    /// Output buffer is too small to contain command or response.
    BufferTooSmall,
    /// Response header is malformed.
    InvalidHeader,
    /// Command execution returned a non-success response code.
    TpmError(u32),
}

/// Builds a `TPM2_Startup` command into `out` buffer.
///
/// Returns the number of bytes written.
///
/// # Errors
/// Returns [`TpmCommandError::BufferTooSmall`] if `out` is smaller than 12 bytes.
pub fn build_startup_command(startup_type: u16, out: &mut [u8]) -> Result<usize, TpmCommandError> {
    if out.len() < 12 {
        return Err(TpmCommandError::BufferTooSmall);
    }

    out[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    out[2..6].copy_from_slice(&12_u32.to_be_bytes());
    out[6..10].copy_from_slice(&TPM_CC_STARTUP.to_be_bytes());
    out[10..12].copy_from_slice(&startup_type.to_be_bytes());

    Ok(12)
}

/// Builds a `TPM2_SelfTest` command into `out` buffer.
///
/// Returns the number of bytes written.
///
/// # Errors
/// Returns [`TpmCommandError::BufferTooSmall`] if `out` is smaller than 11 bytes.
pub fn build_self_test_command(full_test: bool, out: &mut [u8]) -> Result<usize, TpmCommandError> {
    if out.len() < 11 {
        return Err(TpmCommandError::BufferTooSmall);
    }

    out[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    out[2..6].copy_from_slice(&11_u32.to_be_bytes());
    out[6..10].copy_from_slice(&TPM_CC_SELF_TEST.to_be_bytes());
    out[10] = u8::from(full_test);

    Ok(11)
}

/// Builds a `TPM2_GetRandom` command into `out` buffer.
///
/// Returns the number of bytes written.
///
/// # Errors
/// Returns [`TpmCommandError::BufferTooSmall`] if `out` is smaller than 12 bytes.
pub fn build_get_random_command(
    bytes_requested: u16,
    out: &mut [u8],
) -> Result<usize, TpmCommandError> {
    if out.len() < 12 {
        return Err(TpmCommandError::BufferTooSmall);
    }

    out[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    out[2..6].copy_from_slice(&12_u32.to_be_bytes());
    out[6..10].copy_from_slice(&TPM_CC_GET_RANDOM.to_be_bytes());
    out[10..12].copy_from_slice(&bytes_requested.to_be_bytes());

    Ok(12)
}

/// Parses and validates a TPM 2.0 response header from `resp` buffer.
///
/// # Errors
/// Returns [`TpmCommandError::BufferTooSmall`] if buffer is smaller than 10 bytes,
/// or [`TpmCommandError::TpmError`] if the response code indicates failure.
pub fn parse_response_header(resp: &[u8]) -> Result<TpmResponseHeader, TpmCommandError> {
    if resp.len() < 10 {
        return Err(TpmCommandError::BufferTooSmall);
    }

    let mut tag_raw = [0_u8; 2];
    tag_raw.copy_from_slice(&resp[0..2]);
    let tag = u16::from_be_bytes(tag_raw);

    let mut sz_raw = [0_u8; 4];
    sz_raw.copy_from_slice(&resp[2..6]);
    let param_size = u32::from_be_bytes(sz_raw);

    let mut rc_raw = [0_u8; 4];
    rc_raw.copy_from_slice(&resp[6..10]);
    let response_code = u32::from_be_bytes(rc_raw);

    if response_code != TPM_RC_SUCCESS {
        return Err(TpmCommandError::TpmError(response_code));
    }

    Ok(TpmResponseHeader {
        tag,
        param_size,
        response_code,
    })
}
