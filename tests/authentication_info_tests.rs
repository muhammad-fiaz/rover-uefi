//! Integration tests for Authentication Info Protocol.

use core::ffi::c_void;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::security::{
    AuthNodeHeader, AuthenticationInfo, EFI_AUTHENTICATION_CHAP_LOCAL_GUID,
    EFI_AUTHENTICATION_CHAP_RADIUS_GUID, EFI_AUTHENTICATION_INFO_PROTOCOL_GUID,
    EfiAuthenticationInfoProtocol,
};

static mut MOCK_AUTH_DATA: [u8; 16] = [0x77; 16];

unsafe extern "efiapi" fn mock_auth_get(
    _this: *const EfiAuthenticationInfoProtocol,
    controller_handle: EfiHandle,
    buffer: *mut *mut c_void,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if controller_handle == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        unsafe {
            *buffer = core::ptr::addr_of_mut!(MOCK_AUTH_DATA).cast::<c_void>();
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_auth_set(
    _this: *const EfiAuthenticationInfoProtocol,
    controller_handle: EfiHandle,
    buffer: *const c_void,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if controller_handle == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        unsafe {
            let src = core::slice::from_raw_parts(buffer.cast::<u8>(), 16);
            let dst = core::slice::from_raw_parts_mut(
                core::ptr::addr_of_mut!(MOCK_AUTH_DATA).cast::<u8>(),
                16,
            );
            dst.copy_from_slice(src);
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_auth_info_guids_and_header() {
    assert_eq!(
        EFI_AUTHENTICATION_INFO_PROTOCOL_GUID,
        EfiGuid::new(
            0x7671_D9D0,
            0x53DB,
            0x4173,
            [0xAA, 0x69, 0x23, 0x27, 0xF2, 0x1F, 0x0B, 0xC7],
        )
    );
    assert_eq!(
        EFI_AUTHENTICATION_CHAP_RADIUS_GUID,
        EfiGuid::new(
            0xD606_2B50,
            0x15CA,
            0x11DA,
            [0x92, 0x19, 0x00, 0x10, 0x83, 0xFF, 0xCA, 0x4D],
        )
    );
    assert_eq!(
        EFI_AUTHENTICATION_CHAP_LOCAL_GUID,
        EfiGuid::new(
            0xC280_C73E,
            0x15CA,
            0x11DA,
            [0xB0, 0xCA, 0x00, 0x10, 0x83, 0xFF, 0xCA, 0x4D],
        )
    );

    let header = AuthNodeHeader {
        guid: EFI_AUTHENTICATION_CHAP_LOCAL_GUID,
        length: core::mem::size_of::<AuthNodeHeader>() as u16,
    };
    let length = header.length;
    assert_eq!(length, 18);
}

#[test]
fn test_authentication_info_client_wrapper() {
    let mock_proto = EfiAuthenticationInfoProtocol {
        get: mock_auth_get,
        set: mock_auth_set,
    };

    let client = AuthenticationInfo::new(&raw const mock_proto);
    assert_eq!(client.raw(), &raw const mock_proto);

    let valid_handle = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let invalid_handle = EfiHandle::from_ptr(0x9999 as *mut c_void);

    let buf_ptr = client.get(valid_handle).unwrap_or(core::ptr::null_mut());
    assert!(!buf_ptr.is_null());
    let byte0 = unsafe { *buf_ptr.cast::<u8>() };
    assert_eq!(byte0, 0x77);

    assert_eq!(
        client.get(invalid_handle),
        Err(EfiStatus::INVALID_PARAMETER)
    );

    let new_data = [0x99u8; 16];
    assert!(client.set(valid_handle, &new_data).is_ok());

    let updated_ptr = client.get(valid_handle).unwrap_or(core::ptr::null_mut());
    let updated_byte0 = unsafe { *updated_ptr.cast::<u8>() };
    assert_eq!(updated_byte0, 0x99);

    assert_eq!(
        client.set(invalid_handle, &new_data),
        Err(EfiStatus::UNSUPPORTED)
    );
}
