//! Integration tests for Android AVF, Boot Memory, Debug, Embedded GPIO,
//! Hardware Interrupt, FDT Client, MMC Host, ESRT Management, Progress, and LockBox protocols.
#![allow(
    clippy::expect_used,
    clippy::doc_markdown,
    clippy::cast_possible_truncation
)]

use core::ffi::c_void;
use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::android::avf::{
    Avf, GBL_EFI_AVF_PROTOCOL_GUID, GBL_EFI_AVF_PROTOCOL_REVISION, GblEfiAvfProtocol,
};
use rover_uefi::protocols::android::boot_memory::{
    BootMemory, GBL_EFI_BOOT_MEMORY_PROTOCOL_GUID, GBL_EFI_BOOT_MEMORY_PROTOCOL_REVISION,
    GblEfiBootBufferType, GblEfiBootMemoryProtocol,
};
use rover_uefi::protocols::android::debug::{
    GBL_EFI_DEBUG_PROTOCOL_GUID, GBL_EFI_DEBUG_PROTOCOL_REVISION, GblDebug, GblEfiDebugErrorTag,
    GblEfiDebugProtocol,
};
use rover_uefi::protocols::device_path::EfiDevicePath;
use rover_uefi::protocols::driver::fdt_client::{
    FDT_CLIENT_PROTOCOL_GUID, FdtClient, FdtClientProtocol,
};
use rover_uefi::protocols::driver::gpio::{
    EMBEDDED_GPIO_PROTOCOL_GUID, EmbeddedGpio, EmbeddedGpioProtocol, GpioMode, GpioPin, GpioPull,
    PLATFORM_GPIO_PROTOCOL_GUID,
};
use rover_uefi::protocols::driver::hardware_interrupt::{
    HARDWARE_INTERRUPT_PROTOCOL_GUID, HARDWARE_INTERRUPT2_PROTOCOL_GUID, HardwareInterrupt,
    HardwareInterrupt2, HardwareInterrupt2Protocol, HardwareInterruptHandler,
    HardwareInterruptProtocol, HardwareInterruptSource, HardwareInterruptTriggerType,
};
use rover_uefi::protocols::firmware::esrt::{
    ESRT_FW_TYPE_SYSTEMFIRMWARE, ESRT_MANAGEMENT_PROTOCOL_GUID, EfiSystemResourceEntry,
    EsrtManagement, EsrtManagementProtocol, LAST_ATTEMPT_STATUS_SUCCESS,
};
use rover_uefi::protocols::firmware::lock_box::{
    LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE, LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY,
    LOCK_BOX_PROTOCOL_GUID, LockBoxDescriptor, SMM_LOCK_BOX_COMMAND_SAVE,
    SMM_LOCK_BOX_COMMUNICATION_GUID, SmmLockBoxParameterHeader, SmmLockBoxParameterSave,
};
use rover_uefi::protocols::firmware::progress::{
    FIRMWARE_MANAGEMENT_PROGRESS_PROTOCOL_GUID, FIRMWARE_MANAGEMENT_PROGRESS_VERSION,
    FirmwareManagementProgress, FirmwareManagementProgressProtocol,
};
use rover_uefi::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;
use rover_uefi::protocols::storage::mmc_host::{
    EMBEDDED_MMC_HOST_PROTOCOL_GUID, MMC_CMD_WAIT_RESPONSE, MMC_CMD0, MMC_HOST_PROTOCOL_REVISION,
    MmcHost, MmcHostProtocol, MmcState, mmc_cmd,
};

// -----------------------------------------------------------------------------
// Android AVF Tests
// -----------------------------------------------------------------------------

unsafe extern "efiapi" fn mock_dice_handover(
    _this: *mut GblEfiAvfProtocol,
    size: *mut usize,
    buffer: *mut u8,
) -> EfiStatus {
    if size.is_null() || buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *size = 8;
        core::ptr::copy_nonoverlapping(b"DICE1234".as_ptr(), buffer, 8);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_secret_keeper(
    _this: *mut GblEfiAvfProtocol,
    size: *mut usize,
    buffer: *mut u8,
) -> EfiStatus {
    if size.is_null() || buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *size = 4;
        core::ptr::copy_nonoverlapping(b"SECR".as_ptr(), buffer, 4);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_android_avf() {
    assert_eq!(
        GBL_EFI_AVF_PROTOCOL_GUID,
        EfiGuid::new(
            0xE7F1_C4A6,
            0x0A52,
            0x4F61,
            [0xBD, 0x98, 0x9E, 0x60, 0xB5, 0x59, 0x45, 0x2A],
        )
    );

    let mut raw = GblEfiAvfProtocol {
        revision: GBL_EFI_AVF_PROTOCOL_REVISION,
        read_vendor_dice_handover: mock_dice_handover,
        read_secret_keeper_public_key: mock_secret_keeper,
    };

    let mut avf = Avf::new(&mut raw);
    assert_eq!(avf.revision(), GBL_EFI_AVF_PROTOCOL_REVISION);

    let mut buf = [0u8; 16];
    let dice_len = avf.read_vendor_dice_handover(&mut buf).expect("dice read");
    assert_eq!(dice_len, 8);
    assert_eq!(&buf[..8], b"DICE1234");

    let mut sec_buf = [0u8; 8];
    let sec_len = avf
        .read_secret_keeper_public_key(&mut sec_buf)
        .expect("sec read");
    assert_eq!(sec_len, 4);
    assert_eq!(&sec_buf[..4], b"SECR");
}

// -----------------------------------------------------------------------------
// Android Boot Memory Tests
// -----------------------------------------------------------------------------

static mut STATIC_BUFFER: [u8; 128] = [0x5A; 128];

unsafe extern "efiapi" fn mock_get_partition_buffer(
    _this: *mut GblEfiBootMemoryProtocol,
    _name: *const u8,
    size: *mut usize,
    addr: *mut *mut c_void,
    flags: *mut u32,
) -> EfiStatus {
    unsafe {
        *size = 128;
        *addr = (&raw mut STATIC_BUFFER).cast();
        *flags = 0x1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sync_partition_buffer(
    _this: *mut GblEfiBootMemoryProtocol,
    _sync_preloaded: Bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_boot_buffer(
    _this: *mut GblEfiBootMemoryProtocol,
    _buf_type: u32,
    size: *mut usize,
    addr: *mut *mut c_void,
) -> EfiStatus {
    unsafe {
        *size = 64;
        *addr = (&raw mut STATIC_BUFFER).cast();
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_android_boot_memory() {
    assert_eq!(
        GBL_EFI_BOOT_MEMORY_PROTOCOL_GUID,
        EfiGuid::new(
            0x309F_2874,
            0xAD59,
            0x4FD2,
            [0xAF, 0x5E, 0xCE, 0x0F, 0x4A, 0xB4, 0x01, 0xA6],
        )
    );

    let mut raw = GblEfiBootMemoryProtocol {
        revision: GBL_EFI_BOOT_MEMORY_PROTOCOL_REVISION,
        get_partition_buffer: mock_get_partition_buffer,
        sync_partition_buffer: mock_sync_partition_buffer,
        get_boot_buffer: mock_get_boot_buffer,
    };

    let mut bm = BootMemory::new(&mut raw);
    assert_eq!(bm.revision(), GBL_EFI_BOOT_MEMORY_PROTOCOL_REVISION);

    let (ptr, sz, flags) = bm.get_partition_buffer(b"boot_a\0").expect("get part");
    assert!(!ptr.is_null());
    assert_eq!(sz, 128);
    assert_eq!(flags, 1);

    bm.sync_partition_buffer(true).expect("sync buffer");

    let (b_ptr, b_sz) = bm
        .get_boot_buffer(GblEfiBootBufferType::Kernel)
        .expect("get kernel buf");
    assert!(!b_ptr.is_null());
    assert_eq!(b_sz, 64);
}

// -----------------------------------------------------------------------------
// Android Debug Protocol Tests
// -----------------------------------------------------------------------------

static mut LAST_ERROR_TAG: u64 = 999;

unsafe extern "efiapi" fn mock_fatal_error(
    _this: *mut GblEfiDebugProtocol,
    _frame: *const c_void,
    tag: u64,
) -> EfiStatus {
    unsafe {
        LAST_ERROR_TAG = tag;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_android_debug() {
    assert_eq!(
        GBL_EFI_DEBUG_PROTOCOL_GUID,
        EfiGuid::new(
            0x98CA_3DA1,
            0xC1AC,
            0x4402,
            [0x9C, 0x16, 0x75, 0x58, 0xD3, 0xED, 0x57, 0x05],
        )
    );

    let mut raw = GblEfiDebugProtocol {
        revision: GBL_EFI_DEBUG_PROTOCOL_REVISION,
        fatal_error: mock_fatal_error,
    };

    let mut dbg = GblDebug::new(&mut raw);
    assert_eq!(dbg.revision(), GBL_EFI_DEBUG_PROTOCOL_REVISION);

    let status = dbg.fatal_error(core::ptr::null(), GblEfiDebugErrorTag::AssertionError);
    assert!(status.is_success());
    assert_eq!(unsafe { LAST_ERROR_TAG }, 0);

    let status2 = dbg.fatal_error(core::ptr::null(), GblEfiDebugErrorTag::BootError);
    assert!(status2.is_success());
    assert_eq!(unsafe { LAST_ERROR_TAG }, 3);
}

// -----------------------------------------------------------------------------
// Embedded GPIO Tests
// -----------------------------------------------------------------------------

static mut GPIO_PIN_STATE: usize = 0;
static mut GPIO_PIN_MODE: GpioMode = GpioMode::Input;
static mut GPIO_PIN_PULL: GpioPull = GpioPull::None;

unsafe extern "efiapi" fn mock_gpio_get(
    _this: *mut EmbeddedGpioProtocol,
    _pin: GpioPin,
    value: *mut usize,
) -> EfiStatus {
    unsafe {
        *value = GPIO_PIN_STATE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gpio_set(
    _this: *mut EmbeddedGpioProtocol,
    _pin: GpioPin,
    mode: GpioMode,
) -> EfiStatus {
    unsafe {
        GPIO_PIN_MODE = mode;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gpio_get_mode(
    _this: *mut EmbeddedGpioProtocol,
    _pin: GpioPin,
    mode: *mut GpioMode,
) -> EfiStatus {
    unsafe {
        *mode = GPIO_PIN_MODE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gpio_set_pull(
    _this: *mut EmbeddedGpioProtocol,
    _pin: GpioPin,
    pull: GpioPull,
) -> EfiStatus {
    unsafe {
        GPIO_PIN_PULL = pull;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_embedded_gpio() {
    assert_eq!(
        EMBEDDED_GPIO_PROTOCOL_GUID,
        EfiGuid::new(
            0x17A0_A3D7,
            0xC0A5,
            0x4635,
            [0xBB, 0xD5, 0x07, 0x21, 0x87, 0xDF, 0xE2, 0xEE],
        )
    );
    assert_eq!(
        PLATFORM_GPIO_PROTOCOL_GUID,
        EfiGuid::new(
            0x52CE_9845,
            0x5AF4,
            0x43E2,
            [0xBA, 0xFD, 0x23, 0x08, 0x12, 0x54, 0x7A, 0xC2],
        )
    );

    let mut raw = EmbeddedGpioProtocol {
        get: mock_gpio_get,
        set: mock_gpio_set,
        get_mode: mock_gpio_get_mode,
        set_pull: mock_gpio_set_pull,
    };

    let pin = GpioPin::new(0, 42);
    let mut gpio = EmbeddedGpio::new(&mut raw);
    unsafe {
        GPIO_PIN_STATE = 1;
    }
    assert_eq!(gpio.get(pin).expect("gpio get"), 1);

    gpio.set(pin, GpioMode::Output1).expect("gpio set");
    assert_eq!(
        gpio.get_mode(pin).expect("gpio get_mode"),
        GpioMode::Output1
    );

    gpio.set_pull(pin, GpioPull::PullUp).expect("gpio set_pull");
    assert_eq!(unsafe { GPIO_PIN_PULL }, GpioPull::PullUp);
}

// -----------------------------------------------------------------------------
// Hardware Interrupt Tests
// -----------------------------------------------------------------------------

static mut IRQ_UNMASKED: bool = false;
static mut IRQ_TRIGGER: HardwareInterruptTriggerType = HardwareInterruptTriggerType::LevelHigh;

unsafe extern "efiapi" fn mock_irq_register(
    _this: *mut HardwareInterruptProtocol,
    _source: HardwareInterruptSource,
    _handler: Option<HardwareInterruptHandler>,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq_enable(
    _this: *mut HardwareInterruptProtocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    unsafe {
        IRQ_UNMASKED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq_disable(
    _this: *mut HardwareInterruptProtocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    unsafe {
        IRQ_UNMASKED = false;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq_get_state(
    _this: *mut HardwareInterruptProtocol,
    _source: HardwareInterruptSource,
    state: *mut Bool,
) -> EfiStatus {
    unsafe {
        *state = u8::from(IRQ_UNMASKED);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq_eoi(
    _this: *mut HardwareInterruptProtocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_register(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
    _handler: Option<HardwareInterruptHandler>,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_enable(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_disable(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_get_state(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
    state: *mut Bool,
) -> EfiStatus {
    unsafe {
        *state = 1;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_eoi(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_get_trigger(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
    trigger: *mut HardwareInterruptTriggerType,
) -> EfiStatus {
    unsafe {
        *trigger = IRQ_TRIGGER;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_irq2_set_trigger(
    _this: *mut HardwareInterrupt2Protocol,
    _source: HardwareInterruptSource,
    trigger: HardwareInterruptTriggerType,
) -> EfiStatus {
    unsafe {
        IRQ_TRIGGER = trigger;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_hardware_interrupt() {
    assert_eq!(
        HARDWARE_INTERRUPT_PROTOCOL_GUID,
        EfiGuid::new(
            0x2890_B3EA,
            0x053D,
            0x1643,
            [0xAD, 0x0C, 0xD6, 0x48, 0x08, 0xDA, 0x3F, 0xF1],
        )
    );
    assert_eq!(
        HARDWARE_INTERRUPT2_PROTOCOL_GUID,
        EfiGuid::new(
            0x3289_8322,
            0x2DA1,
            0x474A,
            [0xBA, 0xAA, 0xF3, 0xF7, 0xCF, 0x56, 0x94, 0x70],
        )
    );

    let mut raw = HardwareInterruptProtocol {
        register_interrupt_source: mock_irq_register,
        enable_interrupt_source: mock_irq_enable,
        disable_interrupt_source: mock_irq_disable,
        get_interrupt_source_state: mock_irq_get_state,
        end_of_interrupt: mock_irq_eoi,
    };

    let mut irq = HardwareInterrupt::new(&mut raw);
    irq.enable(33).expect("enable irq");
    assert!(irq.is_enabled(33).expect("is_enabled"));
    irq.disable(33).expect("disable irq");
    assert!(!irq.is_enabled(33).expect("is_enabled"));

    let mut raw2 = HardwareInterrupt2Protocol {
        register_interrupt_source: mock_irq2_register,
        enable_interrupt_source: mock_irq2_enable,
        disable_interrupt_source: mock_irq2_disable,
        get_interrupt_source_state: mock_irq2_get_state,
        end_of_interrupt: mock_irq2_eoi,
        get_trigger_type: mock_irq2_get_trigger,
        set_trigger_type: mock_irq2_set_trigger,
    };

    let mut irq2 = HardwareInterrupt2::new(&mut raw2);
    irq2.set_trigger_type(33, HardwareInterruptTriggerType::EdgeRising)
        .expect("set trigger");
    assert_eq!(
        irq2.get_trigger_type(33).expect("get trigger"),
        HardwareInterruptTriggerType::EdgeRising
    );
}

// -----------------------------------------------------------------------------
// FDT Client Tests
// -----------------------------------------------------------------------------

static FDT_SAMPLE_PROP: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

unsafe extern "efiapi" fn mock_fdt_get_prop(
    _this: *mut FdtClientProtocol,
    _node: i32,
    _name: *const u8,
    prop: *mut *const c_void,
    size: *mut u32,
) -> EfiStatus {
    unsafe {
        *prop = FDT_SAMPLE_PROP.as_ptr().cast();
        *size = 4;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fdt_set_prop(
    _this: *mut FdtClientProtocol,
    _node: i32,
    _name: *const u8,
    _val: *const c_void,
    _len: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fdt_find_node(
    _this: *mut FdtClientProtocol,
    _compat: *const u8,
    node: *mut i32,
) -> EfiStatus {
    unsafe {
        *node = 0x42;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fdt_find_next_node(
    _this: *mut FdtClientProtocol,
    _compat: *const u8,
    _prev: i32,
    _node: *mut i32,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

unsafe extern "efiapi" fn mock_fdt_chosen_insert(
    _this: *mut FdtClientProtocol,
    node: *mut i32,
) -> EfiStatus {
    unsafe {
        *node = 0x10;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fdt_find_node_prop(
    _this: *mut FdtClientProtocol,
    _compat: *const u8,
    _name: *const u8,
    _prop: *mut *const c_void,
    _size: *mut u32,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_fdt_find_node_reg(
    _this: *mut FdtClientProtocol,
    _compat: *const u8,
    _reg: *mut *const c_void,
    _addr_cells: *mut usize,
    _size_cells: *mut usize,
    _reg_size: *mut u32,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_fdt_find_mem_reg(
    _this: *mut FdtClientProtocol,
    _node: *mut i32,
    _reg: *mut *const c_void,
    _addr_cells: *mut usize,
    _size_cells: *mut usize,
    _reg_size: *mut u32,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_fdt_find_next_mem_reg(
    _this: *mut FdtClientProtocol,
    _prev: i32,
    _node: *mut i32,
    _reg: *mut *const c_void,
    _addr_cells: *mut usize,
    _size_cells: *mut usize,
    _reg_size: *mut u32,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

#[test]
fn test_fdt_client() {
    assert_eq!(
        FDT_CLIENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xE11F_ACA0,
            0x4710,
            0x4C8E,
            [0xA7, 0xA2, 0x01, 0xBA, 0xA2, 0x59, 0x1B, 0x4C],
        )
    );

    let mut raw = FdtClientProtocol {
        get_node_property: mock_fdt_get_prop,
        set_node_property: mock_fdt_set_prop,
        find_compatible_node: mock_fdt_find_node,
        find_next_compatible_node: mock_fdt_find_next_node,
        find_compatible_node_property: mock_fdt_find_node_prop,
        find_compatible_node_reg: mock_fdt_find_node_reg,
        find_memory_node_reg: mock_fdt_find_mem_reg,
        find_next_memory_node_reg: mock_fdt_find_next_mem_reg,
        get_or_insert_chosen_node: mock_fdt_chosen_insert,
    };

    let mut fdt = FdtClient::new(&mut raw);
    let prop = fdt
        .get_node_property(0x42, b"clock-frequency\0")
        .expect("get prop");
    assert_eq!(prop, &[0x12, 0x34, 0x56, 0x78]);

    let node = fdt.find_compatible_node(b"arm,pl011\0").expect("find node");
    assert_eq!(node, 0x42);

    let chosen = fdt.get_or_insert_chosen_node().expect("chosen");
    assert_eq!(chosen, 0x10);
}

// -----------------------------------------------------------------------------
// MMC Host Tests
// -----------------------------------------------------------------------------

unsafe extern "efiapi" fn mock_mmc_present(_this: *mut MmcHostProtocol) -> bool {
    true
}

unsafe extern "efiapi" fn mock_mmc_readonly(_this: *mut MmcHostProtocol) -> bool {
    false
}

unsafe extern "efiapi" fn mock_mmc_build_path(
    _this: *mut MmcHostProtocol,
    _path: *mut *mut EfiDevicePath,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_notify(
    _this: *mut MmcHostProtocol,
    _state: MmcState,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_send(
    _this: *mut MmcHostProtocol,
    _cmd: u32,
    _arg: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_recv(
    _this: *mut MmcHostProtocol,
    _resp: u32,
    _buf: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_read(
    _this: *mut MmcHostProtocol,
    _lba: u64,
    _len: usize,
    _buf: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_write(
    _this: *mut MmcHostProtocol,
    _lba: u64,
    _len: usize,
    _buf: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_set_ios(
    _this: *mut MmcHostProtocol,
    _freq: u32,
    _width: u32,
    _timing: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_mmc_multi(_this: *mut MmcHostProtocol) -> bool {
    true
}

#[test]
fn test_mmc_host() {
    assert_eq!(
        EMBEDDED_MMC_HOST_PROTOCOL_GUID,
        EfiGuid::new(
            0x3E59_1C00,
            0x9E4A,
            0x11DF,
            [0x92, 0x44, 0x00, 0x02, 0xA5, 0xD5, 0xC5, 0x1B],
        )
    );
    assert_eq!(MMC_HOST_PROTOCOL_REVISION, 0x0001_0002);
    assert_eq!(MMC_CMD0, mmc_cmd(0, 1 << 18));
    assert_ne!(MMC_CMD_WAIT_RESPONSE, 0);

    let mut raw = MmcHostProtocol {
        revision: MMC_HOST_PROTOCOL_REVISION,
        is_card_present: mock_mmc_present,
        is_read_only: mock_mmc_readonly,
        build_device_path: mock_mmc_build_path,
        notify_state: mock_mmc_notify,
        send_command: mock_mmc_send,
        receive_response: mock_mmc_recv,
        read_block_data: mock_mmc_read,
        write_block_data: mock_mmc_write,
        set_ios: Some(mock_mmc_set_ios),
        is_multi_block: Some(mock_mmc_multi),
    };

    let mut host = MmcHost::new(&mut raw);
    assert_eq!(host.revision(), MMC_HOST_PROTOCOL_REVISION);
    assert!(host.is_card_present());
    assert!(!host.is_read_only());
    assert!(host.is_multi_block());
    host.notify_state(MmcState::Transfer).expect("notify state");
    host.send_command(MMC_CMD0, 0).expect("cmd0");
}

// -----------------------------------------------------------------------------
// ESRT Management Tests
// -----------------------------------------------------------------------------

static SAMPLE_CLASS: EfiGuid = EfiGuid::new(
    0x1122_3344,
    0x5566,
    0x7788,
    [0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00],
);

unsafe extern "efiapi" fn mock_esrt_get(
    guid: *const EfiGuid,
    entry: *mut EfiSystemResourceEntry,
) -> EfiStatus {
    unsafe {
        (*entry).fw_class = *guid;
        (*entry).fw_type = ESRT_FW_TYPE_SYSTEMFIRMWARE;
        (*entry).fw_version = 0x0002_0001;
        (*entry).lowest_supported_fw_version = 0x0001_0000;
        (*entry).capsule_flags = 0;
        (*entry).last_attempt_version = 0x0002_0001;
        (*entry).last_attempt_status = LAST_ATTEMPT_STATUS_SUCCESS;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_esrt_update(_entry: *const EfiSystemResourceEntry) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_esrt_register(_entry: *const EfiSystemResourceEntry) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_esrt_unregister(_guid: *const EfiGuid) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_esrt_sync() -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_esrt_lock() -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_esrt_management() {
    assert_eq!(
        ESRT_MANAGEMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xA340_C064,
            0x723C,
            0x4A9C,
            [0xA4, 0xDD, 0xD5, 0xB4, 0x7A, 0x26, 0xFB, 0xB0],
        )
    );

    let mut raw = EsrtManagementProtocol {
        get_esrt_entry: mock_esrt_get,
        update_esrt_entry: mock_esrt_update,
        register_esrt_entry: mock_esrt_register,
        unregister_esrt_entry: mock_esrt_unregister,
        sync_esrt_fmp: mock_esrt_sync,
        lock_esrt_repository: mock_esrt_lock,
    };

    let mut esrt = EsrtManagement::new(&mut raw);
    let entry = esrt.get_entry(&SAMPLE_CLASS).expect("get esrt entry");
    assert_eq!(entry.fw_class, SAMPLE_CLASS);
    assert_eq!(entry.fw_type, ESRT_FW_TYPE_SYSTEMFIRMWARE);
    assert_eq!(entry.fw_version, 0x0002_0001);

    esrt.update_entry(&entry).expect("update");
    esrt.register_entry(&entry).expect("register");
    esrt.unregister_entry(&SAMPLE_CLASS).expect("unregister");
    esrt.sync_fmp().expect("sync");
    esrt.lock_repository().expect("lock");
}

// -----------------------------------------------------------------------------
// Firmware Management Progress Tests
// -----------------------------------------------------------------------------

#[test]
fn test_firmware_progress() {
    assert_eq!(
        FIRMWARE_MANAGEMENT_PROGRESS_PROTOCOL_GUID,
        EfiGuid::new(
            0x1849_BDA2,
            0x6952,
            0x4E86,
            [0xA1, 0xDB, 0x55, 0x9A, 0x3C, 0x47, 0x9D, 0xF1],
        )
    );

    let mut proto = FirmwareManagementProgressProtocol::default();
    assert_eq!(proto.version, FIRMWARE_MANAGEMENT_PROGRESS_VERSION);

    let mut client = FirmwareManagementProgress::new(&mut proto);
    assert_eq!(client.version(), 1);
    assert_eq!(client.watchdog_seconds(), 0);

    let cyan = EfiGraphicsOutputBltPixel {
        blue: 0xFF,
        green: 0xFF,
        red: 0x00,
        reserved: 0x00,
    };
    client.set_foreground_color(cyan);
    client.set_watchdog_seconds(60);

    assert_eq!(client.foreground_color(), cyan);
    assert_eq!(client.watchdog_seconds(), 60);
}

// -----------------------------------------------------------------------------
// LockBox Tests
// -----------------------------------------------------------------------------

#[test]
fn test_lock_box() {
    assert_eq!(
        LOCK_BOX_PROTOCOL_GUID,
        EfiGuid::new(
            0xBD44_5D79,
            0xB7AD,
            0x4F04,
            [0x9A, 0xD8, 0x29, 0xBD, 0x20, 0x40, 0xEB, 0x3C],
        )
    );
    assert_eq!(
        SMM_LOCK_BOX_COMMUNICATION_GUID,
        EfiGuid::new(
            0x2A3C_FEBD,
            0x27E8,
            0x4D0A,
            [0x8B, 0x79, 0xD6, 0x88, 0xC2, 0xA3, 0xE1, 0xC0],
        )
    );

    let desc = LockBoxDescriptor::new(
        SAMPLE_CLASS,
        0x1000_0000,
        4096,
        LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE,
    )
    .expect("valid lockbox descriptor");
    assert!(desc.is_restore_in_place());
    assert!(!desc.is_s3_only());

    // Mutual exclusivity check
    let invalid = LockBoxDescriptor::new(
        SAMPLE_CLASS,
        0x1000_0000,
        4096,
        LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE | LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY,
    );
    assert_eq!(invalid, Err(EfiStatus::INVALID_PARAMETER));

    let param = SmmLockBoxParameterSave {
        header: SmmLockBoxParameterHeader {
            command: SMM_LOCK_BOX_COMMAND_SAVE,
            data_length: core::mem::size_of::<SmmLockBoxParameterSave>() as u32,
            return_status: 0,
        },
        guid: SAMPLE_CLASS,
        buffer: 0x2000_0000,
        length: 512,
    };
    assert_eq!(param.header.command, 1);
    assert_eq!(param.length, 512);
}
