//! Integration tests for Key Management Service (KMS) protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::security::{
    EFI_KMS_ATTRIBUTE_TYPE_INTEGER, EFI_KMS_DATA_TYPE_ASCII, EFI_KMS_FORMAT_AESCBC_256_GUID,
    EFI_KMS_FORMAT_GENERIC_256_GUID, EFI_KMS_FORMAT_SHA256_256_GUID, EFI_KMS_PROTOCOL_GUID,
    EfiKmsClientInfo, EfiKmsKeyAttribute, EfiKmsKeyDescriptor, EfiKmsProtocol,
    KeyManagementService,
};

// Mock functions for KMS protocol
unsafe extern "efiapi" fn mock_get_service_status(_this: *const EfiKmsProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_client(
    _this: *const EfiKmsProtocol,
    client: *const EfiKmsClientInfo,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if client.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_create_key(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if key_descriptor_count.is_null() || key_descriptors.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        let count = *key_descriptor_count;
        if count > 0 {
            (*key_descriptors).key_format = EFI_KMS_FORMAT_GENERIC_256_GUID;
            (*key_descriptors).key_status = EfiStatus::SUCCESS;
            *key_descriptor_count = 1;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_key(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if key_descriptor_count.is_null() || key_descriptors.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        let count = *key_descriptor_count;
        if count > 0 {
            (*key_descriptors).key_format = EFI_KMS_FORMAT_AESCBC_256_GUID;
            (*key_descriptors).key_status = EfiStatus::SUCCESS;
            *key_descriptor_count = 1;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_add_key(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if key_descriptor_count.is_null() || key_descriptors.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*key_descriptors).key_status = EfiStatus::SUCCESS;
        *key_descriptor_count = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_delete_key(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    key_descriptor_count: *mut u16,
    key_descriptors: *mut EfiKmsKeyDescriptor,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if key_descriptor_count.is_null() || key_descriptors.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*key_descriptors).key_status = EfiStatus::SUCCESS;
        *key_descriptor_count = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_key_attributes(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    _key_id_size: *const u8,
    _key_id: *const c_void,
    key_attributes_count: *mut u16,
    key_attributes: *mut EfiKmsKeyAttribute,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    if key_attributes_count.is_null() || key_attributes.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*key_attributes).key_attribute_type = EFI_KMS_ATTRIBUTE_TYPE_INTEGER;
        (*key_attributes).key_attribute_status = EfiStatus::SUCCESS;
        *key_attributes_count = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_add_key_attributes(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    _key_id_size: *const u8,
    _key_id: *const c_void,
    _key_attributes_count: *mut u16,
    _key_attributes: *mut EfiKmsKeyAttribute,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_delete_key_attributes(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    _key_id_size: *const u8,
    _key_id: *const c_void,
    _key_attributes_count: *mut u16,
    _key_attributes: *mut EfiKmsKeyAttribute,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_get_key_by_attributes(
    _this: *const EfiKmsProtocol,
    _client: *const EfiKmsClientInfo,
    _key_attribute_count: *mut UIntN,
    _key_attributes: *mut EfiKmsKeyAttribute,
    _key_descriptor_count: *mut UIntN,
    _key_descriptors: *mut EfiKmsKeyDescriptor,
    _client_data_size: *mut UIntN,
    _client_data: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

#[test]
fn test_kms_guid_and_constants() {
    assert_eq!(
        EFI_KMS_PROTOCOL_GUID,
        EfiGuid::new(
            0xEC3A_978D,
            0x7C4E,
            0x48FA,
            [0x9A, 0xBE, 0x6A, 0xD9, 0x1C, 0xC8, 0xF8, 0x11],
        )
    );
    assert_eq!(
        EFI_KMS_FORMAT_GENERIC_256_GUID,
        EfiGuid::new(
            0x70F6_4793,
            0xC323,
            0x4261,
            [0xAC, 0x2C, 0xD8, 0x76, 0xF2, 0x7C, 0x53, 0x45],
        )
    );
    assert_eq!(
        EFI_KMS_FORMAT_SHA256_256_GUID,
        EfiGuid::new(
            0x6BB4_F5CD,
            0x8022,
            0x448D,
            [0xBC, 0x6D, 0x77, 0x1B, 0xAE, 0x93, 0x5F, 0xC6],
        )
    );
    assert_eq!(EFI_KMS_DATA_TYPE_ASCII, 2);
    assert_eq!(EFI_KMS_ATTRIBUTE_TYPE_INTEGER, 0x01);
}

#[test]
fn test_kms_client_wrapper() {
    let mock_proto = EfiKmsProtocol {
        get_service_status: mock_get_service_status,
        register_client: mock_register_client,
        create_key: mock_create_key,
        get_key: mock_get_key,
        add_key: mock_add_key,
        delete_key: mock_delete_key,
        get_key_attributes: mock_get_key_attributes,
        add_key_attributes: mock_add_key_attributes,
        delete_key_attributes: mock_delete_key_attributes,
        get_key_by_attributes: mock_get_key_by_attributes,
        protocol_version: 0x0002_0040,
        service_id: EfiGuid::ZERO,
        service_name: core::ptr::null(),
        service_version: 1,
        service_available: true,
        client_id_supported: true,
        client_id_required: false,
        client_id_max_size: 64,
        client_name_string_types: EFI_KMS_DATA_TYPE_ASCII,
        client_name_required: false,
        client_name_max_count: 32,
        client_data_supported: false,
        client_data_max_size: 0,
        key_id_variable_len_supported: true,
        key_id_max_size: 256,
        key_formats_count: 1,
        key_formats: core::ptr::null(),
        key_attributes_supported: true,
        key_attribute_id_string_types: EFI_KMS_DATA_TYPE_ASCII,
        key_attribute_id_max_count: 32,
        key_attributes_count: 0,
        key_attributes: core::ptr::null(),
    };

    let kms = KeyManagementService::new(&mock_proto);

    assert_eq!(kms.protocol_version(), 0x0002_0040);
    assert!(kms.is_service_available());
    assert!(kms.is_client_id_supported());
    assert!(kms.is_key_attributes_supported());

    assert!(kms.get_service_status().is_ok());

    let client_info = EfiKmsClientInfo::default();
    assert!(kms.register_client(&client_info).is_ok());

    let mut desc = [EfiKmsKeyDescriptor::default()];
    let created = kms.create_key(Some(&client_info), &mut desc).unwrap_or(0);
    assert_eq!(created, 1);
    assert_eq!(desc[0].key_format, EFI_KMS_FORMAT_GENERIC_256_GUID);

    let retrieved = kms.get_key(Some(&client_info), &mut desc).unwrap_or(0);
    assert_eq!(retrieved, 1);
    assert_eq!(desc[0].key_format, EFI_KMS_FORMAT_AESCBC_256_GUID);

    let added = kms.add_key(Some(&client_info), &mut desc).unwrap_or(0);
    assert_eq!(added, 1);

    let deleted = kms.delete_key(Some(&client_info), &mut desc).unwrap_or(0);
    assert_eq!(deleted, 1);

    let mut attrs = [EfiKmsKeyAttribute::default()];
    let attr_cnt = kms
        .get_key_attributes(Some(&client_info), b"key1", &mut attrs)
        .unwrap_or(0);
    assert_eq!(attr_cnt, 1);
    assert_eq!(attrs[0].key_attribute_type, EFI_KMS_ATTRIBUTE_TYPE_INTEGER);
}
