//! Integration tests for User Credential 2 Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::hii::EfiHiiHandle;
use rover_uefi::protocols::security::{
    EFI_CREDENTIAL_CAPABILITIES_ENROLL, EFI_CREDENTIAL_LOGON_FLAG_AUTO,
    EFI_CREDENTIAL_LOGON_FLAG_DEFAULT, EFI_HII_USER_CREDENTIAL_FORMSET_GUID,
    EFI_USER_CREDENTIAL_CLASS_PASSWORD, EFI_USER_CREDENTIAL_PROTOCOL_GUID,
    EFI_USER_CREDENTIAL2_PROTOCOL_GUID, EFI_USER_INFO_NAME_RECORD, EfiUserCredential2Protocol,
    EfiUserInfo, EfiUserInfoHandle, EfiUserProfileHandle, UserCredential2,
};

static DUMMY_USER_1: usize = 0x1000;
static DUMMY_INFO_1: usize = 0x3000;
static DUMMY_HII: usize = 0x8000;

unsafe extern "efiapi" fn mock_cred_enroll(
    _this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
) -> EfiStatus {
    if user == EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_cred_form(
    _this: *const EfiUserCredential2Protocol,
    hii: *mut EfiHiiHandle,
    form_set_id: *mut EfiGuid,
    form_id: *mut u16,
) -> EfiStatus {
    if hii.is_null() || form_set_id.is_null() || form_id.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *hii = EfiHiiHandle(DUMMY_HII as *mut c_void);
        *form_set_id = EFI_HII_USER_CREDENTIAL_FORMSET_GUID;
        *form_id = 0x0042;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_tile(
    _this: *const EfiUserCredential2Protocol,
    width: *mut UIntN,
    height: *mut UIntN,
    hii: *mut EfiHiiHandle,
    image: *mut u16,
) -> EfiStatus {
    if width.is_null() || height.is_null() || hii.is_null() || image.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *width = 64;
        *height = 64;
        *hii = EfiHiiHandle(DUMMY_HII as *mut c_void);
        *image = 0x0100;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_title(
    _this: *const EfiUserCredential2Protocol,
    hii: *mut EfiHiiHandle,
    string: *mut u16,
) -> EfiStatus {
    if hii.is_null() || string.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *hii = EfiHiiHandle(DUMMY_HII as *mut c_void);
        *string = 0x0200;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_user(
    _this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
    identifier: *mut [u8; 16],
) -> EfiStatus {
    if user.is_null() || identifier.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *identifier = [0x55; 16];
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_select(
    _this: *const EfiUserCredential2Protocol,
    auto_logon: *mut u32,
) -> EfiStatus {
    if auto_logon.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *auto_logon = EFI_CREDENTIAL_LOGON_FLAG_AUTO;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_deselect(
    _this: *const EfiUserCredential2Protocol,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_default(
    _this: *const EfiUserCredential2Protocol,
    auto_logon: *mut u32,
) -> EfiStatus {
    if auto_logon.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *auto_logon = EFI_CREDENTIAL_LOGON_FLAG_DEFAULT;
    }
    EfiStatus::SUCCESS
}

#[allow(clippy::cast_possible_truncation)]
unsafe extern "efiapi" fn mock_cred_get_info(
    _this: *const EfiUserCredential2Protocol,
    user_info: EfiUserInfoHandle,
    info: *mut EfiUserInfo,
    info_size: *mut UIntN,
) -> EfiStatus {
    if user_info.is_null() || info.is_null() || info_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if *info_size >= core::mem::size_of::<EfiUserInfo>() as UIntN {
            *info = EfiUserInfo {
                credential: EFI_USER_CREDENTIAL_CLASS_PASSWORD,
                info_type: EFI_USER_INFO_NAME_RECORD,
                reserved1: 0,
                info_attribs: 0,
                info_size: core::mem::size_of::<EfiUserInfo>() as u32,
            };
            *info_size = core::mem::size_of::<EfiUserInfo>() as UIntN;
            EfiStatus::SUCCESS
        } else {
            EfiStatus::BUFFER_TOO_SMALL
        }
    }
}

unsafe extern "efiapi" fn mock_cred_get_next_info(
    _this: *const EfiUserCredential2Protocol,
    user_info: *mut EfiUserInfoHandle,
) -> EfiStatus {
    if user_info.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if (*user_info).is_null() {
            *user_info = EfiUserInfoHandle::from_ptr(DUMMY_INFO_1 as *mut c_void);
        } else {
            *user_info = EfiUserInfoHandle::null();
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_cred_delete(
    _this: *const EfiUserCredential2Protocol,
    user: EfiUserProfileHandle,
) -> EfiStatus {
    if user == EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_user_credential_guids_and_constants() {
    assert_eq!(
        EFI_USER_CREDENTIAL2_PROTOCOL_GUID,
        EfiGuid::new(
            0xE98A_DB03,
            0xB8B9,
            0x4AF8,
            [0xBA, 0x20, 0x26, 0xE9, 0x11, 0x4C, 0xBC, 0xE5],
        )
    );
    assert_eq!(
        EFI_USER_CREDENTIAL_PROTOCOL_GUID,
        EfiGuid::new(
            0x71EE_5E94,
            0x65B9,
            0x45D5,
            [0x82, 0x1A, 0x3A, 0x4D, 0x86, 0xCF, 0xE6, 0xBE],
        )
    );
    assert_eq!(
        EFI_HII_USER_CREDENTIAL_FORMSET_GUID,
        EfiGuid::new(
            0x337F_4407,
            0x5AEE,
            0x4B83,
            [0xB2, 0xA7, 0x4E, 0xAD, 0xCA, 0x30, 0x88, 0xCD],
        )
    );
    assert_eq!(EFI_CREDENTIAL_CAPABILITIES_ENROLL, 0x0001);
    assert_eq!(EFI_CREDENTIAL_LOGON_FLAG_AUTO, 0x0001);
    assert_eq!(EFI_CREDENTIAL_LOGON_FLAG_DEFAULT, 0x0002);
}

#[test]
fn test_user_credential2_client_wrapper() {
    let dummy_id = EfiGuid::new(
        0x1111_1111,
        0x2222,
        0x3333,
        [0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
    );
    let mock_proto = EfiUserCredential2Protocol {
        identifier: dummy_id,
        provider_type: EFI_USER_CREDENTIAL_CLASS_PASSWORD,
        enroll: mock_cred_enroll,
        form: mock_cred_form,
        tile: mock_cred_tile,
        title: mock_cred_title,
        user: mock_cred_user,
        select: mock_cred_select,
        deselect: mock_cred_deselect,
        default_fn: mock_cred_default,
        get_info: mock_cred_get_info,
        get_next_info: mock_cred_get_next_info,
        capabilities: EFI_CREDENTIAL_CAPABILITIES_ENROLL,
        delete: mock_cred_delete,
    };

    let cred = UserCredential2::new(&raw const mock_proto);

    assert_eq!(cred.identifier(), dummy_id);
    assert_eq!(cred.provider_type(), EFI_USER_CREDENTIAL_CLASS_PASSWORD);
    assert_eq!(cred.capabilities(), EFI_CREDENTIAL_CAPABILITIES_ENROLL);

    let user1 = EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void);
    assert!(cred.enroll(user1).is_ok());
    assert!(
        cred.enroll(EfiUserProfileHandle::from_ptr(0x9999 as *mut c_void))
            .is_err()
    );

    let (hii, formset, form_id) =
        cred.form()
            .unwrap_or((EfiHiiHandle::NULL, EfiGuid::from_bytes([0u8; 16]), 0));
    assert_eq!(hii, EfiHiiHandle(DUMMY_HII as *mut c_void));
    assert_eq!(formset, EFI_HII_USER_CREDENTIAL_FORMSET_GUID);
    assert_eq!(form_id, 0x0042);

    let mut w = 0usize;
    let mut h = 0usize;
    let (icon_hii, img_id) = cred.tile(&mut w, &mut h).unwrap_or((EfiHiiHandle::NULL, 0));
    assert_eq!(icon_hii, EfiHiiHandle(DUMMY_HII as *mut c_void));
    assert_eq!(img_id, 0x0100);
    assert_eq!(w, 64);
    assert_eq!(h, 64);

    let (text_hii, str_id) = cred.title().unwrap_or((EfiHiiHandle::NULL, 0));
    assert_eq!(text_hii, EfiHiiHandle(DUMMY_HII as *mut c_void));
    assert_eq!(str_id, 0x0200);

    let user_id = cred.user(user1).unwrap_or([0u8; 16]);
    assert_eq!(user_id, [0x55; 16]);

    let logon_flags = cred.select().unwrap_or(0);
    assert_eq!(logon_flags, EFI_CREDENTIAL_LOGON_FLAG_AUTO);

    assert!(cred.deselect().is_ok());

    let def_flags = cred.default_logon().unwrap_or(0);
    assert_eq!(def_flags, EFI_CREDENTIAL_LOGON_FLAG_DEFAULT);

    let mut iter_info = EfiUserInfoHandle::null();
    assert!(cred.get_next_info(&mut iter_info).is_ok());
    assert_eq!(
        iter_info,
        EfiUserInfoHandle::from_ptr(DUMMY_INFO_1 as *mut c_void)
    );

    let mut buf = [0u8; 32];
    let bytes = cred.get_info(iter_info, &mut buf).unwrap_or(0);
    assert_eq!(bytes, core::mem::size_of::<EfiUserInfo>());
    let info = unsafe { core::ptr::read_unaligned(buf.as_ptr().cast::<EfiUserInfo>()) };
    assert_eq!(info.credential, EFI_USER_CREDENTIAL_CLASS_PASSWORD);

    assert!(cred.delete(user1).is_ok());
    assert!(
        cred.delete(EfiUserProfileHandle::from_ptr(0x9999 as *mut c_void))
            .is_err()
    );
}
