//! # Key Management Service (KMS) Protocol
//!
//! Provides services to generate, store, retrieve, and manage cryptographic keys,
//! certificates, and associated attributes across hardware security modules (HSMs)
//! or network key servers as defined in the UEFI specification.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_KMS_PROTOCOL_GUID`.
pub const EFI_KMS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEC3A_978D,
    0x7C4E,
    0x48FA,
    [0x9A, 0xBE, 0x6A, 0xD9, 0x1C, 0xC8, 0xF8, 0x11],
);

// ---------------------------------------------------------------------------
// KMS Data Types
// ---------------------------------------------------------------------------

/// No KMS data type specified.
pub const EFI_KMS_DATA_TYPE_NONE: u8 = 0;
/// Binary format data.
pub const EFI_KMS_DATA_TYPE_BINARY: u8 = 1;
/// ASCII format string.
pub const EFI_KMS_DATA_TYPE_ASCII: u8 = 2;
/// UCS-2/UTF-16 Unicode format string.
pub const EFI_KMS_DATA_TYPE_UNICODE: u8 = 4;
/// UTF-8 format string.
pub const EFI_KMS_DATA_TYPE_UTF8: u8 = 8;

// ---------------------------------------------------------------------------
// KMS Attribute Types
// ---------------------------------------------------------------------------

/// No attribute type.
pub const EFI_KMS_ATTRIBUTE_TYPE_NONE: u16 = 0x00;
/// 32-bit signed integer attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_INTEGER: u16 = 0x01;
/// 64-bit signed integer attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_LONG_INTEGER: u16 = 0x02;
/// Arbitrary length big integer attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_BIG_INTEGER: u16 = 0x03;
/// Enumeration attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_ENUMERATION: u16 = 0x04;
/// Boolean attribute (1 byte).
pub const EFI_KMS_ATTRIBUTE_TYPE_BOOLEAN: u16 = 0x05;
/// Byte string attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_BYTE_STRING: u16 = 0x06;
/// Text string attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_TEXT_STRING: u16 = 0x07;
/// Date/time attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_DATE_TIME: u16 = 0x08;
/// Time interval attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_INTERVAL: u16 = 0x09;
/// Structured composite attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_STRUCTURE: u16 = 0x0A;
/// Dynamic field attribute.
pub const EFI_KMS_ATTRIBUTE_TYPE_DYNAMIC: u16 = 0x0B;

// ---------------------------------------------------------------------------
// KMS Key Formats (Generic)
// ---------------------------------------------------------------------------

/// Generic 128-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_128_GUID: EfiGuid = EfiGuid::new(
    0xEC8A_3D69,
    0x6DDF,
    0x4108,
    [0x94, 0x76, 0x73, 0x37, 0xFC, 0x52, 0x21, 0x36],
);

/// Generic 160-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_160_GUID: EfiGuid = EfiGuid::new(
    0xA3B3_E6F8,
    0xEFCA,
    0x4BC1,
    [0x88, 0xFB, 0xCB, 0x87, 0x33, 0x9B, 0x25, 0x79],
);

/// Generic 256-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_256_GUID: EfiGuid = EfiGuid::new(
    0x70F6_4793,
    0xC323,
    0x4261,
    [0xAC, 0x2C, 0xD8, 0x76, 0xF2, 0x7C, 0x53, 0x45],
);

/// Generic 512-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_512_GUID: EfiGuid = EfiGuid::new(
    0x978F_E043,
    0xD7AF,
    0x422E,
    [0x8A, 0x92, 0x2B, 0x48, 0xE4, 0x63, 0xBD, 0xE6],
);

/// Generic 1024-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_1024_GUID: EfiGuid = EfiGuid::new(
    0x43BE_0B44,
    0x874B,
    0x4EAD,
    [0xB0, 0x9C, 0x24, 0x1A, 0x4F, 0xBD, 0x7E, 0xB3],
);

/// Generic 2048-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_2048_GUID: EfiGuid = EfiGuid::new(
    0x4009_3F23,
    0x630C,
    0x4626,
    [0x9C, 0x48, 0x40, 0x37, 0x3B, 0x19, 0xCB, 0xBE],
);

/// Generic 3072-bit key format.
pub const EFI_KMS_FORMAT_GENERIC_3072_GUID: EfiGuid = EfiGuid::new(
    0xB923_7513,
    0x6C44,
    0x4411,
    [0xA9, 0x90, 0x21, 0xE5, 0x56, 0xE0, 0x5A, 0xDE],
);

/// Generic dynamic length key format.
pub const EFI_KMS_FORMAT_GENERIC_DYNAMIC_GUID: EfiGuid = EfiGuid::new(
    0x2156_E996,
    0x66DE,
    0x4B27,
    [0x9C, 0xC9, 0xB0, 0x9F, 0xAC, 0x4D, 0x02, 0xBE],
);

// ---------------------------------------------------------------------------
// KMS Key Formats (Hashes)
// ---------------------------------------------------------------------------

/// MD2 128-bit key format.
pub const EFI_KMS_FORMAT_MD2_128_GUID: EfiGuid = EfiGuid::new(
    0x78BE_11C4,
    0xEE44,
    0x4A22,
    [0x9F, 0x05, 0x03, 0x85, 0x2E, 0xC5, 0xC9, 0x78],
);

/// MDC2 128-bit key format.
pub const EFI_KMS_FORMAT_MDC2_128_GUID: EfiGuid = EfiGuid::new(
    0xF7AD_60F8,
    0xEFA8,
    0x44A3,
    [0x91, 0x13, 0x23, 0x1F, 0x39, 0x9E, 0xB4, 0xC7],
);

/// MD4 128-bit key format.
pub const EFI_KMS_FORMAT_MD4_128_GUID: EfiGuid = EfiGuid::new(
    0xD1C1_7AA1,
    0xCAC5,
    0x400F,
    [0xBE, 0x17, 0xE2, 0xA2, 0xAE, 0x06, 0x67, 0x7C],
);

/// MDC4 128-bit key format.
pub const EFI_KMS_FORMAT_MDC4_128_GUID: EfiGuid = EfiGuid::new(
    0x3FA4_F847,
    0xD8EB,
    0x4DF4,
    [0xBD, 0x49, 0x10, 0x3A, 0x0A, 0x84, 0x7B, 0xBC],
);

/// MD5 128-bit key format.
pub const EFI_KMS_FORMAT_MD5_128_GUID: EfiGuid = EfiGuid::new(
    0xDCBC_3662,
    0x9CDA,
    0x4B52,
    [0xA0, 0x4C, 0x82, 0xEB, 0x1D, 0x23, 0x48, 0xC7],
);

/// MD5-SHA 128-bit key format.
pub const EFI_KMS_FORMAT_MD5SHA_128_GUID: EfiGuid = EfiGuid::new(
    0x1C17_8237,
    0x6897,
    0x459E,
    [0x9D, 0x36, 0x67, 0xCE, 0x8E, 0xF9, 0x4F, 0x76],
);

/// SHA-1 160-bit key format.
pub const EFI_KMS_FORMAT_SHA1_160_GUID: EfiGuid = EfiGuid::new(
    0x453C_5E5A,
    0x482D,
    0x43F0,
    [0x87, 0xC9, 0x59, 0x41, 0xF3, 0xA3, 0x8A, 0xC2],
);

/// SHA-256 256-bit key format.
pub const EFI_KMS_FORMAT_SHA256_256_GUID: EfiGuid = EfiGuid::new(
    0x6BB4_F5CD,
    0x8022,
    0x448D,
    [0xBC, 0x6D, 0x77, 0x1B, 0xAE, 0x93, 0x5F, 0xC6],
);

/// SHA-512 512-bit key format.
pub const EFI_KMS_FORMAT_SHA512_512_GUID: EfiGuid = EfiGuid::new(
    0x2F24_0E12,
    0xE14D,
    0x475C,
    [0x83, 0xB0, 0xEF, 0xFF, 0x22, 0xD7, 0x7B, 0xE7],
);

// ---------------------------------------------------------------------------
// KMS Key Formats (Ciphers & Public Key)
// ---------------------------------------------------------------------------

/// AES-XTS 128-bit key format.
pub const EFI_KMS_FORMAT_AESXTS_128_GUID: EfiGuid = EfiGuid::new(
    0x4776_E33F,
    0xDB47,
    0x479A,
    [0xA2, 0x5F, 0xA1, 0xCD, 0x0A, 0xFA, 0xB3, 0x8B],
);

/// AES-XTS 256-bit key format.
pub const EFI_KMS_FORMAT_AESXTS_256_GUID: EfiGuid = EfiGuid::new(
    0xDC7E_8613,
    0xC4BB,
    0x4DB0,
    [0x84, 0x62, 0x13, 0x51, 0x13, 0x57, 0xAB, 0xE2],
);

/// AES-CBC 128-bit key format.
pub const EFI_KMS_FORMAT_AESCBC_128_GUID: EfiGuid = EfiGuid::new(
    0xA0E8_EE6A,
    0x0E92,
    0x44D4,
    [0x86, 0x1B, 0x0E, 0xAA, 0x4A, 0xCA, 0x44, 0xA2],
);

/// AES-CBC 256-bit key format.
pub const EFI_KMS_FORMAT_AESCBC_256_GUID: EfiGuid = EfiGuid::new(
    0xD7E6_9789,
    0x1F68,
    0x45E8,
    [0x96, 0xEF, 0x3B, 0x64, 0x07, 0xA5, 0xB2, 0xDC],
);

/// RSA-SHA1 1024-bit key format.
pub const EFI_KMS_FORMAT_RSASHA1_1024_GUID: EfiGuid = EfiGuid::new(
    0x5641_7BED,
    0x6BBE,
    0x4882,
    [0x86, 0xA0, 0x3A, 0xE8, 0xBB, 0x17, 0xF8, 0xF9],
);

/// RSA-SHA1 2048-bit key format.
pub const EFI_KMS_FORMAT_RSASHA1_2048_GUID: EfiGuid = EfiGuid::new(
    0xF664_47D4,
    0x75A6,
    0x463E,
    [0xA8, 0x19, 0x07, 0x7F, 0x2D, 0xDA, 0x05, 0xE9],
);

/// RSA-SHA256 2048-bit key format.
pub const EFI_KMS_FORMAT_RSASHA256_2048_GUID: EfiGuid = EfiGuid::new(
    0xA477_AF13,
    0x877D,
    0x4060,
    [0xBA, 0xA1, 0x25, 0xD1, 0xBE, 0xA0, 0x8A, 0xD3],
);

/// RSA-SHA256 3072-bit key format.
pub const EFI_KMS_FORMAT_RSASHA256_3072_GUID: EfiGuid = EfiGuid::new(
    0x4E13_56C2,
    0x0EED,
    0x463F,
    [0x81, 0x47, 0x99, 0x33, 0xAB, 0xDB, 0xC7, 0xD5],
);

// ---------------------------------------------------------------------------
// KMS Data Structures
// ---------------------------------------------------------------------------

/// Client information structure supplied during KMS registration or requests.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiKmsClientInfo {
    /// Size in bytes of the client identifier.
    pub client_id_size: u16,
    /// Pointer to client identifier buffer.
    pub client_id: *const c_void,
    /// Client name string format (one of `EFI_KMS_DATA_TYPE_*`).
    pub client_name_type: u8,
    /// Number of characters in the client name.
    pub client_name_count: u8,
    /// Pointer to client name string buffer.
    pub client_name: *const c_void,
}

impl Default for EfiKmsClientInfo {
    fn default() -> Self {
        Self {
            client_id_size: 0,
            client_id: core::ptr::null(),
            client_name_type: EFI_KMS_DATA_TYPE_NONE,
            client_name_count: 0,
            client_name: core::ptr::null(),
        }
    }
}

/// Key descriptor describing a key to be generated, added, or retrieved from KMS.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiKmsKeyDescriptor {
    /// Size in bytes of the key identifier buffer (0-255).
    pub key_identifier_size: u8,
    /// Pointer to key identifier buffer.
    pub key_identifier: *mut c_void,
    /// Key format GUID specifying algorithm and key size.
    pub key_format: EfiGuid,
    /// Pointer to key value payload buffer.
    pub key_value: *mut c_void,
    /// Status code of the individual key operation.
    pub key_status: EfiStatus,
}

impl Default for EfiKmsKeyDescriptor {
    fn default() -> Self {
        Self {
            key_identifier_size: 0,
            key_identifier: core::ptr::null_mut(),
            key_format: EfiGuid::ZERO,
            key_value: core::ptr::null_mut(),
            key_status: EfiStatus::SUCCESS,
        }
    }
}

/// Key attribute descriptor for associating metadata with a KMS key.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiKmsKeyAttribute {
    /// Data type of the attribute identifier.
    pub key_attribute_identifier_type: u8,
    /// Length of the attribute identifier in units specified by its type.
    pub key_attribute_identifier_count: u8,
    /// Pointer to attribute identifier buffer.
    pub key_attribute_identifier: *mut c_void,
    /// Instance number of the attribute (0xFFFF is wildcard).
    pub key_attribute_instance: u16,
    /// Data type of the attribute value (`EFI_KMS_ATTRIBUTE_TYPE_*`).
    pub key_attribute_type: u16,
    /// Size in bytes of the attribute value buffer.
    pub key_attribute_value_size: u16,
    /// Pointer to attribute value buffer.
    pub key_attribute_value: *mut c_void,
    /// Status code of the individual attribute operation.
    pub key_attribute_status: EfiStatus,
}

impl Default for EfiKmsKeyAttribute {
    fn default() -> Self {
        Self {
            key_attribute_identifier_type: EFI_KMS_DATA_TYPE_NONE,
            key_attribute_identifier_count: 0,
            key_attribute_identifier: core::ptr::null_mut(),
            key_attribute_instance: 1,
            key_attribute_type: EFI_KMS_ATTRIBUTE_TYPE_NONE,
            key_attribute_value_size: 0,
            key_attribute_value: core::ptr::null_mut(),
            key_attribute_status: EfiStatus::SUCCESS,
        }
    }
}

// ---------------------------------------------------------------------------
// Protocol Function Signatures
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_KMS_GET_SERVICE_STATUS`.
pub type EfiKmsGetServiceStatus =
    unsafe extern "efiapi" fn(this: *const EfiKmsProtocol) -> EfiStatus;

/// Function pointer for `EFI_KMS_REGISTER_CLIENT`.
pub type EfiKmsRegisterClient = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_CREATE_KEY`.
pub type EfiKmsCreateKey = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_GET_KEY`.
pub type EfiKmsGetKey = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_ADD_KEY`.
pub type EfiKmsAddKey = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_DELETE_KEY`.
pub type EfiKmsDeleteKey = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_GET_KEY_ATTRIBUTES`.
pub type EfiKmsGetKeyAttributes = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_identifier_size: *const u8,
    key_identifier: *const c_void,
    key_attributes_count: *mut u16,
    key_attributes: *mut EfiKmsKeyAttribute,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_ADD_KEY_ATTRIBUTES`.
pub type EfiKmsAddKeyAttributes = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_identifier_size: *const u8,
    key_identifier: *const c_void,
    key_attributes_count: *mut u16,
    key_attributes: *mut EfiKmsKeyAttribute,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_DELETE_KEY_ATTRIBUTES`.
pub type EfiKmsDeleteKeyAttributes = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_identifier_size: *const u8,
    key_identifier: *const c_void,
    key_attributes_count: *mut u16,
    key_attributes: *mut EfiKmsKeyAttribute,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_KMS_GET_KEY_BY_ATTRIBUTES`.
pub type EfiKmsGetKeyByAttributes = unsafe extern "efiapi" fn(
    this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    key_attribute_count: *mut UIntN,
    key_attributes: *mut EfiKmsKeyAttribute,
    key_descriptor_count: *mut UIntN,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    client_data_size: *mut UIntN,
    client_data: *mut *mut c_void,
) -> EfiStatus;

/// Raw ABI structure for Key Management Service Protocol (`EFI_KMS_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiKmsProtocol {
    /// Initiates connection or checks service availability.
    pub get_service_status: EfiKmsGetServiceStatus,
    /// Registers client identifier or credentials with KMS.
    pub register_client: EfiKmsRegisterClient,
    /// Requests generation of new cryptographic keys.
    pub create_key: EfiKmsCreateKey,
    /// Retrieves existing cryptographic keys by identifier.
    pub get_key: EfiKmsGetKey,
    /// Stores new or replacement keys into KMS database.
    pub add_key: EfiKmsAddKey,
    /// Removes cryptographic keys from KMS database.
    pub delete_key: EfiKmsDeleteKey,
    /// Retrieves attributes associated with a key identifier.
    pub get_key_attributes: EfiKmsGetKeyAttributes,
    /// Attaches attributes to an existing key identifier.
    pub add_key_attributes: EfiKmsAddKeyAttributes,
    /// Removes attributes associated with a key identifier.
    pub delete_key_attributes: EfiKmsDeleteKeyAttributes,
    /// Queries keys matching specified attribute criteria.
    pub get_key_by_attributes: EfiKmsGetKeyByAttributes,
    /// Protocol revision version (0x00020040).
    pub protocol_version: u32,
    /// Optional service GUID.
    pub service_id: EfiGuid,
    /// Optional service name string.
    pub service_name: *const Char16,
    /// Optional service version.
    pub service_version: u32,
    /// True if service is connected and ready for requests.
    pub service_available: bool,
    /// True if service supports client identifiers.
    pub client_id_supported: bool,
    /// True if client identifiers are strictly required.
    pub client_id_required: bool,
    /// Maximum client identifier length in bytes.
    pub client_id_max_size: u16,
    /// Bitmask of supported client name string formats.
    pub client_name_string_types: u8,
    /// True if client names are required.
    pub client_name_required: bool,
    /// Maximum character length of client name.
    pub client_name_max_count: u16,
    /// True if arbitrary client data transfer is supported.
    pub client_data_supported: bool,
    /// Maximum size of client data block.
    pub client_data_max_size: UIntN,
    /// True if variable-length key identifiers are supported.
    pub key_id_variable_len_supported: bool,
    /// Maximum or fixed key identifier size.
    pub key_id_max_size: UIntN,
    /// Number of supported key format GUIDs.
    pub key_formats_count: UIntN,
    /// Pointer to array of supported format GUIDs.
    pub key_formats: *const EfiGuid,
    /// True if key attributes are supported.
    pub key_attributes_supported: bool,
    /// Bitmask of supported key attribute identifier types.
    pub key_attribute_id_string_types: u8,
    /// Maximum character length of attribute identifier.
    pub key_attribute_id_max_count: u16,
    /// Predefined key attributes count.
    pub key_attributes_count: UIntN,
    /// Pointer to array of predefined key attributes.
    pub key_attributes: *const EfiKmsKeyAttribute,
}

// ---------------------------------------------------------------------------
// Safe Client Wrapper
// ---------------------------------------------------------------------------

/// Safe wrapper around [`EfiKmsProtocol`].
pub struct KeyManagementService<'a> {
    proto: &'a EfiKmsProtocol,
}

impl<'a> KeyManagementService<'a> {
    /// Wraps a reference to [`EfiKmsProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiKmsProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiKmsProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to a valid and initialized [`EfiKmsProtocol`].
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiKmsProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Queries service status and initiates connection if not yet connected.
    ///
    /// # Errors
    ///
    /// Returns an error status if KMS service cannot be contacted.
    pub fn get_service_status(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.get_service_status)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers client credentials with the KMS.
    ///
    /// # Errors
    ///
    /// Returns an error status if client registration is rejected.
    pub fn register_client(&self, client: &EfiKmsClientInfo) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.register_client)(
                self.proto,
                client,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Requests generation of new cryptographic keys.
    ///
    /// Returns the number of successfully processed key descriptors.
    ///
    /// # Errors
    ///
    /// Returns an error status if key creation fails.
    pub fn create_key(
        &self,
        client: Option<&EfiKmsClientInfo>,
        descriptors: &mut [EfiKmsKeyDescriptor],
    ) -> Result<u16, EfiStatus> {
        let mut count = u16::try_from(descriptors.len()).unwrap_or(u16::MAX);
        let client_ptr = client.map_or(core::ptr::null(), core::ptr::from_ref);

        let status = unsafe {
            (self.proto.create_key)(
                self.proto,
                client_ptr,
                core::ptr::from_mut(&mut count),
                descriptors.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Retrieves existing cryptographic keys by identifier.
    ///
    /// Returns the number of successfully retrieved key descriptors.
    ///
    /// # Errors
    ///
    /// Returns an error status if key retrieval fails.
    pub fn get_key(
        &self,
        client: Option<&EfiKmsClientInfo>,
        descriptors: &mut [EfiKmsKeyDescriptor],
    ) -> Result<u16, EfiStatus> {
        let mut count = u16::try_from(descriptors.len()).unwrap_or(u16::MAX);
        let client_ptr = client.map_or(core::ptr::null(), core::ptr::from_ref);

        let status = unsafe {
            (self.proto.get_key)(
                self.proto,
                client_ptr,
                core::ptr::from_mut(&mut count),
                descriptors.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Adds or updates cryptographic keys in the KMS database.
    ///
    /// Returns the number of successfully added keys.
    ///
    /// # Errors
    ///
    /// Returns an error status if key insertion fails.
    pub fn add_key(
        &self,
        client: Option<&EfiKmsClientInfo>,
        descriptors: &mut [EfiKmsKeyDescriptor],
    ) -> Result<u16, EfiStatus> {
        let mut count = u16::try_from(descriptors.len()).unwrap_or(u16::MAX);
        let client_ptr = client.map_or(core::ptr::null(), core::ptr::from_ref);

        let status = unsafe {
            (self.proto.add_key)(
                self.proto,
                client_ptr,
                core::ptr::from_mut(&mut count),
                descriptors.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Deletes cryptographic keys from the KMS database.
    ///
    /// Returns the number of successfully deleted keys.
    ///
    /// # Errors
    ///
    /// Returns an error status if key deletion fails.
    pub fn delete_key(
        &self,
        client: Option<&EfiKmsClientInfo>,
        descriptors: &mut [EfiKmsKeyDescriptor],
    ) -> Result<u16, EfiStatus> {
        let mut count = u16::try_from(descriptors.len()).unwrap_or(u16::MAX);
        let client_ptr = client.map_or(core::ptr::null(), core::ptr::from_ref);

        let status = unsafe {
            (self.proto.delete_key)(
                self.proto,
                client_ptr,
                core::ptr::from_mut(&mut count),
                descriptors.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Retrieves attributes associated with a specific key identifier.
    ///
    /// Returns the number of successfully retrieved attributes.
    ///
    /// # Errors
    ///
    /// Returns an error status if attribute lookup fails.
    pub fn get_key_attributes(
        &self,
        client: Option<&EfiKmsClientInfo>,
        key_identifier: &[u8],
        attributes: &mut [EfiKmsKeyAttribute],
    ) -> Result<u16, EfiStatus> {
        let mut count = u16::try_from(attributes.len()).unwrap_or(u16::MAX);
        let id_len = u8::try_from(key_identifier.len()).unwrap_or(u8::MAX);
        let client_ptr = client.map_or(core::ptr::null(), core::ptr::from_ref);

        let status = unsafe {
            (self.proto.get_key_attributes)(
                self.proto,
                client_ptr,
                core::ptr::from_ref(&id_len),
                key_identifier.as_ptr().cast::<c_void>(),
                core::ptr::from_mut(&mut count),
                attributes.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Returns protocol specification version.
    #[must_use]
    pub const fn protocol_version(&self) -> u32 {
        self.proto.protocol_version
    }

    /// Returns whether the KMS is available for use.
    #[must_use]
    pub const fn is_service_available(&self) -> bool {
        self.proto.service_available
    }

    /// Returns whether client identifiers are supported.
    #[must_use]
    pub const fn is_client_id_supported(&self) -> bool {
        self.proto.client_id_supported
    }

    /// Returns whether key attributes are supported.
    #[must_use]
    pub const fn is_key_attributes_supported(&self) -> bool {
        self.proto.key_attributes_supported
    }
}
