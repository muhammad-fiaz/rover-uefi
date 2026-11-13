//! Integration tests for extended manageability protocols:
//! IPMI OEM Blob Transfer Protocol and PLDM SMBIOS Transfer Protocol.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use core::ffi::c_char;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::manageability::ipmi_blob::{
    BLOB_MAX_DATA_PER_PACKET, BlobStatResult, IPMI_BLOB_TRANSFER_PROTOCOL_GUID,
    IPMI_OEM_BLOB_TRANSFER_CMD, IpmiBlobTransfer, IpmiBlobTransferProtocol, blob_flags,
};
use rover_uefi::protocols::manageability::pldm_smbios::{
    PLDM_SMBIOS_TRANSFER_PROTOCOL_GUID, PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION,
    PLDM_TERMINUS_ID_UNASSIGNED, PldmSmbiosStructureTableMetadata, PldmSmbiosTransfer,
    PldmSmbiosTransferProtocol, PldmSmbiosTransferProtocolFunctions,
    PldmSmbiosTransferProtocolV1_0, command_code as pldm_smbios_cmd,
    transfer_flag as pldm_smbios_transfer_flag,
};

#[test]
fn test_ipmi_blob_guid_and_constants() {
    assert_eq!(
        IPMI_BLOB_TRANSFER_PROTOCOL_GUID,
        EfiGuid::new(
            0x0583_7c75,
            0x1d65,
            0x468b,
            [0xb1, 0xc2, 0x81, 0xaf, 0x9a, 0x31, 0x5b, 0x2c]
        )
    );
    assert_eq!(IPMI_OEM_BLOB_TRANSFER_CMD, 0x80);
    assert_eq!(BLOB_MAX_DATA_PER_PACKET, 64);
    assert_eq!(blob_flags::OPEN_R, 1 << 0);
    assert_eq!(blob_flags::OPEN_W, 1 << 1);
    assert_eq!(blob_flags::COMMITTING, 1 << 2);
    assert_eq!(blob_flags::COMMITTED, 1 << 3);
    assert_eq!(blob_flags::COMMIT_ERROR, 1 << 4);
}

// Mock functions for IPMI Blob Transfer
unsafe extern "efiapi" fn mock_blob_get_count(count: *mut u32) -> EfiStatus {
    if count.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *count = 4 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_enumerate(blob_index: u32, blob_id: *mut c_char) -> EfiStatus {
    if blob_id.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let name = match blob_index {
        0 => b"/smbios\0".as_slice(),
        1 => b"/flash/image\0".as_slice(),
        _ => return EfiStatus::NOT_FOUND,
    };
    unsafe {
        core::ptr::copy_nonoverlapping(name.as_ptr().cast(), blob_id, name.len());
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_open(
    _blob_id: *const c_char,
    _flags: u16,
    session_id: *mut u16,
) -> EfiStatus {
    if session_id.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *session_id = 0x42 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_read(
    session_id: u16,
    offset: u32,
    requested_size: u32,
    data: *mut u8,
) -> EfiStatus {
    if session_id != 0x42 || data.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    for i in 0..requested_size {
        unsafe {
            *data.add(i as usize) = ((offset + i) & 0xFF) as u8;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_write(
    session_id: u16,
    _offset: u32,
    _data: *const u8,
    _write_length: u32,
) -> EfiStatus {
    if session_id != 0x42 {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_commit(
    session_id: u16,
    _data_len: u8,
    _data: *const u8,
) -> EfiStatus {
    if session_id != 0x42 {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_close(session_id: u16) -> EfiStatus {
    if session_id == 0x42 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_blob_delete(_blob_id: *const c_char) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_stat(
    _blob_id: *const c_char,
    blob_state: *mut u16,
    size: *mut u32,
    metadata_len: *mut u8,
    metadata: *mut u8,
) -> EfiStatus {
    if !blob_state.is_null() {
        unsafe { *blob_state = blob_flags::OPEN_R | blob_flags::COMMITTED };
    }
    if !size.is_null() {
        unsafe { *size = 1024 };
    }
    if !metadata_len.is_null() && !metadata.is_null() {
        unsafe {
            *metadata = 0xAA;
            *metadata_len = 1;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blob_session_stat(
    session_id: u16,
    blob_state: *mut u16,
    size: *mut u32,
    metadata_len: *mut u8,
    metadata: *mut u8,
) -> EfiStatus {
    if session_id != 0x42 {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { mock_blob_stat(core::ptr::null(), blob_state, size, metadata_len, metadata) }
}

unsafe extern "efiapi" fn mock_blob_write_meta(
    session_id: u16,
    _offset: u32,
    _data: *const u8,
    _len: u32,
) -> EfiStatus {
    if session_id == 0x42 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

#[test]
fn test_ipmi_blob_transfer_wrapper() {
    let proto = IpmiBlobTransferProtocol {
        blob_get_count: mock_blob_get_count,
        blob_enumerate: mock_blob_enumerate,
        blob_open: mock_blob_open,
        blob_read: mock_blob_read,
        blob_write: mock_blob_write,
        blob_commit: mock_blob_commit,
        blob_close: mock_blob_close,
        blob_delete: mock_blob_delete,
        blob_stat: mock_blob_stat,
        blob_session_stat: mock_blob_session_stat,
        blob_write_meta: mock_blob_write_meta,
    };

    let client = IpmiBlobTransfer::new(&proto);
    assert_eq!(client.get_count().unwrap(), 4);

    let mut id_buf = [0u8; 32];
    let name0 = client.enumerate(0, &mut id_buf).unwrap();
    assert_eq!(name0, "/smbios");

    let session = client.open("/smbios", blob_flags::OPEN_R).unwrap();
    assert_eq!(session, 0x42);

    let mut read_buf = [0u8; 4];
    let n = client.read(session, 10, &mut read_buf).unwrap();
    assert_eq!(n, 4);
    assert_eq!(read_buf, [10, 11, 12, 13]);

    client.write(session, 0, &[1, 2, 3]).unwrap();
    client.commit(session, Some(&[0xEE])).unwrap();

    let stat: BlobStatResult = client.stat("/smbios", None).unwrap();
    assert_eq!(stat.size, 1024);
    assert_eq!(stat.state, blob_flags::OPEN_R | blob_flags::COMMITTED);

    client.close(session).unwrap();
    client.delete("/flash/image").unwrap();
}

#[test]
fn test_pldm_smbios_guid_and_metadata() {
    assert_eq!(
        PLDM_SMBIOS_TRANSFER_PROTOCOL_GUID,
        EfiGuid::new(
            0xFA43_1C3C,
            0x816B,
            0x4B32,
            [0xA3, 0xE0, 0xAD, 0x9B, 0x7F, 0x64, 0x27, 0x2E]
        )
    );
    assert_eq!(PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION, 0x0100);
    assert_eq!(PLDM_TERMINUS_ID_UNASSIGNED, 0xFF);
    assert_eq!(pldm_smbios_cmd::GET_SMBIOS_STRUCTURE_TABLE_METADATA, 1);
    assert_eq!(pldm_smbios_transfer_flag::START_AND_END, 5);

    let meta = PldmSmbiosStructureTableMetadata {
        smbios_major_version: 3,
        smbios_minor_version: 6,
        maximum_structure_size: 128,
        smbios_structure_table_length: 2048,
        number_of_smbios_structures: 42,
        smbios_structure_table_integrity_checksum: 0x1234_5678,
    };
    let major = meta.smbios_major_version;
    let count = meta.number_of_smbios_structures;
    assert_eq!(major, 3);
    assert_eq!(count, 42);
}

// Mock functions for PLDM SMBIOS Transfer
static mut MOCK_TERMINUS_CONFIG: (u8, u8) = (0, 0);
static mut MOCK_TABLE_BUF: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

unsafe extern "efiapi" fn mock_pldm_set_terminus(source_id: u8, dest_id: u8) -> EfiStatus {
    unsafe {
        MOCK_TERMINUS_CONFIG = (source_id, dest_id);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_get_metadata(
    _this: *mut PldmSmbiosTransferProtocol,
    buffer: *mut PldmSmbiosStructureTableMetadata,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *buffer = PldmSmbiosStructureTableMetadata {
            smbios_major_version: 3,
            smbios_minor_version: 7,
            maximum_structure_size: 256,
            smbios_structure_table_length: 4096,
            number_of_smbios_structures: 15,
            smbios_structure_table_integrity_checksum: 0xDEAD_BEEF,
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_set_metadata(
    _this: *mut PldmSmbiosTransferProtocol,
    _buffer: *const PldmSmbiosStructureTableMetadata,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_get_table(
    _this: *mut PldmSmbiosTransferProtocol,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus {
    unsafe {
        *buffer = (&raw mut MOCK_TABLE_BUF).cast();
        *buffer_size = 8;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_set_table(_this: *mut PldmSmbiosTransferProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_get_by_type(
    _this: *mut PldmSmbiosTransferProtocol,
    _type_id: u8,
    _instance: u16,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus {
    unsafe {
        *buffer = (&raw mut MOCK_TABLE_BUF).cast();
        *buffer_size = 8;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pldm_get_by_handle(
    _this: *mut PldmSmbiosTransferProtocol,
    _handle: u16,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus {
    unsafe {
        *buffer = (&raw mut MOCK_TABLE_BUF).cast();
        *buffer_size = 8;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_pldm_smbios_wrapper() {
    let mut v1 = PldmSmbiosTransferProtocolV1_0 {
        set_pldm_smbios_transfer_terminus_id: mock_pldm_set_terminus,
        get_smbios_structure_table_metadata: mock_pldm_get_metadata,
        set_smbios_structure_table_metadata: mock_pldm_set_metadata,
        get_smbios_structure_table: mock_pldm_get_table,
        set_smbios_structure_table: mock_pldm_set_table,
        get_smbios_structure_by_type: mock_pldm_get_by_type,
        get_smbios_structure_by_handle: mock_pldm_get_by_handle,
    };

    let mut proto = PldmSmbiosTransferProtocol {
        protocol_version: PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION,
        functions: PldmSmbiosTransferProtocolFunctions {
            version_1_0: &raw mut v1,
        },
    };

    let mut client = PldmSmbiosTransfer::new(&mut proto);
    client.set_terminus_ids(10, 20).unwrap();
    assert_eq!(unsafe { MOCK_TERMINUS_CONFIG }, (10, 20));

    let meta = client.get_metadata().unwrap();
    let major = meta.smbios_major_version;
    let minor = meta.smbios_minor_version;
    let num_structs = meta.number_of_smbios_structures;
    assert_eq!(major, 3);
    assert_eq!(minor, 7);
    assert_eq!(num_structs, 15);

    client.set_metadata(&meta).unwrap();
    client.set_table().unwrap();

    let (slice, _ptr) = client.get_table().unwrap();
    assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8]);
}
