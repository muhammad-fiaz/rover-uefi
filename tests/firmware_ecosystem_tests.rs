//! Comprehensive integration tests for new firmware ecosystem protocols:
//! ESRT, IOMMU, PS/2 Policy, Variable Policy, and Audio I/O.

use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::audio::*;
use rover_uefi::protocols::console::ps2_policy::*;
use rover_uefi::protocols::driver::iommu::*;
use rover_uefi::protocols::security::variable_policy::*;
use rover_uefi::tables::esrt::*;

#[test]
fn test_esrt_guid_constants_and_struct_layout() {
    assert_eq!(
        EFI_SYSTEM_RESOURCE_TABLE_GUID,
        EfiGuid::new(
            0xB122_A263,
            0x3661,
            0x4F68,
            [0x99, 0x29, 0x78, 0xF8, 0xB0, 0xD6, 0x21, 0x80]
        )
    );
    assert_eq!(
        ESRT_MANAGEMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xA340_C064,
            0x723C,
            0x4A9C,
            [0xA4, 0xDD, 0xD5, 0xB4, 0x7A, 0x26, 0xFB, 0xB0]
        )
    );

    assert_eq!(EFI_SYSTEM_RESOURCE_TABLE_FIRMWARE_RESOURCE_VERSION, 1);
    assert_eq!(ESRT_FW_TYPE_SYSTEMFIRMWARE, 0x01);
    assert_eq!(ESRT_FW_TYPE_DEVICEFIRMWARE, 0x02);
    assert_eq!(ESRT_FW_TYPE_UEFIDRIVER, 0x03);

    assert_eq!(LAST_ATTEMPT_STATUS_SUCCESS, 0x00);
    assert_eq!(LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL, 0x01);

    // EfiSystemResourceEntry: EfiGuid (16) + 6 * u32 (24) = 40 bytes
    assert_eq!(core::mem::size_of::<EfiSystemResourceEntry>(), 40);

    // EfiSystemResourceTable: u32 + u32 + u64 = 16 bytes
    assert_eq!(core::mem::size_of::<EfiSystemResourceTable>(), 16);

    // EsrtManagementProtocol has 6 function pointers
    assert_eq!(
        core::mem::size_of::<EsrtManagementProtocol>(),
        6 * core::mem::size_of::<usize>()
    );

    let entry = EfiSystemResourceEntry {
        fw_class: EfiGuid::new(0x1234_5678, 0xABCD, 0xEF01, [1, 2, 3, 4, 5, 6, 7, 8]),
        fw_type: ESRT_FW_TYPE_SYSTEMFIRMWARE,
        fw_version: 0x0002_0001,
        lowest_supported_fw_version: 0x0001_0000,
        capsule_flags: 0,
        last_attempt_version: 0x0002_0001,
        last_attempt_status: LAST_ATTEMPT_STATUS_SUCCESS,
    };
    assert_eq!(entry.fw_version, 0x0002_0001);
    assert!(entry.fw_version >= entry.lowest_supported_fw_version);
}

#[test]
fn test_iommu_guid_and_abi_layout() {
    assert_eq!(
        EDKII_IOMMU_PROTOCOL_GUID,
        EfiGuid::new(
            0x4E93_9DE9,
            0xD948,
            0x4B0F,
            [0x88, 0xED, 0xE6, 0xE1, 0xCE, 0x51, 0x7C, 0x1E]
        )
    );
    assert_eq!(EDKII_IOMMU_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(EDKII_IOMMU_ACCESS_READ, 0x01);
    assert_eq!(EDKII_IOMMU_ACCESS_WRITE, 0x02);
    assert_eq!(EDKII_IOMMU_ATTRIBUTE_DUAL_ADDRESS_CYCLE, 0x8000);

    let op = EdkiiIoMmuOperation::BusMasterCommonBuffer64;
    assert_eq!(op as u32, 5);

    // EdkiiIoMmuProtocol: Revision (u64) + 5 function pointers
    assert_eq!(
        core::mem::size_of::<EdkiiIoMmuProtocol>(),
        8 + 5 * core::mem::size_of::<usize>()
    );
}

#[test]
fn test_ps2_policy_guid_and_abi_layout() {
    assert_eq!(
        EFI_PS2_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0x4DF1_9259,
            0xDC71,
            0x4D46,
            [0xBE, 0xF1, 0x35, 0x7B, 0xB5, 0x78, 0xC4, 0x18]
        )
    );
    assert_eq!(EFI_KEYBOARD_SCROLLLOCK, 0x01);
    assert_eq!(EFI_KEYBOARD_NUMLOCK, 0x02);
    assert_eq!(EFI_KEYBOARD_CAPSLOCK, 0x04);

    let proto = EfiPs2PolicyProtocol {
        keyboard_light: EFI_KEYBOARD_NUMLOCK | EFI_KEYBOARD_CAPSLOCK,
        ps2_init_hardware: dummy_ps2_init,
    };
    assert_eq!(
        proto.keyboard_light,
        EFI_KEYBOARD_NUMLOCK | EFI_KEYBOARD_CAPSLOCK
    );

    let status = unsafe { (proto.ps2_init_hardware)(EfiHandle::NULL) };
    assert_eq!(status, EfiStatus::SUCCESS);
}

unsafe extern "efiapi" fn dummy_ps2_init(_handle: EfiHandle) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_variable_policy_guid_and_structures() {
    assert_eq!(
        EDKII_VARIABLE_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0x81D1_675C,
            0x86F6,
            0x48DF,
            [0xBD, 0x95, 0x9A, 0x6E, 0x4F, 0x09, 0x25, 0xC3]
        )
    );
    assert_eq!(EDKII_VARIABLE_POLICY_PROTOCOL_REVISION, 0x0002_0000);
    assert_eq!(VARIABLE_POLICY_ENTRY_REVISION, 0x0001_0000);
    assert_eq!(VARIABLE_POLICY_NO_MIN_SIZE, 0);
    assert_eq!(VARIABLE_POLICY_NO_MAX_SIZE, u32::MAX);
    assert_eq!(VARIABLE_POLICY_TYPE_LOCK_ON_CREATE, 2);

    let entry = VariablePolicyEntry {
        version: VARIABLE_POLICY_ENTRY_REVISION,
        size: core::mem::size_of::<VariablePolicyEntry>() as u16,
        offset_to_name: 0,
        namespace: EfiGuid::new(0x1111_2222, 0x3333, 0x4444, [1, 2, 3, 4, 5, 6, 7, 8]),
        min_size: 4,
        max_size: 1024,
        attributes_must_have: 0x07,
        attributes_cant_have: 0,
        lock_policy_type: VARIABLE_POLICY_TYPE_LOCK_NOW,
        padding: [0; 3],
    };
    let min_size = entry.min_size;
    let max_size = entry.max_size;
    assert_eq!(min_size, 4);
    assert_eq!(max_size, 1024);

    // EdkiiVariablePolicyProtocol has revision (u64) + 7 function pointers
    assert_eq!(
        core::mem::size_of::<EdkiiVariablePolicyProtocol>(),
        8 + 7 * core::mem::size_of::<usize>()
    );
}

#[test]
fn test_audio_io_guid_and_abi_layout() {
    assert_eq!(
        EFI_AUDIO_IO_PROTOCOL_GUID,
        EfiGuid::new(
            0xF05B_5CEA,
            0x1384,
            0x4B83,
            [0x84, 0x90, 0x76, 0x6E, 0x05, 0x47, 0x91, 0x46]
        )
    );

    assert_eq!(AudioSampleRate::Rate44kHz as u32, 44_100);
    assert_eq!(AudioSampleRate::Rate48kHz as u32, 48_000);
    assert_eq!(AudioSampleRate::Rate192kHz as u32, 192_000);

    assert_eq!(AudioBitsPerSample::Bits16 as u8, 16);
    assert_eq!(AudioBitsPerSample::Bits24 as u8, 24);

    assert_eq!(AudioChannels::Stereo as u8, 2);
    assert_eq!(AudioChannels::Surround51 as u8, 6);

    assert_eq!(AudioPortType::Speaker as u32, 1);
    assert_eq!(AudioPortType::Headphones as u32, 2);

    // EfiAudioIoProtocol has 5 function pointer fields
    assert_eq!(
        core::mem::size_of::<EfiAudioIoProtocol>(),
        5 * core::mem::size_of::<usize>()
    );
}
