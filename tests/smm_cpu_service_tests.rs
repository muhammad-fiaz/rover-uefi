//! Integration tests for SMM CPU Service, SMM CPU Rendezvous, SM Monitor Init,
//! and TPM NVS MM structures.
#![allow(clippy::expect_used)]

use core::ffi::c_void;
use rover_uefi::base::{EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::core::cpu::EfiCpuInterruptHandler;
use rover_uefi::protocols::core::mp_services::EfiProcessorInformation;
use rover_uefi::protocols::firmware::smm_cpu::{
    SM_MONITOR_INIT_PROTOCOL_GUID, SM_MONITOR_STATE_ACTIVATED, SM_MONITOR_STATE_ENABLED,
    SMM_CPU_RENDEZVOUS_PROTOCOL_GUID, SMM_CPU_SERVICE_PROTOCOL_GUID, SmMonitorInit,
    SmMonitorInitProtocol, SmmCpuRendezvous, SmmCpuRendezvousProtocol, SmmCpuService,
    SmmCpuServiceProtocol,
};
use rover_uefi::protocols::security::tpm_instance::{
    FIRMWARE_VOLUME_MEASUREMENT_EXCLUDED_HOB_GUID, MM_TPM_NVS_HOB_GUID, MemoryClearNvs,
    PhysicalPresenceNvs, TcgNvs, TpmNvsMmCommBuffer, TpmNvsMmFunction,
};

static mut WHOAMI_CALLED: bool = false;
static mut RENDEZVOUS_CALLED: bool = false;
static mut MONITOR_LOADED: bool = false;

unsafe extern "efiapi" fn mock_get_processor_info(
    _this: *const SmmCpuServiceProtocol,
    processor_number: usize,
    processor_info: *mut EfiProcessorInformation,
) -> EfiStatus {
    if processor_info.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*processor_info).processor_id = processor_number as u64;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_switch_bsp(
    _this: *const SmmCpuServiceProtocol,
    _processor_number: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_add_processor(
    _this: *const SmmCpuServiceProtocol,
    _processor_id: u64,
    processor_number: *mut usize,
) -> EfiStatus {
    if !processor_number.is_null() {
        unsafe {
            *processor_number = 1;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_remove_processor(
    _this: *const SmmCpuServiceProtocol,
    _processor_number: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_who_am_i(
    _this: *const SmmCpuServiceProtocol,
    processor_number: *mut usize,
) -> EfiStatus {
    if processor_number.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *processor_number = 0;
        WHOAMI_CALLED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_wait_for_all_processor(
    _this: *mut SmmCpuRendezvousProtocol,
    _blocking_mode: bool,
) -> EfiStatus {
    unsafe {
        RENDEZVOUS_CALLED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_load_monitor(_stm_image: u64, _stm_image_size: usize) -> EfiStatus {
    unsafe {
        MONITOR_LOADED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_monitor_state() -> u32 {
    SM_MONITOR_STATE_ENABLED | SM_MONITOR_STATE_ACTIVATED
}

unsafe extern "efiapi" fn mock_register_exception_handler(
    _this: *mut SmmCpuServiceProtocol,
    _exception_type: isize,
    _interrupt_handler: Option<EfiCpuInterruptHandler>,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_add_pi_resource(
    _resource_list: *mut c_void,
    _num_entries: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_delete_pi_resource(
    _resource_list: *mut c_void,
    _num_entries: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_pi_resource(
    _resource_list: *mut c_void,
    _resource_size: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_smm_cpu_service_guids_and_wrappers() {
    assert_eq!(
        SMM_CPU_SERVICE_PROTOCOL_GUID.to_string(),
        "1d202cab-c8ab-4d5c-94f7-3cfcc0d3d335"
    );
    assert_eq!(
        SMM_CPU_RENDEZVOUS_PROTOCOL_GUID.to_string(),
        "aa00d50b-4911-428f-b91a-a59ddb13e24c"
    );
    assert_eq!(
        SM_MONITOR_INIT_PROTOCOL_GUID.to_string(),
        "228f344d-b3de-43bb-a4d7-ea200b1b1482"
    );

    let raw_service = SmmCpuServiceProtocol {
        get_processor_info: mock_get_processor_info,
        switch_bsp: mock_switch_bsp,
        add_processor: mock_add_processor,
        remove_processor: mock_remove_processor,
        who_am_i: mock_who_am_i,
        register_exception_handler: mock_register_exception_handler,
    };
    let service =
        unsafe { SmmCpuService::from_raw(&raw const raw_service) }.expect("valid smm cpu service");

    let core_id = service.who_am_i().expect("who am i succeeded");
    assert_eq!(core_id, 0);
    assert!(unsafe { WHOAMI_CALLED });

    let proc_info = service.get_processor_info(0).expect("proc info ok");
    assert_eq!(proc_info.processor_id, 0);

    let mut raw_rendezvous = SmmCpuRendezvousProtocol {
        wait_for_all_processor: mock_wait_for_all_processor,
    };
    let mut rendezvous =
        unsafe { SmmCpuRendezvous::from_raw(&raw mut raw_rendezvous) }.expect("valid rendezvous");
    assert!(rendezvous.wait_for_all_processor(true).is_ok());
    assert!(unsafe { RENDEZVOUS_CALLED });

    let mut raw_monitor = SmMonitorInitProtocol {
        load_monitor: mock_load_monitor,
        add_pi_resource: mock_add_pi_resource,
        delete_pi_resource: mock_delete_pi_resource,
        get_pi_resource: mock_get_pi_resource,
        get_monitor_state: mock_get_monitor_state,
    };
    let mut monitor =
        unsafe { SmMonitorInit::from_raw(&raw mut raw_monitor) }.expect("valid sm monitor");
    assert!(monitor.load_monitor(0x1000_0000, 0x40000).is_ok());
    assert!(unsafe { MONITOR_LOADED });
    assert!(monitor.is_activated());
}

#[test]
fn test_tpm_nvs_and_fv_excluded_hob() {
    assert_eq!(
        FIRMWARE_VOLUME_MEASUREMENT_EXCLUDED_HOB_GUID.to_string(),
        "dd55d497-5ed5-403c-92b5-703a7259ae2a"
    );
    assert_eq!(
        MM_TPM_NVS_HOB_GUID.to_string(),
        "c96c76eb-bc78-429c-9f4b-da5178c28457"
    );

    let pp_nvs = PhysicalPresenceNvs {
        software_smi: 0xB2,
        parameter: 1,
        response: 0,
        request: 14,
        request_parameter: 0,
        last_request: 14,
        return_code: 0,
    };
    assert_eq!(pp_nvs.software_smi, 0xB2);
    let req = pp_nvs.request;
    assert_eq!(req, 14);

    let mc_nvs = MemoryClearNvs {
        software_smi: 0xB3,
        parameter: 2,
        request: 1,
        return_code: 0,
    };
    assert_eq!(mc_nvs.software_smi, 0xB3);

    let tcg_nvs = TcgNvs {
        physical_presence: pp_nvs,
        memory_clear: mc_nvs,
        pp_request_user_confirm: 1,
        tpm_irq_num: 11,
        is_short_form_pkg_length: false,
    };
    let tcg_req = tcg_nvs.physical_presence.request;
    assert_eq!(tcg_req, 14);
    let irq = tcg_nvs.tpm_irq_num;
    assert_eq!(irq, 11);

    let comm_buf = TpmNvsMmCommBuffer {
        function: TpmNvsMmFunction::ExchangeInfo as u64,
        return_status: 0,
        target_address: EfiPhysicalAddress::from_u64(0x7E00_0000),
        registered_pp_swi_value: 0xB2,
        registered_mc_swi_value: 0xB3,
    };
    let func = comm_buf.function;
    assert_eq!(func, 0);
    let target = comm_buf.target_address;
    assert_eq!(target, EfiPhysicalAddress::from_u64(0x7E00_0000));
}
