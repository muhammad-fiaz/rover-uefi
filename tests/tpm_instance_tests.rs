//! Integration tests for TPM instance GUIDs and platform measurement structures.

use rover_uefi::base::{EfiGuid, EfiPhysicalAddress};
use rover_uefi::protocols::security::tpm_instance::{
    CC_EVENT_ENTRY_HOB_GUID, MEASURED_FV_HOB_GUID, MeasuredHobHeader,
    TCG2_ACPI_COMMUNICATE_BUFFER_HOB_GUID, TCG2_ACPI_COMMUNICATE_BUFFER_HOB_REVISION,
    TPM_DEVICE_INTERFACE_NONE, TPM_DEVICE_INTERFACE_TPM12, TPM_DEVICE_INTERFACE_TPM20_DTPM,
    TPM_DEVICE_SELECTED_GUID, Tcg2AcpiCommunicateBuffer,
};

#[test]
fn test_tpm_instance_guids() {
    assert_eq!(
        TPM_DEVICE_INTERFACE_NONE,
        EfiGuid::new(
            0x0000_0000,
            0x0000,
            0x0000,
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        )
    );
    assert_eq!(
        TPM_DEVICE_INTERFACE_TPM12,
        EfiGuid::new(
            0x8B01_E5B6,
            0x4F19,
            0x46E8,
            [0xAB, 0x93, 0x1C, 0x53, 0x67, 0x1B, 0x90, 0xCC],
        )
    );
    assert_eq!(
        TPM_DEVICE_INTERFACE_TPM20_DTPM,
        EfiGuid::new(
            0x286B_F25A,
            0xC2C3,
            0x408C,
            [0xB3, 0xB4, 0x25, 0xE6, 0x75, 0x8B, 0x73, 0x17],
        )
    );
    assert_eq!(
        TPM_DEVICE_SELECTED_GUID,
        EfiGuid::new(
            0x7F41_58D3,
            0x074D,
            0x456D,
            [0x8C, 0xB2, 0x01, 0xF9, 0xC8, 0xF7, 0x9D, 0xAA],
        )
    );
    assert_eq!(
        CC_EVENT_ENTRY_HOB_GUID,
        EfiGuid::new(
            0x20F8_FD36,
            0x6D00,
            0x40FB,
            [0xB7, 0x04, 0xD1, 0x2C, 0x15, 0x3C, 0x62, 0xEB],
        )
    );
    assert_eq!(
        MEASURED_FV_HOB_GUID,
        EfiGuid::new(
            0xB236_0B42,
            0x7173,
            0x420A,
            [0x86, 0x96, 0x46, 0xCA, 0x6B, 0xAB, 0x10, 0x60],
        )
    );
    assert_eq!(
        TCG2_ACPI_COMMUNICATE_BUFFER_HOB_GUID,
        EfiGuid::new(
            0xCEFE_A14F,
            0x9F1A,
            0x4774,
            [0x8D, 0x18, 0x79, 0x93, 0x8D, 0x48, 0xFE, 0x7D],
        )
    );
    assert_eq!(TCG2_ACPI_COMMUNICATE_BUFFER_HOB_REVISION, 1);
}

#[test]
fn test_tpm_structures() {
    let buf_desc = Tcg2AcpiCommunicateBuffer {
        tcg2_acpi_communicate_buffer: EfiPhysicalAddress::from_u64(0x1000_0000),
        pages: 4,
    };
    assert_eq!(buf_desc.tcg2_acpi_communicate_buffer.to_u64(), 0x1000_0000);
    assert_eq!(buf_desc.pages, 4);

    let fv_header = MeasuredHobHeader { num: 3 };
    assert_eq!(fv_header.num, 3);
}
