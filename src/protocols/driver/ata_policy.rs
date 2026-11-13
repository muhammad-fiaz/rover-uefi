//! # ATA/ATAPI Platform Policy Protocol
//!
//! Provides platform policy configuration for ATA and ATAPI storage host controllers
//! and devices, including Power-Up in Standby (PUIS) and Device Sleep (`DevSleep`).

use crate::base::EfiGuid;

/// ATA/ATAPI Policy Protocol GUID: `{e59cd769-5083-4f26-9094-6c919f916c4e}`.
pub const ATA_ATAPI_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE59C_D769,
    0x5083,
    0x4F26,
    [0x90, 0x94, 0x6C, 0x91, 0x9F, 0x91, 0x6C, 0x4E],
);

/// ATA/ATAPI Policy Protocol interface revision (v1.0).
pub const ATA_ATAPI_POLICY_VERSION: u32 = 0x0001_0000;

/// Raw ATA/ATAPI Policy Protocol structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AtaAtapiPolicyProtocol {
    /// Protocol revision number (`ATA_ATAPI_POLICY_VERSION`).
    pub version: u32,
    /// Power-up in Standby:
    /// - `0`: Disable PUIS
    /// - `1`: Enable PUIS
    /// - Other: Preserve non-volatile hardware setting
    pub puis_enable: u8,
    /// Device Sleep:
    /// - `0`: Disable Device Sleep
    /// - `1`: Enable Device Sleep
    pub device_sleep_enable: u8,
    /// Aggressive Device Sleep:
    /// - `0`: Disable Aggressive Device Sleep
    /// - `1`: Enable Aggressive Device Sleep
    pub aggressive_device_sleep_enable: u8,
    /// Reserved for alignment and future expansion.
    pub reserved: u8,
}

impl Default for AtaAtapiPolicyProtocol {
    fn default() -> Self {
        Self {
            version: ATA_ATAPI_POLICY_VERSION,
            puis_enable: 0,
            device_sleep_enable: 0,
            aggressive_device_sleep_enable: 0,
            reserved: 0,
        }
    }
}

/// Safe client wrapper around [`AtaAtapiPolicyProtocol`].
pub struct AtaAtapiPolicy<'a> {
    raw: &'a mut AtaAtapiPolicyProtocol,
}

impl AtaAtapiPolicy<'_> {
    /// Wraps a raw [`AtaAtapiPolicyProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut AtaAtapiPolicyProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Returns the protocol version.
    #[must_use]
    pub fn version(&self) -> u32 {
        self.raw.version
    }

    /// Returns true if Power-Up in Standby (PUIS) is explicitly enabled.
    #[must_use]
    pub fn is_puis_enabled(&self) -> bool {
        self.raw.puis_enable == 1
    }

    /// Sets the Power-Up in Standby policy.
    pub fn set_puis(&mut self, enabled: bool) {
        self.raw.puis_enable = u8::from(enabled);
    }

    /// Returns true if Device Sleep (`DevSleep`) is enabled.
    #[must_use]
    pub fn is_device_sleep_enabled(&self) -> bool {
        self.raw.device_sleep_enable == 1
    }

    /// Sets the Device Sleep policy.
    pub fn set_device_sleep(&mut self, enabled: bool) {
        self.raw.device_sleep_enable = u8::from(enabled);
    }

    /// Returns true if Aggressive Device Sleep is enabled.
    #[must_use]
    pub fn is_aggressive_device_sleep_enabled(&self) -> bool {
        self.raw.aggressive_device_sleep_enable == 1
    }

    /// Sets the Aggressive Device Sleep policy.
    pub fn set_aggressive_device_sleep(&mut self, enabled: bool) {
        self.raw.aggressive_device_sleep_enable = u8::from(enabled);
    }
}
