//! # Native driver foundations
//!
//! Reusable device, bus, registry, resource, and lifecycle
//! abstractions for future Rover components.
//!
//! These types are distinct from firmware protocol wrappers. They
//! model native device management (matching, ownership, MMIO, port
//! I/O, DMA, interrupts) without requiring hardware access.

pub mod bus;
pub mod device;
pub mod lifecycle;
pub mod registry;
pub mod resources;

pub use bus::{ArchId, BusId};
pub use device::{Device, DeviceId, DeviceType, PowerState};
pub use lifecycle::{Driver, DriverError};
pub use registry::{DriverEntry, DriverMatch, DriverRegistry, MAX_DRIVERS};
pub use resources::{DmaBuffer, Interrupt, MmioRegion, PortIoRegion, Resource};
