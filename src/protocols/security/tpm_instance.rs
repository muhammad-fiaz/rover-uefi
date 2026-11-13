//! # TPM Instance and Platform Measurement Identification
//!
//! Provides platform GUIDs and communication buffer layouts for identifying
//! TPM hardware devices (discrete TPM 1.2, discrete TPM 2.0, none),
//! Confidential Computing HOB records, measured firmware volume HOBs, and
//! TCG2 ACPI communicate buffers for standalone MM.

use crate::base::{EfiGuid, EfiPhysicalAddress};

/// Identifier indicating no TPM device is present.
pub const TPM_DEVICE_INTERFACE_NONE: EfiGuid = EfiGuid::new(
    0x0000_0000,
    0x0000,
    0x0000,
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
);

/// Identifier indicating a discrete TPM 1.2 device interface.
pub const TPM_DEVICE_INTERFACE_TPM12: EfiGuid = EfiGuid::new(
    0x8B01_E5B6,
    0x4F19,
    0x46E8,
    [0xAB, 0x93, 0x1C, 0x53, 0x67, 0x1B, 0x90, 0xCC],
);

/// Identifier indicating a discrete TPM 2.0 (dTPM) device interface.
pub const TPM_DEVICE_INTERFACE_TPM20_DTPM: EfiGuid = EfiGuid::new(
    0x286B_F25A,
    0xC2C3,
    0x408C,
    [0xB3, 0xB4, 0x25, 0xE6, 0x75, 0x8B, 0x73, 0x17],
);

/// Identifier published when a specific TPM device is selected by platform policy.
pub const TPM_DEVICE_SELECTED_GUID: EfiGuid = EfiGuid::new(
    0x7F41_58D3,
    0x074D,
    0x456D,
    [0x8C, 0xB2, 0x01, 0xF9, 0xC8, 0xF7, 0x9D, 0xAA],
);

/// HOB GUID used to pass a Confidential Computing (CC) measurement event from early SEC to DXE.
pub const CC_EVENT_ENTRY_HOB_GUID: EfiGuid = EfiGuid::new(
    0x20F8_FD36,
    0x6D00,
    0x40FB,
    [0xB7, 0x04, 0xD1, 0x2C, 0x15, 0x3C, 0x62, 0xEB],
);

/// HOB GUID used to pass measured Firmware Volume (FV) blobs to DXE drivers.
pub const MEASURED_FV_HOB_GUID: EfiGuid = EfiGuid::new(
    0xB236_0B42,
    0x7173,
    0x420A,
    [0x86, 0x96, 0x46, 0xCA, 0x6B, 0xAB, 0x10, 0x60],
);

/// HOB GUID used to exchange buffer locations between `Tcg2Acpi` and Standalone MM.
pub const TCG2_ACPI_COMMUNICATE_BUFFER_HOB_GUID: EfiGuid = EfiGuid::new(
    0xCEFE_A14F,
    0x9F1A,
    0x4774,
    [0x8D, 0x18, 0x79, 0x93, 0x8D, 0x48, 0xFE, 0x7D],
);

/// Current revision of the Tcg2 Acpi Communicate Buffer HOB.
pub const TCG2_ACPI_COMMUNICATE_BUFFER_HOB_REVISION: u32 = 1;

/// Memory exchange buffer descriptor for TCG2 ACPI and Standalone MM.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tcg2AcpiCommunicateBuffer {
    /// Base physical address of the communicate buffer.
    pub tcg2_acpi_communicate_buffer: EfiPhysicalAddress,
    /// Size of the reserved communicate buffer in 4 KiB pages.
    pub pages: u64,
}

/// Header structure for measured firmware volume HOB payload.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeasuredHobHeader {
    /// Number of measured firmware volume blobs in the HOB payload.
    pub num: u32,
}
