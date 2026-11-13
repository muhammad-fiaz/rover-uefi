//! # Non-Discoverable Device Protocol and Client API
//!
//! Describes devices that are not on a discoverable bus (e.g., MMIO peripherals on ARM,
//! `RISC-V`, or `LoongArch` `SoCs`) but can be controlled by a standard class driver
//! (such as AHCI, `NVMe`, XHCI, SDHCI, or UFS).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Non-Discoverable Device Protocol GUID.
pub const NON_DISCOVERABLE_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0D51_905B,
    0xB77E,
    0x452A,
    [0xA2, 0xC0, 0xEC, 0xA0, 0xCC, 0x8D, 0x51, 0x4A],
);

/// Non-Discoverable AHCI Device GUID.
pub const NON_DISCOVERABLE_AHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xC7D3_5798,
    0xE4D2,
    0x4A93,
    [0xB1, 0x45, 0x54, 0x88, 0x9F, 0x02, 0x58, 0x4B],
);

/// Non-Discoverable AMBA Device GUID.
pub const NON_DISCOVERABLE_AMBA_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0x9444_0339,
    0xCC93,
    0x4506,
    [0xB4, 0xC6, 0xEE, 0x8D, 0x0F, 0x4C, 0xA1, 0x91],
);

/// Non-Discoverable EHCI Device GUID.
pub const NON_DISCOVERABLE_EHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xEAEE_5615,
    0x0CFD,
    0x45FC,
    [0x87, 0x69, 0xA0, 0xD8, 0x56, 0x95, 0xAF, 0x85],
);

/// Non-Discoverable `NVMe` Device GUID.
pub const NON_DISCOVERABLE_NVME_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xC5F2_5542,
    0x2A79,
    0x4A26,
    [0x81, 0xBB, 0x4E, 0xA6, 0x32, 0x33, 0xB3, 0x09],
);

/// Non-Discoverable OHCI Device GUID.
pub const NON_DISCOVERABLE_OHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xB200_05B0,
    0xBB2D,
    0x496F,
    [0x86, 0x9C, 0x23, 0x0B, 0x44, 0x79, 0xE7, 0xD1],
);

/// Non-Discoverable SDHCI Device GUID.
pub const NON_DISCOVERABLE_SDHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0x1DD1_D619,
    0xF9B8,
    0x463E,
    [0x86, 0x81, 0xD1, 0xDC, 0x7C, 0x07, 0xB7, 0x2C],
);

/// Non-Discoverable UFS Device GUID.
pub const NON_DISCOVERABLE_UFS_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0x2EA7_7912,
    0x80A8,
    0x4947,
    [0xBE, 0x69, 0xCD, 0xD0, 0x0A, 0xFB, 0xE5, 0x56],
);

/// Non-Discoverable UHCI Device GUID.
pub const NON_DISCOVERABLE_UHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xA8CD_A0A2,
    0x4F37,
    0x4A1B,
    [0x8E, 0x10, 0x8E, 0xF3, 0xCC, 0x3B, 0xF3, 0xA8],
);

/// Non-Discoverable XHCI Device GUID.
pub const NON_DISCOVERABLE_XHCI_DEVICE_GUID: EfiGuid = EfiGuid::new(
    0xB1BE_0BC5,
    0x6C28,
    0x442D,
    [0xAA, 0x37, 0x15, 0x1B, 0x42, 0x57, 0xBD, 0x78],
);

/// Non-Discoverable Device DMA Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonDiscoverableDeviceDmaType {
    /// Device DMA is cache-coherent.
    Coherent = 0,
    /// Device DMA is non-coherent (requires cache maintenance).
    NonCoherent = 1,
}

/// Function prototype for initializing a non-discoverable device.
pub type NonDiscoverableDeviceInit =
    unsafe extern "efiapi" fn(this: *mut NonDiscoverableDeviceProtocol) -> EfiStatus;

/// Non-Discoverable Device Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NonDiscoverableDeviceProtocol {
    /// Pointer to the GUID identifying the device type (e.g. AHCI, `NVMe`, XHCI).
    pub device_type: *const EfiGuid,
    /// DMA coherency type.
    pub dma_type: NonDiscoverableDeviceDmaType,
    /// Device-specific initialization function.
    pub initialize: Option<NonDiscoverableDeviceInit>,
    /// Pointer to ACPI Address Space Descriptors describing MMIO/IO resources.
    pub resources: *const c_void,
}

/// Safe client-side wrapper for non-discoverable devices.
#[derive(Debug)]
pub struct NonDiscoverableDevice<'a> {
    protocol: &'a mut NonDiscoverableDeviceProtocol,
}

impl<'a> NonDiscoverableDevice<'a> {
    /// Create a new safe view of a non-discoverable device.
    #[must_use]
    pub fn new(protocol: &'a mut NonDiscoverableDeviceProtocol) -> Self {
        Self { protocol }
    }

    /// Return the device type GUID if available.
    #[must_use]
    pub fn device_type(&self) -> Option<EfiGuid> {
        if self.protocol.device_type.is_null() {
            None
        } else {
            // SAFETY: Verified non-null firmware pointer.
            Some(unsafe { *self.protocol.device_type })
        }
    }

    /// Return the DMA coherency type.
    #[must_use]
    pub const fn dma_type(&self) -> NonDiscoverableDeviceDmaType {
        self.protocol.dma_type
    }

    /// Return whether this device has cache-coherent DMA.
    #[must_use]
    pub fn is_coherent(&self) -> bool {
        self.protocol.dma_type == NonDiscoverableDeviceDmaType::Coherent
    }

    /// Initialize the non-discoverable device hardware if callback is installed.
    ///
    /// # Errors
    /// Returns an error if the device initialization callback fails.
    pub fn initialize(&mut self) -> Result<(), EfiStatus> {
        if let Some(init_fn) = self.protocol.initialize {
            let status = unsafe { init_fn(self.protocol) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Ok(())
        }
    }
}
