//! # Driver registry
//!
//! Fixed-capacity driver matching without heap allocation.

use crate::base::driver::device::DeviceType;

/// The maximum registered drivers.
pub const MAX_DRIVERS: usize = 32;

/// A driver match rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DriverMatch {
    /// The device type to match, or `None` for any type.
    pub device_type: Option<DeviceType>,
    /// The vendor identifier to match, or `None` for any vendor.
    pub vendor_id: Option<u16>,
    /// The device identifier to match, or `None` for any device.
    pub device_id: Option<u16>,
    /// The class to match, or `None` for any class.
    pub class_code: Option<u32>,
}

impl DriverMatch {
    /// Creates an exact match rule.
    #[must_use]
    pub const fn new(
        device_type: Option<DeviceType>,
        vendor_id: Option<u16>,
        device_id: Option<u16>,
        class_code: Option<u32>,
    ) -> Self {
        Self {
            device_type,
            vendor_id,
            device_id,
            class_code,
        }
    }

    /// Returns whether the rule matches the supplied identifiers.
    #[must_use]
    pub const fn matches(
        &self,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        class_code: u32,
    ) -> bool {
        let type_ok = match self.device_type {
            None => true,
            Some(expected) => expected as u8 == device_type as u8,
        };
        let vendor_ok = match self.vendor_id {
            None => true,
            Some(expected) => expected == vendor_id,
        };
        let device_ok = match self.device_id {
            None => true,
            Some(expected) => expected == device_id,
        };
        let class_ok = match self.class_code {
            None => true,
            Some(expected) => expected == class_code,
        };
        type_ok && vendor_ok && device_ok && class_ok
    }
}

/// A registered driver entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DriverEntry {
    /// The driver name identifier.
    pub name: u64,
    /// The match rule.
    pub matcher: DriverMatch,
}

/// A registry of drivers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriverRegistry {
    entries: [Option<DriverEntry>; MAX_DRIVERS],
    count: usize,
}

impl DriverRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        let entries = [None; MAX_DRIVERS];
        Self { entries, count: 0 }
    }

    /// Returns the registered driver count.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Registers a driver, returning `false` when full.
    pub fn register(&mut self, entry: DriverEntry) -> bool {
        if self.count >= MAX_DRIVERS {
            return false;
        }
        if let Some(slot) = self.entries.get_mut(self.count) {
            *slot = Some(entry);
            self.count = self.count.saturating_add(1);
            true
        } else {
            false
        }
    }

    /// Finds the first driver matching the identifiers.
    #[must_use]
    pub fn find(
        &self,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        class_code: u32,
    ) -> Option<&DriverEntry> {
        for index in 0..self.count {
            if let Some(Some(entry)) = self.entries.get(index)
                && entry
                    .matcher
                    .matches(device_type, vendor_id, device_id, class_code)
            {
                return Some(entry);
            }
        }
        None
    }
}

impl Default for DriverRegistry {
    /// Creates an empty registry.
    fn default() -> Self {
        Self::new()
    }
}
