//! # Boot Integrity Services (BIS) Protocol
//!
//! Provides digital signature verification of boot images against certificates
//! for authorization and platform integrity checks as defined in EFI Specification 1.10+.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Boot Integrity Services Protocol GUID: `{0b64aab0-5429-11d4-9816-00a0c91fadcf}`.
pub const EFI_BIS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0B64_AAB0,
    0x5429,
    0x11D4,
    [0x98, 0x16, 0x00, 0xA0, 0xC9, 0x1F, 0xAD, 0xCF],
);

/// Boot Object Authorization Parameter Set GUID: `{edd35e31-07b9-11d2-83a3-00a0c91fadcf}`.
pub const BOOT_OBJECT_AUTHORIZATION_PARMSET_GUID: EfiGuid = EfiGuid::new(
    0xEDD3_5E31,
    0x07B9,
    0x11D2,
    [0x83, 0xA3, 0x00, 0xA0, 0xC9, 0x1F, 0xAD, 0xCF],
);

/// Opaque application handle identifying a BIS session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BisApplicationHandle(pub *mut c_void);

impl BisApplicationHandle {
    /// Null handle constant.
    pub const NULL: Self = Self(core::ptr::null_mut());

    /// Creates a handle from a raw pointer.
    #[must_use]
    pub const fn from_ptr(ptr: *mut c_void) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *mut c_void {
        self.0
    }

    /// Checks if handle is null.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

/// Algorithm identifier type.
pub type BisAlgId = u16;

/// Certificate identifier type.
pub type BisCertId = u32;

/// BIS Version 1 major version number.
pub const BIS_VERSION_1: u32 = 1;
/// Current major version of BIS interface.
pub const BIS_CURRENT_VERSION_MAJOR: u32 = BIS_VERSION_1;

/// DSA signature algorithm identifier.
pub const BIS_ALG_DSA: BisAlgId = 41;
/// RSA MD5 signature algorithm identifier.
pub const BIS_ALG_RSA_MD5: BisAlgId = 42;

/// DSA certificate identifier.
pub const BIS_CERT_ID_DSA: BisCertId = 41;
/// RSA MD5 certificate identifier.
pub const BIS_CERT_ID_RSA_MD5: BisCertId = 42;
/// Mask applied to truncated certificate hashes.
pub const BIS_CERT_ID_MASK: u32 = 0xFF7F_7FFF;

/// Linear data buffer descriptor returned from or passed into BIS methods.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiBisData {
    /// Length of payload in bytes.
    pub length: u32,
    /// Pointer to payload memory buffer.
    pub data: *mut u8,
}

impl EfiBisData {
    /// Creates an empty data descriptor.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            length: 0,
            data: core::ptr::null_mut(),
        }
    }
}

/// Version of the BIS interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiBisVersion {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
}

/// Supported signature and certificate combinations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiBisSignatureInfo {
    /// Truncated hash of platform authorization certificate.
    pub certificate_id: BisCertId,
    /// Signature algorithm identifier (`BIS_ALG_*`).
    pub algorithm_id: BisAlgId,
    /// Length of algorithm key in bits.
    pub key_length: u16,
}

/// Function pointer for `EFI_BIS_INITIALIZE`.
pub type EfiBisInitialize = unsafe extern "efiapi" fn(
    this: *mut EfiBisProtocol,
    app_handle: *mut BisApplicationHandle,
    interface_version: *mut EfiBisVersion,
    target_address: *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_SHUTDOWN`.
pub type EfiBisShutdown = unsafe extern "efiapi" fn(app_handle: BisApplicationHandle) -> EfiStatus;

/// Function pointer for `EFI_BIS_FREE`.
pub type EfiBisFree = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    to_free: *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_GET_BOOT_OBJECT_AUTHORIZATION_CERTIFICATE`.
pub type EfiBisGetBootObjectAuthorizationCertificate = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    certificate: *mut *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_GET_BOOT_OBJECT_AUTHORIZATION_CHECKFLAG`.
pub type EfiBisGetBootObjectAuthorizationCheckFlag = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    check_is_required: *mut bool,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_GET_BOOT_OBJECT_AUTHORIZATION_UPDATE_TOKEN`.
pub type EfiBisGetBootObjectAuthorizationUpdateToken = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    update_token: *mut *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_GET_SIGNATURE_INFO`.
pub type EfiBisGetSignatureInfo = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    signature_info: *mut *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_UPDATE_BOOT_OBJECT_AUTHORIZATION`.
pub type EfiBisUpdateBootObjectAuthorization = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    request_credential: *mut EfiBisData,
    new_update_token: *mut *mut EfiBisData,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_VERIFY_BOOT_OBJECT`.
pub type EfiBisVerifyBootObject = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    credentials: *mut EfiBisData,
    data_object: *mut EfiBisData,
    is_verified: *mut bool,
) -> EfiStatus;

/// Function pointer for `EFI_BIS_VERIFY_OBJECT_WITH_CREDENTIAL`.
pub type EfiBisVerifyObjectWithCredential = unsafe extern "efiapi" fn(
    app_handle: BisApplicationHandle,
    credentials: *mut EfiBisData,
    data_object: *mut EfiBisData,
    section_name: *mut EfiBisData,
    authority_certificate: *mut EfiBisData,
    is_verified: *mut bool,
) -> EfiStatus;

/// Raw ABI structure for `EFI_BIS_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBisProtocol {
    /// Initializes BIS instance and starts session.
    pub initialize: EfiBisInitialize,
    /// Terminates session and invalidates application handle.
    pub shutdown: EfiBisShutdown,
    /// Frees memory allocated by BIS service.
    pub free: EfiBisFree,
    /// Retrieves Boot Object Authorization Certificate.
    pub get_boot_object_authorization_certificate: EfiBisGetBootObjectAuthorizationCertificate,
    /// Queries whether Boot Authorization check is required on this platform.
    pub get_boot_object_authorization_check_flag: EfiBisGetBootObjectAuthorizationCheckFlag,
    /// Retrieves unique update token for authorization parameter changes.
    pub get_boot_object_authorization_update_token: EfiBisGetBootObjectAuthorizationUpdateToken,
    /// Retrieves supported signature algorithms and certificate types.
    pub get_signature_info: EfiBisGetSignatureInfo,
    /// Updates configurable Boot Object Authorization parameters.
    pub update_boot_object_authorization: EfiBisUpdateBootObjectAuthorization,
    /// Verifies boot object against signed manifest.
    pub verify_boot_object: EfiBisVerifyBootObject,
    /// Verifies boot object with explicit authority certificate.
    pub verify_object_with_credential: EfiBisVerifyObjectWithCredential,
}

/// Safe client wrapper around [`EfiBisProtocol`].
pub struct Bis<'a> {
    proto: &'a mut EfiBisProtocol,
}

impl<'a> Bis<'a> {
    /// Creates a new `Bis` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiBisProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiBisProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Initializes the BIS service and starts an application session.
    ///
    /// # Errors
    ///
    /// Returns an error status if initialization fails or version is incompatible.
    pub fn initialize(
        &mut self,
        desired_major: u32,
    ) -> Result<(BisApplicationHandle, EfiBisVersion), EfiStatus> {
        let mut handle = BisApplicationHandle::NULL;
        let mut ver = EfiBisVersion {
            major: desired_major,
            minor: 0,
        };
        let status = unsafe {
            (self.proto.initialize)(
                self.proto,
                core::ptr::from_mut(&mut handle),
                core::ptr::from_mut(&mut ver),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((handle, ver))
        } else {
            Err(status)
        }
    }

    /// Shuts down an active application session.
    ///
    /// # Errors
    ///
    /// Returns an error status if shutdown fails.
    pub fn shutdown(&mut self, handle: BisApplicationHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.shutdown)(handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Checks if boot object authorization verification is required.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails.
    pub fn get_check_flag(&mut self, handle: BisApplicationHandle) -> Result<bool, EfiStatus> {
        let mut required = false;
        let status = unsafe {
            (self.proto.get_boot_object_authorization_check_flag)(
                handle,
                core::ptr::from_mut(&mut required),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(required)
        } else {
            Err(status)
        }
    }

    /// Verifies the integrity of a boot object using credentials.
    ///
    /// # Errors
    ///
    /// Returns an error status if verification cannot be performed.
    pub fn verify_boot_object(
        &mut self,
        handle: BisApplicationHandle,
        credentials: &mut EfiBisData,
        data_object: &mut EfiBisData,
    ) -> Result<bool, EfiStatus> {
        let mut is_verified = false;
        let status = unsafe {
            (self.proto.verify_boot_object)(
                handle,
                core::ptr::from_mut(credentials),
                core::ptr::from_mut(data_object),
                core::ptr::from_mut(&mut is_verified),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(is_verified)
        } else {
            Err(status)
        }
    }
}
