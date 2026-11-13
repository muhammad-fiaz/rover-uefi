// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Multiprocessor (MP) Services Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::{Bool, UIntN};
use rover_uefi::base::{EfiEvent, EfiStatus};
use rover_uefi::protocols::core::{
    EFI_MP_SERVICES_PROTOCOL_GUID, EfiApProcedure, EfiCpuPhysicalLocation, EfiMpServicesProtocol,
    EfiProcessorInformation, MpServices, PROCESSOR_AS_BSP_BIT, PROCESSOR_ENABLED_BIT,
    PROCESSOR_HEALTH_STATUS_BIT,
};

unsafe extern "efiapi" fn mock_get_number_of_processors(
    _this: *const EfiMpServicesProtocol,
    total: *mut UIntN,
    enabled: *mut UIntN,
) -> EfiStatus {
    unsafe {
        *total = 8;
        *enabled = 8;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_processor_info(
    _this: *const EfiMpServicesProtocol,
    index: UIntN,
    info: *mut EfiProcessorInformation,
) -> EfiStatus {
    unsafe {
        (*info).processor_id = index as u64;
        (*info).status_flag = if index == 0 {
            PROCESSOR_AS_BSP_BIT | PROCESSOR_ENABLED_BIT | PROCESSOR_HEALTH_STATUS_BIT
        } else {
            PROCESSOR_ENABLED_BIT | PROCESSOR_HEALTH_STATUS_BIT
        };
        (*info).location = EfiCpuPhysicalLocation {
            package: 0,
            core: u32::try_from(index).unwrap_or(0),
            thread: 0,
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_who_am_i(
    _this: *const EfiMpServicesProtocol,
    index: *mut UIntN,
) -> EfiStatus {
    unsafe {
        *index = 0;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_startup_all_aps(
    _this: *const EfiMpServicesProtocol,
    _proc: EfiApProcedure,
    _single: Bool,
    _event: EfiEvent,
    _timeout: UIntN,
    _arg: *mut c_void,
    _failed: *mut *mut UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_startup_this_ap(
    _this: *const EfiMpServicesProtocol,
    _proc: EfiApProcedure,
    _cpu: UIntN,
    _event: EfiEvent,
    _timeout: UIntN,
    _arg: *mut c_void,
    _finished: *mut Bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_switch_bsp(
    _this: *const EfiMpServicesProtocol,
    _cpu: UIntN,
    _enable_old: Bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_enable_disable_ap(
    _this: *const EfiMpServicesProtocol,
    _cpu: UIntN,
    _enable: Bool,
    _health: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn mp_services_protocol_and_wrapper() {
    assert_eq!(
        EFI_MP_SERVICES_PROTOCOL_GUID.to_string(),
        "3fdda605-a76e-4f46-ad29-12f4531b3d08"
    );

    let proto = EfiMpServicesProtocol {
        get_number_of_processors: mock_get_number_of_processors,
        get_processor_info: mock_get_processor_info,
        startup_all_aps: dummy_startup_all_aps,
        startup_this_ap: dummy_startup_this_ap,
        switch_bsp: dummy_switch_bsp,
        enable_disable_ap: dummy_enable_disable_ap,
        who_am_i: mock_who_am_i,
    };

    let wrapper = MpServices::new(&proto);
    let Ok((total, enabled)) = wrapper.processor_counts() else {
        panic!("processor counts failed");
    };
    assert_eq!(total, 8);
    assert_eq!(enabled, 8);

    let Ok(bsp_info) = wrapper.processor_info(0) else {
        panic!("bsp info failed");
    };
    assert_eq!(bsp_info.processor_id, 0);
    assert!((bsp_info.status_flag & PROCESSOR_AS_BSP_BIT) != 0);

    let Ok(ap_info) = wrapper.processor_info(1) else {
        panic!("ap info failed");
    };
    assert_eq!(ap_info.processor_id, 1);
    assert_eq!(ap_info.status_flag & PROCESSOR_AS_BSP_BIT, 0);

    assert_eq!(wrapper.current_processor_index(), Ok(0));
}

#[test]
fn pei_mp_services_and_services2_guids_and_layout() {
    use rover_uefi::protocols::firmware::ppi::{
        EFI_PEI_MP_SERVICES_PPI_GUID, EFI_PEI_MP_SERVICES2_PPI_GUID, EfiPeiMpServices2Ppi,
        EfiPeiMpServicesPpi,
    };

    assert_eq!(
        EFI_PEI_MP_SERVICES_PPI_GUID.to_string(),
        "ee16160a-e8be-47a6-820a-c6900db0250a"
    );
    assert_eq!(
        EFI_PEI_MP_SERVICES2_PPI_GUID.to_string(),
        "5cb9cb3d-31a4-480c-9498-29d269bacfba"
    );

    assert_eq!(
        core::mem::size_of::<EfiPeiMpServicesPpi>(),
        core::mem::size_of::<usize>() * 7
    );
    assert_eq!(
        core::mem::size_of::<EfiPeiMpServices2Ppi>(),
        core::mem::size_of::<usize>() * 7
    );
}
