//! # Boot Integrity Services (BIS) Protocol Tests
//!
//! Unit tests for Boot Integrity Services protocol definitions and client wrapper.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::security::bis::{
    BIS_ALG_DSA, BIS_ALG_RSA_MD5, BIS_CERT_ID_DSA, BIS_CERT_ID_MASK, BIS_CERT_ID_RSA_MD5,
    BIS_CURRENT_VERSION_MAJOR, BIS_VERSION_1, BOOT_OBJECT_AUTHORIZATION_PARMSET_GUID, Bis,
    BisApplicationHandle, EFI_BIS_PROTOCOL_GUID, EfiBisData, EfiBisProtocol, EfiBisSignatureInfo,
    EfiBisVersion,
};

#[test]
fn test_bis_guids_and_constants() {
    assert_ne!(
        EFI_BIS_PROTOCOL_GUID,
        BOOT_OBJECT_AUTHORIZATION_PARMSET_GUID
    );

    assert_eq!(BIS_VERSION_1, 1);
    assert_eq!(BIS_CURRENT_VERSION_MAJOR, 1);
    assert_eq!(BIS_ALG_DSA, 41);
    assert_eq!(BIS_ALG_RSA_MD5, 42);
    assert_eq!(BIS_CERT_ID_DSA, 41);
    assert_eq!(BIS_CERT_ID_RSA_MD5, 42);
    assert_eq!(BIS_CERT_ID_MASK, 0xFF7F_7FFF);

    let empty = EfiBisData::empty();
    assert_eq!(empty.length, 0);
    assert!(empty.data.is_null());

    let ver = EfiBisVersion { major: 1, minor: 0 };
    assert_eq!(ver.major, 1);

    let sig_info = EfiBisSignatureInfo {
        certificate_id: BIS_CERT_ID_RSA_MD5,
        algorithm_id: BIS_ALG_RSA_MD5,
        key_length: 2048,
    };
    assert_eq!(sig_info.key_length, 2048);
}

static INIT_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static SHUTDOWN_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static CHECK_FLAG_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static VERIFY_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

const MOCK_APP_HANDLE: usize = 0xABCD_0001;

unsafe extern "efiapi" fn mock_bis_init(
    _this: *mut EfiBisProtocol,
    app_handle: *mut BisApplicationHandle,
    interface_version: *mut EfiBisVersion,
    _target_address: *mut EfiBisData,
) -> EfiStatus {
    INIT_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !app_handle.is_null() {
        unsafe {
            *app_handle = BisApplicationHandle::from_ptr(MOCK_APP_HANDLE as *mut core::ffi::c_void);
        }
    }
    if !interface_version.is_null() {
        unsafe {
            (*interface_version).major = BIS_CURRENT_VERSION_MAJOR;
            (*interface_version).minor = 0;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_shutdown(app_handle: BisApplicationHandle) -> EfiStatus {
    if app_handle.as_ptr() as usize == MOCK_APP_HANDLE {
        SHUTDOWN_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_bis_free(
    _app_handle: BisApplicationHandle,
    _to_free: *mut EfiBisData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_get_cert(
    _app_handle: BisApplicationHandle,
    _certificate: *mut *mut EfiBisData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_get_check_flag(
    _app_handle: BisApplicationHandle,
    check_is_required: *mut bool,
) -> EfiStatus {
    CHECK_FLAG_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !check_is_required.is_null() {
        unsafe {
            *check_is_required = true;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_get_update_token(
    _app_handle: BisApplicationHandle,
    _update_token: *mut *mut EfiBisData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_get_sig_info(
    _app_handle: BisApplicationHandle,
    _signature_info: *mut *mut EfiBisData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_update_auth(
    _app_handle: BisApplicationHandle,
    _request_credential: *mut EfiBisData,
    _new_update_token: *mut *mut EfiBisData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_verify(
    _app_handle: BisApplicationHandle,
    _credentials: *mut EfiBisData,
    _data_object: *mut EfiBisData,
    is_verified: *mut bool,
) -> EfiStatus {
    VERIFY_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !is_verified.is_null() {
        unsafe {
            *is_verified = true;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bis_verify_with_credential(
    _app_handle: BisApplicationHandle,
    _credentials: *mut EfiBisData,
    _data_object: *mut EfiBisData,
    _section_name: *mut EfiBisData,
    _authority_certificate: *mut EfiBisData,
    is_verified: *mut bool,
) -> EfiStatus {
    if !is_verified.is_null() {
        unsafe {
            *is_verified = true;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_bis_client_wrapper() {
    let mut raw = EfiBisProtocol {
        initialize: mock_bis_init,
        shutdown: mock_bis_shutdown,
        free: mock_bis_free,
        get_boot_object_authorization_certificate: mock_bis_get_cert,
        get_boot_object_authorization_check_flag: mock_bis_get_check_flag,
        get_boot_object_authorization_update_token: mock_bis_get_update_token,
        get_signature_info: mock_bis_get_sig_info,
        update_boot_object_authorization: mock_bis_update_auth,
        verify_boot_object: mock_bis_verify,
        verify_object_with_credential: mock_bis_verify_with_credential,
    };

    let mut client = Bis::new(&mut raw);
    assert!(!client.raw().is_null());

    let Ok((handle, ver)) = client.initialize(BIS_VERSION_1) else {
        panic!("initialize failed");
    };
    assert_eq!(handle.as_ptr() as usize, MOCK_APP_HANDLE);
    assert_eq!(ver.major, 1);
    assert!(INIT_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    let Ok(flag) = client.get_check_flag(handle) else {
        panic!("get_check_flag failed");
    };
    assert!(flag);
    assert!(CHECK_FLAG_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    let mut creds = EfiBisData::empty();
    let mut obj = EfiBisData::empty();
    let Ok(verified) = client.verify_boot_object(handle, &mut creds, &mut obj) else {
        panic!("verify_boot_object failed");
    };
    assert!(verified);
    assert!(VERIFY_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(client.shutdown(handle).is_ok());
    assert!(SHUTDOWN_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}
