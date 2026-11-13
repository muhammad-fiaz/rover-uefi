//! User Credential 2 Protocol definition and safe wrapper.
//!
//! Attached to a device handle, the User Credential 2 Protocol identifies
//! an individual means of authenticating a user (e.g. password, smart card,
//! fingerprint, face biometric, or PIN token).

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::hii::EfiHiiHandle;
use crate::protocols::security::user_manager::{
    EfiUserInfo, EfiUserInfoHandle, EfiUserProfileHandle,
};

/// UEFI User Credential 2 Protocol GUID.
pub const EFI_USER_CREDENTIAL2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE98A_DB03,
    0xB8B9,
    0x4AF8,
    [0xBA, 0x20, 0x26, 0xE9, 0x11, 0x4C, 0xBC, 0xE5],
);

/// Legacy UEFI 2.2 User Credential Protocol GUID.
pub const EFI_USER_CREDENTIAL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x71EE_5E94,
    0x65B9,
    0x45D5,
    [0x82, 0x1A, 0x3A, 0x4D, 0x86, 0xCF, 0xE6, 0xBE],
);

/// Form set class identifier for HII forms used during user identification.
pub const EFI_HII_USER_CREDENTIAL_FORMSET_GUID: EfiGuid = EfiGuid::new(
    0x337F_4407,
    0x5AEE,
    0x4B83,
    [0xB2, 0xA7, 0x4E, 0xAD, 0xCA, 0x30, 0x88, 0xCD],
);

/// Credential provider capability: supports pre-OS enrollment.
pub const EFI_CREDENTIAL_CAPABILITIES_ENROLL: u64 = 0x0000_0000_0000_0001;

/// Credential provider logon flag: auto-logon supported.
pub const EFI_CREDENTIAL_LOGON_FLAG_AUTO: u32 = 0x0000_0001;

/// Credential provider logon flag: default credential provider.
pub const EFI_CREDENTIAL_LOGON_FLAG_DEFAULT: u32 = 0x0000_0002;

/// Function prototype for enrolling a user on a credential provider.
pub type EfiCredential2Enroll = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
) -> EfiStatus;

/// Function prototype for querying user identification form details.
pub type EfiCredential2Form = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    hii: *mut EfiHiiHandle,
    form_set_id: *mut EfiGuid,
    form_id: *mut u16,
) -> EfiStatus;

/// Function prototype for querying tile bitmap representation.
pub type EfiCredential2Tile = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    width: *mut UIntN,
    height: *mut UIntN,
    hii: *mut EfiHiiHandle,
    image: *mut u16,
) -> EfiStatus;

/// Function prototype for querying title string representation.
pub type EfiCredential2Title = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    hii: *mut EfiHiiHandle,
    string: *mut u16,
) -> EfiStatus;

/// Function prototype for retrieving the user identifier.
pub type EfiCredential2User = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
    identifier: *mut [u8; 16],
) -> EfiStatus;

/// Function prototype for selecting a credential provider.
pub type EfiCredential2Select = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    auto_logon: *mut u32,
) -> EfiStatus;

/// Function prototype for deselecting a credential provider.
pub type EfiCredential2Deselect =
    unsafe extern "efiapi" fn(this: *const EfiUserCredential2Protocol) -> EfiStatus;

/// Function prototype for retrieving default logon behavior.
pub type EfiCredential2Default = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    auto_logon: *mut u32,
) -> EfiStatus;

/// Function prototype for reading credential provider info.
pub type EfiCredential2GetInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    user_info: EfiUserInfoHandle,
    info: *mut EfiUserInfo,
    info_size: *mut UIntN,
) -> EfiStatus;

/// Function prototype for enumerating credential provider info.
pub type EfiCredential2GetNextInfo = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    user_info: *mut EfiUserInfoHandle,
) -> EfiStatus;

/// Function prototype for deleting a enrolled user from credential provider.
pub type EfiCredential2Delete = unsafe extern "efiapi" fn(
    this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
) -> EfiStatus;

/// UEFI User Credential 2 Protocol structure.
#[repr(C)]
pub struct EfiUserCredential2Protocol {
    /// Uniquely identifies this credential provider instance.
    pub identifier: EfiGuid,
    /// Identifies the class/type of credential provider.
    pub provider_type: EfiGuid,
    /// Enrolls a user on this credential provider.
    pub enroll: EfiCredential2Enroll,
    /// Returns form information for UI interaction.
    pub form: EfiCredential2Form,
    /// Returns tile bitmap information.
    pub tile: EfiCredential2Tile,
    /// Returns title string information.
    pub title: EfiCredential2Title,
    /// Returns authenticated user identifier.
    pub user: EfiCredential2User,
    /// Called when credential provider is selected.
    pub select: EfiCredential2Select,
    /// Called when credential provider is deselected.
    pub deselect: EfiCredential2Deselect,
    /// Queries default logon behavior.
    pub default_fn: EfiCredential2Default,
    /// Reads user info attached to provider.
    pub get_info: EfiCredential2GetInfo,
    /// Enumerates user info records.
    pub get_next_info: EfiCredential2GetNextInfo,
    /// Provider capabilities bitmask.
    pub capabilities: u64,
    /// Deletes a enrolled user.
    pub delete: EfiCredential2Delete,
}

/// Safe idiomatic Rust wrapper around `EfiUserCredential2Protocol`.
pub struct UserCredential2 {
    proto: *const EfiUserCredential2Protocol,
}

impl UserCredential2 {
    /// Creates a new `UserCredential2` wrapper from a raw protocol pointer.
    #[must_use]
    pub const fn new(proto: *const EfiUserCredential2Protocol) -> Self {
        Self { proto }
    }

    /// Returns the underlying raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiUserCredential2Protocol {
        self.proto
    }

    /// Returns the unique GUID identifying this credential provider instance.
    #[must_use]
    pub fn identifier(&self) -> EfiGuid {
        unsafe { (*self.proto).identifier }
    }

    /// Returns the credential class GUID (e.g. Password, `SmartCard`, Fingerprint).
    #[must_use]
    pub fn provider_type(&self) -> EfiGuid {
        unsafe { (*self.proto).provider_type }
    }

    /// Returns the capability bitmask for this credential provider.
    #[must_use]
    pub fn capabilities(&self) -> u64 {
        unsafe { (*self.proto).capabilities }
    }

    /// Enrolls a user profile on this credential provider.
    ///
    /// # Errors
    ///
    /// Returns an error status if enrollment fails or is unsupported.
    pub fn enroll(&self, user: EfiUserProfileHandle) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).enroll)(self.proto, user) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns HII form details for interactive authentication.
    ///
    /// Returns `(HiiHandle, FormSetGuid, FormId)`.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if this provider does not require a form, or an error status.
    pub fn form(&self) -> Result<(EfiHiiHandle, EfiGuid, u16), EfiStatus> {
        let mut hii = EfiHiiHandle::NULL;
        let mut form_set = EfiGuid::from_bytes([0u8; 16]);
        let mut form_id = 0u16;

        let status = unsafe {
            ((*self.proto).form)(
                self.proto,
                core::ptr::from_mut(&mut hii),
                core::ptr::from_mut(&mut form_set),
                core::ptr::from_mut(&mut form_id),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((hii, form_set, form_id))
        } else {
            Err(status)
        }
    }

    /// Retrieves tile bitmap information.
    ///
    /// Returns `(HiiHandle, ImageId)` and updates width/height.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no bitmap is available, or an error status.
    pub fn tile(
        &self,
        width: &mut usize,
        height: &mut usize,
    ) -> Result<(EfiHiiHandle, u16), EfiStatus> {
        let mut w = *width as UIntN;
        let mut h = *height as UIntN;
        let mut hii = EfiHiiHandle::NULL;
        let mut image_id = 0u16;

        let status = unsafe {
            ((*self.proto).tile)(
                self.proto,
                core::ptr::from_mut(&mut w),
                core::ptr::from_mut(&mut h),
                core::ptr::from_mut(&mut hii),
                core::ptr::from_mut(&mut image_id),
            )
        };
        if status == EfiStatus::SUCCESS {
            *width = w as usize;
            *height = h as usize;
            Ok((hii, image_id))
        } else {
            Err(status)
        }
    }

    /// Retrieves title string representation for this credential provider.
    ///
    /// Returns `(HiiHandle, StringId)`.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no title string is available, or an error status.
    pub fn title(&self) -> Result<(EfiHiiHandle, u16), EfiStatus> {
        let mut hii = EfiHiiHandle::NULL;
        let mut string_id = 0u16;

        let status = unsafe {
            ((*self.proto).title)(
                self.proto,
                core::ptr::from_mut(&mut hii),
                core::ptr::from_mut(&mut string_id),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((hii, string_id))
        } else {
            Err(status)
        }
    }

    /// Returns the user identifier associated with the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns `NOT_READY` if not authenticated, or an error status.
    pub fn user(&self, user: EfiUserProfileHandle) -> Result<[u8; 16], EfiStatus> {
        let mut id = [0u8; 16];
        let status =
            unsafe { ((*self.proto).user)(self.proto, user, core::ptr::from_mut(&mut id)) };
        if status == EfiStatus::SUCCESS {
            Ok(id)
        } else {
            Err(status)
        }
    }

    /// Selects this credential provider, returning logon flags.
    ///
    /// # Errors
    ///
    /// Returns an error status if selection fails.
    pub fn select(&self) -> Result<u32, EfiStatus> {
        let mut flags = 0u32;
        let status = unsafe { ((*self.proto).select)(self.proto, core::ptr::from_mut(&mut flags)) };
        if status == EfiStatus::SUCCESS {
            Ok(flags)
        } else {
            Err(status)
        }
    }

    /// Deselects this credential provider.
    ///
    /// # Errors
    ///
    /// Returns an error status if deselection fails.
    pub fn deselect(&self) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).deselect)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns default logon flags for this credential provider.
    ///
    /// # Errors
    ///
    /// Returns an error status if default query fails.
    pub fn default_logon(&self) -> Result<u32, EfiStatus> {
        let mut flags = 0u32;
        let status =
            unsafe { ((*self.proto).default_fn)(self.proto, core::ptr::from_mut(&mut flags)) };
        if status == EfiStatus::SUCCESS {
            Ok(flags)
        } else {
            Err(status)
        }
    }

    /// Reads user information attached to this credential provider into `buffer`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error status if buffer is too small or record not found.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn get_info(
        &self,
        user_info: EfiUserInfoHandle,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            ((*self.proto).get_info)(
                self.proto,
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

    /// Enumerates the next user info record handle.
    ///
    /// # Errors
    ///
    /// Returns an error status when enumeration is finished or invalid.
    pub fn get_next_info(&self, user_info: &mut EfiUserInfoHandle) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).get_next_info)(self.proto, user_info) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Deletes the enrolled user profile from this credential provider.
    ///
    /// # Errors
    ///
    /// Returns an error status if deletion fails or is unsupported.
    pub fn delete(&self, user: EfiUserProfileHandle) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).delete)(self.proto, user) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
