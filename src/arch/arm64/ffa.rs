//! # ARM Firmware Framework for Arm A-profile (FF-A)
//!
//! Provides ABI definitions, function identifiers, partition messaging, and
//! memory permission services for communication between Normal World / S-EL0
//! and the Secure Partition Manager (SPM) or Secure OS, compliant with ARM DEN0077 and DEN0140.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::too_many_arguments,
    clippy::similar_names,
    clippy::must_use_candidate
)]

/// FF-A Error function ID.
pub const ARM_FID_FFA_ERROR: u32 = 0x8400_0060;
/// FF-A Success function ID (32-bit calling convention).
pub const ARM_FID_FFA_SUCCESS_32: u32 = 0x8400_0061;
/// FF-A Success function ID (64-bit calling convention).
pub const ARM_FID_FFA_SUCCESS_64: u32 = 0xC400_0061;
/// FF-A Interrupt function ID.
pub const ARM_FID_FFA_INTERRUPT: u32 = 0x8400_0062;

/// FF-A Version query.
pub const ARM_FID_FFA_VERSION: u32 = 0x8400_0063;
/// FF-A Features discovery.
pub const ARM_FID_FFA_FEATURES: u32 = 0x8400_0064;
/// FF-A RX buffer acquire.
pub const ARM_FID_FFA_RX_ACQUIRE: u32 = 0x8400_0084;
/// FF-A RX buffer release.
pub const ARM_FID_FFA_RX_RELEASE: u32 = 0x8400_0065;
/// FF-A RX/TX buffer map (64-bit).
pub const ARM_FID_FFA_RXTX_MAP_64: u32 = 0xC400_0066;
/// FF-A RX/TX buffer unmap.
pub const ARM_FID_FFA_RXTX_UNMAP: u32 = 0x8400_0067;
/// FF-A Partition information query.
pub const ARM_FID_FFA_PARTITION_INFO_GET: u32 = 0x8400_0068;
/// FF-A Partition information query returning via registers.
pub const ARM_FID_FFA_PARTITION_INFO_GET_REGS: u32 = 0xC400_008B;
/// FF-A Calling partition ID get.
pub const ARM_FID_FFA_ID_GET: u32 = 0x8400_0069;
/// FF-A SPM partition ID get.
pub const ARM_FID_FFA_SPM_ID_GET: u32 = 0x8400_0085;
/// FF-A Console log (64-bit).
pub const ARM_FID_FFA_CONSOLE_LOG_64: u32 = 0xC400_008A;

/// FF-A Direct Message Request (32-bit).
pub const ARM_FID_FFA_MSG_SEND_DIRECT_REQ_32: u32 = 0x8400_006F;
/// FF-A Direct Message Request (64-bit).
pub const ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64: u32 = 0xC400_006F;
/// FF-A Direct Message Response (32-bit).
pub const ARM_FID_FFA_MSG_SEND_DIRECT_RESP_32: u32 = 0x8400_0070;
/// FF-A Direct Message Response (64-bit).
pub const ARM_FID_FFA_MSG_SEND_DIRECT_RESP_64: u32 = 0xC400_0070;

/// FF-A Memory Donate (64-bit).
pub const ARM_FID_FFA_MEM_DONATE_64: u32 = 0xC400_0071;
/// FF-A Memory Lend (64-bit).
pub const ARM_FID_FFA_MEM_LEND_64: u32 = 0xC400_0072;
/// FF-A Memory Share (64-bit).
pub const ARM_FID_FFA_MEM_SHARE_64: u32 = 0xC400_0073;
/// FF-A Memory Retrieve Request (64-bit).
pub const ARM_FID_FFA_MEM_RETRIEVE_REQ_64: u32 = 0xC400_0074;
/// FF-A Memory Retrieve Response.
pub const ARM_FID_FFA_MEM_RETRIEVE_RESP: u32 = 0x8400_0075;
/// FF-A Memory Relinquish.
pub const ARM_FID_FFA_MEM_RETRIEVE_RELINQUISH: u32 = 0x8400_0076;
/// FF-A Memory Reclaim.
pub const ARM_FID_FFA_MEM_RETRIEVE_RECLAIM: u32 = 0x8400_0077;
/// FF-A Memory Permission Get (64-bit).
pub const ARM_FID_FFA_MEM_PERM_GET_64: u32 = 0xC400_0088;
/// FF-A Memory Permission Set (64-bit).
pub const ARM_FID_FFA_MEM_PERM_SET_64: u32 = 0xC400_0089;

/// Major FF-A version (1).
pub const ARM_FFA_MAJOR_VERSION: u16 = 1;
/// Minor FF-A version (2).
pub const ARM_FFA_MINOR_VERSION: u16 = 2;

/// Standard destination endpoint ID commonly assigned to OP-TEE / StMM Secure Partition.
pub const ARM_FFA_DESTINATION_ENDPOINT_ID: u16 = 3;

/// FF-A Memory Permission: No Data Access.
pub const ARM_FFA_MEM_PERM_DATA_NO_ACCESS: u32 = 0;
/// FF-A Memory Permission: Read/Write Access.
pub const ARM_FFA_MEM_PERM_DATA_RW: u32 = 1;
/// FF-A Memory Permission: Read-Only Access.
pub const ARM_FFA_MEM_PERM_DATA_RO: u32 = 3;

/// FF-A Memory Permission: Executable.
pub const ARM_FFA_MEM_PERM_CODE_X: u32 = 0;
/// FF-A Memory Permission: Execute Never (`XN`).
pub const ARM_FFA_MEM_PERM_CODE_XN: u32 = 1;

/// Packs data permission (bits 1:0) and code execution permission (bit 2).
#[must_use]
pub const fn ffa_make_mem_perm(data_perm: u32, code_perm: u32) -> u32 {
    ((code_perm & 0x1) << 2) | (data_perm & 0x3)
}

/// Encodes an FF-A 32-bit version word from major and minor components.
#[must_use]
pub const fn ffa_make_version(major: u16, minor: u16) -> u32 {
    ((major as u32 & 0x7FFF) << 16) | (minor as u32)
}

/// Extracts major version from an FF-A 32-bit version word.
#[must_use]
pub const fn ffa_version_major(version: u32) -> u16 {
    ((version >> 16) & 0x7FFF) as u16
}

/// Extracts minor version from an FF-A 32-bit version word.
#[must_use]
pub const fn ffa_version_minor(version: u32) -> u16 {
    (version & 0xFFFF) as u16
}

/// Checks if an FF-A version is ABI-compatible with required major and minimum minor version.
#[must_use]
pub const fn ffa_is_compatible(version: u32, expected_major: u16, min_minor: u16) -> bool {
    let major = ffa_version_major(version);
    let minor = ffa_version_minor(version);
    major == expected_major && minor >= min_minor
}

/// Packs 16-bit source and destination partition IDs into a 32-bit endpoint parameter.
#[must_use]
pub const fn ffa_pack_partition_ids(source_id: u16, dest_id: u16) -> u32 {
    ((source_id as u32) << 16) | (dest_id as u32)
}

/// Extracts the source partition ID from a packed 32-bit endpoint parameter.
#[must_use]
pub const fn ffa_source_partition_id(packed: u32) -> u16 {
    ((packed >> 16) & 0xFFFF) as u16
}

/// Extracts the destination partition ID from a packed 32-bit endpoint parameter.
#[must_use]
pub const fn ffa_dest_partition_id(packed: u32) -> u16 {
    (packed & 0xFFFF) as u16
}

/// Returns whether a given partition ID represents a Secure Partition (bit 15 set).
#[must_use]
pub const fn ffa_is_secure_partition(part_id: u16) -> bool {
    (part_id & 0x8000) != 0
}

/// Standard error codes returned by FF-A interfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfaStatus {
    /// Function completed successfully.
    Success,
    /// Function or feature is not supported (-1).
    NotSupported,
    /// Invalid parameters were supplied (-2).
    InvalidParameters,
    /// Insufficient memory to complete request (-3).
    NoMemory,
    /// Resource is busy (-4).
    Busy,
    /// Call was interrupted (-5).
    Interrupted,
    /// Access denied or insufficient privilege (-6).
    Denied,
    /// Operation in progress; retry later (-7).
    Retry,
    /// Operation aborted (-8).
    Aborted,
    /// No data available (-9).
    NoData,
    /// Interface not ready (-10).
    NotReady,
    /// Unknown or unmapped error code.
    Unknown(i32),
}

impl FfaStatus {
    /// Converts a raw signed integer return code into an [`FfaStatus`].
    #[must_use]
    pub const fn from_raw(code: i64) -> Self {
        match code as i32 {
            0 => Self::Success,
            -1 => Self::NotSupported,
            -2 => Self::InvalidParameters,
            -3 => Self::NoMemory,
            -4 => Self::Busy,
            -5 => Self::Interrupted,
            -6 => Self::Denied,
            -7 => Self::Retry,
            -8 => Self::Aborted,
            -9 => Self::NoData,
            -10 => Self::NotReady,
            other => Self::Unknown(other),
        }
    }

    /// Converts [`FfaStatus`] back into standard signed 32-bit return integer.
    #[must_use]
    pub const fn to_raw(&self) -> i32 {
        match *self {
            Self::Success => 0,
            Self::NotSupported => -1,
            Self::InvalidParameters => -2,
            Self::NoMemory => -3,
            Self::Busy => -4,
            Self::Interrupted => -5,
            Self::Denied => -6,
            Self::Retry => -7,
            Self::Aborted => -8,
            Self::NoData => -9,
            Self::NotReady => -10,
            Self::Unknown(val) => val,
        }
    }
}

/// Low-level invocation of an ARM64 SMC instruction with 8 input registers (`X0..X7`).
///
/// # Safety
/// The caller must ensure that running `smc #0` is valid in the current CPU privilege level.
#[inline]
pub unsafe fn ffa_smc_call(
    fid: u32,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
) -> [u64; 8] {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    {
        let mut r0 = fid as u64;
        let mut r1 = arg1;
        let mut r2 = arg2;
        let mut r3 = arg3;
        let mut r4 = arg4;
        let mut r5 = arg5;
        let mut r6 = arg6;
        let mut r7 = arg7;

        unsafe {
            core::arch::asm!(
                "smc #0",
                inout("x0") r0,
                inout("x1") r1,
                inout("x2") r2,
                inout("x3") r3,
                inout("x4") r4,
                inout("x5") r5,
                inout("x6") r6,
                inout("x7") r7,
                options(nostack)
            );
        }

        [r0, r1, r2, r3, r4, r5, r6, r7]
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    {
        let _ = (fid, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
        [0xFFFF_FFFF_FFFF_FFFF; 8]
    }
}

/// Low-level invocation of an ARM64 HVC instruction with 8 input registers (`X0..X7`).
///
/// # Safety
/// The caller must ensure that running `hvc #0` is valid in the current CPU privilege level.
#[inline]
pub unsafe fn ffa_hvc_call(
    fid: u32,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
) -> [u64; 8] {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    {
        let mut r0 = fid as u64;
        let mut r1 = arg1;
        let mut r2 = arg2;
        let mut r3 = arg3;
        let mut r4 = arg4;
        let mut r5 = arg5;
        let mut r6 = arg6;
        let mut r7 = arg7;

        unsafe {
            core::arch::asm!(
                "hvc #0",
                inout("x0") r0,
                inout("x1") r1,
                inout("x2") r2,
                inout("x3") r3,
                inout("x4") r4,
                inout("x5") r5,
                inout("x6") r6,
                inout("x7") r7,
                options(nostack)
            );
        }

        [r0, r1, r2, r3, r4, r5, r6, r7]
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    {
        let _ = (fid, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
        [0xFFFF_FFFF_FFFF_FFFF; 8]
    }
}

/// Queries the FF-A version supported by the SPM/firmware.
///
/// When `conduit_smc` is true, SMC is used; otherwise HVC is used.
///
/// # Errors
/// Returns [`FfaStatus::NotSupported`] if the interface reports an error or negative version code.
pub fn ffa_version(conduit_smc: bool) -> Result<(u16, u16), FfaStatus> {
    let req_ver = ffa_make_version(ARM_FFA_MAJOR_VERSION, ARM_FFA_MINOR_VERSION) as u64;
    let regs = unsafe {
        if conduit_smc {
            ffa_smc_call(ARM_FID_FFA_VERSION, req_ver, 0, 0, 0, 0, 0, 0)
        } else {
            ffa_hvc_call(ARM_FID_FFA_VERSION, req_ver, 0, 0, 0, 0, 0, 0)
        }
    };

    let ret = regs[0] as i64;
    if ret < 0 || regs[0] as u32 == ARM_FID_FFA_ERROR {
        Err(FfaStatus::from_raw(regs[2] as i64))
    } else {
        let v = regs[0] as u32;
        Ok((ffa_version_major(v), ffa_version_minor(v)))
    }
}

/// Queries the 16-bit partition ID of the calling execution context.
///
/// # Errors
/// Returns [`FfaStatus`] if the query fails.
pub fn ffa_id_get(conduit_smc: bool) -> Result<u16, FfaStatus> {
    let regs = unsafe {
        if conduit_smc {
            ffa_smc_call(ARM_FID_FFA_ID_GET, 0, 0, 0, 0, 0, 0, 0)
        } else {
            ffa_hvc_call(ARM_FID_FFA_ID_GET, 0, 0, 0, 0, 0, 0, 0)
        }
    };

    if regs[0] as u32 == ARM_FID_FFA_SUCCESS_32 {
        Ok((regs[2] & 0xFFFF) as u16)
    } else {
        Err(FfaStatus::from_raw(regs[2] as i64))
    }
}

/// Sends a direct 64-bit message request to a destination partition.
///
/// Returns registers `[X3..X7]` upon receiving a direct response.
///
/// # Errors
/// Returns [`FfaStatus`] if the call returns [`ARM_FID_FFA_ERROR`].
pub fn ffa_msg_send_direct_req64(
    conduit_smc: bool,
    source_id: u16,
    dest_id: u16,
    w3: u64,
    w4: u64,
    w5: u64,
    w6: u64,
    w7: u64,
) -> Result<[u64; 5], FfaStatus> {
    let endpoints = ffa_pack_partition_ids(source_id, dest_id) as u64;
    let regs = unsafe {
        if conduit_smc {
            ffa_smc_call(
                ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64,
                endpoints,
                0,
                w3,
                w4,
                w5,
                w6,
                w7,
            )
        } else {
            ffa_hvc_call(
                ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64,
                endpoints,
                0,
                w3,
                w4,
                w5,
                w6,
                w7,
            )
        }
    };

    if regs[0] as u32 == ARM_FID_FFA_MSG_SEND_DIRECT_RESP_64 {
        Ok([regs[3], regs[4], regs[5], regs[6], regs[7]])
    } else {
        Err(FfaStatus::from_raw(regs[2] as i64))
    }
}

/// Queries memory permission attributes for a page address.
///
/// # Errors
/// Returns [`FfaStatus`] if the permission query fails.
pub fn ffa_mem_perm_get(conduit_smc: bool, base_addr: u64) -> Result<u32, FfaStatus> {
    let regs = unsafe {
        if conduit_smc {
            ffa_smc_call(ARM_FID_FFA_MEM_PERM_GET_64, base_addr, 0, 0, 0, 0, 0, 0)
        } else {
            ffa_hvc_call(ARM_FID_FFA_MEM_PERM_GET_64, base_addr, 0, 0, 0, 0, 0, 0)
        }
    };

    if regs[0] as u32 == ARM_FID_FFA_SUCCESS_64 || regs[0] as u32 == ARM_FID_FFA_SUCCESS_32 {
        Ok(regs[2] as u32)
    } else {
        Err(FfaStatus::from_raw(regs[2] as i64))
    }
}

/// Configures memory permission attributes for a memory range.
///
/// # Errors
/// Returns [`FfaStatus`] if permission setting fails.
pub fn ffa_mem_perm_set(
    conduit_smc: bool,
    base_addr: u64,
    num_pages: u32,
    mem_perm: u32,
) -> Result<(), FfaStatus> {
    let regs = unsafe {
        if conduit_smc {
            ffa_smc_call(
                ARM_FID_FFA_MEM_PERM_SET_64,
                base_addr,
                num_pages as u64,
                mem_perm as u64,
                0,
                0,
                0,
                0,
            )
        } else {
            ffa_hvc_call(
                ARM_FID_FFA_MEM_PERM_SET_64,
                base_addr,
                num_pages as u64,
                mem_perm as u64,
                0,
                0,
                0,
                0,
            )
        }
    };

    if regs[0] as u32 == ARM_FID_FFA_SUCCESS_64 || regs[0] as u32 == ARM_FID_FFA_SUCCESS_32 {
        Ok(())
    } else {
        Err(FfaStatus::from_raw(regs[2] as i64))
    }
}
