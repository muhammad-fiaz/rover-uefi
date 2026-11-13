//! Integration tests for `VariableLock`, `FaultTolerantWrite`, `PlatformLogo`,
//! `GenericMemoryTest`, and `PlatformSpecificResetFilter`.
#![allow(clippy::expect_used)]

use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus, ResetType};
use rover_uefi::protocols::driver::reset_filter::{
    EfiPlatformSpecificResetFilterProtocol, PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID,
    PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID, PlatformSpecificResetFilter,
};
use rover_uefi::protocols::firmware::ftw::{
    EfiFaultTolerantWriteProtocol, FAULT_TOLERANT_WRITE_PROTOCOL_GUID, FaultTolerantWrite,
    SMM_FAULT_TOLERANT_WRITE_PROTOCOL_GUID,
};
use rover_uefi::protocols::firmware::memory_test::{
    EfiGenericMemoryTestProtocol, GENERIC_MEMORY_TEST_PROTOCOL_GUID, GenericMemoryTest,
    MemoryTestLevel,
};
use rover_uefi::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;
use rover_uefi::protocols::graphics::platform_logo::{
    EfiPlatformLogoProtocol, PLATFORM_LOGO_PROTOCOL_GUID, PlatformLogo, PlatformLogoImageType,
};
use rover_uefi::protocols::security::variable_lock::{
    EfiVariableLockProtocol, VARIABLE_LOCK_PROTOCOL_GUID, VariableLock,
};

// VariableLock mock
unsafe extern "efiapi" fn mock_request_to_lock(
    _this: *const EfiVariableLockProtocol,
    var_name: *const u16,
    vendor_guid: *const EfiGuid,
) -> EfiStatus {
    if var_name.is_null() || vendor_guid.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

#[test]
fn test_variable_lock() {
    assert_eq!(
        VARIABLE_LOCK_PROTOCOL_GUID,
        EfiGuid::new(
            0xCD3D_0A05,
            0x9E24,
            0x437C,
            [0xA8, 0x91, 0x1E, 0xE0, 0x53, 0xDB, 0x76, 0x38]
        )
    );

    let proto = EfiVariableLockProtocol {
        request_to_lock: mock_request_to_lock,
    };
    let lock = unsafe { VariableLock::from_raw(&raw const proto) };

    let test_name = [0x0053, 0x0065, 0x0063, 0x0000]; // "Sec\0"
    let guid = EfiGuid::new(1, 2, 3, [4; 8]);
    assert!(lock.request_to_lock(&test_name, &guid).is_ok());

    // Fails on non-null-terminated string
    let bad_name = [0x0053, 0x0065];
    assert!(lock.request_to_lock(&bad_name, &guid).is_err());
}

// FaultTolerantWrite mock
unsafe extern "efiapi" fn mock_get_max_block_size(
    _this: *const EfiFaultTolerantWriteProtocol,
    block_size: *mut usize,
) -> EfiStatus {
    unsafe { *block_size = 65536 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_allocate(
    _this: *const EfiFaultTolerantWriteProtocol,
    _caller_id: *const EfiGuid,
    _private_data_size: usize,
    _number_of_writes: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_write(
    _this: *const EfiFaultTolerantWriteProtocol,
    _lba: u64,
    _offset: usize,
    _length: usize,
    _private_data: *mut core::ffi::c_void,
    _fvb_handle: EfiHandle,
    _buffer: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_restart(
    _this: *const EfiFaultTolerantWriteProtocol,
    _fvb_handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_abort(_this: *const EfiFaultTolerantWriteProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_last_write(
    _this: *const EfiFaultTolerantWriteProtocol,
    _caller_id: *mut EfiGuid,
    _lba: *mut u64,
    _offset: *mut usize,
    _length: *mut usize,
    _private_data_size: *mut usize,
    _private_data: *mut core::ffi::c_void,
    _complete: *mut bool,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

#[test]
fn test_fault_tolerant_write() {
    assert_ne!(
        FAULT_TOLERANT_WRITE_PROTOCOL_GUID,
        SMM_FAULT_TOLERANT_WRITE_PROTOCOL_GUID
    );

    let proto = EfiFaultTolerantWriteProtocol {
        get_max_block_size: mock_get_max_block_size,
        allocate: mock_allocate,
        write: mock_write,
        restart: mock_restart,
        abort: mock_abort,
        get_last_write: mock_get_last_write,
    };
    let ftw = unsafe { FaultTolerantWrite::from_raw(&raw const proto) };

    let sz = ftw.get_max_block_size().expect("block size");
    assert_eq!(sz, 65536);

    let caller = EfiGuid::new(1, 2, 3, [4; 8]);
    assert!(ftw.allocate(&caller, 32, 2).is_ok());

    let mut priv_data = [0_u8; 32];
    let mut write_buf = [0xAA_u8; 128];
    assert!(
        ftw.write(0, 0, &mut priv_data, EfiHandle::NULL, &mut write_buf)
            .is_ok()
    );
    assert!(ftw.restart(EfiHandle::NULL).is_ok());
    assert!(ftw.abort().is_ok());
}

// Memory test mock
unsafe extern "efiapi" fn mock_init_mem_test(
    _this: *const EfiGenericMemoryTestProtocol,
    _level: MemoryTestLevel,
    require_key: *mut bool,
) -> EfiStatus {
    unsafe { *require_key = false };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_perform_mem_test(
    _this: *const EfiGenericMemoryTestProtocol,
    tested_op: *mut u32,
    percent: *mut u32,
    err: *mut bool,
) -> EfiStatus {
    unsafe {
        *tested_op = 1;
        *percent = 100;
        *err = false;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_finished(_this: *const EfiGenericMemoryTestProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_generic_memory_test() {
    assert_eq!(
        GENERIC_MEMORY_TEST_PROTOCOL_GUID,
        EfiGuid::new(
            0x30E3_D083,
            0x9D3C,
            0x4FC3,
            [0xA4, 0x62, 0x17, 0xC7, 0xE1, 0x28, 0x50, 0xE3]
        )
    );

    let proto = EfiGenericMemoryTestProtocol {
        initialize_memory_test: mock_init_mem_test,
        perform_memory_test: mock_perform_mem_test,
        finished: mock_finished,
    };
    let test = unsafe { GenericMemoryTest::from_raw(&raw const proto) };

    let req_key = test.initialize(MemoryTestLevel::Quick).expect("init");
    assert!(!req_key);

    let progress = test.perform_step().expect("step");
    assert_eq!(progress.percent_complete, 100);
    assert!(!progress.error_detected);

    assert!(test.finished().is_ok());
}

// Platform logo mock
static mut MOCK_PIXEL: EfiGraphicsOutputBltPixel = EfiGraphicsOutputBltPixel {
    blue: 255,
    green: 128,
    red: 64,
    reserved: 0,
};

unsafe extern "efiapi" fn mock_get_logo(
    _this: *const EfiPlatformLogoProtocol,
    instance: u32,
    image_type: *mut PlatformLogoImageType,
    image: *mut *mut EfiGraphicsOutputBltPixel,
    coord_x: *mut usize,
    coord_y: *mut usize,
    width: *mut usize,
    height: *mut usize,
) -> EfiStatus {
    if instance == 0 {
        unsafe {
            *image_type = PlatformLogoImageType::PlatformLogo;
            *image = &raw mut MOCK_PIXEL;
            *coord_x = 100;
            *coord_y = 200;
            *width = 640;
            *height = 480;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_platform_logo() {
    assert_eq!(
        PLATFORM_LOGO_PROTOCOL_GUID,
        EfiGuid::new(
            0x53CD_299F,
            0x2C0F,
            0x4AE5,
            [0xA5, 0xE0, 0xB8, 0x0D, 0xD8, 0x04, 0xDD, 0x57]
        )
    );

    let proto = EfiPlatformLogoProtocol {
        get_image: mock_get_logo,
    };
    let logo = unsafe { PlatformLogo::from_raw(&raw const proto) };

    let img = logo.get_image(0).expect("logo image");
    assert_eq!(img.image_type, PlatformLogoImageType::PlatformLogo);
    assert_eq!(img.coord_x, 100);
    assert_eq!(img.coord_y, 200);
    assert_eq!(img.width, 640);
    assert_eq!(img.height, 480);
    assert!(!img.image_data.is_null());

    assert!(logo.get_image(1).is_err());
}

// Reset filter mock
static mut FILTER_CALLED: bool = false;

unsafe extern "efiapi" fn mock_reset_filter(
    _reset_type: ResetType,
    _reset_status: EfiStatus,
    _data_size: usize,
    _reset_data: *mut core::ffi::c_void,
) {
    unsafe { FILTER_CALLED = true };
}

#[test]
fn test_reset_filter() {
    assert_ne!(
        PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID,
        PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID
    );

    let proto = EfiPlatformSpecificResetFilterProtocol {
        reset_filter: mock_reset_filter,
    };
    let filter = unsafe { PlatformSpecificResetFilter::from_raw(&raw const proto) };

    unsafe { FILTER_CALLED = false };
    filter.invoke_filter(ResetType::COLD, EfiStatus::SUCCESS, None);
    assert!(unsafe { FILTER_CALLED });
}
