//! Unit tests for ARM Semihosting architecture primitives.

use rover_uefi::arch::arm64::semihost::*;

#[test]
fn test_semihost_constants() {
    assert_eq!(SYS_OPEN, 0x01);
    assert_eq!(SYS_CLOSE, 0x02);
    assert_eq!(SYS_WRITEC, 0x03);
    assert_eq!(SYS_WRITE0, 0x04);
    assert_eq!(SYS_WRITE, 0x05);
    assert_eq!(SYS_READ, 0x06);
    assert_eq!(SYS_TIME, 0x11);
    assert_eq!(SYS_ELAPSED, 0x30);
    assert_eq!(SYS_TICKFREQ, 0x31);

    assert_eq!(FILE_MODE_READ_TEXT, 0);
    assert_eq!(FILE_MODE_READ_BINARY, 1);
    assert_eq!(FILE_MODE_WRITE_BINARY, 5);
}

#[test]
fn test_semihost_data_structures() {
    let heap_info = SemihostHeapInfo {
        heap_base: 0x4000_0000,
        heap_limit: 0x4800_0000,
        stack_base: 0x4900_0000,
        stack_limit: 0x4880_0000,
    };
    assert_eq!(heap_info.heap_base, 0x4000_0000);
    assert_eq!(heap_info.heap_limit, 0x4800_0000);

    let mut buf = [0u8; 64];
    let open_args = SemihostOpenArgs {
        filename: buf.as_ptr(),
        mode: FILE_MODE_READ_BINARY,
        filename_len: 4,
    };
    assert_eq!(open_args.mode, 1);
    assert_eq!(open_args.filename_len, 4);

    let rw_args = SemihostReadWriteArgs {
        handle: 3,
        buffer: buf.as_mut_ptr(),
        length: 64,
    };
    assert_eq!(rw_args.handle, 3);
    assert_eq!(rw_args.length, 64);

    let seek_args = SemihostSeekArgs {
        handle: 3,
        offset: 1024,
    };
    assert_eq!(seek_args.offset, 1024);
}

#[test]
fn test_semihost_call_fallback() {
    // On non-baremetal / test host target, semihost_call returns fallback
    let res = unsafe { semihost_call(SYS_TIME, 0) };
    #[cfg(not(target_arch = "aarch64"))]
    assert_eq!(res, usize::MAX);
    #[cfg(target_arch = "aarch64")]
    let _ = res;
}
