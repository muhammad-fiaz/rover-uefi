//! EFI Byte Code (EBC) interpreter, VM test, and debugger integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::debug::ebc::{
    EFI_EBC_INTERPRETER_PROTOCOL_GUID, EFI_EBC_PROTOCOL_GUID,
    EFI_EBC_SIMPLE_DEBUGGER_PROTOCOL_GUID, EFI_EBC_VM_TEST_PROTOCOL_GUID, Ebc, EbcSimpleDebugger,
    EbcVmContext, EbcVmTest, EfiEbcProtocol, EfiEbcSimpleDebuggerProtocol, EfiEbcVmTestProtocol,
    exception_flags, opcodes, stop_flags, vm_flags,
};

#[test]
fn test_ebc_guids() {
    let expected_ebc = EfiGuid::new(
        0x13AC_6DD1,
        0x73D0,
        0x11D4,
        [0xB0, 0x6B, 0x00, 0xAA, 0x00, 0xBD, 0x6D, 0xE7],
    );
    assert_eq!(EFI_EBC_PROTOCOL_GUID, expected_ebc);
    assert_eq!(EFI_EBC_INTERPRETER_PROTOCOL_GUID, expected_ebc);

    let expected_vm = EfiGuid::new(
        0xAAEA_CCFD,
        0xF27B,
        0x4C17,
        [0xB6, 0x10, 0x75, 0xCA, 0x1F, 0x2D, 0xFB, 0x52],
    );
    assert_eq!(EFI_EBC_VM_TEST_PROTOCOL_GUID, expected_vm);

    let expected_dbg = EfiGuid::new(
        0x2A72_D11E,
        0x7376,
        0x40F6,
        [0x9C, 0x68, 0x23, 0xFA, 0x2F, 0xE3, 0x63, 0xF1],
    );
    assert_eq!(EFI_EBC_SIMPLE_DEBUGGER_PROTOCOL_GUID, expected_dbg);
}

#[test]
fn test_ebc_opcodes_and_flags() {
    assert_eq!(opcodes::BREAK, 0x00);
    assert_eq!(opcodes::JMP, 0x01);
    assert_eq!(opcodes::CALL, 0x03);
    assert_eq!(opcodes::RET, 0x04);
    assert_eq!(opcodes::CMPEQ, 0x05);
    assert_eq!(opcodes::ADD, 0x0C);
    assert_eq!(opcodes::SUB, 0x0D);
    assert_eq!(opcodes::MOVI, 0x37);
    assert_eq!(opcodes::MOVREL, 0x39);

    assert_eq!(stop_flags::APP_DONE, 0x0001);
    assert_eq!(stop_flags::BREAKPOINT, 0x0002);
    assert_eq!(vm_flags::CC, 0x0001);
    assert_eq!(vm_flags::STEP, 0x0002);
    assert_eq!(exception_flags::FATAL, 0x8000_0000);
    assert_eq!(exception_flags::NONE, 0x0000_0000);
}

unsafe extern "efiapi" fn mock_create_thunk(
    _this: *mut EfiEbcProtocol,
    _image_handle: EfiHandle,
    _ebc_entry_point: *const core::ffi::c_void,
    thunk: *mut *mut core::ffi::c_void,
) -> EfiStatus {
    if thunk.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *thunk = 0xCAFE as *mut core::ffi::c_void;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unload_image(
    _this: *mut EfiEbcProtocol,
    _image_handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_version(
    _this: *mut EfiEbcProtocol,
    version: *mut u64,
) -> EfiStatus {
    if version.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *version = 0x0001_0002;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_ebc_interpreter_wrapper() {
    let mut mock_proto = EfiEbcProtocol {
        create_thunk: Some(mock_create_thunk),
        unload_image: Some(mock_unload_image),
        register_icache_flush: None,
        get_version: Some(mock_get_version),
    };

    let Some(mut ebc) = (unsafe { Ebc::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap ebc protocol");
    };

    let Ok(thunk) = (unsafe { ebc.create_thunk(EfiHandle::NULL, core::ptr::null()) }) else {
        panic!("create_thunk failed");
    };
    assert_eq!(thunk as usize, 0xCAFE);

    let res_unload = ebc.unload_image(EfiHandle::NULL);
    assert_eq!(res_unload, Ok(()));

    let Ok(ver) = ebc.version() else {
        panic!("version failed");
    };
    assert_eq!(ver, 0x0001_0002);
}

unsafe extern "efiapi" fn mock_vm_execute(
    _this: *mut EfiEbcVmTestProtocol,
    vm_ptr: *mut EbcVmContext,
    instruction_count: *mut UIntN,
) -> EfiStatus {
    if vm_ptr.is_null() || instruction_count.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*vm_ptr).gpr[0] = 42;
        *instruction_count = 1;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_ebc_vm_test_wrapper() {
    let mut mock_proto = EfiEbcVmTestProtocol {
        execute: Some(mock_vm_execute),
        assemble: None,
        disassemble: None,
    };

    let Some(mut vm_test) = (unsafe { EbcVmTest::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap vm test protocol");
    };

    let mut vm = EbcVmContext {
        gpr: [0; 8],
        flags: 0,
        ip: core::ptr::null_mut(),
        last_exception: 0,
        exception_flags: 0,
        stop_flags: 0,
        compiler_version: 1,
        high_stack_bottom: 0,
        low_stack_top: 0,
        stack_ret_addr: 0,
        stack_magic_ptr: core::ptr::null_mut(),
        image_handle: EfiHandle::NULL,
        system_table: core::ptr::null_mut(),
        last_addr_converted: 0,
        last_addr_converted_value: 0,
        frame_ptr: core::ptr::null_mut(),
        entry_point: core::ptr::null_mut(),
        image_base: 0,
        stack_pool: core::ptr::null_mut(),
        stack_top: core::ptr::null_mut(),
    };

    let mut count = 5;
    let res = vm_test.execute(&mut vm, &mut count);
    assert_eq!(res, Ok(()));
    assert_eq!(vm.gpr[0], 42);
    assert_eq!(count, 1);
}

unsafe extern "efiapi" fn mock_configure(
    _this: *mut EfiEbcSimpleDebuggerProtocol,
    config_id: u32,
    config_value: UIntN,
) -> EfiStatus {
    if config_id == 1 && config_value == 1 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

#[test]
fn test_ebc_debugger_wrapper() {
    let mut mock_proto = EfiEbcSimpleDebuggerProtocol {
        debugger: None,
        signal_exception: None,
        dasm: None,
        configure: Some(mock_configure),
    };

    let Some(mut dbg) = (unsafe { EbcSimpleDebugger::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap debugger protocol");
    };

    let res = dbg.configure(1, 1);
    assert_eq!(res, Ok(()));

    let res_invalid = dbg.configure(2, 0);
    assert_eq!(res_invalid, Err(EfiStatus::UNSUPPORTED));
}
