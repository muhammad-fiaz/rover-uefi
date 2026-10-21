//! Security arch tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::SECURITY_ARCH_GUID;
use rover_uefi::protocols::security::security_arch::protocol::EfiSecurityArchProtocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        SECURITY_ARCH_GUID.to_bytes(),
        [
            0xE3, 0x23, 0x64, 0xA4, 0x17, 0x46, 0xF1, 0x49, 0xB9, 0xFF, 0xD1, 0xBF, 0xA9, 0x11,
            0x58, 0x39
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(SECURITY_ARCH_GUID.to_bytes()),
        SECURITY_ARCH_GUID
    );
}

#[test]
fn protocol_has_single_callback() {
    assert_eq!(size_of::<EfiSecurityArchProtocol>(), size_of::<usize>());
    assert_eq!(
        offset_of!(EfiSecurityArchProtocol, file_authentication_state),
        0
    );
}
