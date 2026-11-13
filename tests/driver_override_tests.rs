//! Driver Override protocols integration tests.

use core::ffi::c_void;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::device_path::EfiDevicePath;
use rover_uefi::protocols::driver::override_protocol::{
    BusSpecificDriverOverride, DriverFamilyOverride,
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL_GUID,
    EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID, EfiBusSpecificDriverOverrideProtocol,
    EfiDriverFamilyOverrideProtocol, EfiPlatformDriverOverrideProtocol, PlatformDriverOverride,
};

#[test]
fn test_driver_override_guids() {
    assert_eq!(
        EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
        EfiGuid::new(
            0x6B30_C738,
            0xA391,
            0x401E,
            [0x92, 0x60, 0x8A, 0x76, 0x79, 0x90, 0x67, 0xEB]
        )
    );
    assert_eq!(
        EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID,
        EfiGuid::new(
            0x3BC1_B285,
            0x8A15,
            0x4A82,
            [0xAA, 0xBF, 0x4D, 0x7D, 0x13, 0xFB, 0x32, 0x65]
        )
    );
    assert_eq!(
        EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL_GUID,
        EfiGuid::new(
            0xB1EE_129E,
            0xDA36,
            0x4181,
            [0x91, 0xF8, 0x04, 0xA4, 0x92, 0x37, 0x66, 0xA7]
        )
    );
}

unsafe extern "efiapi" fn mock_platform_get_driver(
    _this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus {
    if driver_image_handle.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if controller_handle.to_ptr() == 0x1000 as *mut c_void {
        unsafe {
            *driver_image_handle = EfiHandle::from_ptr(0x5000 as *mut c_void);
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

static mut DUMMY_PATH: [u8; 4] = [0x7F, 0xFF, 0x04, 0x00];

unsafe extern "efiapi" fn mock_platform_get_driver_path(
    _this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_path: *mut *mut EfiDevicePath,
) -> EfiStatus {
    if driver_image_path.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if controller_handle.to_ptr() == 0x1000 as *mut c_void {
        unsafe {
            *driver_image_path = core::ptr::addr_of_mut!(DUMMY_PATH).cast();
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_platform_driver_loaded(
    _this: *const EfiPlatformDriverOverrideProtocol,
    _controller_handle: EfiHandle,
    _driver_image_path: *const EfiDevicePath,
    _driver_image_handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_platform_driver_override_wrapper() {
    let raw_proto = EfiPlatformDriverOverrideProtocol {
        get_driver: mock_platform_get_driver,
        get_driver_path: mock_platform_get_driver_path,
        driver_loaded: mock_platform_driver_loaded,
    };

    let client = PlatformDriverOverride::new(&raw_proto);
    let valid_controller = EfiHandle::from_ptr(0x1000 as *mut c_void);
    let invalid_controller = EfiHandle::from_ptr(0x2000 as *mut c_void);

    // Get driver
    assert_eq!(
        client.get_driver(valid_controller),
        Ok(Some(EfiHandle::from_ptr(0x5000 as *mut c_void)))
    );
    assert_eq!(client.get_driver(invalid_controller), Ok(None));

    // Get driver path
    let path = client.get_driver_path(valid_controller).unwrap_or(None);
    assert!(!path.unwrap_or(core::ptr::null_mut()).is_null());
    assert_eq!(client.get_driver_path(invalid_controller), Ok(None));

    // Driver loaded notification
    let dummy_path = core::ptr::addr_of!(DUMMY_PATH).cast::<EfiDevicePath>();
    let driver_handle = EfiHandle::from_ptr(0x5000 as *mut c_void);
    assert_eq!(
        unsafe { client.driver_loaded(valid_controller, dummy_path, driver_handle) },
        Ok(())
    );

    // from_raw
    assert!(unsafe { PlatformDriverOverride::from_raw(core::ptr::null()) }.is_none());
}

unsafe extern "efiapi" fn mock_bus_get_driver(
    _this: *const EfiBusSpecificDriverOverrideProtocol,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus {
    if driver_image_handle.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *driver_image_handle = EfiHandle::from_ptr(0x7000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_bus_specific_driver_override_wrapper() {
    let raw_proto = EfiBusSpecificDriverOverrideProtocol {
        get_driver: mock_bus_get_driver,
    };

    let client = BusSpecificDriverOverride::new(&raw_proto);
    assert_eq!(
        client.get_driver(),
        Ok(Some(EfiHandle::from_ptr(0x7000 as *mut c_void)))
    );

    assert!(unsafe { BusSpecificDriverOverride::from_raw(core::ptr::null()) }.is_none());
}

unsafe extern "efiapi" fn mock_family_get_version(
    _this: *const EfiDriverFamilyOverrideProtocol,
) -> u32 {
    0x0002_0010
}

#[test]
fn test_driver_family_override_wrapper() {
    let raw_proto = EfiDriverFamilyOverrideProtocol {
        get_version: mock_family_get_version,
    };

    let client = DriverFamilyOverride::new(&raw_proto);
    assert_eq!(client.version(), 0x0002_0010);

    assert!(unsafe { DriverFamilyOverride::from_raw(core::ptr::null()) }.is_none());
}
