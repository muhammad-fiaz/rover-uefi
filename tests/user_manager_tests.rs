//! Integration tests for User Manager protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::security::{
    EFI_EVENT_GROUP_USER_PROFILE_CHANGED, EFI_USER_CREDENTIAL_CLASS_PASSWORD,
    EFI_USER_CREDENTIAL_CLASS_SMART_CARD, EFI_USER_CREDENTIAL2_PROTOCOL_GUID,
    EFI_USER_INFO_ACCESS_SETUP_ADMIN_GUID, EFI_USER_INFO_NAME_RECORD,
    EFI_USER_MANAGER_PROTOCOL_GUID, EfiUserInfo, EfiUserInfoHandle, EfiUserManagerProtocol,
    EfiUserProfileHandle, UserManager,
};

static DUMMY_USER_1: usize = 0x1000;
static DUMMY_USER_2: usize = 0x2000;
static DUMMY_INFO_1: usize = 0x3000;

unsafe extern "efiapi" fn mock_user_create(
    _this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus {
    if user.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *user = EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_user_delete(
    _this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
) -> EfiStatus {
    if user == EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_user_get_next(
    _this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus {
    if user.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if (*user).is_null() {
            *user = EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void);
        } else if *user == EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void) {
            *user = EfiUserProfileHandle::from_ptr(DUMMY_USER_2 as *mut c_void);
        } else {
            *user = EfiUserProfileHandle::null();
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_user_current(
    _this: *const EfiUserManagerProtocol,
    current_user: *mut EfiUserProfileHandle,
) -> EfiStatus {
    if current_user.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *current_user = EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_user_identify(
    _this: *const EfiUserManagerProtocol,
    user: *mut EfiUserProfileHandle,
) -> EfiStatus {
    if user.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *user = EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_user_find(
    _this: *const EfiUserManagerProtocol,
    _user: *mut EfiUserProfileHandle,
    _user_info: *mut EfiUserInfoHandle,
    _info: *const EfiUserInfo,
    _info_size: UIntN,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_user_notify(
    _this: *const EfiUserManagerProtocol,
    _changed: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[allow(clippy::cast_possible_truncation)]
unsafe extern "efiapi" fn mock_user_get_info(
    _this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: EfiUserInfoHandle,
    info: *mut EfiUserInfo,
    info_size: *mut UIntN,
) -> EfiStatus {
    if user.is_null() || user_info.is_null() || info.is_null() || info_size.is_null() {
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

unsafe extern "efiapi" fn mock_user_set_info(
    _this: *const EfiUserManagerProtocol,
    _user: EfiUserProfileHandle,
    _user_info: *mut EfiUserInfoHandle,
    _info: *const EfiUserInfo,
    _info_size: UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_user_delete_info(
    _this: *const EfiUserManagerProtocol,
    user: EfiUserProfileHandle,
    user_info: EfiUserInfoHandle,
) -> EfiStatus {
    if user == EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void)
        && user_info == EfiUserInfoHandle::from_ptr(DUMMY_INFO_1 as *mut c_void)
    {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_user_get_next_info(
    _this: *const EfiUserManagerProtocol,
    _user: EfiUserProfileHandle,
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

#[test]
fn test_user_manager_guids_and_constants() {
    assert_eq!(
        EFI_USER_MANAGER_PROTOCOL_GUID,
        EfiGuid::new(
            0x6FD5_B00C,
            0xD426,
            0x4283,
            [0x98, 0x87, 0x6C, 0xF5, 0xCF, 0x1C, 0xB1, 0xFE],
        )
    );
    assert_eq!(
        EFI_EVENT_GROUP_USER_PROFILE_CHANGED,
        EfiGuid::new(
            0xBAF1_E6DE,
            0x209E,
            0x4ADB,
            [0x8D, 0x96, 0xFD, 0x8B, 0x71, 0xF3, 0xF6, 0x83],
        )
    );
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
        EFI_USER_CREDENTIAL_CLASS_PASSWORD,
        EfiGuid::new(
            0xF8E5_058C,
            0xCCB6,
            0x4714,
            [0xB2, 0x20, 0x3F, 0x7E, 0x3A, 0x64, 0x0B, 0xD1],
        )
    );
    assert_eq!(
        EFI_USER_CREDENTIAL_CLASS_SMART_CARD,
        EfiGuid::new(
            0x5F03_BA33,
            0x8C6B,
            0x4C24,
            [0xAA, 0x2E, 0x14, 0xA2, 0x65, 0x7B, 0xD4, 0x54],
        )
    );
    assert_eq!(
        EFI_USER_INFO_ACCESS_SETUP_ADMIN_GUID,
        EfiGuid::new(
            0x85B7_5607,
            0xF7CE,
            0x471E,
            [0xB7, 0xE4, 0x2A, 0xEA, 0x5F, 0x72, 0x32, 0xEE],
        )
    );
}

#[test]
fn test_user_manager_client_wrapper() {
    let mock_proto = EfiUserManagerProtocol {
        create: mock_user_create,
        delete: mock_user_delete,
        get_next: mock_user_get_next,
        current: mock_user_current,
        identify: mock_user_identify,
        find: mock_user_find,
        notify: mock_user_notify,
        get_info: mock_user_get_info,
        set_info: mock_user_set_info,
        delete_info: mock_user_delete_info,
        get_next_info: mock_user_get_next_info,
    };

    let um = UserManager::new(&mock_proto);

    let user1 = um.create().unwrap_or_else(|_| EfiUserProfileHandle::null());
    assert_eq!(
        user1,
        EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void)
    );

    assert_eq!(
        um.current()
            .unwrap_or_else(|_| EfiUserProfileHandle::null()),
        user1
    );
    assert_eq!(
        um.identify()
            .unwrap_or_else(|_| EfiUserProfileHandle::null()),
        user1
    );

    // Enumerate users
    let mut iter_user = EfiUserProfileHandle::null();
    assert!(um.get_next(&mut iter_user).is_ok());
    assert_eq!(
        iter_user,
        EfiUserProfileHandle::from_ptr(DUMMY_USER_1 as *mut c_void)
    );

    assert!(um.get_next(&mut iter_user).is_ok());
    assert_eq!(
        iter_user,
        EfiUserProfileHandle::from_ptr(DUMMY_USER_2 as *mut c_void)
    );

    assert!(um.get_next(&mut iter_user).is_ok());
    assert!(iter_user.is_null());

    // Enumerate user info
    let mut iter_info = EfiUserInfoHandle::null();
    assert!(um.get_next_info(user1, &mut iter_info).is_ok());
    assert_eq!(
        iter_info,
        EfiUserInfoHandle::from_ptr(DUMMY_INFO_1 as *mut c_void)
    );

    // Read user info
    let mut info_buf = [0u8; 32];
    let bytes_read = um.get_info(user1, iter_info, &mut info_buf).unwrap_or(0);
    assert_eq!(bytes_read, core::mem::size_of::<EfiUserInfo>());

    let info_header = unsafe { core::ptr::read_unaligned(info_buf.as_ptr().cast::<EfiUserInfo>()) };
    assert_eq!(info_header.info_type, EFI_USER_INFO_NAME_RECORD);
    assert_eq!(info_header.credential, EFI_USER_CREDENTIAL_CLASS_PASSWORD);

    // Delete info
    assert!(um.delete_info(user1, iter_info).is_ok());

    // Notify
    let dummy_handle = EfiHandle::from_ptr(0x5000 as *mut c_void);
    assert!(um.notify(dummy_handle).is_ok());

    // Delete user
    assert!(um.delete(user1).is_ok());
    assert!(
        um.delete(EfiUserProfileHandle::from_ptr(0x9999 as *mut c_void))
            .is_err()
    );
}
