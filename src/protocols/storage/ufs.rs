//! # Universal Flash Storage (`UFS`) Protocols and Client API
//!
//! Provides the UEFI `UFS` Device Configuration Protocol and `UFS` Host
//! Controller Platform Protocol used across smartphones, tablets, Android devices,
//! laptops, and embedded systems.
//!
//! Defined in UEFI Specification 2.7+ (Section 14.11).

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// EFI `UFS` Device Config Protocol GUID.
pub const EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB81B_FAB0,
    0x0EB3,
    0x4CF9,
    [0x84, 0x65, 0x7F, 0xA9, 0x86, 0x36, 0x16, 0x64],
);

/// `UFS` Host Controller Protocol GUID.
pub const UFS_HOST_CONTROLLER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEBC0_1AF5,
    0x07A9,
    0x489E,
    [0xB7, 0xCE, 0xDC, 0x08, 0x9E, 0x45, 0x9B, 0x2F],
);

/// `UFS` Host Controller Platform Protocol GUID.
pub const UFS_HC_PLATFORM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3D18_BA13,
    0xD9B1,
    0x4DD4,
    [0xB9, 0x16, 0xD3, 0x07, 0x96, 0x53, 0x9E, 0xD8],
);

/// Function prototype to read/write `UFS` device descriptors.
pub type EfiUfsDeviceConfigRwDescriptor = unsafe extern "efiapi" fn(
    this: *mut EfiUfsDeviceConfigProtocol,
    read: bool,
    desc_id: u8,
    index: u8,
    selector: u8,
    descriptor: *mut u8,
    desc_size: *mut u32,
) -> EfiStatus;

/// Function prototype to read/write `UFS` flag descriptors.
pub type EfiUfsDeviceConfigRwFlag = unsafe extern "efiapi" fn(
    this: *mut EfiUfsDeviceConfigProtocol,
    read: bool,
    flag_id: u8,
    flag: *mut u8,
) -> EfiStatus;

/// Function prototype to read/write `UFS` attribute descriptors.
pub type EfiUfsDeviceConfigRwAttribute = unsafe extern "efiapi" fn(
    this: *mut EfiUfsDeviceConfigProtocol,
    read: bool,
    attr_id: u8,
    index: u8,
    selector: u8,
    attribute: *mut u8,
    attr_size: *mut u32,
) -> EfiStatus;

/// EFI `UFS` Device Config Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiUfsDeviceConfigProtocol {
    /// Reads or writes specified device descriptor of a `UFS` device.
    pub rw_ufs_descriptor: Option<EfiUfsDeviceConfigRwDescriptor>,
    /// Reads or writes specified single-bit flag of a `UFS` device.
    pub rw_ufs_flag: Option<EfiUfsDeviceConfigRwFlag>,
    /// Reads or writes specified attribute parameter of a `UFS` device.
    pub rw_ufs_attribute: Option<EfiUfsDeviceConfigRwAttribute>,
}

/// Host controller capabilities and specification version.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UfsHcInfo {
    /// Controller capabilities bitmask.
    pub capabilities: u32,
    /// Specification version supported by hardware.
    pub version: u32,
}

/// `UFS` Interconnect Layer (`UIC`) Command descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UicCommand {
    /// Command opcode (e.g. `DME_GET`, `DME_SET`, `DME_PEER_GET`).
    pub opcode: u32,
    /// Command argument 1.
    pub arg1: u32,
    /// Command argument 2.
    pub arg2: u32,
    /// Command argument 3.
    pub arg3: u32,
}

/// `UFS` Host Controller Driver Interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UfsHcDriverInterface {
    /// Pointer to host controller MMIO/PCI protocol.
    pub ufs_hc_protocol: *mut c_void,
    /// Function executing a raw `UIC` command.
    pub ufs_exec_uic_command: Option<
        unsafe extern "efiapi" fn(
            this: *mut UfsHcDriverInterface,
            uic_command: *mut UicCommand,
        ) -> EfiStatus,
    >,
}

/// Callback phase types during `UFS` initialization.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UfsHcPlatformCallbackPhase {
    /// Prior to Host Controller Enable (`HCE`).
    PreHce = 0,
    /// After Host Controller Enable (`HCE`).
    PostHce = 1,
    /// Prior to `UniPro` / M-PHY link startup.
    PreLinkStartup = 2,
    /// After `UniPro` / M-PHY link startup.
    PostLinkStartup = 3,
}

/// Reference clock frequency attribute values for `UFS` cards.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UfsCardRefClkFreq {
    /// 19.2 MHz reference clock.
    Freq19p2Mhz = 0,
    /// 26 MHz reference clock.
    Freq26Mhz = 1,
    /// 38.4 MHz reference clock.
    Freq38p4Mhz = 2,
    /// Obsolete / platform-default frequency.
    Obsolete = 3,
}

/// `UFS` Host Controller Platform Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UfsHcPlatformProtocol {
    /// Protocol revision version.
    pub version: u32,
    /// Override host controller information hook.
    pub override_hc_info: Option<
        unsafe extern "efiapi" fn(
            controller_handle: EfiHandle,
            hc_info: *mut UfsHcInfo,
        ) -> EfiStatus,
    >,
    /// Platform flow callback hook.
    pub callback: Option<
        unsafe extern "efiapi" fn(
            controller_handle: EfiHandle,
            callback_phase: UfsHcPlatformCallbackPhase,
            callback_data: *mut c_void,
        ) -> EfiStatus,
    >,
    /// Card reference clock frequency.
    pub ref_clk_freq: UfsCardRefClkFreq,
    /// Skip Host Controller Enable (`HCE`) re-enable.
    pub skip_hce: bool,
}

/// Safe client wrapper for `UFS` device configuration.
#[derive(Debug)]
pub struct UfsDeviceConfig<'a> {
    protocol: &'a mut EfiUfsDeviceConfigProtocol,
}

impl<'a> UfsDeviceConfig<'a> {
    /// Create a new safe view of a `UFS` device configuration interface.
    #[must_use]
    pub fn new(protocol: &'a mut EfiUfsDeviceConfigProtocol) -> Self {
        Self { protocol }
    }

    /// Read a `UFS` device flag value.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn read_flag(&mut self, flag_id: u8) -> Result<u8, EfiStatus> {
        if let Some(flag_fn) = self.protocol.rw_ufs_flag {
            let mut flag: u8 = 0;
            let status = unsafe { flag_fn(self.protocol, true, flag_id, &raw mut flag) };
            if status == EfiStatus::SUCCESS {
                Ok(flag)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Write a `UFS` device flag value.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn write_flag(&mut self, flag_id: u8, mut value: u8) -> Result<(), EfiStatus> {
        if let Some(flag_fn) = self.protocol.rw_ufs_flag {
            let status = unsafe { flag_fn(self.protocol, false, flag_id, &raw mut value) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}

/// Safe client wrapper for `UFS` Host Controller platform operations.
#[derive(Debug)]
pub struct UfsHcPlatform<'a> {
    protocol: &'a UfsHcPlatformProtocol,
}

impl<'a> UfsHcPlatform<'a> {
    /// Create a new safe view of a `UFS` Host Controller Platform protocol.
    #[must_use]
    pub fn new(protocol: &'a UfsHcPlatformProtocol) -> Self {
        Self { protocol }
    }

    /// Return the protocol version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.protocol.version
    }

    /// Return the reference clock frequency.
    #[must_use]
    pub const fn ref_clk_freq(&self) -> UfsCardRefClkFreq {
        self.protocol.ref_clk_freq
    }
}
