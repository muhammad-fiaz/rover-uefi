//! Tests for `LoadFile` and `LoadFile2` protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::{Bool, UIntN};
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::device_path::{DEVICE_PATH_END, END_ENTIRE, EfiDevicePath};
use rover_uefi::protocols::storage::load_file::{
    EfiLoadFile2Protocol, EfiLoadFileProtocol, LOAD_FILE_PROTOCOL_GUID, LOAD_FILE2_PROTOCOL_GUID,
    LoadFile, LoadFile2,
};

static DUMMY_PAYLOAD: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];

unsafe extern "efiapi" fn mock_load_file(
    _this: *mut EfiLoadFileProtocol,
    _file_path: *const EfiDevicePath,
    _boot_policy: Bool,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    if buffer_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if buffer.is_null() {
            *buffer_size = DUMMY_PAYLOAD.len();
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        if *buffer_size < DUMMY_PAYLOAD.len() {
            *buffer_size = DUMMY_PAYLOAD.len();
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        core::ptr::copy_nonoverlapping(
            DUMMY_PAYLOAD.as_ptr(),
            buffer.cast::<u8>(),
            DUMMY_PAYLOAD.len(),
        );
        *buffer_size = DUMMY_PAYLOAD.len();
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_load_file2(
    _this: *mut EfiLoadFile2Protocol,
    _file_path: *const EfiDevicePath,
    _boot_policy: Bool,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    if buffer_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if buffer.is_null() {
            *buffer_size = DUMMY_PAYLOAD.len();
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        if *buffer_size < DUMMY_PAYLOAD.len() {
            *buffer_size = DUMMY_PAYLOAD.len();
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        core::ptr::copy_nonoverlapping(
            DUMMY_PAYLOAD.as_ptr(),
            buffer.cast::<u8>(),
            DUMMY_PAYLOAD.len(),
        );
        *buffer_size = DUMMY_PAYLOAD.len();
        EfiStatus::SUCCESS
    }
}

#[test]
fn load_file_guids_and_protocol_sizes() {
    assert_eq!(
        LOAD_FILE_PROTOCOL_GUID.to_bytes(),
        [
            0x91, 0x30, 0xEC, 0x56, 0x4C, 0x95, 0xD2, 0x11, 0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        LOAD_FILE2_PROTOCOL_GUID.to_bytes(),
        [
            0xC1, 0xC0, 0x06, 0x40, 0xB3, 0xFC, 0x3E, 0x40, 0x99, 0x6D, 0x4A, 0x6C, 0x87, 0x24,
            0xE0, 0x6D
        ]
    );

    assert_eq!(
        core::mem::size_of::<EfiLoadFileProtocol>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiLoadFile2Protocol>(),
        core::mem::size_of::<usize>()
    );
}

#[test]
fn load_file_operations() {
    let dummy_dp = EfiDevicePath::new(DEVICE_PATH_END, END_ENTIRE, 4);
    let proto = EfiLoadFileProtocol {
        load_file: mock_load_file,
    };
    let wrapper = LoadFile::new(&proto);
    assert!(core::ptr::eq(wrapper.protocol(), &raw const proto));

    // Query file size
    let size = unsafe { wrapper.query_file_size(&raw const dummy_dp, true) };
    assert_eq!(size, Ok(DUMMY_PAYLOAD.len()));

    // Load into buffer
    let mut dest = [0_u8; 8];
    let loaded = unsafe { wrapper.load_into_buffer(&raw const dummy_dp, true, &mut dest) };
    assert_eq!(loaded, Ok(8));
    assert_eq!(dest, DUMMY_PAYLOAD);
}

#[test]
fn load_file2_operations() {
    let dummy_dp = EfiDevicePath::new(DEVICE_PATH_END, END_ENTIRE, 4);
    let proto = EfiLoadFile2Protocol {
        load_file: mock_load_file2,
    };
    let wrapper = LoadFile2::new(&proto);
    assert!(core::ptr::eq(wrapper.protocol(), &raw const proto));

    // Query file size
    let size = unsafe { wrapper.query_file_size(&raw const dummy_dp) };
    assert_eq!(size, Ok(DUMMY_PAYLOAD.len()));

    // Load into buffer
    let mut dest = [0_u8; 8];
    let loaded = unsafe { wrapper.load_into_buffer(&raw const dummy_dp, &mut dest) };
    assert_eq!(loaded, Ok(8));
    assert_eq!(dest, DUMMY_PAYLOAD);
}
