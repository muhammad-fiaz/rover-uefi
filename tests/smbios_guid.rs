//! SMBIOS GUID tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::smbios::guid::{SMBIOS_2X_GUID, SMBIOS_3X_GUID};

#[test]
fn guid_bytes() {
    assert_eq!(
        SMBIOS_2X_GUID.to_bytes(),
        [
            0x31, 0x2D, 0x9D, 0xEB, 0x88, 0x2D, 0xD3, 0x11, 0x9A, 0x16, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(SMBIOS_3X_GUID.to_bytes()),
        SMBIOS_3X_GUID
    );
}
