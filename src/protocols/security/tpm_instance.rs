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

/// HOB GUID used to pass all excluded Firmware Volumes to DXE measurement drivers:
/// `{dd55d497-5ed5-403c-92b5-703a7259ae2a}`.
pub const FIRMWARE_VOLUME_MEASUREMENT_EXCLUDED_HOB_GUID: EfiGuid = EfiGuid::new(
    0xDD55_D497,
    0x5ED5,
    0x403C,
    [0x92, 0xB5, 0x70, 0x3A, 0x72, 0x59, 0xAE, 0x2A],
);

/// HOB GUID for TPM NVS memory communication in Management Mode:
/// `{c96c76eb-bc78-429c-9f4b-da5178c28457}`.
pub const MM_TPM_NVS_HOB_GUID: EfiGuid = EfiGuid::new(
    0xC96C_76EB,
    0xBC78,
    0x429C,
    [0x9F, 0x4B, 0xDA, 0x51, 0x78, 0xC2, 0x84, 0x57],
);

/// TPM physical presence operation NVS record for ACPI table patching.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalPresenceNvs {
    /// Software SMI number for triggering physical presence evaluation.
    pub software_smi: u8,
    /// Parameter passed to the software SMI handler.
    pub parameter: u32,
    /// Response status code from the physical presence handler.
    pub response: u32,
    /// Request opcode passed to the physical presence handler.
    pub request: u32,
    /// Optional parameter associated with the physical presence request.
    pub request_parameter: u32,
    /// Last executed physical presence request opcode.
    pub last_request: u32,
    /// Return code returned to the caller.
    pub return_code: u32,
}

/// Memory clear (MOR) operation NVS record for ACPI table patching.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryClearNvs {
    /// Software SMI number for triggering memory overwrite request.
    pub software_smi: u8,
    /// Parameter passed to the memory clear SMI handler.
    pub parameter: u32,
    /// Request opcode passed to the memory clear SMI handler.
    pub request: u32,
    /// Return status code returned to the caller.
    pub return_code: u32,
}

/// Aggregated TCG NVS structure used by firmware to interface ACPI tables with TPM MM handlers.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TcgNvs {
    /// Physical presence NVS interface fields.
    pub physical_presence: PhysicalPresenceNvs,
    /// Memory clear NVS interface fields.
    pub memory_clear: MemoryClearNvs,
    /// Flag indicating whether user confirmation is required for physical presence requests.
    pub pp_request_user_confirm: u32,
    /// Hardware interrupt vector assigned to TPM device.
    pub tpm_irq_num: u32,
    /// Indicates whether AML package length encoding uses the short form.
    pub is_short_form_pkg_length: bool,
}

/// Function opcode for TPM NVS MM communication buffer transactions.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpmNvsMmFunction {
    /// Exchange SWI value and NVS region address for patching TPM ACPI tables.
    ExchangeInfo = 0,
}

/// Communication buffer layout for TPM NVS MM transactions.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TpmNvsMmCommBuffer {
    /// Transaction function opcode ([`TpmNvsMmFunction`]).
    pub function: u64,
    /// Return status code resulting from MM execution.
    pub return_status: u64,
    /// Target physical address of the TPM NVS data region.
    pub target_address: EfiPhysicalAddress,
    /// Registered physical presence software SMI vector.
    pub registered_pp_swi_value: u64,
    /// Registered memory clear software SMI vector.
    pub registered_mc_swi_value: u64,
}
