// SPDX-License-Identifier: Apache-2.0
//! Integration tests for CPU architectural protocol.

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::core::{
    CpuArch, EFI_CPU_ARCH_PROTOCOL_GUID, EfiCpuArchProtocol, EfiCpuFlushType, EfiCpuInitType,
};

static mut INTERRUPT_STATE: Bool = 1;
static mut LAST_FLUSH_LEN: u64 = 0;
static mut LAST_ATTR_SET: u64 = 0;

unsafe extern "efiapi" fn mock_flush_data_cache(
    _this: *mut EfiCpuArchProtocol,
    _start: EfiPhysicalAddress,
    length: u64,
    _flush_type: EfiCpuFlushType,
) -> EfiStatus {
    unsafe {
        LAST_FLUSH_LEN = length;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_enable_interrupt(_this: *mut EfiCpuArchProtocol) -> EfiStatus {
    unsafe {
        INTERRUPT_STATE = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_disable_interrupt(_this: *mut EfiCpuArchProtocol) -> EfiStatus {
    unsafe {
        INTERRUPT_STATE = 0;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_interrupt_state(
    _this: *mut EfiCpuArchProtocol,
    state: *mut Bool,
) -> EfiStatus {
    if state.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *state = INTERRUPT_STATE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_init(
    _this: *mut EfiCpuArchProtocol,
    _init_type: EfiCpuInitType,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_interrupt_handler(
    _this: *mut EfiCpuArchProtocol,
    _interrupt_type: isize,
    _interrupt_handler: Option<rover_uefi::protocols::core::EfiCpuInterruptHandler>,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_timer_value(
    _this: *mut EfiCpuArchProtocol,
    _timer_index: u32,
    timer_value: *mut u64,
    timer_period: *mut u64,
) -> EfiStatus {
    if timer_value.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *timer_value = 1_000_000;
        if !timer_period.is_null() {
            *timer_period = 1000;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_memory_attributes(
    _this: *mut EfiCpuArchProtocol,
    _base_address: EfiPhysicalAddress,
    _length: u64,
    attributes: u64,
) -> EfiStatus {
    unsafe {
        LAST_ATTR_SET = attributes;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_cpu_arch_protocol() {
    assert_eq!(
        EFI_CPU_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x8188_3cc7_8000_e7bc_11d4_6f42_26ba_ccb1
    );

    let raw = EfiCpuArchProtocol {
        flush_data_cache: mock_flush_data_cache,
        enable_interrupt: mock_enable_interrupt,
        disable_interrupt: mock_disable_interrupt,
        get_interrupt_state: mock_get_interrupt_state,
        init: mock_init,
        register_interrupt_handler: mock_register_interrupt_handler,
        get_timer_value: mock_get_timer_value,
        set_memory_attributes: mock_set_memory_attributes,
        number_of_timers: 4,
        dma_buffer_alignment: 64,
    };
    let cpu = CpuArch::new(&raw);

    assert_eq!(cpu.number_of_timers(), 4);
    assert_eq!(cpu.dma_buffer_alignment(), 64);

    let flush_res =
        cpu.flush_data_cache(EfiPhysicalAddress::NULL, 4096, EfiCpuFlushType::WriteBack);
    assert!(flush_res.is_ok());
    assert_eq!(unsafe { LAST_FLUSH_LEN }, 4096);

    let state = cpu.get_interrupt_state();
    let Ok(is_enabled) = state else {
        panic!("expected success");
    };
    assert!(is_enabled);

    assert!(cpu.disable_interrupt().is_ok());
    let state_after = cpu.get_interrupt_state();
    let Ok(is_enabled_after) = state_after else {
        panic!("expected success");
    };
    assert!(!is_enabled_after);

    assert!(cpu.enable_interrupt().is_ok());

    let timer = cpu.get_timer_value(0);
    let Ok((val, period)) = timer else {
        panic!("expected success");
    };
    assert_eq!(val, 1_000_000);
    assert_eq!(period, 1000);

    let attr_res = cpu.set_memory_attributes(EfiPhysicalAddress::NULL, 0x10000, 0x8);
    assert!(attr_res.is_ok());
    assert_eq!(unsafe { LAST_ATTR_SET }, 0x8);
}
