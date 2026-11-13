//! # Capsule Update Policy and Last Attempt Status
//!
//! Provides platform policy services invoked prior to firmware capsule execution,
//! verifying system power state, thermals, environment conditions, version rollback
//! policies, and device security locking. Also defines granular status code ranges
//! and errors for EFI System Resource Table (ESRT) and Firmware Management Protocol (FMP)
//! `LastAttemptStatus` reporting.

use crate::base::{EfiGuid, EfiStatus};

/// The `CAPSULE_UPDATE_POLICY_PROTOCOL_GUID`.
pub const CAPSULE_UPDATE_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4877_84C5,
    0x6299,
    0x4BA6,
    [0xB0, 0x96, 0x5C, 0xC5, 0x27, 0x7C, 0xF7, 0x57],
);

/// Function pointer for `CAPSULE_UPDATE_POLICY_CHECK_SYSTEM_POWER`.
pub type CapsuleUpdatePolicyCheckSystemPower = unsafe extern "efiapi" fn(
    this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus;

/// Function pointer for `CAPSULE_UPDATE_POLICY_CHECK_SYSTEM_THERMAL`.
pub type CapsuleUpdatePolicyCheckSystemThermal = unsafe extern "efiapi" fn(
    this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus;

/// Function pointer for `CAPSULE_UPDATE_POLICY_CHECK_SYSTEM_ENVIRONMENT`.
pub type CapsuleUpdatePolicyCheckSystemEnvironment = unsafe extern "efiapi" fn(
    this: *const CapsuleUpdatePolicyProtocol,
    good: *mut bool,
) -> EfiStatus;

/// Function pointer for `CAPSULE_UPDATE_POLICY_IS_LOWEST_SUPPORTED_VERSION_CHECK_REQUIRED`.
pub type CapsuleUpdatePolicyIsLowestSupportedVersionCheckRequired =
    unsafe extern "efiapi" fn(this: *const CapsuleUpdatePolicyProtocol) -> bool;

/// Function pointer for `CAPSULE_UPDATE_POLICY_IS_FMP_DEVICE_AT_LOCK_EVENT_REQUIRED`.
pub type CapsuleUpdatePolicyIsLockFmpDeviceAtLockEventGuidRequired =
    unsafe extern "efiapi" fn(this: *const CapsuleUpdatePolicyProtocol) -> bool;

/// Interface structure for Capsule Update Policy Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct CapsuleUpdatePolicyProtocol {
    /// Determines whether current system power state supports capsule update.
    pub check_system_power: CapsuleUpdatePolicyCheckSystemPower,
    /// Determines whether current system thermal state supports capsule update.
    pub check_system_thermal: CapsuleUpdatePolicyCheckSystemThermal,
    /// Determines whether current system environment supports capsule update.
    pub check_system_environment: CapsuleUpdatePolicyCheckSystemEnvironment,
    /// Determines whether lowest supported version rollback check is enforced.
    pub is_lowest_supported_version_check_required:
        CapsuleUpdatePolicyIsLowestSupportedVersionCheckRequired,
    /// Determines whether the FMP device must be locked at the lock event.
    pub is_lock_fmp_device_at_lock_event_guid_required:
        CapsuleUpdatePolicyIsLockFmpDeviceAtLockEventGuidRequired,
}

/// Safe wrapper around [`CapsuleUpdatePolicyProtocol`].
pub struct CapsuleUpdatePolicy<'a> {
    proto: &'a CapsuleUpdatePolicyProtocol,
}

impl<'a> CapsuleUpdatePolicy<'a> {
    /// Wraps a reference to [`CapsuleUpdatePolicyProtocol`].
    #[must_use]
    pub const fn new(proto: &'a CapsuleUpdatePolicyProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`CapsuleUpdatePolicyProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const CapsuleUpdatePolicyProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Checks whether the system power state is adequate for a capsule update.
    ///
    /// # Errors
    ///
    /// Returns an error status if system power state cannot be determined.
    pub fn check_system_power(&self) -> Result<bool, EfiStatus> {
        let mut good = false;
        let status =
            unsafe { (self.proto.check_system_power)(self.proto, core::ptr::from_mut(&mut good)) };
        if status == EfiStatus::SUCCESS {
            Ok(good)
        } else {
            Err(status)
        }
    }

    /// Checks whether the system thermal state is adequate for a capsule update.
    ///
    /// # Errors
    ///
    /// Returns an error status if system thermal state cannot be determined.
    pub fn check_system_thermal(&self) -> Result<bool, EfiStatus> {
        let mut good = false;
        let status = unsafe {
            (self.proto.check_system_thermal)(self.proto, core::ptr::from_mut(&mut good))
        };
        if status == EfiStatus::SUCCESS {
            Ok(good)
        } else {
            Err(status)
        }
    }

    /// Checks whether the overall system environment supports a capsule update.
    ///
    /// # Errors
    ///
    /// Returns an error status if system environment cannot be determined.
    pub fn check_system_environment(&self) -> Result<bool, EfiStatus> {
        let mut good = false;
        let status = unsafe {
            (self.proto.check_system_environment)(self.proto, core::ptr::from_mut(&mut good))
        };
        if status == EfiStatus::SUCCESS {
            Ok(good)
        } else {
            Err(status)
        }
    }

    /// Checks whether the lowest supported version check must be performed.
    #[must_use]
    pub fn is_lowest_supported_version_check_required(&self) -> bool {
        unsafe { (self.proto.is_lowest_supported_version_check_required)(self.proto) }
    }

    /// Checks whether the FMP device must be locked at the designated lock event.
    #[must_use]
    pub fn is_lock_fmp_device_at_lock_event_guid_required(&self) -> bool {
        unsafe { (self.proto.is_lock_fmp_device_at_lock_event_guid_required)(self.proto) }
    }
}

/// Last Attempt Status error code definitions for ESRT and FMP reporting.
pub mod last_attempt_status {
    /// Minimum value of the FMP reserved vendor error code range.
    pub const FMP_RESERVED_MIN: u32 = 0x1000;
    /// Maximum value of the FMP reserved vendor error code range.
    pub const FMP_RESERVED_MAX: u32 = 0x1FFF;

    /// Driver-specific error range minimum.
    pub const DRIVER_MIN: u32 = 0x1000;
    /// Driver-specific error range maximum.
    pub const DRIVER_MAX: u32 = 0x107F;

    /// Dependency evaluator error range minimum.
    pub const DEPENDENCY_MIN: u32 = 0x1080;
    /// Dependency evaluator error range maximum.
    pub const DEPENDENCY_MAX: u32 = 0x109F;

    /// Dependency checker error range minimum.
    pub const DEPENDENCY_CHECK_MIN: u32 = 0x10A0;
    /// Dependency checker error range maximum.
    pub const DEPENDENCY_CHECK_MAX: u32 = 0x10BF;

    /// Device library implementation error range minimum.
    pub const DEVICE_LIBRARY_MIN: u32 = 0x1800;
    /// Device library implementation error range maximum.
    pub const DEVICE_LIBRARY_MAX: u32 = 0x1FFF;

    // Granular driver error codes
    /// Failed to retrieve FMP capsule header.
    pub const ERROR_GET_FMP_HEADER: u32 = 0x1000;
    /// Progress reporting callback error.
    pub const ERROR_PROGRESS_CALLBACK: u32 = 0x1001;
    /// System power check API returned error.
    pub const ERROR_CHECK_POWER_API: u32 = 0x1002;
    /// System thermal check API returned error.
    pub const ERROR_CHECK_SYS_THERMAL_API: u32 = 0x1003;
    /// System thermal state prevents capsule update.
    pub const ERROR_THERMAL_INADEQUATE: u32 = 0x1004;
    /// System environment check API returned error.
    pub const ERROR_CHECK_SYS_ENV_API: u32 = 0x1005;
    /// System environment state prevents capsule update.
    pub const ERROR_SYSTEM_ENV_INADEQUATE: u32 = 0x1006;
    /// Invalid FMP capsule header size.
    pub const ERROR_GET_FMP_HEADER_SIZE: u32 = 0x1007;
    /// Invalid aggregate capsule header size.
    pub const ERROR_GET_ALL_HEADER_SIZE: u32 = 0x1008;
    /// Unsupported FMP capsule header version.
    pub const ERROR_GET_FMP_HEADER_VERSION: u32 = 0x1009;
    /// No payload image provided in capsule.
    pub const ERROR_IMAGE_NOT_PROVIDED: u32 = 0x100A;
    /// Target image is marked not updatable.
    pub const ERROR_IMAGE_NOT_UPDATABLE: u32 = 0x100B;
    /// Capsule security certificate is invalid.
    pub const ERROR_INVALID_CERTIFICATE: u32 = 0x100C;
    /// Target image index is out of bounds.
    pub const ERROR_INVALID_IMAGE_INDEX: u32 = 0x100D;
    /// Invalid key length encountered in authentication block.
    pub const ERROR_INVALID_KEY_LENGTH: u32 = 0x100E;
    /// Invalid key length value encountered.
    pub const ERROR_INVALID_KEY_LENGTH_VALUE: u32 = 0x100F;
    /// Version is lower than the lowest supported version.
    pub const ERROR_VERSION_TOO_LOW: u32 = 0x1010;
    /// FMP target device is locked against updates.
    pub const ERROR_DEVICE_LOCKED: u32 = 0x1011;
    /// Image cryptographic authentication failed.
    pub const ERROR_IMAGE_AUTH_FAILURE: u32 = 0x1012;
    /// Required protocol argument missing.
    pub const ERROR_PROTOCOL_ARG_MISSING: u32 = 0x1013;

    // Granular dependency evaluation error codes
    /// Failed to retrieve capsule dependency expression.
    pub const ERROR_DEPEX_GET_FAILURE: u32 = 0x1080;
    /// Missing terminating `END` opcode in dependency expression.
    pub const ERROR_DEPEX_NO_END_OPCODE: u32 = 0x1081;
    /// Unknown opcode encountered in dependency stream.
    pub const ERROR_DEPEX_UNKNOWN_OPCODE: u32 = 0x1082;
    /// GUID operand extends beyond dependency stream boundary.
    pub const ERROR_DEPEX_GUID_OVERFLOW: u32 = 0x1083;
    /// Version operand extends beyond dependency stream boundary.
    pub const ERROR_DEPEX_VERSION_OVERFLOW: u32 = 0x1084;
    /// Version string operand extends beyond dependency stream boundary.
    pub const ERROR_DEPEX_VERSION_STR_OVERFLOW: u32 = 0x1085;
    /// Target FMP protocol instance not found during dependency evaluation.
    pub const ERROR_DEPEX_FMP_NOT_FOUND: u32 = 0x1086;
    /// Evaluation stack push failure (overflow).
    pub const ERROR_DEPEX_STACK_OVERFLOW: u32 = 0x1087;
    /// Evaluation stack pop failure (underflow).
    pub const ERROR_DEPEX_STACK_UNDERFLOW: u32 = 0x1088;
    /// Declared length opcode is not the first opcode in expression.
    pub const ERROR_DEPEX_DECLARE_LENGTH_NOT_FIRST: u32 = 0x1089;
    /// Declared length operand extends beyond stream boundary.
    pub const ERROR_DEPEX_DECLARE_LENGTH_OVERFLOW: u32 = 0x108A;
    /// Declared length does not match actual dependency expression size.
    pub const ERROR_DEPEX_DECLARE_LENGTH_MISMATCH: u32 = 0x108B;

    // Granular dependency check error codes
    /// Required FMP protocol not found during preflight dependency checks.
    pub const ERROR_DEPCHECK_FMP_NOT_FOUND: u32 = 0x10A0;
    /// Memory allocation failure while allocating FMP info buffer.
    pub const ERROR_DEPCHECK_INFO_ALLOC_FAILED: u32 = 0x10A1;
    /// Memory allocation failure while allocating descriptor version buffer.
    pub const ERROR_DEPCHECK_DESC_VER_ALLOC_FAILED: u32 = 0x10A2;
    /// Memory allocation failure while allocating descriptor size buffer.
    pub const ERROR_DEPCHECK_DESC_SIZE_ALLOC_FAILED: u32 = 0x10A3;
    /// Memory allocation failure while allocating FMP version buffer.
    pub const ERROR_DEPCHECK_FMP_VER_ALLOC_FAILED: u32 = 0x10A4;

    /// Validates whether `code` falls within the designated FMP vendor reserved range.
    #[must_use]
    pub const fn is_fmp_reserved_error(code: u32) -> bool {
        code >= FMP_RESERVED_MIN && code <= FMP_RESERVED_MAX
    }

    /// Validates whether `code` falls within the driver error code range.
    #[must_use]
    pub const fn is_driver_error(code: u32) -> bool {
        code >= DRIVER_MIN && code <= DRIVER_MAX
    }

    /// Validates whether `code` falls within the dependency evaluation error code range.
    #[must_use]
    pub const fn is_dependency_error(code: u32) -> bool {
        code >= DEPENDENCY_MIN && code <= DEPENDENCY_MAX
    }

    /// Validates whether `code` falls within the device library error code range.
    #[must_use]
    pub const fn is_device_library_error(code: u32) -> bool {
        code >= DEVICE_LIBRARY_MIN && code <= DEVICE_LIBRARY_MAX
    }
}
