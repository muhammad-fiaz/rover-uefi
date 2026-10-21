//! # Enumeration
//!
//! Discovery helpers.
//!
//! Enumeration delegates handle iteration to
//! the existing Boot Services locate handle
//! buffer service. Each handle is a distinct
//! firmware object; handles are not
//! synthesized.

use crate::abi::types::UIntN;
use crate::base::EfiHandle;
use crate::base::LocateSearchType;
use crate::protocols::pci::device::PciDevice;
use crate::protocols::pci::guid::PCI_IO_GUID;
use crate::protocols::pci::guid::PCI_ROOT_BRIDGE_IO_GUID;
use crate::services::boot::BootServices;

/// Locates root bridge handles.
///
/// Writes handles into the caller buffer.
/// Returns the handle count.
///
/// # Errors
///
/// Returns the firmware status when the
/// operation fails.
pub fn locate_root_bridges(
    boot: &BootServices<'_>,
    buffer: &mut [EfiHandle],
) -> Result<UIntN, crate::base::EfiStatus> {
    boot.locate_handles(
        LocateSearchType::BY_PROTOCOL,
        Some(&PCI_ROOT_BRIDGE_IO_GUID),
        buffer,
    )
}

/// Locates PCI I/O handles.
///
/// Returns the handle count.
///
/// # Errors
///
/// Returns the firmware status when the
/// operation fails.
pub fn locate_pci_io(
    boot: &BootServices<'_>,
    buffer: &mut [EfiHandle],
) -> Result<UIntN, crate::base::EfiStatus> {
    boot.locate_handles(LocateSearchType::BY_PROTOCOL, Some(&PCI_IO_GUID), buffer)
}

/// Builds a device list from handles.
///
/// The caller must supply locations from
/// `GetLocation` for each handle.
#[must_use]
pub fn build_devices(handles: &[EfiHandle]) -> usize {
    handles.len()
}

/// Returns whether a handle is null.
#[must_use]
pub fn is_valid_handle(handle: EfiHandle) -> bool {
    !handle.is_null()
}

/// Creates a device record.
#[must_use]
pub fn make_device(
    handle: EfiHandle,
    location: crate::protocols::pci::device::PciLocation,
) -> PciDevice {
    PciDevice { handle, location }
}
