//! Integration tests for Deferred Image Load Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::device_path::types::EfiDevicePath;
use rover_uefi::protocols::security::{
    DeferredImageDetails, DeferredImageLoad, EFI_DEFERRED_IMAGE_LOAD_PROTOCOL_GUID,
    EfiDeferredImageLoadProtocol,
};

static DUMMY_IMAGE: [u8; 64] = [0x5A; 64];

unsafe extern "efiapi" fn mock_deferred_image_info(
    _this: *const EfiDeferredImageLoadProtocol,
    image_index: UIntN,
    image_device_path: *mut *const EfiDevicePath,
    image: *mut *const c_void,
    image_size: *mut UIntN,
    boot_option: *mut bool,
) -> EfiStatus {
    if image_device_path.is_null()
        || image.is_null()
        || image_size.is_null()
        || boot_option.is_null()
    {
        return EfiStatus::INVALID_PARAMETER;
    }
    if image_index == 0 {
        unsafe {
            *image_device_path = core::ptr::null();
            *image = core::ptr::addr_of!(DUMMY_IMAGE).cast::<c_void>();
            *image_size = 64;
            *boot_option = true;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_deferred_image_guid_and_details() {
    assert_eq!(
        EFI_DEFERRED_IMAGE_LOAD_PROTOCOL_GUID,
        EfiGuid::new(
            0x1585_3D7C,
            0x3DDF,
            0x43E0,
            [0xA1, 0xCB, 0xEB, 0xF8, 0x5B, 0x8F, 0x87, 0x2C],
        )
    );

    let details = DeferredImageDetails {
        device_path: core::ptr::null(),
        image: core::ptr::null(),
        image_size: 1024,
        boot_option: false,
    };
    assert_eq!(details.image_size, 1024);
    assert!(!details.boot_option);
}

#[test]
fn test_deferred_image_client_wrapper() {
    let mock_proto = EfiDeferredImageLoadProtocol {
        get_image_info: mock_deferred_image_info,
    };

    let client = DeferredImageLoad::new(&raw const mock_proto);
    assert_eq!(client.raw(), &raw const mock_proto);

    let info0 = client.get_image_info(0).unwrap_or(DeferredImageDetails {
        device_path: core::ptr::null(),
        image: core::ptr::null(),
        image_size: 0,
        boot_option: false,
    });

    assert_eq!(info0.image_size, 64);
    assert!(info0.boot_option);
    assert!(!info0.image.is_null());

    let byte0 = unsafe { *info0.image.cast::<u8>() };
    assert_eq!(byte0, 0x5A);

    assert_eq!(client.get_image_info(1), Err(EfiStatus::NOT_FOUND));
}
