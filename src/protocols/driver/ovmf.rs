//! # Virtual Machine Platform Protocols
//!
//! Provides protocol interfaces used by `QEMU` and virtual machines to boot
//! direct Linux kernels and coordinate confidential computing memory acceptance
//! (`SEV-SNP`, `TDX`).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// OVMF Loaded x86 Linux Kernel Protocol GUID.
pub const OVMF_LOADED_X86_LINUX_KERNEL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA3ED_C05D,
    0xB618,
    0x4FF6,
    [0x95, 0x52, 0x76, 0xD7, 0x88, 0x63, 0x43, 0xC8],
);

/// OVMF `SEV` Memory Acceptance Protocol GUID.
pub const OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xC5A0_10FE,
    0x38A7,
    0x4531,
    [0x8A, 0x4A, 0x05, 0x00, 0xD2, 0xFD, 0x16, 0x49],
);

/// OVMF Direct Loaded x86 Linux Kernel Descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OvmfLoadedX86LinuxKernel {
    /// Pointer to zero-page setup header buffer.
    pub setup_buf: *mut c_void,
    /// Pointer to kernel 32/64-bit payload buffer.
    pub kernel_buf: *mut c_void,
    /// Pointer to ASCII kernel command-line string.
    pub command_line: *mut u8,
    /// Pointer to initial ramdisk (`initrd`) buffer.
    pub initrd_data: *mut c_void,
    /// Size of zero-page setup buffer in bytes.
    pub setup_size: usize,
    /// Initial size of the kernel buffer in bytes.
    pub kernel_initial_size: usize,
    /// Size of initial ramdisk buffer in bytes.
    pub initrd_size: usize,
    /// Size of command-line string buffer in bytes.
    pub command_line_size: usize,
}

/// Function pointer to permit unaccepted memory upon `ExitBootServices`.
pub type OvmfSevAllowUnacceptedMemory =
    unsafe extern "efiapi" fn(this: *mut OvmfSevMemoryAcceptanceProtocol) -> EfiStatus;

/// OVMF `SEV` Memory Acceptance Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OvmfSevMemoryAcceptanceProtocol {
    /// Request firmware to leave unaccepted memory unaccepted on OS handoff.
    pub allow_unaccepted_memory: Option<OvmfSevAllowUnacceptedMemory>,
}
