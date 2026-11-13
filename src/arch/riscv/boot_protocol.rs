//! # RISC-V Boot Protocol
//!
//! Mandatory for RISC-V UEFI platforms as defined by the RISC-V UEFI Specification.
//!
//! Provides services for querying the primary boot hart ID (`mhartid` / bootstrap processor)
//! that transferred control to the UEFI firmware or operating system loader.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// RISC-V EFI Boot Protocol GUID (`{ccd15fec-6f73-4eec-8395-3e69e4b940bf}`).
pub const RISCV_BOOT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCCD1_5FEC,
    0x6F73,
    0x4EEC,
    [0x83, 0x95, 0x3E, 0x69, 0xE4, 0xB9, 0x40, 0xBF],
);

/// Initial revision of the RISC-V Boot Protocol (1.0).
pub const RISCV_BOOT_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Function pointer to retrieve the primary boot hart ID.
pub type RiscvGetBootHartId =
    unsafe extern "efiapi" fn(this: *mut RiscvBootProtocol, boot_hart_id: *mut UIntN) -> EfiStatus;

/// RISC-V Boot Protocol ABI structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RiscvBootProtocol {
    /// Revision of the protocol interface (`RISCV_BOOT_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Function to obtain the boot hart ID.
    pub get_boot_hart_id: Option<RiscvGetBootHartId>,
}

/// Safe native client wrapper around [`RiscvBootProtocol`].
#[derive(Debug)]
pub struct RiscvBoot<'a> {
    protocol: &'a mut RiscvBootProtocol,
}

impl<'a> RiscvBoot<'a> {
    /// Wrap a mutable reference to an active [`RiscvBootProtocol`].
    #[must_use]
    pub fn new(protocol: &'a mut RiscvBootProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the interface revision reported by the firmware.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Query the physical boot hart ID of the executing system.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if the function pointer is missing, or
    /// the underlying `EfiStatus` returned by the platform.
    pub fn boot_hart_id(&mut self) -> Result<usize, EfiStatus> {
        let Some(get_hart_id_fn) = self.protocol.get_boot_hart_id else {
            return Err(EfiStatus::UNSUPPORTED);
        };

        let mut hart_id = UIntN::MIN;
        let status = unsafe { get_hart_id_fn(self.protocol, &raw mut hart_id) };

        if status == EfiStatus::SUCCESS {
            Ok(hart_id)
        } else {
            Err(status)
        }
    }
}
