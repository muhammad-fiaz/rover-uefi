//! Firmware discovery tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::acpi::guid::ACPI_20_GUID;
use rover_uefi::protocols::smbios::guid::SMBIOS_3X_GUID;

#[test]
fn discovery_guids_distinct_with_bytes() {
    assert_ne!(ACPI_20_GUID, SMBIOS_3X_GUID);
    assert!(!ACPI_20_GUID.is_zero());
    assert!(!SMBIOS_3X_GUID.is_zero());
    assert_eq!(
        ACPI_20_GUID.to_bytes(),
        [
            0x71, 0xE8, 0x68, 0x88, 0xF1, 0xE4, 0xD3, 0x11, 0xBC, 0x22, 0x00, 0x80, 0xC7, 0x3C,
            0x88, 0x81
        ]
    );
    assert_eq!(
        SMBIOS_3X_GUID.to_bytes(),
        [
            0x44, 0x15, 0xFD, 0xF2, 0x94, 0x97, 0x2C, 0x4A, 0x99, 0x2E, 0xE5, 0xBB, 0xCF, 0x20,
            0xE3, 0x94
        ]
    );
    assert_eq!(EfiGuid::from_bytes(ACPI_20_GUID.to_bytes()), ACPI_20_GUID);
    assert_eq!(
        EfiGuid::from_bytes(SMBIOS_3X_GUID.to_bytes()),
        SMBIOS_3X_GUID
    );
}
