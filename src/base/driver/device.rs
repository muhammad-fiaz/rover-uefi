//! # Device model
//!
//! Native device descriptors and power states.

/// A device identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeviceId(pub u64);

impl DeviceId {
    /// Creates an identifier from a raw value.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn to_raw(self) -> u64 {
        self.0
    }
}

/// A device type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceType {
    /// A `PCI` or `PCIe` device.
    Pci = 0,
    /// A `USB` device.
    Usb = 1,
    /// A `NVMe` controller.
    Nvme = 2,
    /// An `AHCI` or `SATA` controller.
    Ahci = 3,
    /// An Ethernet controller.
    Ethernet = 4,
    /// A graphics controller.
    Graphics = 5,
    /// An audio controller.
    Audio = 6,
    /// A timer.
    Timer = 7,
    /// A platform controller.
    Platform = 8,
    /// An unknown device.
    Unknown = 255,
}

/// A power state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PowerState {
    /// The device is powered on.
    On = 0,
    /// The device is powered off.
    Off = 1,
    /// The device is in a transitional state.
    Transition = 2,
}

/// A native device descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Device {
    /// The device identifier.
    pub id: DeviceId,
    /// The device type.
    pub device_type: DeviceType,
    /// The vendor identifier.
    pub vendor_id: u16,
    /// The device identifier within the vendor.
    pub device_id: u16,
    /// The class code.
    pub class_code: u32,
    /// The power state.
    pub power: PowerState,
}

impl Device {
    /// Creates a device descriptor.
    #[must_use]
    pub const fn new(
        id: DeviceId,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        class_code: u32,
        power: PowerState,
    ) -> Self {
        Self {
            id,
            device_type,
            vendor_id,
            device_id,
            class_code,
            power,
        }
    }

    /// Returns whether the device is powered on.
    #[must_use]
    pub const fn is_on(&self) -> bool {
        matches!(self.power, PowerState::On)
    }
}
