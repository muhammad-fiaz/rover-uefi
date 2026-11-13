//! Integration tests for Redfish Credential protocols.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::network::redfish_credential::{
    EfiRedfishCredential2Protocol, EfiRedfishCredentialProtocol, REDFISH_CREDENTIAL_PROTOCOL_GUID,
    REDFISH_CREDENTIAL_PROTOCOL_REVISION, REDFISH_CREDENTIAL2_PROTOCOL_GUID, RedfishAuthMethod,
    RedfishCredential, RedfishCredential2, RedfishCredentialStopServiceType,
};

static mut MOCK_USER: [u8; 6] = *b"admin\0";
static mut MOCK_PASS: [u8; 9] = *b"password\0";
static mut SERVICE_STOPPED: bool = false;
static mut REGISTERED_SERVICE: usize = 0;

unsafe extern "efiapi" fn mock_get_auth_info(
    _this: *const EfiRedfishCredentialProtocol,
    auth_method: *mut RedfishAuthMethod,
    user_id: *mut *mut u8,
    password: *mut *mut u8,
) -> EfiStatus {
    unsafe {
        *auth_method = RedfishAuthMethod::RedfishSession;
        *user_id = (&raw mut MOCK_USER).cast::<u8>();
        *password = (&raw mut MOCK_PASS).cast::<u8>();
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_stop_service(
    _this: *const EfiRedfishCredentialProtocol,
    _stop_type: RedfishCredentialStopServiceType,
) -> EfiStatus {
    unsafe {
        SERVICE_STOPPED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_auth_info2(
    _this: *const EfiRedfishCredential2Protocol,
    auth_method: *mut RedfishAuthMethod,
    user_id: *mut *mut u8,
    password: *mut *mut u8,
) -> EfiStatus {
    unsafe {
        *auth_method = RedfishAuthMethod::HttpBasic;
        *user_id = (&raw mut MOCK_USER).cast::<u8>();
        *password = (&raw mut MOCK_PASS).cast::<u8>();
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_stop_service2(
    _this: *const EfiRedfishCredential2Protocol,
    _stop_type: RedfishCredentialStopServiceType,
) -> EfiStatus {
    unsafe {
        SERVICE_STOPPED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_service(
    _this: *const EfiRedfishCredential2Protocol,
    service: *const core::ffi::c_void,
) -> EfiStatus {
    unsafe {
        REGISTERED_SERVICE = service as usize;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_service(
    _this: *const EfiRedfishCredential2Protocol,
    _service: *const core::ffi::c_void,
) -> EfiStatus {
    unsafe {
        REGISTERED_SERVICE = 0;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_redfish_credential_guids_and_constants() {
    assert_eq!(
        REDFISH_CREDENTIAL_PROTOCOL_GUID,
        EfiGuid::new(
            0x0880_4377,
            0xAF7A,
            0x4496,
            [0x8A, 0x7B, 0x17, 0x59, 0x00, 0xE9, 0xAB, 0x46],
        )
    );
    assert_eq!(
        REDFISH_CREDENTIAL2_PROTOCOL_GUID,
        EfiGuid::new(
            0x936B_81DC,
            0x348C,
            0x42E3,
            [0x9E, 0x82, 0x02, 0x91, 0x4F, 0xD3, 0x48, 0x86],
        )
    );
    assert_eq!(REDFISH_CREDENTIAL_PROTOCOL_REVISION, 0x0001_0000);
}

#[test]
fn test_redfish_credential_wrapper() {
    let mock_proto = EfiRedfishCredentialProtocol {
        get_auth_info: mock_get_auth_info,
        stop_service: mock_stop_service,
    };

    let cred = unsafe { RedfishCredential::from_raw(&raw const mock_proto) };
    let info = cred.get_auth_info();
    assert!(info.is_ok());
    let (method, user_ptr, pass_ptr) = info.unwrap_or((
        RedfishAuthMethod::None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    ));
    assert_eq!(method, RedfishAuthMethod::RedfishSession);
    assert!(!user_ptr.is_null());
    assert!(!pass_ptr.is_null());

    assert!(
        cred.stop_service(RedfishCredentialStopServiceType::ExitBootService)
            .is_ok()
    );
    assert!(unsafe { SERVICE_STOPPED });
}

#[test]
fn test_redfish_credential2_wrapper() {
    let mock_proto = EfiRedfishCredential2Protocol {
        revision: REDFISH_CREDENTIAL_PROTOCOL_REVISION,
        get_auth_info: mock_get_auth_info2,
        stop_service: mock_stop_service2,
        register_redfish_service: mock_register_service,
        unregister_redfish_service: mock_unregister_service,
    };

    let cred2 = unsafe { RedfishCredential2::from_raw(&raw const mock_proto) };
    assert_eq!(cred2.revision(), REDFISH_CREDENTIAL_PROTOCOL_REVISION);

    let info = cred2.get_auth_info();
    assert!(info.is_ok());
    let (method, _, _) = info.unwrap_or((
        RedfishAuthMethod::None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    ));
    assert_eq!(method, RedfishAuthMethod::HttpBasic);

    let dummy_svc = 0x1234_5678 as *const core::ffi::c_void;
    assert!(unsafe { cred2.register_service(dummy_svc) }.is_ok());
    assert_eq!(unsafe { REGISTERED_SERVICE }, 0x1234_5678);

    assert!(unsafe { cred2.unregister_service(dummy_svc) }.is_ok());
    assert_eq!(unsafe { REGISTERED_SERVICE }, 0);
}
