//! # Host Emulator Protocol Interfaces
//!
//! Provides protocol abstractions for host-based firmware execution,
//! virtual platform emulators, OS thunk services, and synthetic devices.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr,
    clippy::not_unsafe_ptr_arg_deref
)]

use crate::base::{EfiGuid, EfiStatus, EfiTime, EfiTimeCapabilities};
use core::ffi::c_void;

/// Emulator Thunk Protocol GUID.
///
/// `{5cf32e0b-8edf-2e44-9cda-93205e99ec1c}`
pub const EMU_THUNK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5cf3_2e0b,
    0x8edf,
    0x2e44,
    [0x9c, 0xda, 0x93, 0x20, 0x5e, 0x99, 0xec, 0x1c],
);

/// Emulator Graphics Window Protocol GUID.
///
/// `{30fd316a-6728-2e41-a690-0d1333d8cac1}`
pub const EMU_GRAPHICS_WINDOW_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x30fd_316a,
    0x6728,
    0x2e41,
    [0xa6, 0x90, 0x0d, 0x13, 0x33, 0xd8, 0xca, 0xc1],
);

/// Emulator Thread Thunk Protocol GUID.
///
/// `{3b1e4b7c-09d8-944f-a408-1309eb8b4427}`
pub const EMU_THREAD_THUNK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3b1e_4b7c,
    0x09d8,
    0x944f,
    [0xa4, 0x08, 0x13, 0x09, 0xeb, 0x8b, 0x44, 0x27],
);

/// Emulator Block I/O Protocol GUID.
///
/// `{6888a4ae-afce-e84b-9102-f7b9dae6a030}`
pub const EMU_BLOCK_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6888_a4ae,
    0xafce,
    0xe84b,
    [0x91, 0x02, 0xf7, 0xb9, 0xda, 0xe6, 0xa0, 0x30],
);

// --- Emulator Thread Thunk Protocol ---

/// Mutex handle in host environment.
pub type EmuHostMutex = *mut c_void;

/// Host thread entry routine.
pub type EmuThreadEntry = unsafe extern "efiapi" fn(context: *mut c_void) -> *mut c_void;

/// Emulator Thread Thunk Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmuThreadThunkProtocol {
    /// Locks a host mutex.
    pub mutex_lock: unsafe extern "efiapi" fn(mutex: EmuHostMutex) -> usize,
    /// Unlocks a host mutex.
    pub mutex_unlock: unsafe extern "efiapi" fn(mutex: EmuHostMutex) -> usize,
    /// Attempts to lock a host mutex without blocking.
    pub mutex_try_lock: unsafe extern "efiapi" fn(mutex: EmuHostMutex) -> usize,
    /// Initializes a host mutex.
    pub mutex_init: unsafe extern "efiapi" fn() -> EmuHostMutex,
    /// Destroys a host mutex.
    pub mutex_destroy: unsafe extern "efiapi" fn(mutex: EmuHostMutex) -> usize,
    /// Creates a host thread.
    pub create_thread: unsafe extern "efiapi" fn(
        thread: *mut c_void,
        attribute: *mut c_void,
        start: EmuThreadEntry,
        context: *mut c_void,
    ) -> usize,
    /// Exits the calling host thread.
    pub exit_thread: unsafe extern "efiapi" fn(value_ptr: *mut c_void),
    /// Returns the host thread ID of the calling thread.
    pub self_fn: unsafe extern "efiapi" fn() -> usize,
}

/// Safe client wrapper around [`EmuThreadThunkProtocol`].
pub struct EmuThreadThunk<'a> {
    proto: &'a EmuThreadThunkProtocol,
}

impl<'a> EmuThreadThunk<'a> {
    /// Wraps a raw [`EmuThreadThunkProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a EmuThreadThunkProtocol) -> Self {
        Self { proto }
    }

    /// Initializes a new host mutex.
    #[must_use]
    pub fn mutex_init(&self) -> EmuHostMutex {
        unsafe { (self.proto.mutex_init)() }
    }

    /// Locks a host mutex.
    pub fn mutex_lock(&self, mutex: EmuHostMutex) -> usize {
        unsafe { (self.proto.mutex_lock)(mutex) }
    }

    /// Unlocks a host mutex.
    pub fn mutex_unlock(&self, mutex: EmuHostMutex) -> usize {
        unsafe { (self.proto.mutex_unlock)(mutex) }
    }

    /// Attempts to lock a host mutex.
    pub fn mutex_try_lock(&self, mutex: EmuHostMutex) -> usize {
        unsafe { (self.proto.mutex_try_lock)(mutex) }
    }

    /// Destroys a host mutex.
    pub fn mutex_destroy(&self, mutex: EmuHostMutex) -> usize {
        unsafe { (self.proto.mutex_destroy)(mutex) }
    }

    /// Returns the calling host thread ID.
    #[must_use]
    pub fn thread_id(&self) -> usize {
        unsafe { (self.proto.self_fn)() }
    }
}

// --- Emulator Graphics Window Protocol ---

/// Emulator Graphics Window Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmuGraphicsWindowProtocol {
    /// Closes the graphics window.
    pub close: unsafe extern "efiapi" fn(this: *mut EmuGraphicsWindowProtocol) -> EfiStatus,
    /// Resizes the graphics window.
    pub size: unsafe extern "efiapi" fn(
        this: *mut EmuGraphicsWindowProtocol,
        width: u32,
        height: u32,
    ) -> EfiStatus,
    /// Checks for a pending keystroke in the graphics window.
    pub check_key: unsafe extern "efiapi" fn(this: *mut EmuGraphicsWindowProtocol) -> EfiStatus,
    /// Retrieves a keystroke from the graphics window.
    pub get_key: unsafe extern "efiapi" fn(
        this: *mut EmuGraphicsWindowProtocol,
        key: *mut c_void,
    ) -> EfiStatus,
    /// Sets keyboard toggle state in the graphics window.
    pub key_set_state: unsafe extern "efiapi" fn(
        this: *mut EmuGraphicsWindowProtocol,
        key_toggle_state: *mut u8,
    ) -> EfiStatus,
    /// Registers a key notification callback.
    pub register_key_notify_callback:
        unsafe extern "efiapi" fn(context: *mut c_void, key_data: *mut c_void),
}

/// Safe client wrapper around [`EmuGraphicsWindowProtocol`].
pub struct EmuGraphicsWindow<'a> {
    proto: &'a mut EmuGraphicsWindowProtocol,
}

impl<'a> EmuGraphicsWindow<'a> {
    /// Wraps a mutable reference to [`EmuGraphicsWindowProtocol`].
    pub fn new(proto: &'a mut EmuGraphicsWindowProtocol) -> Self {
        Self { proto }
    }

    /// Sets window dimensions in pixels.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.size)(core::ptr::from_mut(self.proto), width, height) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Closes the emulator graphics window.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn close(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.close)(core::ptr::from_mut(self.proto)) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Checks if a key event is pending.
    #[must_use]
    pub fn has_key(&mut self) -> bool {
        let status = unsafe { (self.proto.check_key)(core::ptr::from_mut(self.proto)) };
        status == EfiStatus::SUCCESS
    }
}

// --- Emulator Block I/O Protocol ---

/// Emulator Block I/O Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmuBlockIoProtocol {
    /// Resets the block device.
    pub reset: unsafe extern "efiapi" fn(
        this: *mut EmuBlockIoProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /// Reads blocks from the device.
    pub read_blocks: unsafe extern "efiapi" fn(
        this: *mut EmuBlockIoProtocol,
        media_id: u32,
        lba: u64,
        token: *mut c_void,
        buffer_size: usize,
        buffer: *mut c_void,
    ) -> EfiStatus,
    /// Writes blocks to the device.
    pub write_blocks: unsafe extern "efiapi" fn(
        this: *mut EmuBlockIoProtocol,
        media_id: u32,
        lba: u64,
        token: *mut c_void,
        buffer_size: usize,
        buffer: *const c_void,
    ) -> EfiStatus,
    /// Flushes unwritten blocks to the device.
    pub flush_blocks:
        unsafe extern "efiapi" fn(this: *mut EmuBlockIoProtocol, token: *mut c_void) -> EfiStatus,
    /// Creates a media mapping for the block device.
    pub create_mapping:
        unsafe extern "efiapi" fn(this: *mut EmuBlockIoProtocol, media: *mut c_void) -> EfiStatus,
}

/// Safe client wrapper around [`EmuBlockIoProtocol`].
pub struct EmuBlockIo<'a> {
    proto: &'a mut EmuBlockIoProtocol,
}

impl<'a> EmuBlockIo<'a> {
    /// Wraps a mutable reference to [`EmuBlockIoProtocol`].
    pub fn new(proto: &'a mut EmuBlockIoProtocol) -> Self {
        Self { proto }
    }

    /// Resets the virtual block device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn reset(&mut self, extended_verification: bool) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.proto.reset)(core::ptr::from_mut(self.proto), extended_verification) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Reads blocks synchronously from the virtual block device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on read failure.
    pub fn read_blocks(
        &mut self,
        media_id: u32,
        lba: u64,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.read_blocks)(
                core::ptr::from_mut(self.proto),
                media_id,
                lba,
                core::ptr::null_mut(),
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Writes blocks synchronously to the virtual block device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on write failure.
    pub fn write_blocks(
        &mut self,
        media_id: u32,
        lba: u64,
        buffer: &[u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.write_blocks)(
                core::ptr::from_mut(self.proto),
                media_id,
                lba,
                core::ptr::null_mut(),
                buffer.len(),
                buffer.as_ptr().cast(),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Flushes outstanding writes to the virtual device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on flush failure.
    pub fn flush(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.flush_blocks)(core::ptr::from_mut(self.proto), core::ptr::null_mut())
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}

// --- Emulator Thunk Protocol ---

/// Emulator Thunk Protocol interface for hosting UEFI services on OS APIs.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EmuThunkProtocol {
    /// Writes bytes to standard error.
    pub write_std_err: unsafe extern "efiapi" fn(buffer: *const u8, count: usize) -> usize,
    /// Configures standard input.
    pub config_std_in: unsafe extern "efiapi" fn() -> EfiStatus,
    /// Writes bytes to standard output.
    pub write_std_out: unsafe extern "efiapi" fn(buffer: *const u8, count: usize) -> usize,
    /// Reads bytes from standard input.
    pub read_std_in: unsafe extern "efiapi" fn(buffer: *mut u8, count: usize) -> usize,
    /// Polls standard input for ready characters.
    pub poll_std_in: unsafe extern "efiapi" fn() -> bool,

    /// Allocates memory from host OS allocator.
    pub malloc: unsafe extern "efiapi" fn(size: usize) -> *mut c_void,
    /// Allocates virtual memory from host OS.
    pub valloc: unsafe extern "efiapi" fn(size: usize) -> *mut c_void,
    /// Frees memory allocated by host OS.
    pub free: unsafe extern "efiapi" fn(ptr: *mut c_void) -> bool,

    /// Retrieves entry point of loaded PE/COFF image.
    pub pe_coff_get_entry_point: unsafe extern "efiapi" fn(
        pe32_data: *mut c_void,
        entry_point: *mut *mut c_void,
    ) -> EfiStatus,
    /// Relocation action hook.
    pub pe_coff_relocate_extra_action: unsafe extern "efiapi" fn(image_context: *mut c_void),
    /// Unload action hook.
    pub pe_coff_unload_extra_action: unsafe extern "efiapi" fn(image_context: *mut c_void),

    /// Enables CPU interrupts in the emulator.
    pub enable_interrupt: unsafe extern "efiapi" fn(),
    /// Disables CPU interrupts in the emulator.
    pub disable_interrupt: unsafe extern "efiapi" fn(),
    /// Queries high-resolution performance counter frequency in Hz.
    pub query_performance_frequency: unsafe extern "efiapi" fn() -> u64,
    /// Queries high-resolution performance counter tick count.
    pub query_performance_counter: unsafe extern "efiapi" fn() -> u64,

    /// Sleeps for a duration in nanoseconds.
    pub sleep: unsafe extern "efiapi" fn(nanoseconds: u64),
    /// Yields or sleeps the CPU.
    pub cpu_sleep: unsafe extern "efiapi" fn(),
    /// Exits the emulator process with a status code.
    pub exit: unsafe extern "efiapi" fn(status: usize),
    /// Retrieves wall-clock time from host OS.
    pub get_time:
        unsafe extern "efiapi" fn(time: *mut EfiTime, capabilities: *mut EfiTimeCapabilities),
    /// Sets wall-clock time on host OS.
    pub set_time: unsafe extern "efiapi" fn(time: *mut EfiTime) -> EfiStatus,
    /// Sets host timer callback.
    pub set_timer: unsafe extern "efiapi" fn(
        period_ms: u64,
        callback: unsafe extern "efiapi" fn(delta_ms: u64),
    ),

    /// Enumerates next emulator protocol instance.
    pub get_next_protocol:
        unsafe extern "efiapi" fn(emu_bus_driver: bool, instance: *mut *mut c_void) -> EfiStatus,
}

/// Safe client wrapper around [`EmuThunkProtocol`].
pub struct EmuThunk<'a> {
    proto: &'a EmuThunkProtocol,
}

impl<'a> EmuThunk<'a> {
    /// Wraps a raw [`EmuThunkProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a EmuThunkProtocol) -> Self {
        Self { proto }
    }

    /// Queries host performance counter frequency in Hz.
    #[must_use]
    pub fn performance_frequency(&self) -> u64 {
        unsafe { (self.proto.query_performance_frequency)() }
    }

    /// Queries host performance counter tick count.
    #[must_use]
    pub fn performance_counter(&self) -> u64 {
        unsafe { (self.proto.query_performance_counter)() }
    }

    /// Sleeps for a given duration in nanoseconds.
    pub fn sleep(&self, nanoseconds: u64) {
        unsafe { (self.proto.sleep)(nanoseconds) }
    }

    /// Retrieves current host wall-clock time.
    #[must_use]
    pub fn get_time(&self) -> EfiTime {
        let mut time = EfiTime::default();
        unsafe {
            (self.proto.get_time)(core::ptr::from_mut(&mut time), core::ptr::null_mut());
        }
        time
    }
}
