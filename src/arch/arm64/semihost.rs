//! # ARM Semihosting Architecture Primitives
//!
//! Provides standard ARM Semihosting call interfaces for debug console output,
//! host filesystem access, execution timing, and debugger agent interaction.
//! Defined in the ARM Semihosting Specification (ARM DUI 0471).

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr
)]

/// Open a file on the host system.
pub const SYS_OPEN: usize = 0x01;
/// Close an open file on the host system.
pub const SYS_CLOSE: usize = 0x02;
/// Write a single character to the debug console.
pub const SYS_WRITEC: usize = 0x03;
/// Write a null-terminated string to the debug console.
pub const SYS_WRITE0: usize = 0x04;
/// Write bytes to an open file or console.
pub const SYS_WRITE: usize = 0x05;
/// Read bytes from an open file or console.
pub const SYS_READ: usize = 0x06;
/// Read a single character from the console.
pub const SYS_READC: usize = 0x07;
/// Determine whether a return code indicates an error.
pub const SYS_ISERROR: usize = 0x08;
/// Check whether a file handle is connected to an interactive terminal.
pub const SYS_ISTTY: usize = 0x09;
/// Seek to an offset in an open file.
pub const SYS_SEEK: usize = 0x0A;
/// Return the length of an open file in bytes.
pub const SYS_FLEN: usize = 0x0C;
/// Get a temporary file name from the host.
pub const SYS_TMPNAM: usize = 0x0D;
/// Remove a file on the host system.
pub const SYS_REMOVE: usize = 0x0E;
/// Rename a file on the host system.
pub const SYS_RENAME: usize = 0x0F;
/// Return the execution time in centiseconds.
pub const SYS_CLOCK: usize = 0x10;
/// Return the number of seconds since 00:00 January 1, 1970.
pub const SYS_TIME: usize = 0x11;
/// Pass a command string to the host command interpreter.
pub const SYS_SYSTEM: usize = 0x12;
/// Return the host errno value for the last failing call.
pub const SYS_ERRNO: usize = 0x13;
/// Get command-line arguments passed from the debugger host.
pub const SYS_GET_CMDLINE: usize = 0x15;
/// Return heap and stack layout information from the host.
pub const SYS_HEAPINFO: usize = 0x16;
/// Return elapsed tick count since execution started.
pub const SYS_ELAPSED: usize = 0x30;
/// Return the timer tick frequency in Hz.
pub const SYS_TICKFREQ: usize = 0x31;

// --- File Modes ---

/// Open for reading in text mode.
pub const FILE_MODE_READ_TEXT: usize = 0;
/// Open for reading in binary mode.
pub const FILE_MODE_READ_BINARY: usize = 1;
/// Open for reading and writing (update) in text mode.
pub const FILE_MODE_READ_UPDATE_TEXT: usize = 2;
/// Open for reading and writing (update) in binary mode.
pub const FILE_MODE_READ_UPDATE_BINARY: usize = 3;
/// Open for writing in text mode (truncate if exists).
pub const FILE_MODE_WRITE_TEXT: usize = 4;
/// Open for writing in binary mode (truncate if exists).
pub const FILE_MODE_WRITE_BINARY: usize = 5;
/// Open for reading and writing in text mode (truncate if exists).
pub const FILE_MODE_WRITE_UPDATE_TEXT: usize = 6;
/// Open for reading and writing in binary mode (truncate if exists).
pub const FILE_MODE_WRITE_UPDATE_BINARY: usize = 7;
/// Open for appending in text mode.
pub const FILE_MODE_APPEND_TEXT: usize = 8;
/// Open for appending in binary mode.
pub const FILE_MODE_APPEND_BINARY: usize = 9;
/// Open for reading and appending in text mode.
pub const FILE_MODE_APPEND_UPDATE_TEXT: usize = 10;
/// Open for reading and appending in binary mode.
pub const FILE_MODE_APPEND_UPDATE_BINARY: usize = 11;

/// Structure describing host heap and stack limits from `SYS_HEAPINFO`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SemihostHeapInfo {
    /// Base address of heap memory.
    pub heap_base: usize,
    /// Limit address of heap memory.
    pub heap_limit: usize,
    /// Base address of stack memory.
    pub stack_base: usize,
    /// Limit address of stack memory.
    pub stack_limit: usize,
}

/// Arguments passed to `SYS_OPEN`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SemihostOpenArgs {
    /// Pointer to filename buffer.
    pub filename: *const u8,
    /// File mode constant.
    pub mode: usize,
    /// Length of filename in bytes.
    pub filename_len: usize,
}

/// Arguments passed to `SYS_READ` or `SYS_WRITE`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SemihostReadWriteArgs {
    /// File handle returned by `SYS_OPEN`.
    pub handle: usize,
    /// Pointer to buffer.
    pub buffer: *mut u8,
    /// Number of bytes to transfer.
    pub length: usize,
}

/// Arguments passed to `SYS_SEEK`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SemihostSeekArgs {
    /// File handle returned by `SYS_OPEN`.
    pub handle: usize,
    /// Target byte offset.
    pub offset: usize,
}

/// Arguments passed to `SYS_GET_CMDLINE`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SemihostCommandLineArgs {
    /// Pointer to destination buffer for command line.
    pub buffer: *mut u8,
    /// Size of buffer in bytes.
    pub size: usize,
}

/// Executes an ARM Semihosting call.
///
/// # Safety
/// On bare-metal AArch64 hardware, executes `hlt 0xf000` with the given operation and arguments.
/// The caller must ensure arguments point to valid memory buffers.
#[inline]
#[must_use]
pub unsafe fn semihost_call(operation: usize, parameter: usize) -> usize {
    #[cfg(target_arch = "aarch64")]
    {
        let mut result = operation;
        unsafe {
            core::arch::asm!(
                "hlt 0xf000",
                inout("x0") result,
                in("x1") parameter,
                options(nostack)
            );
        }
        result
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = (operation, parameter);
        usize::MAX
    }
}

/// Writes a null-terminated string to the debug console.
///
/// # Safety
/// String pointer must be valid and null-terminated.
pub unsafe fn semihost_write0(s: *const u8) {
    if !s.is_null() {
        unsafe {
            let _ = semihost_call(SYS_WRITE0, s as usize);
        }
    }
}

/// Writes a single byte character to the debug console.
pub fn semihost_write_char(c: u8) {
    let ch = c;
    unsafe {
        let _ = semihost_call(SYS_WRITEC, core::ptr::from_ref(&ch) as usize);
    }
}

/// Queries host wall-clock time in seconds since Unix epoch.
#[must_use]
pub fn semihost_time() -> usize {
    unsafe { semihost_call(SYS_TIME, 0) }
}

/// Queries elapsed processor ticks since reset.
#[must_use]
pub fn semihost_elapsed() -> usize {
    unsafe { semihost_call(SYS_ELAPSED, 0) }
}

/// Queries tick frequency in Hz.
#[must_use]
pub fn semihost_tick_frequency() -> usize {
    unsafe { semihost_call(SYS_TICKFREQ, 0) }
}
