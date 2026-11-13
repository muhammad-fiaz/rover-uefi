//! # Media Sanitize Protocol and Client API
//!
//! Provides services to securely erase, clear, purge, or sanitize block media devices
//! (such as `NVMe` SSDs, SATA drives, `eMMC`, and `UFS` storage) according to NIST SP 800-88
//! sanitization guidelines.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Media Sanitize Protocol GUID.
pub const MEDIA_SANITIZE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0D79_9A99,
    0x25AF,
    0x429E,
    [0x92, 0x72, 0xD0, 0xB2, 0x7D, 0x6D, 0x5F, 0x14],
);

/// Media Sanitize Protocol Revision (1.0).
pub const MEDIA_SANITIZE_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Sanitize actions for purge operations.
pub mod purge_action {
    /// No purge action requested.
    pub const NO_ACTION: u32 = 0x0000_0000;
    /// Overwrite media with specified 32-bit pattern.
    pub const OVERWRITE: u32 = 0x0000_0001;
    /// Erase physical blocks with indeterminate pattern.
    pub const BLOCK_ERASE: u32 = 0x0000_0002;
    /// Cryptographic erase (delete internal encryption keys).
    pub const CRYPTO_ERASE: u32 = 0x0000_0004;
    /// System reset required after purge completion.
    pub const RESET_REQUIRED: u32 = 0x0000_0008;
    /// Do not deallocate (trim) flash media after sanitize.
    pub const NO_DEALLOCATE: u32 = 0x0000_0010;
    /// Invert overwrite pattern between passes.
    pub const INVERT_OW_PATTERN: u32 = 0x0000_0020;
    /// Allow exit without restrictions.
    pub const ALLOW_UNRESTRICTED_SANITIZE_EXIT: u32 = 0x0000_0040;
}

/// Secure erase actions for media format operations.
pub mod format_action {
    /// No secure erase operation requested.
    pub const NO_SECURE_ERASE: u32 = 0x0;
    /// User data erase.
    pub const USER_DATA_ERASE: u32 = 0x1;
    /// Cryptographic erase.
    pub const CRYPTOGRAPHIC_ERASE: u32 = 0x2;
}

/// Overwrite block media with fixed pattern.
pub type BlockMediaClear = unsafe extern "efiapi" fn(
    this: *mut MediaSanitizeProtocol,
    media_id: u32,
    pass_count: u32,
    sector_ow_buffer: *const c_void,
) -> EfiStatus;

/// Purge block media using hardware sanitize engine (overwrite, block erase, crypto erase).
pub type BlockMediaPurge = unsafe extern "efiapi" fn(
    this: *mut MediaSanitizeProtocol,
    media_id: u32,
    purge_action: u32,
    overwrite_pattern: u32,
) -> EfiStatus;

/// Format block media modifying sector/LBA size and applying secure erase.
pub type BlockMediaFormat = unsafe extern "efiapi" fn(
    this: *mut MediaSanitizeProtocol,
    media_id: u32,
    lba_size: u32,
    secure_erase_action: u32,
) -> EfiStatus;

/// Media Sanitize Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MediaSanitizeProtocol {
    /// Specification revision (`MEDIA_SANITIZE_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Pointer to `EFI_BLOCK_IO_MEDIA` data structure.
    pub media: *mut c_void,
    /// Bitmap of supported sanitize capabilities.
    pub sanitize_capabilities: u32,
    /// Clear media function pointer.
    pub media_clear: Option<BlockMediaClear>,
    /// Purge media function pointer.
    pub media_purge: Option<BlockMediaPurge>,
    /// Format media function pointer.
    pub media_format: Option<BlockMediaFormat>,
}

/// Safe client wrapper for Media Sanitize operations.
#[derive(Debug)]
pub struct MediaSanitize<'a> {
    protocol: &'a mut MediaSanitizeProtocol,
}

impl<'a> MediaSanitize<'a> {
    /// Create a new safe view of the Media Sanitize protocol.
    #[must_use]
    pub fn new(protocol: &'a mut MediaSanitizeProtocol) -> Self {
        Self { protocol }
    }

    /// Return the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Return the sanitize capabilities bitmask.
    #[must_use]
    pub const fn capabilities(&self) -> u32 {
        self.protocol.sanitize_capabilities
    }

    /// Purge media using hardware sanitize engine (e.g. crypto erase or block erase).
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn purge(&mut self, media_id: u32, action: u32, pattern: u32) -> Result<(), EfiStatus> {
        if let Some(purge_fn) = self.protocol.media_purge {
            let status = unsafe { purge_fn(self.protocol, media_id, action, pattern) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
