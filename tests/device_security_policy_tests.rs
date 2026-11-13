//! Integration tests for Device Security and Device Security Policy protocols.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::security::device_security::{
    DEVICE_IDENTIFIER_REVISION, DEVICE_IDENTIFIER_TYPE_PCI_GUID, DEVICE_IDENTIFIER_TYPE_USB_GUID,
    DEVICE_SECURITY_POLICY_PROTOCOL_GUID, DEVICE_SECURITY_POLICY_PROTOCOL_REVISION,
    DEVICE_SECURITY_POLICY_REVISION, DEVICE_SECURITY_PROTOCOL_GUID,
    DEVICE_SECURITY_PROTOCOL_REVISION, DEVICE_SECURITY_STATE_REVISION, DeviceSecurity,
    DeviceSecurityPolicy, DeviceSecurityPolicyData, DeviceSecurityStateData, EfiDeviceIdentifier,
    EfiDeviceSecurityPolicyProtocol, EfiDeviceSecurityProtocol, TPM2_SERVICE_FFA_GUID,
    policy as device_security_policy, security_state as device_security_state, tpm2_ffa,
};

#[test]
fn test_device_security_guids_and_constants() {
    assert_eq!(
        DEVICE_SECURITY_PROTOCOL_GUID,
        EfiGuid::new(
            0x5D6B_38C8,
            0x5510,
            0x4458,
            [0xB4, 0x8D, 0x95, 0x81, 0xCF, 0xA7, 0xB0, 0x0D]
        )
    );
    assert_eq!(
        DEVICE_SECURITY_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0x4386_C174,
            0xAE19,
            0x4896,
            [0x95, 0x73, 0x2F, 0x7E, 0x77, 0x98, 0x96, 0x2E]
        )
    );
    assert_eq!(
        TPM2_SERVICE_FFA_GUID,
        EfiGuid::new(
            0x17B8_62A4,
            0x1806,
            0x4FAF,
            [0x86, 0xB3, 0x08, 0x9A, 0x58, 0x35, 0x38, 0x61]
        )
    );
    assert_eq!(DEVICE_SECURITY_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(DEVICE_IDENTIFIER_REVISION, 0x0001_0000);
    assert_eq!(DEVICE_SECURITY_POLICY_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(DEVICE_SECURITY_POLICY_REVISION, 0x0001_0000);
    assert_eq!(DEVICE_SECURITY_STATE_REVISION, 0x0001_0000);

    assert_eq!(device_security_policy::MEASUREMENT_REQUIRED, 1);
    assert_eq!(device_security_policy::AUTHENTICATION_REQUIRED, 1);
    assert_eq!(device_security_state::SUCCESS, 0);
    assert_eq!(device_security_state::ERROR_DEVICE_ERROR, (1 << 31) | 0x11);
    assert_eq!(tpm2_ffa::GET_INTERFACE_VERSION, 0x0F00_0001);
    assert_eq!(tpm2_ffa::SUCCESS_OK, 0x0500_0001);
    assert_eq!(tpm2_ffa::ERROR_DENIED, 0x8E00_000A);
}

unsafe extern "efiapi" fn mock_device_auth(
    _this: *mut EfiDeviceSecurityProtocol,
    device_id: *mut EfiDeviceIdentifier,
) -> EfiStatus {
    if device_id.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_device_security_wrapper() {
    let mut proto = EfiDeviceSecurityProtocol {
        revision: DEVICE_SECURITY_PROTOCOL_REVISION,
        device_authenticate: mock_device_auth,
    };

    let mut client = DeviceSecurity::new(&mut proto);
    let mut dev = EfiDeviceIdentifier {
        version: DEVICE_IDENTIFIER_REVISION,
        device_type: DEVICE_IDENTIFIER_TYPE_PCI_GUID,
        device_handle: EfiHandle::NULL,
    };
    client.authenticate(&mut dev).unwrap();
}

static mut MOCK_LAST_STATE: DeviceSecurityStateData = DeviceSecurityStateData {
    revision: 0,
    measurement_state: 0,
    authentication_state: 0,
};

unsafe extern "efiapi" fn mock_get_policy(
    _this: *mut EfiDeviceSecurityPolicyProtocol,
    _dev: *const EfiDeviceIdentifier,
    policy: *mut DeviceSecurityPolicyData,
) -> EfiStatus {
    unsafe {
        *policy = DeviceSecurityPolicyData {
            revision: DEVICE_SECURITY_POLICY_REVISION,
            measurement_policy: device_security_policy::MEASUREMENT_REQUIRED,
            authentication_policy: device_security_policy::AUTHENTICATION_REQUIRED,
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_notify_state(
    _this: *mut EfiDeviceSecurityPolicyProtocol,
    _dev: *const EfiDeviceIdentifier,
    state: *const DeviceSecurityStateData,
) -> EfiStatus {
    unsafe {
        MOCK_LAST_STATE = *state;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_device_security_policy_wrapper() {
    let mut proto = EfiDeviceSecurityPolicyProtocol {
        revision: DEVICE_SECURITY_POLICY_PROTOCOL_REVISION,
        get_device_policy: mock_get_policy,
        notify_device_state: mock_notify_state,
    };

    let mut client = DeviceSecurityPolicy::new(&mut proto);
    let dev = EfiDeviceIdentifier {
        version: DEVICE_IDENTIFIER_REVISION,
        device_type: DEVICE_IDENTIFIER_TYPE_USB_GUID,
        device_handle: EfiHandle::NULL,
    };

    let policy = client.get_policy(&dev).unwrap();
    assert_eq!(policy.revision, DEVICE_SECURITY_POLICY_REVISION);
    assert_eq!(
        policy.measurement_policy,
        device_security_policy::MEASUREMENT_REQUIRED
    );

    let state = DeviceSecurityStateData {
        revision: DEVICE_SECURITY_STATE_REVISION,
        measurement_state: device_security_state::SUCCESS,
        authentication_state: device_security_state::SUCCESS,
    };
    client.notify_state(&dev, &state).unwrap();
    assert_eq!(unsafe { MOCK_LAST_STATE }, state);
}
