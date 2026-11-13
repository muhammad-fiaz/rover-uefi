//! Tests for Heterogeneous Multiprocessor HOB (MP Information 2).

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::protocols::core::mp_services::{
    CORE_TYPE_EFFICIENT, CORE_TYPE_PERFORMANCE, CORE_TYPE_STANDARD, EfiCpuPhysicalLocation,
    EfiCpuPhysicalLocation2, EfiProcessorInformation, MP_INFORMATION2_GUID,
    MP_INFORMATION2_HOB_REVISION, MpInformation2Entry, MpInformation2HobData,
};

#[test]
fn test_mp_information2_structures() {
    assert_eq!(
        MP_INFORMATION2_GUID.to_string(),
        "417a7f64-f4e9-4b32-846a-5cc4d8621879"
    );
    assert_eq!(MP_INFORMATION2_HOB_REVISION, 1);

    let p_core = MpInformation2Entry {
        processor_info: EfiProcessorInformation {
            processor_id: 0,
            status_flag: 1,
            location: EfiCpuPhysicalLocation {
                package: 0,
                core: 0,
                thread: 0,
            },
            extended_information: EfiCpuPhysicalLocation2 {
                package: 0,
                die: 0,
                tile: 0,
                module: 0,
                core: 0,
                thread: 0,
            },
        },
        core_type: CORE_TYPE_PERFORMANCE,
        reserved: [0u8; 7],
    };

    let e_core = MpInformation2Entry {
        processor_info: EfiProcessorInformation {
            processor_id: 8,
            status_flag: 1,
            location: EfiCpuPhysicalLocation {
                package: 0,
                core: 8,
                thread: 0,
            },
            extended_information: EfiCpuPhysicalLocation2 {
                package: 0,
                die: 0,
                tile: 0,
                module: 1,
                core: 0,
                thread: 0,
            },
        },
        core_type: CORE_TYPE_EFFICIENT,
        reserved: [0u8; 7],
    };

    assert!(p_core.is_performance_core());
    assert!(!p_core.is_efficient_core());
    assert!(e_core.is_efficient_core());
    assert!(!e_core.is_performance_core());

    let hob = MpInformation2HobData {
        number_of_processors: 16,
        entry_size: core::mem::size_of::<MpInformation2Entry>() as u16,
        version: MP_INFORMATION2_HOB_REVISION,
        reserved: [0u8; 3],
        processor_index: 0,
    };

    assert_eq!(hob.number_of_processors, 16);
    assert_eq!(CORE_TYPE_STANDARD, 0);
}
