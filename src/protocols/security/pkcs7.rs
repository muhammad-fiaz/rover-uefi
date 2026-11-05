//! # PKCS7 Verify Protocol
//!
//! Defined by the UEFI Specification Section 32.5: PKCS7 Verify Protocol.
//!
//! Provides cryptographic verification of PKCS #7 signed data blobs and detached
//! digital signatures against an X.509 certificate trust anchor.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_PKCS7_VERIFY_PROTOCOL_GUID`.
pub const EFI_PKCS7_VERIFY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3B45_564A,
    0xE0A0,
    0x429A,
    [0xA8, 0x3A, 0xAD, 0x56, 0x1E, 0x9A, 0x22, 0xA4],
);

/// Function pointer for `EFI_PKCS7_VERIFY_BUFFER`.
pub type EfiPkcs7VerifyBuffer = unsafe extern "efiapi" fn(
    this: *const EfiPkcs7VerifyProtocol,
    signed_data: *const c_void,
    signed_data_size: UIntN,
    in_data: *const c_void,
    in_data_size: UIntN,
    allowed_db: *const c_void,
    allowed_db_size: UIntN,
    revoked_db: *const c_void,
    revoked_db_size: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_PKCS7_VERIFY_SIGNATURE`.
pub type EfiPkcs7VerifySignature = unsafe extern "efiapi" fn(
    this: *const EfiPkcs7VerifyProtocol,
    signature: *const c_void,
    signature_size: UIntN,
    in_data: *const c_void,
    in_data_size: UIntN,
    allowed_db: *const c_void,
    allowed_db_size: UIntN,
    revoked_db: *const c_void,
    revoked_db_size: UIntN,
) -> EfiStatus;

/// The raw ABI structure for `EFI_PKCS7_VERIFY_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPkcs7VerifyProtocol {
    /// Verifies an encapsulated PKCS #7 signed message.
    pub verify_buffer: EfiPkcs7VerifyBuffer,
    /// Verifies a detached PKCS #7 signature against data content.
    pub verify_signature: EfiPkcs7VerifySignature,
}
