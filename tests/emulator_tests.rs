//! Unit tests for host emulator protocol interfaces.

use rover_uefi::base::{EfiStatus, EfiTime};
use rover_uefi::protocols::driver::emulator::{
    EMU_BLOCK_IO_PROTOCOL_GUID, EMU_GRAPHICS_WINDOW_PROTOCOL_GUID, EMU_THREAD_THUNK_PROTOCOL_GUID,
    EMU_THUNK_PROTOCOL_GUID, EmuBlockIo, EmuBlockIoProtocol, EmuGraphicsWindow,
    EmuGraphicsWindowProtocol, EmuHostMutex, EmuThreadEntry, EmuThreadThunk,
    EmuThreadThunkProtocol, EmuThunk, EmuThunkProtocol,
};

#[test]
fn test_emulator_guids() {
    assert_eq!(
        EMU_THUNK_PROTOCOL_GUID.to_string(),
        "5cf32e0b-8edf-2e44-9cda-93205e99ec1c"
    );
    assert_eq!(
        EMU_GRAPHICS_WINDOW_PROTOCOL_GUID.to_string(),
        "30fd316a-6728-2e41-a690-0d1333d8cac1"
    );
    assert_eq!(
        EMU_THREAD_THUNK_PROTOCOL_GUID.to_string(),
        "3b1e4b7c-09d8-944f-a408-1309eb8b4427"
    );
    assert_eq!(
        EMU_BLOCK_IO_PROTOCOL_GUID.to_string(),
        "6888a4ae-afce-e84b-9102-f7b9dae6a030"
    );
}

// Mock EmuThreadThunk
static mut MOCK_MUTEX: u32 = 0xABCD;

unsafe extern "efiapi" fn mock_mutex_lock(mutex: EmuHostMutex) -> usize {
    assert!(!mutex.is_null());
    0
}

unsafe extern "efiapi" fn mock_mutex_unlock(mutex: EmuHostMutex) -> usize {
    assert!(!mutex.is_null());
    0
}

unsafe extern "efiapi" fn mock_mutex_try_lock(mutex: EmuHostMutex) -> usize {
    assert!(!mutex.is_null());
    0
}

unsafe extern "efiapi" fn mock_mutex_init() -> EmuHostMutex {
    (&raw mut MOCK_MUTEX).cast()
}

unsafe extern "efiapi" fn mock_mutex_destroy(_mutex: EmuHostMutex) -> usize {
    0
}

unsafe extern "efiapi" fn mock_create_thread(
    _thread: *mut core::ffi::c_void,
    _attribute: *mut core::ffi::c_void,
    _start: EmuThreadEntry,
    _context: *mut core::ffi::c_void,
) -> usize {
    0
}

unsafe extern "efiapi" fn mock_exit_thread(_value: *mut core::ffi::c_void) {}

unsafe extern "efiapi" fn mock_self_fn() -> usize {
    42
}

#[test]
fn test_emu_thread_thunk_wrapper() {
    let proto = EmuThreadThunkProtocol {
        mutex_lock: mock_mutex_lock,
        mutex_unlock: mock_mutex_unlock,
        mutex_try_lock: mock_mutex_try_lock,
        mutex_init: mock_mutex_init,
        mutex_destroy: mock_mutex_destroy,
        create_thread: mock_create_thread,
        exit_thread: mock_exit_thread,
        self_fn: mock_self_fn,
    };

    let thunk = EmuThreadThunk::new(&proto);
    let m = thunk.mutex_init();
    assert_eq!(thunk.mutex_lock(m), 0);
    assert_eq!(thunk.mutex_try_lock(m), 0);
    assert_eq!(thunk.mutex_unlock(m), 0);
    assert_eq!(thunk.mutex_destroy(m), 0);
    assert_eq!(thunk.thread_id(), 42);
}

// Mock EmuGraphicsWindow
unsafe extern "efiapi" fn mock_gw_close(_this: *mut EmuGraphicsWindowProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gw_size(
    _this: *mut EmuGraphicsWindowProtocol,
    width: u32,
    height: u32,
) -> EfiStatus {
    assert_eq!(width, 1024);
    assert_eq!(height, 768);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gw_check_key(_this: *mut EmuGraphicsWindowProtocol) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gw_get_key(
    _this: *mut EmuGraphicsWindowProtocol,
    _key: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gw_key_set_state(
    _this: *mut EmuGraphicsWindowProtocol,
    _state: *mut u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_gw_register_notify(
    _context: *mut core::ffi::c_void,
    _key: *mut core::ffi::c_void,
) {
}

#[test]
fn test_emu_graphics_window_wrapper() {
    let mut proto = EmuGraphicsWindowProtocol {
        close: mock_gw_close,
        size: mock_gw_size,
        check_key: mock_gw_check_key,
        get_key: mock_gw_get_key,
        key_set_state: mock_gw_key_set_state,
        register_key_notify_callback: mock_gw_register_notify,
    };

    let mut gw = EmuGraphicsWindow::new(&mut proto);
    assert!(gw.resize(1024, 768).is_ok());
    assert!(gw.has_key());
    assert!(gw.close().is_ok());
}

// Mock EmuBlockIo
unsafe extern "efiapi" fn mock_bio_reset(_this: *mut EmuBlockIoProtocol, _ext: bool) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bio_read(
    _this: *mut EmuBlockIoProtocol,
    media_id: u32,
    lba: u64,
    _token: *mut core::ffi::c_void,
    buffer_size: usize,
    buffer: *mut core::ffi::c_void,
) -> EfiStatus {
    assert_eq!(media_id, 1);
    assert_eq!(lba, 0);
    assert_eq!(buffer_size, 512);
    unsafe {
        core::ptr::write_bytes(buffer.cast::<u8>(), 0xAA, buffer_size);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bio_write(
    _this: *mut EmuBlockIoProtocol,
    media_id: u32,
    lba: u64,
    _token: *mut core::ffi::c_void,
    buffer_size: usize,
    _buffer: *const core::ffi::c_void,
) -> EfiStatus {
    assert_eq!(media_id, 1);
    assert_eq!(lba, 0);
    assert_eq!(buffer_size, 512);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bio_flush(
    _this: *mut EmuBlockIoProtocol,
    _token: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_bio_create_mapping(
    _this: *mut EmuBlockIoProtocol,
    _media: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_emu_block_io_wrapper() {
    let mut proto = EmuBlockIoProtocol {
        reset: mock_bio_reset,
        read_blocks: mock_bio_read,
        write_blocks: mock_bio_write,
        flush_blocks: mock_bio_flush,
        create_mapping: mock_bio_create_mapping,
    };

    let mut bio = EmuBlockIo::new(&mut proto);
    assert!(bio.reset(false).is_ok());

    let mut read_buf = [0u8; 512];
    assert!(bio.read_blocks(1, 0, &mut read_buf).is_ok());
    assert_eq!(read_buf[0], 0xAA);
    assert_eq!(read_buf[511], 0xAA);

    let write_buf = [0x55u8; 512];
    assert!(bio.write_blocks(1, 0, &write_buf).is_ok());
    assert!(bio.flush().is_ok());
}

// Mock EmuThunk
unsafe extern "efiapi" fn mock_thunk_write_std_err(_buf: *const u8, count: usize) -> usize {
    count
}
unsafe extern "efiapi" fn mock_thunk_config_std_in() -> EfiStatus {
    EfiStatus::SUCCESS
}
unsafe extern "efiapi" fn mock_thunk_write_std_out(_buf: *const u8, count: usize) -> usize {
    count
}
unsafe extern "efiapi" fn mock_thunk_read_std_in(_buf: *mut u8, count: usize) -> usize {
    count
}
unsafe extern "efiapi" fn mock_thunk_poll_std_in() -> bool {
    false
}
unsafe extern "efiapi" fn mock_thunk_malloc(size: usize) -> *mut core::ffi::c_void {
    size as *mut _
}
unsafe extern "efiapi" fn mock_thunk_valloc(size: usize) -> *mut core::ffi::c_void {
    size as *mut _
}
unsafe extern "efiapi" fn mock_thunk_free(_ptr: *mut core::ffi::c_void) -> bool {
    true
}
unsafe extern "efiapi" fn mock_thunk_entry(
    _data: *mut core::ffi::c_void,
    _ep: *mut *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}
unsafe extern "efiapi" fn mock_thunk_relocate(_ctx: *mut core::ffi::c_void) {}
unsafe extern "efiapi" fn mock_thunk_unload(_ctx: *mut core::ffi::c_void) {}
unsafe extern "efiapi" fn mock_thunk_enable_irq() {}
unsafe extern "efiapi" fn mock_thunk_disable_irq() {}
unsafe extern "efiapi" fn mock_thunk_perf_freq() -> u64 {
    1_000_000_000
}
unsafe extern "efiapi" fn mock_thunk_perf_cnt() -> u64 {
    5000
}
unsafe extern "efiapi" fn mock_thunk_sleep(_ns: u64) {}
unsafe extern "efiapi" fn mock_thunk_cpu_sleep() {}
unsafe extern "efiapi" fn mock_thunk_exit(_code: usize) {}
unsafe extern "efiapi" fn mock_thunk_get_time(
    time: *mut EfiTime,
    _caps: *mut rover_uefi::base::EfiTimeCapabilities,
) {
    if !time.is_null() {
        unsafe {
            (*time).year = 2026;
            (*time).month = 11;
            (*time).day = 13;
        }
    }
}
unsafe extern "efiapi" fn mock_thunk_set_time(_time: *mut EfiTime) -> EfiStatus {
    EfiStatus::SUCCESS
}
unsafe extern "efiapi" fn mock_thunk_set_timer(_p: u64, _cb: unsafe extern "efiapi" fn(u64)) {}
unsafe extern "efiapi" fn mock_thunk_next_proto(
    _bus: bool,
    _inst: *mut *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

#[test]
fn test_emu_thunk_wrapper() {
    let proto = EmuThunkProtocol {
        write_std_err: mock_thunk_write_std_err,
        config_std_in: mock_thunk_config_std_in,
        write_std_out: mock_thunk_write_std_out,
        read_std_in: mock_thunk_read_std_in,
        poll_std_in: mock_thunk_poll_std_in,
        malloc: mock_thunk_malloc,
        valloc: mock_thunk_valloc,
        free: mock_thunk_free,
        pe_coff_get_entry_point: mock_thunk_entry,
        pe_coff_relocate_extra_action: mock_thunk_relocate,
        pe_coff_unload_extra_action: mock_thunk_unload,
        enable_interrupt: mock_thunk_enable_irq,
        disable_interrupt: mock_thunk_disable_irq,
        query_performance_frequency: mock_thunk_perf_freq,
        query_performance_counter: mock_thunk_perf_cnt,
        sleep: mock_thunk_sleep,
        cpu_sleep: mock_thunk_cpu_sleep,
        exit: mock_thunk_exit,
        get_time: mock_thunk_get_time,
        set_time: mock_thunk_set_time,
        set_timer: mock_thunk_set_timer,
        get_next_protocol: mock_thunk_next_proto,
    };

    let thunk = EmuThunk::new(&proto);
    assert_eq!(thunk.performance_frequency(), 1_000_000_000);
    assert_eq!(thunk.performance_counter(), 5000);
    thunk.sleep(100);
    let time = thunk.get_time();
    assert_eq!(time.year, 2026);
    assert_eq!(time.month, 11);
    assert_eq!(time.day, 13);
}
