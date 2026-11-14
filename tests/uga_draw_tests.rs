//! Integration tests for Universal Graphics Adapter (UGA) Draw Protocol.

use std::sync::atomic::{AtomicU32, Ordering};

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::graphics::uga_draw::*;

static H_RES: AtomicU32 = AtomicU32::new(1024);
static V_RES: AtomicU32 = AtomicU32::new(768);
static COLOR_DEPTH: AtomicU32 = AtomicU32::new(32);
static REFRESH_RATE: AtomicU32 = AtomicU32::new(60);
static LAST_BLT_OP: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_get_mode(
    _this: *const EfiUgaDrawProtocol,
    horizontal_resolution: *mut u32,
    vertical_resolution: *mut u32,
    color_depth: *mut u32,
    refresh_rate: *mut u32,
) -> EfiStatus {
    if !horizontal_resolution.is_null() {
        unsafe { *horizontal_resolution = H_RES.load(Ordering::SeqCst) };
    }
    if !vertical_resolution.is_null() {
        unsafe { *vertical_resolution = V_RES.load(Ordering::SeqCst) };
    }
    if !color_depth.is_null() {
        unsafe { *color_depth = COLOR_DEPTH.load(Ordering::SeqCst) };
    }
    if !refresh_rate.is_null() {
        unsafe { *refresh_rate = REFRESH_RATE.load(Ordering::SeqCst) };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_mode(
    _this: *const EfiUgaDrawProtocol,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    color_depth: u32,
    refresh_rate: u32,
) -> EfiStatus {
    H_RES.store(horizontal_resolution, Ordering::SeqCst);
    V_RES.store(vertical_resolution, Ordering::SeqCst);
    COLOR_DEPTH.store(color_depth, Ordering::SeqCst);
    REFRESH_RATE.store(refresh_rate, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_blt(
    _this: *const EfiUgaDrawProtocol,
    _blt_buffer: *mut EfiUgaPixel,
    blt_operation: EfiUgaBltOperation,
    _source_x: UIntN,
    _source_y: UIntN,
    _destination_x: UIntN,
    _destination_y: UIntN,
    _width: UIntN,
    _height: UIntN,
    _delta: UIntN,
) -> EfiStatus {
    LAST_BLT_OP.store(blt_operation as u32, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

#[test]
fn test_uga_draw_constants_and_guids() {
    assert_eq!(
        EFI_UGA_DRAW_PROTOCOL_GUID,
        EfiGuid::new(
            0x982c_058b,
            0x0073,
            0x4183,
            [0xac, 0xbb, 0x69, 0xb4, 0x2c, 0x11, 0xce, 0x7f]
        )
    );
    assert_eq!(UgaDraw::GUID, EFI_UGA_DRAW_PROTOCOL_GUID);
    assert_eq!(core::mem::size_of::<EfiUgaPixel>(), 4);
}

#[test]
fn test_uga_pixel_rgb() {
    let px = EfiUgaPixel::rgb(255, 128, 64);
    assert_eq!(px.red, 255);
    assert_eq!(px.green, 128);
    assert_eq!(px.blue, 64);
    assert_eq!(px.reserved, 0);
}

#[test]
fn test_uga_draw_client_wrapper() {
    let mock_proto = EfiUgaDrawProtocol {
        get_mode: mock_get_mode,
        set_mode: mock_set_mode,
        blt: mock_blt,
    };
    let client = unsafe { UgaDraw::from_raw(&raw const mock_proto) };

    let mode = client.get_mode();
    assert!(mode.is_ok());
    if let Ok(m) = mode {
        assert_eq!(m.horizontal_resolution, 1024);
        assert_eq!(m.vertical_resolution, 768);
        assert_eq!(m.color_depth, 32);
        assert_eq!(m.refresh_rate, 60);
    }

    let new_mode = UgaModeInfo {
        horizontal_resolution: 1920,
        vertical_resolution: 1080,
        color_depth: 32,
        refresh_rate: 60,
    };
    assert!(client.set_mode(&new_mode).is_ok());

    let updated_mode = client.get_mode();
    assert!(updated_mode.is_ok());
    if let Ok(m) = updated_mode {
        assert_eq!(m.horizontal_resolution, 1920);
        assert_eq!(m.vertical_resolution, 1080);
    }

    let mut fill_px = [EfiUgaPixel::rgb(255, 0, 0)];
    assert!(
        client
            .blt(
                Some(&mut fill_px),
                EfiUgaBltOperation::VideoFill,
                0,
                0,
                10,
                10,
                100,
                100,
                0,
            )
            .is_ok()
    );
    assert_eq!(
        LAST_BLT_OP.load(Ordering::SeqCst),
        EfiUgaBltOperation::VideoFill as u32
    );

    assert!(
        unsafe {
            client.blt_raw(
                core::ptr::null_mut(),
                EfiUgaBltOperation::VideoToVideo,
                0,
                0,
                50,
                50,
                200,
                200,
                0,
            )
        }
        .is_ok()
    );
    assert_eq!(
        LAST_BLT_OP.load(Ordering::SeqCst),
        EfiUgaBltOperation::VideoToVideo as u32
    );
}
