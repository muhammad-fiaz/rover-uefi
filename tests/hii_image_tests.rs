//! Integration tests for HII Image and HII Image Ex protocols.

use std::sync::atomic::{AtomicU16, AtomicU32, Ordering};

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::hii::{
    EFI_HII_IMAGE_PROTOCOL_GUID, EfiHiiHandle, EfiImageOutput,
};
use rover_uefi::protocols::firmware::hii_image::*;

static LAST_ADDED_ID: AtomicU16 = AtomicU16::new(0);
static LAST_DRAW_FLAGS: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_new_image(
    _this: *const EfiHiiImageProtocol,
    _package_list: EfiHiiHandle,
    image_id: *mut EfiImageId,
    _image: *const EfiImageInput,
) -> EfiStatus {
    if !image_id.is_null() {
        unsafe {
            *image_id = 0x0042;
        }
    }
    LAST_ADDED_ID.store(0x0042, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_image(
    _this: *const EfiHiiImageProtocol,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageInput,
) -> EfiStatus {
    if image_id == 0x0042 && !image.is_null() {
        unsafe {
            (*image).width = 64;
            (*image).height = 48;
            (*image).flags = EFI_IMAGE_TRANSPARENT;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_set_image(
    _this: *const EfiHiiImageProtocol,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    _image: *const EfiImageInput,
) -> EfiStatus {
    if image_id == 0x0042 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_draw_image(
    _this: *const EfiHiiImageProtocol,
    flags: u32,
    _image: *const EfiImageInput,
    _blt: *mut *mut EfiImageOutput,
    _blt_x: UIntN,
    _blt_y: UIntN,
) -> EfiStatus {
    LAST_DRAW_FLAGS.store(flags, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_draw_image_id(
    _this: *const EfiHiiImageProtocol,
    flags: u32,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    _blt: *mut *mut EfiImageOutput,
    _blt_x: UIntN,
    _blt_y: UIntN,
) -> EfiStatus {
    if image_id == 0x0042 {
        LAST_DRAW_FLAGS.store(flags, Ordering::SeqCst);
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

static EX_ADDED_ID: AtomicU16 = AtomicU16::new(0);
static EX_DRAW_FLAGS: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_new_image_ex(
    _this: *const EfiHiiImageExProtocol,
    _package_list: EfiHiiHandle,
    image_id: *mut EfiImageId,
    _image: *const EfiImageInput,
) -> EfiStatus {
    if !image_id.is_null() {
        unsafe {
            *image_id = 0x0084;
        }
    }
    EX_ADDED_ID.store(0x0084, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_image_ex(
    _this: *const EfiHiiImageExProtocol,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageInput,
) -> EfiStatus {
    if image_id == 0x0084 && !image.is_null() {
        unsafe {
            (*image).width = 128;
            (*image).height = 96;
            (*image).flags = 0;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_set_image_ex(
    _this: *const EfiHiiImageExProtocol,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    _image: *const EfiImageInput,
) -> EfiStatus {
    if image_id == 0x0084 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_draw_image_ex(
    _this: *const EfiHiiImageExProtocol,
    flags: u32,
    _image: *const EfiImageInput,
    _blt: *mut *mut EfiImageOutput,
    _blt_x: UIntN,
    _blt_y: UIntN,
) -> EfiStatus {
    EX_DRAW_FLAGS.store(flags, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_draw_image_id_ex(
    _this: *const EfiHiiImageExProtocol,
    flags: u32,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    _blt: *mut *mut EfiImageOutput,
    _blt_x: UIntN,
    _blt_y: UIntN,
) -> EfiStatus {
    if image_id == 0x0084 {
        EX_DRAW_FLAGS.store(flags, Ordering::SeqCst);
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_get_image_info(
    _this: *const EfiHiiImageExProtocol,
    _package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageOutput,
) -> EfiStatus {
    if image_id == 0x0084 && !image.is_null() {
        unsafe {
            (*image).width = 256;
            (*image).height = 192;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_hii_image_constants_and_guids() {
    assert_eq!(
        EFI_HII_IMAGE_PROTOCOL_GUID,
        EfiGuid::new(
            0x31a6_406a,
            0x6b56,
            0x4e25,
            [0x89, 0x21, 0x99, 0x2d, 0xc3, 0xfc, 0xfe, 0x16]
        )
    );
    assert_eq!(
        EFI_HII_IMAGE_EX_PROTOCOL_GUID,
        EfiGuid::new(
            0x1a12_41e6,
            0x8f19,
            0x41a9,
            [0xbc, 0x0e, 0xe8, 0xef, 0x39, 0xe0, 0x65, 0x46]
        )
    );
    assert_eq!(EFI_IMAGE_TRANSPARENT, 0x0000_0001);
    assert_eq!(EFI_HII_DRAW_FLAG_CLIP, 0x0000_0001);
    assert_eq!(EFI_HII_DRAW_FLAG_FORCE_TRANS, 0x0000_0010);
    assert_eq!(EFI_HII_DRAW_FLAG_FORCE_OPAQUE, 0x0000_0020);
    assert_eq!(EFI_HII_DRAW_FLAG_TRANSPARENT, 0x0000_0030);
    assert_eq!(EFI_HII_DIRECT_TO_SCREEN, 0x0000_0080);
    assert_eq!(EFI_HII_DRAW_FLAG_DEFAULT, 0x0000_0000);
}

#[test]
fn test_hii_image_wrapper() {
    let mock_proto = EfiHiiImageProtocol {
        new_image: mock_new_image,
        get_image: mock_get_image,
        set_image: mock_set_image,
        draw_image: mock_draw_image,
        draw_image_id: mock_draw_image_id,
    };
    let client = unsafe { HiiImage::from_raw(&raw const mock_proto) };
    let dummy_handle = EfiHiiHandle(core::ptr::null_mut());

    let input = EfiImageInput {
        flags: EFI_IMAGE_TRANSPARENT,
        width: 64,
        height: 48,
        bitmap: core::ptr::null_mut(),
    };

    let id = client.new_image(dummy_handle, &input);
    assert_eq!(id, Ok(0x0042));
    assert_eq!(LAST_ADDED_ID.load(Ordering::SeqCst), 0x0042);

    let retrieved = client.get_image(dummy_handle, 0x0042);
    assert!(retrieved.is_ok());
    if let Ok(img) = retrieved {
        assert_eq!(img.width, 64);
        assert_eq!(img.height, 48);
        assert_eq!(img.flags, EFI_IMAGE_TRANSPARENT);
    }
    assert_eq!(
        client.get_image(dummy_handle, 0x9999).err(),
        Some(EfiStatus::NOT_FOUND)
    );

    assert!(client.set_image(dummy_handle, 0x0042, &input).is_ok());
    assert_eq!(
        client.set_image(dummy_handle, 0x9999, &input).err(),
        Some(EfiStatus::NOT_FOUND)
    );

    let mut blt_out: *mut EfiImageOutput = core::ptr::null_mut();
    assert!(
        client
            .draw_image(EFI_HII_DRAW_FLAG_CLIP, &input, &mut blt_out, 10, 20,)
            .is_ok()
    );
    assert_eq!(
        LAST_DRAW_FLAGS.load(Ordering::SeqCst),
        EFI_HII_DRAW_FLAG_CLIP
    );

    assert!(
        client
            .draw_image_id(
                EFI_HII_DIRECT_TO_SCREEN,
                dummy_handle,
                0x0042,
                &mut blt_out,
                0,
                0,
            )
            .is_ok()
    );
    assert_eq!(
        LAST_DRAW_FLAGS.load(Ordering::SeqCst),
        EFI_HII_DIRECT_TO_SCREEN
    );
}

#[test]
fn test_hii_image_ex_wrapper() {
    let mock_proto_ex = EfiHiiImageExProtocol {
        new_image_ex: mock_new_image_ex,
        get_image_ex: mock_get_image_ex,
        set_image_ex: mock_set_image_ex,
        draw_image_ex: mock_draw_image_ex,
        draw_image_id_ex: mock_draw_image_id_ex,
        get_image_info: mock_get_image_info,
    };
    let client_ex = unsafe { HiiImageEx::from_raw(&raw const mock_proto_ex) };
    let dummy_handle = EfiHiiHandle(core::ptr::null_mut());

    let input = EfiImageInput {
        flags: 0,
        width: 128,
        height: 96,
        bitmap: core::ptr::null_mut(),
    };

    let id = client_ex.new_image_ex(dummy_handle, &input);
    assert_eq!(id, Ok(0x0084));
    assert_eq!(EX_ADDED_ID.load(Ordering::SeqCst), 0x0084);

    let retrieved = client_ex.get_image_ex(dummy_handle, 0x0084);
    assert!(retrieved.is_ok());
    if let Ok(img) = retrieved {
        assert_eq!(img.width, 128);
        assert_eq!(img.height, 96);
    }
    assert_eq!(
        client_ex.get_image_ex(dummy_handle, 0x1111).err(),
        Some(EfiStatus::NOT_FOUND)
    );

    assert!(client_ex.set_image_ex(dummy_handle, 0x0084, &input).is_ok());

    let mut blt_out: *mut EfiImageOutput = core::ptr::null_mut();
    assert!(
        client_ex
            .draw_image_ex(EFI_HII_DRAW_FLAG_TRANSPARENT, &input, &mut blt_out, 5, 15,)
            .is_ok()
    );
    assert_eq!(
        EX_DRAW_FLAGS.load(Ordering::SeqCst),
        EFI_HII_DRAW_FLAG_TRANSPARENT
    );

    assert!(
        client_ex
            .draw_image_id_ex(
                EFI_HII_DRAW_FLAG_FORCE_OPAQUE,
                dummy_handle,
                0x0084,
                &mut blt_out,
                0,
                0,
            )
            .is_ok()
    );
    assert_eq!(
        EX_DRAW_FLAGS.load(Ordering::SeqCst),
        EFI_HII_DRAW_FLAG_FORCE_OPAQUE
    );

    let info = client_ex.get_image_info(dummy_handle, 0x0084);
    assert!(info.is_ok());
    if let Ok(out) = info {
        assert_eq!(out.width, 256);
        assert_eq!(out.height, 192);
    }
}
