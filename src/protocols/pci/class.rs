//! # Device classes
//!
//! PCI class code identification.

/// Mass storage base class.
pub const BASE_CLASS_MASS_STORAGE: u8 = 0x01;

/// Network base class.
pub const BASE_CLASS_NETWORK: u8 = 0x02;

/// Display base class.
pub const BASE_CLASS_DISPLAY: u8 = 0x03;

/// Serial bus base class.
pub const BASE_CLASS_SERIAL_BUS: u8 = 0x0C;

/// Mass storage SATA subclass.
pub const SUBCLASS_SATA: u8 = 0x06;

/// Network Ethernet subclass.
pub const SUBCLASS_ETHERNET: u8 = 0x00;

/// Display VGA subclass.
pub const SUBCLASS_VGA: u8 = 0x00;

/// Returns whether the class is mass storage.
#[must_use]
pub fn is_mass_storage(base_class: u8) -> bool {
    base_class == BASE_CLASS_MASS_STORAGE
}

/// Returns whether the class is a network
/// controller.
#[must_use]
pub fn is_network(base_class: u8, subclass: u8) -> bool {
    base_class == BASE_CLASS_NETWORK && subclass == SUBCLASS_ETHERNET
}

/// Returns whether the class is a display
/// controller.
#[must_use]
pub fn is_display(base_class: u8) -> bool {
    base_class == BASE_CLASS_DISPLAY
}
