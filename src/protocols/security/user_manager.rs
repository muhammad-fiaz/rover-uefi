//! # User Manager and User Credential Protocols
//!
//! Provides services for user identity, enrollment, profile enumeration, access policies,
//! and credential provider interactions as defined in the UEFI Specification.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// The `EFI_USER_MANAGER_PROTOCOL_GUID`.
pub const EFI_USER_MANAGER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6FD5_B00C,
    0xD426,
    0x4283,
    [0x98, 0x87, 0x6C, 0xF5, 0xCF, 0x1C, 0xB1, 0xFE],
);

/// Event group signaled whenever the active user profile changes.
pub const EFI_EVENT_GROUP_USER_PROFILE_CHANGED: EfiGuid = EfiGuid::new(
    0xBAF1_E6DE,
    0x209E,
    0x4ADB,
    [0x8D, 0x96, 0xFD, 0x8B, 0x71, 0xF3, 0xF6, 0x83],
);

/// The `EFI_USER_CREDENTIAL2_PROTOCOL_GUID`.
pub const EFI_USER_CREDENTIAL2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE98A_DB03,
    0xB8B9,
    0x4AF8,
    [0xBA, 0x20, 0x26, 0xE9, 0x11, 0x4C, 0xBC, 0xE5],
);

// ---------------------------------------------------------------------------
// Credential Class GUIDs
// ---------------------------------------------------------------------------

/// Unknown credential provider class.
pub const EFI_USER_CREDENTIAL_CLASS_UNKNOWN: EfiGuid = EfiGuid::new(
    0x5CF3_2E68,
    0x7660,
    0x449B,
    [0x80, 0xE6, 0x7E, 0xA3, 0x6E, 0x03, 0xF6, 0xA8],
);

/// Password credential provider class.
pub const EFI_USER_CREDENTIAL_CLASS_PASSWORD: EfiGuid = EfiGuid::new(
    0xF8E5_058C,
    0xCCB6,
    0x4714,
    [0xB2, 0x20, 0x3F, 0x7E, 0x3A, 0x64, 0x0B, 0xD1],
);

/// Smart Card credential provider class.
pub const EFI_USER_CREDENTIAL_CLASS_SMART_CARD: EfiGuid = EfiGuid::new(
    0x5F03_BA33,
    0x8C6B,
    0x4C24,
    [0xAA, 0x2E, 0x14, 0xA2, 0x65, 0x7B, 0xD4, 0x54],
);

/// Fingerprint biometric credential class.
pub const EFI_USER_CREDENTIAL_CLASS_FINGERPRINT: EfiGuid = EfiGuid::new(
    0x32CB_A21F,
    0xF308,
    0x4CBC,
    [0x9A, 0xB5, 0xF5, 0xA3, 0x69, 0x9F, 0x04, 0x4A],
);

/// Handprint biometric credential class.
pub const EFI_USER_CREDENTIAL_CLASS_HANDPRINT: EfiGuid = EfiGuid::new(
    0x5917_EF16,
    0xF723,
    0x4BB9,
    [0xA6, 0x4B, 0xD8, 0xC5, 0x32, 0xF4, 0xD8, 0xB5],
);

/// Secure Card credential class.
pub const EFI_USER_CREDENTIAL_CLASS_SECURE_CARD: EfiGuid = EfiGuid::new(
    0x8A6B_4A83,
    0x42FE,
    0x45D2,
    [0xA2, 0xEF, 0x46, 0xF0, 0x6C, 0x7D, 0x98, 0x52],
);

// ---------------------------------------------------------------------------
// Setup Access Level GUIDs
// ---------------------------------------------------------------------------

/// Administrator access permission in platform setup.
pub const EFI_USER_INFO_ACCESS_SETUP_ADMIN_GUID: EfiGuid = EfiGuid::new(
    0x85B7_5607,
    0xF7CE,
    0x471E,
    [0xB7, 0xE4, 0x2A, 0xEA, 0x5F, 0x72, 0x32, 0xEE],
);

/// Normal user access permission in platform setup.
pub const EFI_USER_INFO_ACCESS_SETUP_NORMAL_GUID: EfiGuid = EfiGuid::new(
    0x1DB2_9AE0,
    0x9DCB,
    0x43BC,
    [0x8D, 0x87, 0x5D, 0xA1, 0x49, 0x64, 0xDD, 0xE2],
);

/// Restricted user access permission in platform setup.
pub const EFI_USER_INFO_ACCESS_SETUP_RESTRICTED_GUID: EfiGuid = EfiGuid::new(
    0xBDB3_8125,
    0x4D63,
    0x49F4,
    [0x82, 0x12, 0x61, 0xCF, 0x5A, 0x19, 0x0A, 0xF8],
);

// ---------------------------------------------------------------------------
// User Information Record Constants & Attributes
// ---------------------------------------------------------------------------

/// Opaque user profile handle.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfiUserProfileHandle(pub *mut c_void);

impl EfiUserProfileHandle {
    /// Returns a null user profile handle.
    #[must_use]
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Wraps a raw pointer into a user profile handle.
    #[must_use]
    pub const fn from_ptr(ptr: *mut c_void) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *mut c_void {
        self.0
    }

    /// Returns true if the handle is null.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

/// Opaque user information record handle.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfiUserInfoHandle(pub *mut c_void);

impl EfiUserInfoHandle {
    /// Returns a null user info handle.
    #[must_use]
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Wraps a raw pointer into a user info handle.
    #[must_use]
    pub const fn from_ptr(ptr: *mut c_void) -> Self {
        Self(ptr)
    }

    /// Returns the raw pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *mut c_void {
        self.0
    }

    /// Returns true if the handle is null.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

/// User info storage attribute mask.
pub const EFI_USER_INFO_STORAGE: u16 = 0x000F;
/// Stored only in volatile RAM.
pub const EFI_USER_INFO_STORAGE_VOLATILE: u16 = 0x0000;
/// Stored non-volatilization in credential provider.
pub const EFI_USER_INFO_STORAGE_CREDENTIAL_NV: u16 = 0x0001;
/// Stored non-volatilization in platform NVRAM.
pub const EFI_USER_INFO_STORAGE_PLATFORM_NV: u16 = 0x0002;

/// User info access attribute mask.
pub const EFI_USER_INFO_ACCESS: u16 = 0x0070;
/// Visible to all users.
pub const EFI_USER_INFO_PUBLIC: u16 = 0x0010;
/// Visible only to this user.
pub const EFI_USER_INFO_PRIVATE: u16 = 0x0020;
/// Accessible only after authentication.
pub const EFI_USER_INFO_PROTECTED: u16 = 0x0030;
/// Only one instance of this record type allowed per profile.
pub const EFI_USER_INFO_EXCLUSIVE: u16 = 0x0080;

/// User info record: empty placeholder.
pub const EFI_USER_INFO_EMPTY_RECORD: u8 = 0x00;
/// User info record: username string (UTF-16).
pub const EFI_USER_INFO_NAME_RECORD: u8 = 0x01;
/// User info record: profile creation timestamp.
pub const EFI_USER_INFO_CREATE_DATE_RECORD: u8 = 0x02;
/// User info record: profile last used timestamp.
pub const EFI_USER_INFO_USAGE_DATE_RECORD: u8 = 0x03;
/// User info record: selection counter (u64).
pub const EFI_USER_INFO_USAGE_COUNT_RECORD: u8 = 0x04;
/// User info record: unique 16-byte user identifier.
pub const EFI_USER_INFO_IDENTIFIER_RECORD: u8 = 0x05;
/// User info record: credential type GUID.
pub const EFI_USER_INFO_CREDENTIAL_TYPE_RECORD: u8 = 0x06;
/// User info record: credential provider GUID.
pub const EFI_USER_INFO_CREDENTIAL_PROVIDER_RECORD: u8 = 0x08;
/// User info record: access rights and policy.
pub const EFI_USER_INFO_ACCESS_POLICY_RECORD: u8 = 0x0E;
/// User info record: identity requirement policy.
pub const EFI_USER_INFO_IDENTITY_POLICY_RECORD: u8 = 0x0F;

/// User info header structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiUserInfo {
    /// Credential GUID associated with this information record, or zero.
    pub credential: EfiGuid,
    /// Type of user information record.
    pub info_type: u8,
    /// Reserved byte; must be 0.
    pub reserved1: u8,
    /// Storage and access attributes (`EFI_USER_INFO_*`).
    pub info_attribs: u16,
    /// Size of this record in bytes including this header.
    pub info_size: u32,
}

// ---------------------------------------------------------------------------
// Protocol Function Types
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_USER_PROFILE_CREATE`.
pub type EfiUserProfileCreate = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_DELETE`.
pub type EfiUserProfileDelete = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_GET_NEXT`.
pub type EfiUserProfileGetNext = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_CURRENT`.
pub type EfiUserProfileCurrent = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    current_user: *mut EfiUserProfileHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_IDENTIFY`.
pub type EfiUserProfileIdentify = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_FIND`.
pub type EfiUserProfileFind = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
    user_info: *mut EfiUserInfoHandle,
    info: *const EfiUserInfo,
    info_size: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_NOTIFY`.
pub type EfiUserProfileNotify =
    unsafe extern "efiapi" fn(this: *const EfiUserManagerProtocol, changed: EfiHandle) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_GET_INFO`.
pub type EfiUserProfileGetInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: EfiUserInfoHandle,
    info: *mut EfiUserInfo,
    info_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_SET_INFO`.
pub type EfiUserProfileSetInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: *mut EfiUserInfoHandle,
    info: *const EfiUserInfo,
    info_size: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_DELETE_INFO`.
pub type EfiUserProfileDeleteInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: EfiUserInfoHandle,
) -> EfiStatus;

/// Function pointer for `EFI_USER_PROFILE_GET_NEXT_INFO`.
pub type EfiUserProfileGetNextInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: *mut EfiUserInfoHandle,
) -> EfiStatus;

/// Raw ABI structure for User Manager Protocol (`EFI_USER_MANAGER_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiUserManagerProtocol {
    /// Creates a new empty user profile.
    pub create: EfiUserProfileCreate,
    /// Removes an existing user profile.
    pub delete: EfiUserProfileDelete,
    /// Enumerates enrolled user profiles.
    pub get_next: EfiUserProfileGetNext,
    /// Retrieves current active user profile handle.
    pub current: EfiUserProfileCurrent,
    /// Interactively identifies user via credential provider.
    pub identify: EfiUserProfileIdentify,
    /// Finds user profile by record content.
    pub find: EfiUserProfileFind,
    /// Receives asynchronous credential status update.
    pub notify: EfiUserProfileNotify,
    /// Retrieves record information from user profile.
    pub get_info: EfiUserProfileGetInfo,
    /// Adds or updates record information in user profile.
    pub set_info: EfiUserProfileSetInfo,
    /// Removes record information from user profile.
    pub delete_info: EfiUserProfileDeleteInfo,
    /// Enumerates information records attached to a profile.
    pub get_next_info: EfiUserProfileGetNextInfo,
}

// ---------------------------------------------------------------------------
// Safe Client Wrapper
// ---------------------------------------------------------------------------

/// Safe wrapper around [`EfiUserManagerProtocol`].
pub struct UserManager<'a> {
    proto: &'a EfiUserManagerProtocol,
}

impl<'a> UserManager<'a> {
    /// Wraps a reference to [`EfiUserManagerProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiUserManagerProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiUserManagerProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to a valid and initialized [`EfiUserManagerProtocol`].
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiUserManagerProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Creates a new user profile and returns its handle.
    ///
    /// # Errors
    ///
    /// Returns an error status if user creation is unauthorized or unsupported.
    pub fn create(&self) -> Result<EfiUserProfileHandle, EfiStatus> {
        let mut user = EfiUserProfileHandle::null();
        let status = unsafe { (self.proto.create)(self.proto, core::ptr::from_mut(&mut user)) };
        if status == EfiStatus::SUCCESS {
            Ok(user)
        } else {
            Err(status)
        }
    }

    /// Deletes the specified user profile.
    ///
    /// # Errors
    ///
    /// Returns an error status if profile deletion fails or is unauthorized.
    pub fn delete(&self, user: EfiUserProfileHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.delete)(self.proto, user) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves the next enrolled user profile handle, or null when finished.
    ///
    /// # Errors
    ///
    /// Returns an error status if profile enumeration is disallowed.
    pub fn get_next(&self, user: &mut EfiUserProfileHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.get_next)(self.proto, user) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves the currently active user profile handle.
    ///
    /// # Errors
    ///
    /// Returns an error status if no active profile exists or retrieval fails.
    pub fn current(&self) -> Result<EfiUserProfileHandle, EfiStatus> {
        let mut cur = EfiUserProfileHandle::null();
        let status = unsafe { (self.proto.current)(self.proto, core::ptr::from_mut(&mut cur)) };
        if status == EfiStatus::SUCCESS {
            Ok(cur)
        } else {
            Err(status)
        }
    }

    /// Performs interactive user identification against enrolled credentials.
    ///
    /// # Errors
    ///
    /// Returns an error status if identification fails or user aborts.
    pub fn identify(&self) -> Result<EfiUserProfileHandle, EfiStatus> {
        let mut user = EfiUserProfileHandle::null();
        let status = unsafe { (self.proto.identify)(self.proto, core::ptr::from_mut(&mut user)) };
        if status == EfiStatus::SUCCESS {
            Ok(user)
        } else {
            Err(status)
        }
    }

    /// Notifies the User Manager of a credential provider status change.
    ///
    /// # Errors
    ///
    /// Returns an error status if notification fails.
    pub fn notify(&self, changed: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.notify)(self.proto, changed) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads a user information record into `buffer`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error status if the record does not exist or access is denied.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn get_info(
        &self,
        user: EfiUserProfileHandle,
        user_info: EfiUserInfoHandle,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            (self.proto.get_info)(
                self.proto,
                user,
                user_info,
                buffer.as_mut_ptr().cast::<EfiUserInfo>(),
                core::ptr::from_mut(&mut size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(size as usize)
        } else {
            Err(status)
        }
    }

    /// Deletes a specific information record from a user profile.
    ///
    /// # Errors
    ///
    /// Returns an error status if the record cannot be deleted.
    pub fn delete_info(
        &self,
        user: EfiUserProfileHandle,
        user_info: EfiUserInfoHandle,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.delete_info)(self.proto, user, user_info) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Enumerates the next user info handle on profile `user`.
    ///
    /// # Errors
    ///
    /// Returns an error status when enumeration is finished or invalid.
    pub fn get_next_info(
        &self,
        user: EfiUserProfileHandle,
        user_info: &mut EfiUserInfoHandle,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.get_next_info)(self.proto, user, user_info) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
