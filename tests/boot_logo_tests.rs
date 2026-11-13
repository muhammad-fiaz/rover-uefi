//! Integration tests for Boot Logo and Boot Logo 2 protocols.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::graphics::boot_logo::{
    BOOT_LOGO_PROTOCOL_GUID, BOOT_LOGO2_PROTOCOL_GUID, BootLogo, BootLogo2, BootLogo2Protocol,
    BootLogoProtocol,
};
use rover_uefi::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

static mut LOGO1_CALLED: bool = false;
static mut LOGO2_CALLED: bool = false;

unsafe extern "efiapi" fn mock_set_logo1(
    _this: *mut BootLogoProtocol,
    _blt_buffer: *const EfiGraphicsOutputBltPixel,
    _x: usize,
    _y: usize,
    _width: usize,
    _height: usize,
) -> EfiStatus {
    unsafe {
        LOGO1_CALLED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_logo2(
    _this: *mut BootLogo2Protocol,
    _blt_buffer: *const EfiGraphicsOutputBltPixel,
    _x: usize,
    _y: usize,
    _width: usize,
    _height: usize,
) -> EfiStatus {
    unsafe {
        LOGO2_CALLED = true;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_boot_logo_guids() {
    assert_eq!(
        BOOT_LOGO_PROTOCOL_GUID,
        EfiGuid::new(
            0xCDEA_2BD3,
            0xFC25,
            0x4C1C,
            [0xB9, 0x7C, 0xB3, 0x11, 0x86, 0x06, 0x49, 0x90],
        )
    );
    assert_eq!(
        BOOT_LOGO2_PROTOCOL_GUID,
        EfiGuid::new(
            0x4B5D_C1DF,
            0x1EAA,
            0x48B2,
            [0xA7, 0xE9, 0xEA, 0xC4, 0x89, 0xA0, 0x0B, 0x5C],
        )
    );
}

#[test]
fn test_boot_logo_clients() {
    let mut proto1 = BootLogoProtocol {
        set_boot_logo: Some(mock_set_logo1),
    };
    let mut logo1 = BootLogo::new(&mut proto1);
    let pixel = EfiGraphicsOutputBltPixel {
        blue: 0,
        green: 0,
        red: 0,
        reserved: 0,
    };
    assert!(unsafe { logo1.set_logo(&raw const pixel, 10, 10, 100, 100) }.is_ok());
    assert!(unsafe { LOGO1_CALLED });

    let mut proto2 = BootLogo2Protocol {
        set_boot_logo: Some(mock_set_logo2),
        get_boot_logo: None,
    };
    let mut logo2 = BootLogo2::new(&mut proto2);
    assert!(unsafe { logo2.set_logo(&raw const pixel, 20, 20, 200, 200) }.is_ok());
    assert!(unsafe { LOGO2_CALLED });
}
