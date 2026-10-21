//! Tests for native driver foundations.

use core::fmt::Write as _;

use rover_uefi::base::driver::bus::ArchId;
use rover_uefi::base::driver::device::{Device, DeviceId, DeviceType, PowerState};
use rover_uefi::base::driver::lifecycle::DriverError;
use rover_uefi::base::driver::registry::{DriverEntry, DriverMatch, DriverRegistry};
use rover_uefi::base::driver::resources::{
    DmaBuffer, Interrupt, MmioRegion, PortIoRegion, Resource,
};

#[test]
fn device_roundtrip() {
    let device = Device::new(
        DeviceId::new(7),
        DeviceType::Pci,
        0x1234,
        0x5678,
        0x01_02_03,
        PowerState::On,
    );
    assert_eq!(device.id.to_raw(), 7);
    assert!(device.is_on());
    assert_eq!(device.vendor_id, 0x1234);
}

#[test]
fn resource_validation() {
    assert!(MmioRegion::new(0, 0).is_none());
    assert!(MmioRegion::new(0x1000, 0x100).is_some());
    assert!(MmioRegion::new(u64::MAX, 2).is_none());
    let Some(region) = MmioRegion::new(0x1000, 0x100) else {
        panic!("expected region");
    };
    assert!(region.contains(0x1000));
    assert!(!region.contains(0x1100));
    assert!(PortIoRegion::new(0x60, 4).is_some());
    assert!(PortIoRegion::new(0xFFFE, 4).is_none());
    assert!(DmaBuffer::new(0x1000, 0x100, 16).is_some());
    assert!(DmaBuffer::new(0x1001, 0x100, 16).is_none());
    assert!(DmaBuffer::new(0x1000, 0x100, 3).is_none());
    let _interrupt = Interrupt::new(5, 1);
    let resource = Resource::Mmio(region);
    assert!(resource.is_mmio());
}

#[test]
fn architecture_widths() {
    assert_eq!(ArchId::from_raw(1), ArchId::X86_64);
    assert!(ArchId::from_raw(1).is_64_bit());
    assert!(!ArchId::from_raw(0).is_64_bit());
    assert_eq!(ArchId::from_raw(99), ArchId::Unknown);
}

#[test]
fn registry_matching() {
    let mut registry = DriverRegistry::new();
    let entry = DriverEntry {
        name: 1,
        matcher: DriverMatch::new(Some(DeviceType::Pci), Some(0x1234), None, None),
    };
    assert!(registry.register(entry));
    assert_eq!(registry.count(), 1);
    assert!(registry.find(DeviceType::Pci, 0x1234, 0x5678, 0).is_some());
    assert!(registry.find(DeviceType::Usb, 0x1234, 0x5678, 0).is_none());
}

#[test]
fn driver_error_display() {
    let mut formatter = String::new();
    let _ = write!(formatter, "{}", DriverError::Unsupported);
    assert!(formatter.contains("unsupported"));
}
