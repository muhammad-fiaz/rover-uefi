//! Driver protocol tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::driver::binding::EfiDriverBindingProtocol;
use rover_uefi::protocols::driver::component_name::{
    EfiComponentName2Protocol, EfiComponentNameProtocol,
};
use rover_uefi::protocols::driver::guid::{
    COMPONENT_NAME_GUID, COMPONENT_NAME2_GUID, DRIVER_BINDING_GUID, LOADED_IMAGE_GUID,
};
use rover_uefi::protocols::driver::loaded_image::EfiLoadedImageProtocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        DRIVER_BINDING_GUID.to_bytes(),
        [
            0xAB, 0x31, 0xA0, 0x18, 0x43, 0xB4, 0x1A, 0x4D, 0xA5, 0xC0, 0x0C, 0x09, 0x26, 0x1E,
            0x9F, 0x71
        ]
    );
    assert_eq!(
        LOADED_IMAGE_GUID.to_bytes(),
        [
            0xA1, 0x31, 0x1B, 0x5B, 0x62, 0x95, 0xD2, 0x11, 0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(DRIVER_BINDING_GUID.to_bytes()),
        DRIVER_BINDING_GUID
    );
    assert_ne!(COMPONENT_NAME_GUID, COMPONENT_NAME2_GUID);
}

#[test]
fn driver_binding_layout() {
    let expected_size = if size_of::<usize>() == 8 { 48 } else { 24 };
    assert_eq!(size_of::<EfiDriverBindingProtocol>(), expected_size);
    assert_eq!(offset_of!(EfiDriverBindingProtocol, supported), 0);
    assert_eq!(
        offset_of!(EfiDriverBindingProtocol, version),
        3 * size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiDriverBindingProtocol, image_handle),
        3 * size_of::<usize>() + size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiDriverBindingProtocol, driver_binding_handle),
        3 * size_of::<usize>() + 2 * size_of::<usize>()
    );
}

#[test]
fn component_name_layout() {
    assert_eq!(
        size_of::<EfiComponentNameProtocol>(),
        3 * size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiComponentName2Protocol>(),
        3 * size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiComponentNameProtocol, supported_languages),
        2 * size_of::<usize>()
    );
}

#[test]
fn loaded_image_layout() {
    assert_eq!(offset_of!(EfiLoadedImageProtocol, revision), 0);
    assert_eq!(
        offset_of!(EfiLoadedImageProtocol, parent_handle),
        size_of::<usize>()
    );
    assert!(
        offset_of!(EfiLoadedImageProtocol, unload) > offset_of!(EfiLoadedImageProtocol, image_size)
    );
    assert_eq!(
        size_of::<EfiLoadedImageProtocol>(),
        if size_of::<usize>() == 8 { 96 } else { 56 }
    );
}
