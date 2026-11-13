//! Integration tests for Memory Profile Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus, MemoryType};
use rover_uefi::protocols::firmware::{
    EfiMemoryProfileProtocol, MEMORY_PROFILE_CONTEXT_REVISION, MEMORY_PROFILE_CONTEXT_SIGNATURE,
    MEMORY_PROFILE_DRIVER_INFO_REVISION, MEMORY_PROFILE_DRIVER_INFO_SIGNATURE,
    MEMORY_PROFILE_PROTOCOL_GUID, MemoryProfile, MemoryProfileAction, MemoryProfileCommonHeader,
    MemoryProfileContext, SMM_MEMORY_PROFILE_PROTOCOL_GUID,
};

#[test]
fn test_memory_profile_guids_and_constants() {
    assert_eq!(
        MEMORY_PROFILE_PROTOCOL_GUID,
        EfiGuid::new(
            0x821C_9A09,
            0x541A,
            0x40F6,
            [0x9F, 0x43, 0x0A, 0xD1, 0x93, 0xA1, 0x2C, 0xFE],
        )
    );
    assert_eq!(
        SMM_MEMORY_PROFILE_PROTOCOL_GUID,
        EfiGuid::new(
            0xE22B_BCCA,
            0x516A,
            0x46A8,
            [0x80, 0xE2, 0x67, 0x45, 0xE8, 0x36, 0x93, 0xBD],
        )
    );
    assert_eq!(MEMORY_PROFILE_CONTEXT_SIGNATURE, 0x5443_504D);
    assert_eq!(MEMORY_PROFILE_CONTEXT_REVISION, 0x0002);
    assert_eq!(MEMORY_PROFILE_DRIVER_INFO_SIGNATURE, 0x4944_504D);
    assert_eq!(MEMORY_PROFILE_DRIVER_INFO_REVISION, 0x0003);

    assert_eq!(MemoryProfileAction::AllocatePages as u32, 1);
    assert_eq!(MemoryProfileAction::FreePages as u32, 2);
    assert_eq!(MemoryProfileAction::AllocatePool as u32, 3);
    assert_eq!(MemoryProfileAction::FreePool as u32, 4);

    let context_len = u16::try_from(size_of::<MemoryProfileContext>()).unwrap_or(0);
    let header = MemoryProfileCommonHeader {
        signature: MEMORY_PROFILE_CONTEXT_SIGNATURE,
        length: context_len,
        revision: MEMORY_PROFILE_CONTEXT_REVISION,
    };
    assert_eq!(header.signature, 0x5443_504D);
    assert_eq!(header.revision, 2);
}

static mut MOCK_RECORDING_STATE: bool = false;
static mut MOCK_RECORD_CALL_COUNT: u32 = 0;
static mut MOCK_REGISTER_CALL_COUNT: u32 = 0;
static mut MOCK_UNREGISTER_CALL_COUNT: u32 = 0;

unsafe extern "efiapi" fn mock_get_data(
    _this: *mut EfiMemoryProfileProtocol,
    profile_size: *mut u64,
    profile_buffer: *mut c_void,
) -> EfiStatus {
    if profile_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let required_size = 16u64;
    unsafe {
        if profile_buffer.is_null() || *profile_size < required_size {
            *profile_size = required_size;
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        *profile_size = required_size;
        let dest = profile_buffer.cast::<u8>();
        for i in 0..16 {
            *dest.add(i) = 0xAA;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_image(
    _this: *mut EfiMemoryProfileProtocol,
    _file_path: *const rover_uefi::protocols::device_path::types::EfiDevicePath,
    _image_base: u64,
    _image_size: u64,
    _file_type: u8,
) -> EfiStatus {
    unsafe {
        MOCK_REGISTER_CALL_COUNT += 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_image(
    _this: *mut EfiMemoryProfileProtocol,
    _file_path: *const rover_uefi::protocols::device_path::types::EfiDevicePath,
    _image_base: u64,
    _image_size: u64,
) -> EfiStatus {
    unsafe {
        MOCK_UNREGISTER_CALL_COUNT += 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_recording_state(
    _this: *mut EfiMemoryProfileProtocol,
    recording_state: *mut bool,
) -> EfiStatus {
    if recording_state.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *recording_state = MOCK_RECORDING_STATE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_recording_state(
    _this: *mut EfiMemoryProfileProtocol,
    recording_state: bool,
) -> EfiStatus {
    unsafe {
        MOCK_RECORDING_STATE = recording_state;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_record(
    _this: *mut EfiMemoryProfileProtocol,
    _caller_address: u64,
    _action: MemoryProfileAction,
    _memory_type: MemoryType,
    _buffer: *mut c_void,
    _size: UIntN,
    _action_string: *const u8,
) -> EfiStatus {
    unsafe {
        MOCK_RECORD_CALL_COUNT += 1;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_memory_profile_client_wrapper() {
    let mut raw_proto = EfiMemoryProfileProtocol {
        get_data: mock_get_data,
        register_image: mock_register_image,
        unregister_image: mock_unregister_image,
        get_recording_state: mock_get_recording_state,
        set_recording_state: mock_set_recording_state,
        record: mock_record,
    };

    let mut client = MemoryProfile::new(&mut raw_proto);

    // Initial recording state is false
    let enabled_res = client.is_recording_enabled();
    assert!(enabled_res.is_ok());
    if let Ok(enabled) = enabled_res {
        assert!(!enabled);
    }

    // Set recording state to true
    let set_res = client.set_recording_enabled(true);
    assert!(set_res.is_ok());

    let enabled_res2 = client.is_recording_enabled();
    assert!(enabled_res2.is_ok());
    if let Ok(enabled) = enabled_res2 {
        assert!(enabled);
    }

    // Register image
    let reg_res = client.register_image(None, 0x1000_0000, 0x4000, 0x07);
    assert!(reg_res.is_ok());

    // Record an allocation
    let dummy_ptr = 0x2000_0000 as *mut c_void;
    let record_res = unsafe {
        client.record(
            0x1000_0100,
            MemoryProfileAction::AllocatePool,
            MemoryType::EXIT_BOOT_SERVICES_DATA,
            dummy_ptr,
            256,
            Some("TestDriverAlloc"),
        )
    };
    assert!(record_res.is_ok());

    // Unregister image
    let unreg_res = client.unregister_image(None, 0x1000_0000, 0x4000);
    assert!(unreg_res.is_ok());

    // Get data - buffer too small
    let mut small_buf = [0u8; 8];
    let data_small = client.get_data(&mut small_buf);
    assert_eq!(data_small, Err(EfiStatus::BUFFER_TOO_SMALL));

    // Get data - proper buffer
    let mut full_buf = [0u8; 32];
    let data_res = client.get_data(&mut full_buf);
    assert!(data_res.is_ok());
    if let Ok(written) = data_res {
        assert_eq!(written, 16);
        assert_eq!(full_buf[0], 0xAA);
        assert_eq!(full_buf[15], 0xAA);
    }
}
