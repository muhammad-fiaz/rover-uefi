//! Unit and integration tests for Management Mode (MM / SMM) Dispatch protocols.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::firmware::mm_dispatch::{
    EFI_MM_GPI_DISPATCH_PROTOCOL_GUID, EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID,
    EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID, EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID,
    EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID, EFI_MM_SW_DISPATCH_PROTOCOL_GUID,
    EFI_MM_SX_DISPATCH_PROTOCOL_GUID, EFI_MM_USB_DISPATCH_PROTOCOL_GUID, MmGpiDispatch,
    MmGpiDispatchProtocol, MmGpiRegisterContext, MmIoTrapDispatch, MmIoTrapDispatchProtocol,
    MmIoTrapRegisterContext, MmIoTrapType, MmPeriodicTimerDispatch,
    MmPeriodicTimerDispatchProtocol, MmPeriodicTimerRegisterContext, MmPowerButtonDispatch,
    MmPowerButtonDispatchProtocol, MmPowerButtonRegisterContext, MmStandbyButtonDispatch,
    MmStandbyButtonDispatchProtocol, MmStandbyButtonRegisterContext, MmSwDispatch,
    MmSwDispatchProtocol, MmSwRegisterContext, MmSxDispatch, MmSxDispatchProtocol,
    MmSxRegisterContext, MmUsbDispatch, MmUsbDispatchProtocol, MmUsbRegisterContext,
    PowerButtonPhase, SleepPhase, SleepType, StandbyButtonPhase, UsbMmiType,
};
use rover_uefi::protocols::firmware::mm_system_table::EfiMmiHandler;

unsafe extern "efiapi" fn dummy_mmi_handler(
    _dispatch_handle: EfiHandle,
    _context: *const c_void,
    _comm_buffer: *mut c_void,
    _comm_buffer_size: *mut UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- SW Mock Functions ---
unsafe extern "efiapi" fn mock_sw_register(
    _this: *const MmSwDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *mut MmSwRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).sw_mmi_input_value, 0x55);
        *handle = EfiHandle::from_ptr(0x1000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sw_unregister(
    _this: *const MmSwDispatchProtocol,
    handle: EfiHandle,
) -> EfiStatus {
    assert_eq!(handle, EfiHandle::from_ptr(0x1000 as *mut c_void));
    EfiStatus::SUCCESS
}

// --- Sx Mock Functions ---
unsafe extern "efiapi" fn mock_sx_register(
    _this: *const MmSxDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmSxRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).sleep_type, SleepType::S3);
        assert_eq!((*context).phase, SleepPhase::Entry);
        *handle = EfiHandle::from_ptr(0x2000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sx_unregister(
    _this: *const MmSxDispatchProtocol,
    handle: EfiHandle,
) -> EfiStatus {
    assert_eq!(handle, EfiHandle::from_ptr(0x2000 as *mut c_void));
    EfiStatus::SUCCESS
}

// --- Periodic Timer Mock Functions ---
unsafe extern "efiapi" fn mock_timer_register(
    _this: *const MmPeriodicTimerDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmPeriodicTimerRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).period, 10_000_000);
        assert_eq!((*context).mmi_tick_interval, 1_000_000);
        *handle = EfiHandle::from_ptr(0x3000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_timer_unregister(
    _this: *const MmPeriodicTimerDispatchProtocol,
    handle: EfiHandle,
) -> EfiStatus {
    assert_eq!(handle, EfiHandle::from_ptr(0x3000 as *mut c_void));
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_timer_get_interval(
    _this: *const MmPeriodicTimerDispatchProtocol,
    _interval: *mut *mut u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- Power Button Mock Functions ---
unsafe extern "efiapi" fn mock_pwr_register(
    _this: *const MmPowerButtonDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmPowerButtonRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).phase, PowerButtonPhase::Entry);
        *handle = EfiHandle::from_ptr(0x4000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_pwr_unregister(
    _this: *const MmPowerButtonDispatchProtocol,
    _handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- Standby Button Mock Functions ---
unsafe extern "efiapi" fn mock_stb_register(
    _this: *const MmStandbyButtonDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmStandbyButtonRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).phase, StandbyButtonPhase::Exit);
        *handle = EfiHandle::from_ptr(0x5000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_stb_unregister(
    _this: *const MmStandbyButtonDispatchProtocol,
    _handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- I/O Trap Mock Functions ---
unsafe extern "efiapi" fn mock_io_register(
    _this: *const MmIoTrapDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *mut MmIoTrapRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).address, 0x80);
        assert_eq!((*context).length, 4);
        assert_eq!((*context).trap_type, MmIoTrapType::WriteTrap);
        *handle = EfiHandle::from_ptr(0x6000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_io_unregister(
    _this: *const MmIoTrapDispatchProtocol,
    _handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- GPI Mock Functions ---
unsafe extern "efiapi" fn mock_gpi_register(
    _this: *const MmGpiDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmGpiRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).gpi_num, 7);
        *handle = EfiHandle::from_ptr(0x7000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gpi_unregister(
    _this: *const MmGpiDispatchProtocol,
    _handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

// --- USB Mock Functions ---
unsafe extern "efiapi" fn mock_usb_register(
    _this: *const MmUsbDispatchProtocol,
    _handler: EfiMmiHandler,
    context: *const MmUsbRegisterContext,
    handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        assert_eq!((*context).mmi_type, UsbMmiType::UsbLegacy);
        *handle = EfiHandle::from_ptr(0x8000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_usb_unregister(
    _this: *const MmUsbDispatchProtocol,
    _handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_mm_sw_dispatch() {
    assert_eq!(MmSwDispatch::GUID, EFI_MM_SW_DISPATCH_PROTOCOL_GUID);

    let raw = MmSwDispatchProtocol {
        register: mock_sw_register,
        unregister: mock_sw_unregister,
        maximum_swi_value: 0xFF,
    };

    let client = MmSwDispatch::new(&raw);
    assert_eq!(client.maximum_swi_value(), 0xFF);

    let mut ctx = MmSwRegisterContext {
        sw_mmi_input_value: 0x55,
    };
    let handle = client
        .register(dummy_mmi_handler, &mut ctx)
        .expect("sw register");
    assert_eq!(handle, EfiHandle::from_ptr(0x1000 as *mut c_void));

    assert!(client.unregister(handle).is_ok());
}

#[test]
fn test_mm_sx_dispatch() {
    assert_eq!(MmSxDispatch::GUID, EFI_MM_SX_DISPATCH_PROTOCOL_GUID);

    let raw = MmSxDispatchProtocol {
        register: mock_sx_register,
        unregister: mock_sx_unregister,
    };

    let client = MmSxDispatch::new(&raw);
    let ctx = MmSxRegisterContext {
        sleep_type: SleepType::S3,
        phase: SleepPhase::Entry,
    };
    let handle = client
        .register(dummy_mmi_handler, &ctx)
        .expect("sx register");
    assert_eq!(handle, EfiHandle::from_ptr(0x2000 as *mut c_void));

    assert!(client.unregister(handle).is_ok());
}

#[test]
fn test_mm_periodic_timer_dispatch() {
    assert_eq!(
        MmPeriodicTimerDispatch::GUID,
        EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID
    );

    let raw = MmPeriodicTimerDispatchProtocol {
        register: mock_timer_register,
        unregister: mock_timer_unregister,
        get_next_mmi_interval: mock_timer_get_interval,
    };

    let client = MmPeriodicTimerDispatch::new(&raw);
    let ctx = MmPeriodicTimerRegisterContext {
        period: 10_000_000,
        mmi_tick_interval: 1_000_000,
    };
    let handle = client
        .register(dummy_mmi_handler, &ctx)
        .expect("periodic register");
    assert_eq!(handle, EfiHandle::from_ptr(0x3000 as *mut c_void));

    assert!(client.unregister(handle).is_ok());
}

#[test]
fn test_mm_power_and_standby_button_dispatch() {
    assert_eq!(
        MmPowerButtonDispatch::GUID,
        EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID
    );
    assert_eq!(
        MmStandbyButtonDispatch::GUID,
        EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID
    );

    let raw_pwr = MmPowerButtonDispatchProtocol {
        register: mock_pwr_register,
        unregister: mock_pwr_unregister,
    };
    let pwr = MmPowerButtonDispatch::new(&raw_pwr);
    let pwr_ctx = MmPowerButtonRegisterContext {
        phase: PowerButtonPhase::Entry,
    };
    assert!(pwr.register(dummy_mmi_handler, &pwr_ctx).is_ok());

    let raw_stb = MmStandbyButtonDispatchProtocol {
        register: mock_stb_register,
        unregister: mock_stb_unregister,
    };
    let stb = MmStandbyButtonDispatch::new(&raw_stb);
    let stb_ctx = MmStandbyButtonRegisterContext {
        phase: StandbyButtonPhase::Exit,
    };
    assert!(stb.register(dummy_mmi_handler, &stb_ctx).is_ok());
}

#[test]
fn test_mm_io_trap_and_gpi_and_usb_dispatch() {
    assert_eq!(
        MmIoTrapDispatch::GUID,
        EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID
    );
    assert_eq!(MmGpiDispatch::GUID, EFI_MM_GPI_DISPATCH_PROTOCOL_GUID);
    assert_eq!(MmUsbDispatch::GUID, EFI_MM_USB_DISPATCH_PROTOCOL_GUID);

    let raw_io = MmIoTrapDispatchProtocol {
        register: mock_io_register,
        unregister: mock_io_unregister,
    };
    let io = MmIoTrapDispatch::new(&raw_io);
    let mut io_ctx = MmIoTrapRegisterContext {
        address: 0x80,
        length: 4,
        trap_type: MmIoTrapType::WriteTrap,
    };
    assert!(io.register(dummy_mmi_handler, &mut io_ctx).is_ok());

    let raw_gpi = MmGpiDispatchProtocol {
        register: mock_gpi_register,
        unregister: mock_gpi_unregister,
        num_supported_gpis: 32,
    };
    let gpi = MmGpiDispatch::new(&raw_gpi);
    assert_eq!(gpi.num_supported_gpis(), 32);
    let gpi_ctx = MmGpiRegisterContext { gpi_num: 7 };
    assert!(gpi.register(dummy_mmi_handler, &gpi_ctx).is_ok());

    let raw_usb = MmUsbDispatchProtocol {
        register: mock_usb_register,
        unregister: mock_usb_unregister,
    };
    let usb = MmUsbDispatch::new(&raw_usb);
    let usb_ctx = MmUsbRegisterContext {
        mmi_type: UsbMmiType::UsbLegacy,
        device: core::ptr::null_mut(),
    };
    assert!(usb.register(dummy_mmi_handler, &usb_ctx).is_ok());
}
