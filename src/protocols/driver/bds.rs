//! # Boot Device Selection (BDS) architectural protocol
//!
//! Defined in PI Specification Volume 2: DXE.
//!
//! When the DXE core has completed dispatching drivers from the dispatch
//! queue, control transfers to the BDS architectural protocol entry point.

use crate::base::EfiGuid;

/// The BDS architectural protocol `GUID`.
pub const EFI_BDS_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x665E_3FF6,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The BDS entry point signature.
pub type EfiBdsEntry = unsafe extern "efiapi" fn(*mut EfiBdsArchProtocol);

/// The BDS architectural protocol interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBdsArchProtocol {
    /// Transfers control to the Boot Device Selection phase.
    pub entry: EfiBdsEntry,
}

/// A safe wrapper for [`EfiBdsArchProtocol`].
pub struct BdsArchProtocol<'a> {
    protocol: &'a EfiBdsArchProtocol,
}

impl<'a> BdsArchProtocol<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiBdsArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns a reference to the raw protocol structure.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiBdsArchProtocol {
        self.protocol
    }

    /// Invokes the BDS entry point to hand off firmware execution.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the protocol function pointer is valid
    /// and that system state is ready for the BDS phase.
    pub unsafe fn entry(&self) {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        unsafe {
            (self.protocol.entry)(ptr);
        }
    }
}

impl core::fmt::Debug for BdsArchProtocol<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BdsArchProtocol")
            .field("protocol", &self.protocol)
            .finish()
    }
}
