//! Security2 tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::guid::{SECURITY_ARCH_GUID, SECURITY2_ARCH_GUID};
use rover_uefi::protocols::security::security_arch::security2::EfiSecurity2ArchProtocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        SECURITY2_ARCH_GUID.to_bytes(),
        [
            0x58, 0x2F, 0xAB, 0x94, 0x38, 0x14, 0xF1, 0x4E, 0x91, 0x52, 0x18, 0x94, 0x1A, 0x3A,
            0x0E, 0x68
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(SECURITY2_ARCH_GUID.to_bytes()),
        SECURITY2_ARCH_GUID
    );
}

#[test]
fn protocols_are_distinct() {
    assert_ne!(SECURITY2_ARCH_GUID, SECURITY_ARCH_GUID);
}

#[test]
fn protocol_has_single_callback() {
    assert_eq!(size_of::<EfiSecurity2ArchProtocol>(), size_of::<usize>());
    assert_eq!(offset_of!(EfiSecurity2ArchProtocol, file_authentication), 0);
}
