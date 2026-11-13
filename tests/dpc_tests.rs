//! Integration tests for Deferred Procedure Call (DPC) Protocol.
#![allow(clippy::expect_used)]

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::dpc::{
    DPC_PROTOCOL_GUID, Dpc, DpcProcedure, DpcProtocol, TPL_APPLICATION, TPL_CALLBACK,
    TPL_HIGH_LEVEL, TPL_NOTIFY,
};

static mut QUEUED_TPL: usize = 0;
static mut DISPATCH_COUNT: usize = 0;

unsafe extern "efiapi" fn mock_queue_dpc(
    _this: *mut DpcProtocol,
    dpc_tpl: usize,
    _dpc_procedure: DpcProcedure,
    _dpc_context: *mut c_void,
) -> EfiStatus {
    unsafe {
        QUEUED_TPL = dpc_tpl;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_dispatch_dpc(_this: *mut DpcProtocol) -> EfiStatus {
    unsafe {
        DISPATCH_COUNT += 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_dpc_proc(_context: *mut c_void) {}

#[test]
fn test_dpc_guid_and_tpl_levels() {
    assert_eq!(
        DPC_PROTOCOL_GUID.to_string(),
        "480f8ae9-0c46-4aa9-bc89-db9fba619806"
    );

    assert_eq!(TPL_APPLICATION, 4);
    assert_eq!(TPL_CALLBACK, 8);
    assert_eq!(TPL_NOTIFY, 16);
    assert_eq!(TPL_HIGH_LEVEL, 31);
}

#[test]
fn test_dpc_wrapper() {
    let mut raw_proto = DpcProtocol {
        queue_dpc: mock_queue_dpc,
        dispatch_dpc: mock_dispatch_dpc,
    };

    let mut client = unsafe { Dpc::from_raw(&raw mut raw_proto) }.expect("valid Dpc instance");

    let queue_res = unsafe { client.queue(TPL_CALLBACK, dummy_dpc_proc, core::ptr::null_mut()) };
    assert!(queue_res.is_ok());
    assert_eq!(unsafe { QUEUED_TPL }, TPL_CALLBACK);

    let dispatch_res = client.dispatch();
    assert!(dispatch_res.is_ok());
    assert_eq!(unsafe { DISPATCH_COUNT }, 1);
}
