//! Capsule Update Policy and Last Attempt Status integration tests.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::capsule_policy::{
    CAPSULE_UPDATE_POLICY_PROTOCOL_GUID, CapsuleUpdatePolicy, CapsuleUpdatePolicyProtocol,
    last_attempt_status,
};

#[test]
fn test_capsule_policy_guid_and_constants() {
    assert_eq!(
        CAPSULE_UPDATE_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0x4877_84C5,
            0x6299,
            0x4BA6,
            [0xB0, 0x96, 0x5C, 0xC5, 0x27, 0x7C, 0xF7, 0x57]
        )
    );
}

#[test]
fn test_last_attempt_status_ranges() {
    assert!(last_attempt_status::is_fmp_reserved_error(
        last_attempt_status::FMP_RESERVED_MIN
    ));
    assert!(last_attempt_status::is_fmp_reserved_error(
        last_attempt_status::FMP_RESERVED_MAX
    ));
    assert!(!last_attempt_status::is_fmp_reserved_error(0x0FFF));
    assert!(!last_attempt_status::is_fmp_reserved_error(0x2000));

    assert!(last_attempt_status::is_driver_error(
        last_attempt_status::ERROR_GET_FMP_HEADER
    ));
    assert!(last_attempt_status::is_driver_error(
        last_attempt_status::ERROR_VERSION_TOO_LOW
    ));
    assert!(!last_attempt_status::is_driver_error(
        last_attempt_status::ERROR_DEPEX_GET_FAILURE
    ));

    assert!(last_attempt_status::is_dependency_error(
        last_attempt_status::ERROR_DEPEX_GET_FAILURE
    ));
    assert!(last_attempt_status::is_dependency_error(
        last_attempt_status::ERROR_DEPEX_NO_END_OPCODE
    ));
    assert!(!last_attempt_status::is_dependency_error(
        last_attempt_status::ERROR_GET_FMP_HEADER
    ));

    assert!(last_attempt_status::is_device_library_error(
        last_attempt_status::DEVICE_LIBRARY_MIN
    ));
    assert!(last_attempt_status::is_device_library_error(
        last_attempt_status::DEVICE_LIBRARY_MAX
    ));
    assert!(!last_attempt_status::is_device_library_error(0x1000));
}

unsafe extern "efiapi" fn mock_check_system_power(
    _this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus {
    if good.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *good = true };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_check_system_thermal(
    _this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus {
    if good.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *good = true };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_check_system_environment(
    _this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus {
    if good.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *good = false };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_is_lowest_supported_version_check_required(
    _this: *const CapsuleUpdatePolicyProtocol,
) -> bool {
    true
}

unsafe extern "efiapi" fn mock_is_lock_fmp_device_at_lock_event_guid_required(
    _this: *const CapsuleUpdatePolicyProtocol,
) -> bool {
    false
}

#[test]
fn test_capsule_update_policy_wrapper() {
    let raw_proto = CapsuleUpdatePolicyProtocol {
        check_system_power: mock_check_system_power,
        check_system_thermal: mock_check_system_thermal,
        check_system_environment: mock_check_system_environment,
        is_lowest_supported_version_check_required: mock_is_lowest_supported_version_check_required,
        is_lock_fmp_device_at_lock_event_guid_required:
            mock_is_lock_fmp_device_at_lock_event_guid_required,
    };

    let client = CapsuleUpdatePolicy::new(&raw_proto);
    assert_eq!(client.check_system_power(), Ok(true));
    assert_eq!(client.check_system_thermal(), Ok(true));
    assert_eq!(client.check_system_environment(), Ok(false));
    assert!(client.is_lowest_supported_version_check_required());
    assert!(!client.is_lock_fmp_device_at_lock_event_guid_required());

    assert!(unsafe { CapsuleUpdatePolicy::from_raw(core::ptr::null()) }.is_none());
    let wrapped = unsafe { CapsuleUpdatePolicy::from_raw(core::ptr::from_ref(&raw_proto)) };
    assert!(wrapped.is_some());
}
