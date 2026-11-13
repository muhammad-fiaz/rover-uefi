//! # Security
//!
//! RNG, hashing, authentication, measured boot, and Secure Boot
//! interfaces.

pub mod aes;
pub mod cc_measurement;
pub mod device_security;
pub mod ecc;
pub mod gcm;
pub mod guid;
pub mod hash;
pub mod hkdf;
pub mod hmac;
pub mod mor;
pub mod pkcs7;
pub mod rng;
pub mod rsa;
pub mod secure_boot;
pub mod security_arch;
pub mod sha1;
pub mod sha224;
pub mod sha256;
pub mod sha3;
pub mod sha512;
pub mod tcg2;
pub mod tcg_pp;
pub mod types;
pub mod validation;
pub mod variable_lock;
pub mod variable_policy;
pub mod x509;

pub use mor::{
    MEMORY_ONLY_RESET_CONTROL_GUID, MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_GUID,
    MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_NAME, MEMORY_OVERWRITE_REQUEST_VARIABLE_NAME,
    MOR_CLEAR_MEMORY_BIT_MASK, MOR_DISABLEAUTODETECT_BIT_MASK, MemoryOverwriteControl,
    MorLockState,
};
pub use tcg_pp::{
    PHYSICAL_PRESENCE_VARIABLE_NAME, TCG_PHYSICAL_PRESENCE_ACTIVATE, TCG_PHYSICAL_PRESENCE_CLEAR,
    TCG_PHYSICAL_PRESENCE_DATA_GUID, TCG_PHYSICAL_PRESENCE_DEACTIVATE,
    TCG_PHYSICAL_PRESENCE_DEACTIVATE_DISABLE, TCG_PHYSICAL_PRESENCE_DISABLE,
    TCG_PHYSICAL_PRESENCE_ENABLE, TCG_PHYSICAL_PRESENCE_ENABLE_ACTIVATE,
    TCG_PHYSICAL_PRESENCE_NO_ACTION, TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_FALSE,
    TCG_PHYSICAL_PRESENCE_SET_OWNER_INSTALL_TRUE, TCG_PP_RETURN_TPM_BLOCKED,
    TCG_PP_RETURN_TPM_FAILURE, TCG_PP_RETURN_TPM_SUCCESS, TCG_PP_RETURN_USER_ABORT,
    TcgPhysicalPresence, TcgPhysicalPresenceFlags,
};

pub use cc_measurement::{
    EFI_CC_MEASUREMENT_PROTOCOL_GUID, EfiCcGetCapability, EfiCcGetEventLog,
    EfiCcHashLogExtendEvent, EfiCcMapPcrToMrIndex, EfiCcMeasurementBootServiceCapability,
    EfiCcMeasurementEvent, EfiCcMeasurementEventHeader, EfiCcMeasurementProtocol, EfiCcType,
};
pub use device_security::{
    DEVICE_IDENTIFIER_REVISION, DEVICE_IDENTIFIER_TYPE_PCI_GUID, DEVICE_IDENTIFIER_TYPE_USB_GUID,
    DEVICE_SECURITY_PROTOCOL_GUID, DEVICE_SECURITY_PROTOCOL_REVISION, EfiDeviceAuthenticate,
    EfiDeviceIdentifier, EfiDeviceSecurityProtocol,
};
pub use ecc::{Curve, EcdsaError, Modulus, Point, PublicKey, Signature};
pub use guid::{
    HASH_GUID, HASH_SERVICE_BINDING_GUID, HASH2_GUID, HASH2_SERVICE_BINDING_GUID, RNG_GUID,
    SECURITY_ARCH_GUID, SECURITY2_ARCH_GUID, TCG2_FINAL_EVENTS_TABLE_GUID, TCG2_GUID,
};
pub use pkcs7::{
    EFI_PKCS7_VERIFY_PROTOCOL_GUID, EfiPkcs7VerifyBuffer, EfiPkcs7VerifyProtocol,
    EfiPkcs7VerifySignature,
};
pub use sha3::{
    Sha3_224, Sha3_256, Sha3_384, Sha3_512, Sha3Hasher, Shake128, Shake256, ShakeSqueezer,
    sha3_224, sha3_256, sha3_384, sha3_512, shake128_into, shake256_into,
};
pub use types::{GLOBAL_VARIABLE_GUID, IMAGE_SECURITY_DATABASE_GUID};
pub use variable_lock::{
    EfiVariableLockProtocol, EfiVariableLockRequestToLock, VARIABLE_LOCK_PROTOCOL_GUID,
    VariableLock,
};
pub use variable_policy::{
    DisableVariablePolicy, DumpVariablePolicy, GetLockOnVariableStateVariablePolicyInfo,
    GetVariablePolicyInfo, IsVariablePolicyEnabled, LockVariablePolicy, RegisterVariablePolicy,
    VARIABLE_POLICY_ENTRY_REVISION, VARIABLE_POLICY_NO_CANT_ATTR, VARIABLE_POLICY_NO_MAX_SIZE,
    VARIABLE_POLICY_NO_MIN_SIZE, VARIABLE_POLICY_NO_MUST_ATTR, VARIABLE_POLICY_PROTOCOL_GUID,
    VARIABLE_POLICY_PROTOCOL_REVISION, VARIABLE_POLICY_TYPE_LOCK_NOW,
    VARIABLE_POLICY_TYPE_LOCK_ON_CREATE, VARIABLE_POLICY_TYPE_LOCK_ON_VAR_STATE,
    VARIABLE_POLICY_TYPE_NO_LOCK, VariableLockOnVarStatePolicy, VariablePolicy,
    VariablePolicyEntry, VariablePolicyProtocol,
};
