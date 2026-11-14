// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Unit tests for advanced firmware protocols, PPIs, SMM events, and capsules.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::{
    DisplayDisplayPayload, EFI_BDS_ARCH_PROTOCOL_GUID, EFI_DXE_MM_READY_TO_LOCK_PROTOCOL_GUID,
    EFI_DXE_SMM_READY_TO_LOCK_PROTOCOL_GUID, EFI_JSON_CAPSULE_DATA_TABLE_GUID,
    EFI_JSON_CAPSULE_ID_GUID, EFI_JSON_CAPSULE_RESULT_TABLE_GUID, EFI_JSON_CONFIG_DATA_TABLE_GUID,
    EFI_MM_END_OF_DXE_PROTOCOL_GUID, EFI_MM_END_OF_S3_RESUME_PROTOCOL_GUID,
    EFI_MM_READY_TO_LOCK_PROTOCOL_GUID, EFI_PEI_CAPSULE_ON_DISK_PPI_GUID, EFI_PEI_CAPSULE_PPI_GUID,
    EFI_PEI_END_OF_PEI_PHASE_PPI_GUID, EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID,
    EFI_PEI_RESET_PPI_GUID, EFI_PEI_RESET2_PPI_GUID, EFI_PEI_S3_RESUME2_PPI_GUID,
    EFI_PEI_STALL_PPI_GUID, EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID, EFI_RSC_HANDLER_PROTOCOL_GUID,
    EFI_SMM_END_OF_DXE_PROTOCOL_GUID, EFI_SMM_READY_TO_LOCK_PROTOCOL_GUID,
    EFI_SMM_VARIABLE_WRITE_GUID, EfiBdsArchProtocol, EfiJsonCapsuleHeader,
    EfiPeiReadOnlyVariable2Ppi, EfiRscHandlerProtocol, OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID,
    OvmfSevMemoryAcceptanceProtocol, SMM_VARIABLE_FUNCTION_GET_VARIABLE,
    SMM_VARIABLE_FUNCTION_SET_VARIABLE, WINDOWS_UX_CAPSULE_GUID,
    parse_guid_defined_section2_header,
};

#[test]
fn test_advanced_capsule_guids_and_headers() {
    assert!(!EFI_PEI_CAPSULE_PPI_GUID.is_zero());
    assert!(!EFI_PEI_CAPSULE_ON_DISK_PPI_GUID.is_zero());
    assert!(!WINDOWS_UX_CAPSULE_GUID.is_zero());
    assert!(!EFI_JSON_CONFIG_DATA_TABLE_GUID.is_zero());
    assert!(!EFI_JSON_CAPSULE_DATA_TABLE_GUID.is_zero());
    assert!(!EFI_JSON_CAPSULE_RESULT_TABLE_GUID.is_zero());
    assert!(!EFI_JSON_CAPSULE_ID_GUID.is_zero());

    let payload = DisplayDisplayPayload {
        version: 1,
        checksum: 0,
        image_type: 0,
        reserved: 0,
        mode: 1,
        offset_x: 100,
        offset_y: 200,
    };
    assert_eq!(payload.version, 1);
    assert_eq!({ payload.offset_x }, 100);

    let json_hdr = EfiJsonCapsuleHeader {
        version: 1,
        capsule_id: 42,
        payload_length: 1024,
    };
    assert_eq!({ json_hdr.capsule_id }, 42);
    assert_eq!({ json_hdr.payload_length }, 1024);
}

#[test]
fn test_smm_notification_and_variable_communication() {
    assert_eq!(
        EFI_MM_READY_TO_LOCK_PROTOCOL_GUID,
        EFI_SMM_READY_TO_LOCK_PROTOCOL_GUID
    );
    assert_eq!(
        EFI_DXE_MM_READY_TO_LOCK_PROTOCOL_GUID,
        EFI_DXE_SMM_READY_TO_LOCK_PROTOCOL_GUID
    );
    assert_eq!(
        EFI_MM_END_OF_DXE_PROTOCOL_GUID,
        EFI_SMM_END_OF_DXE_PROTOCOL_GUID
    );
    assert!(!EFI_MM_END_OF_S3_RESUME_PROTOCOL_GUID.is_zero());
    assert!(!EFI_SMM_VARIABLE_WRITE_GUID.is_zero());

    assert_eq!(SMM_VARIABLE_FUNCTION_GET_VARIABLE, 1);
    assert_eq!(SMM_VARIABLE_FUNCTION_SET_VARIABLE, 3);
}

unsafe extern "efiapi" fn mock_allow(_this: *const OvmfSevMemoryAcceptanceProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bds_entry(_this: *mut EfiBdsArchProtocol) {}

unsafe extern "efiapi" fn mock_register(
    _callback: rover_uefi::protocols::firmware::EfiRscHandlerCallback,
    _tpl: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister(
    _callback: rover_uefi::protocols::firmware::EfiRscHandlerCallback,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_var(
    _this: *const EfiPeiReadOnlyVariable2Ppi,
    _name: *const rover_uefi::abi::types::Char16,
    _guid: *const EfiGuid,
    _attr: *mut u32,
    _size: *mut rover_uefi::abi::types::UIntN,
    _data: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

unsafe extern "efiapi" fn mock_next_var(
    _this: *const EfiPeiReadOnlyVariable2Ppi,
    _size: *mut rover_uefi::abi::types::UIntN,
    _name: *mut rover_uefi::abi::types::Char16,
    _guid: *mut EfiGuid,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

#[test]
fn test_sev_memory_acceptance_protocol() {
    assert!(!OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID.is_zero());

    let proto = OvmfSevMemoryAcceptanceProtocol {
        allow_unaccepted_memory: Some(mock_allow),
    };
    let wrapper = rover_uefi::protocols::firmware::SevMemoryAcceptance::new(&proto);
    assert!(wrapper.allow_unaccepted_memory().is_ok());
}

#[test]
fn test_bds_and_rsc_arch_protocols() {
    assert!(!EFI_BDS_ARCH_PROTOCOL_GUID.is_zero());
    assert!(!EFI_RSC_HANDLER_PROTOCOL_GUID.is_zero());

    let bds = EfiBdsArchProtocol {
        entry: mock_bds_entry,
    };
    assert_eq!(
        bds.entry as *const () as usize,
        mock_bds_entry as *const () as usize
    );

    let rsc = EfiRscHandlerProtocol {
        register: mock_register,
        unregister: mock_unregister,
    };
    assert_eq!(
        rsc.register as *const () as usize,
        mock_register as *const () as usize
    );
    assert_eq!(
        rsc.unregister as *const () as usize,
        mock_unregister as *const () as usize
    );
}

#[test]
fn test_core_ppis() {
    assert!(!EFI_PEI_END_OF_PEI_PHASE_PPI_GUID.is_zero());
    assert!(!EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID.is_zero());
    assert!(!EFI_PEI_RESET_PPI_GUID.is_zero());
    assert!(!EFI_PEI_RESET2_PPI_GUID.is_zero());
    assert!(!EFI_PEI_STALL_PPI_GUID.is_zero());
    assert!(!EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID.is_zero());
    assert!(!EFI_PEI_S3_RESUME2_PPI_GUID.is_zero());

    let ro_var = EfiPeiReadOnlyVariable2Ppi {
        get_variable: mock_get_var,
        next_variable_name: mock_next_var,
    };
    assert_eq!(
        ro_var.get_variable as *const () as usize,
        mock_get_var as *const () as usize
    );
    assert_eq!(
        ro_var.next_variable_name as *const () as usize,
        mock_next_var as *const () as usize
    );
}

#[test]
fn test_extended_guid_defined_section2() {
    let mut buffer = [0_u8; 32];
    // Guid: 16 bytes
    let test_guid = EfiGuid::new(
        0x1234_5678,
        0xABCD,
        0xEF01,
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
    );
    buffer[..16].copy_from_slice(&test_guid.to_bytes());
    // DataOffset: 28 (2 bytes) -> 8 (CommonHeader2) + 20 = 28
    buffer[16..18].copy_from_slice(&28_u16.to_le_bytes());
    // Attributes: 0x0001 (2 bytes)
    buffer[18..20].copy_from_slice(&0x0001_u16.to_le_bytes());
    // Payload starts at inner_offset = 28 - 8 = 20
    buffer[20] = 0xAA;

    let res = parse_guid_defined_section2_header(&buffer);
    assert!(res.is_some());
    let (hdr, data) = res.expect("valid extended section header");
    assert_eq!(hdr.section_definition_guid, test_guid);
    assert_eq!(hdr.data_offset, 28);
    assert_eq!(hdr.attributes, 1);
    assert_eq!(data[0], 0xAA);
}
