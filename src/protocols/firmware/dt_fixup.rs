//! # Device Tree Fixup Protocol and Client API
//!
//! Modifies a flattened device tree (`DTB` / `FDT`) in memory to align it with
//! firmware's view of the hardware platform before handing off control to an
//! operating system kernel (e.g. Linux on `AArch64` and `RISC-V`).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Device Tree Fixup Protocol GUID.
pub const DT_FIXUP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x60ED_6BA9,
    0xDFEF,
    0x4799,
    [0xAC, 0x7B, 0x75, 0xE0, 0xF8, 0x33, 0x45, 0x6C],
);

/// Device Tree Fixup Protocol revision (1.0).
pub const DT_FIXUP_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Function pointer to perform device tree fixups.
pub type DtFixup = unsafe extern "efiapi" fn(
    this: *mut DtFixupProtocol,
    fdt: *mut c_void,
    buffer_size: *mut usize,
) -> EfiStatus;

/// Device Tree Fixup Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DtFixupProtocol {
    /// Protocol revision (`DT_FIXUP_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Apply firmware device tree fixups to `FDT` buffer.
    pub fixup: Option<DtFixup>,
}

/// Safe client wrapper for Device Tree Fixup operations.
#[derive(Debug)]
pub struct DeviceTreeFixup<'a> {
    protocol: &'a mut DtFixupProtocol,
}

impl<'a> DeviceTreeFixup<'a> {
    /// Create a new safe view of the Device Tree Fixup protocol.
    #[must_use]
    pub fn new(protocol: &'a mut DtFixupProtocol) -> Self {
        Self { protocol }
    }

    /// Return the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Apply device tree fixups to a given buffer.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn fixup(&mut self, fdt_buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        if let Some(fixup_fn) = self.protocol.fixup {
            let mut size = fdt_buffer.len();
            let status = unsafe {
                fixup_fn(
                    self.protocol,
                    fdt_buffer.as_mut_ptr().cast::<c_void>(),
                    &raw mut size,
                )
            };
            if status == EfiStatus::SUCCESS {
                Ok(size)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
