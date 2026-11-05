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
pub mod types;
pub mod validation;
pub mod x509;

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
