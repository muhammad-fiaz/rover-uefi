//! Integration tests for Standalone MM infrastructure types and GUIDs.

use rover_uefi::base::{EfiGuid, EfiPhysicalAddress};
use rover_uefi::protocols::firmware::standalone_mm::{
    EFI_MM_PEI_MMRAM_MEMORY_RESERVE_GUID, EfiMmCommunicateFvDispatch,
    EfiMmCommunicateFvDispatchData, EfiMmramHobDescriptorBlockHeader, MM_FV_DISPATCH_GUID,
    MP_INFORMATION_HOB_GUID, MpInformationHobHeader,
};

#[test]
fn test_standalone_mm_guids() {
    assert_eq!(
        EFI_MM_PEI_MMRAM_MEMORY_RESERVE_GUID,
        EfiGuid::new(
            0x0703_F912,
            0xBF8D,
            0x4E2A,
            [0xBE, 0x07, 0xAB, 0x27, 0x25, 0x25, 0xC5, 0x92],
        )
    );
    assert_eq!(
        MP_INFORMATION_HOB_GUID,
        EfiGuid::new(
            0xBA33_F15D,
            0x4000,
            0x45C1,
            [0x8E, 0x88, 0xF9, 0x16, 0x92, 0xD4, 0x57, 0xE3],
        )
    );
    assert_eq!(
        MM_FV_DISPATCH_GUID,
        EfiGuid::new(
            0xB656_94CC,
            0x09E3,
            0x4C3B,
            [0xB5, 0xCD, 0x05, 0xF4, 0x4D, 0x3C, 0xDB, 0xFF],
        )
    );
}

#[test]
fn test_standalone_mm_structures() {
    let fv_msg = EfiMmCommunicateFvDispatch {
        header_guid: MM_FV_DISPATCH_GUID,
        message_length: core::mem::size_of::<EfiMmCommunicateFvDispatchData>(),
        data: EfiMmCommunicateFvDispatchData {
            address: EfiPhysicalAddress::from_u64(0x2000_0000),
            size: 0x10_0000,
        },
    };
    let addr = fv_msg.data.address.to_u64();
    let size = fv_msg.data.size;
    assert_eq!(addr, 0x2000_0000);
    assert_eq!(size, 0x10_0000);

    let mmram_header = EfiMmramHobDescriptorBlockHeader {
        number_of_mm_reserved_regions: 2,
    };
    assert_eq!(mmram_header.number_of_mm_reserved_regions, 2);

    let mp_header = MpInformationHobHeader {
        number_of_processors: 8,
        number_of_enabled_processors: 8,
    };
    let num_procs = mp_header.number_of_processors;
    assert_eq!(num_procs, 8);
}
