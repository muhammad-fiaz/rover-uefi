//! # USB descriptors
//!
//! Standard descriptor parsing.
//!
//! Parsing uses byte slices with checked
//! lengths and little-endian reads.

/// Descriptor type constants.
pub const DESC_DEVICE: u8 = 0x01;

/// Configuration descriptor type.
pub const DESC_CONFIGURATION: u8 = 0x02;

/// String descriptor type.
pub const DESC_STRING: u8 = 0x03;

/// Interface descriptor type.
pub const DESC_INTERFACE: u8 = 0x04;

/// Endpoint descriptor type.
pub const DESC_ENDPOINT: u8 = 0x05;

/// A device descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceDescriptor {
    /// The USB version.
    pub bcd_usb: u16,
    /// The device class.
    pub device_class: u8,
    /// The vendor ID.
    pub id_vendor: u16,
    /// The product ID.
    pub id_product: u16,
    /// The configuration count.
    pub num_configurations: u8,
}

/// A configuration descriptor header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConfigDescriptor {
    /// The total length.
    pub total_length: u16,
    /// The interface count.
    pub num_interfaces: u8,
    /// The configuration value.
    pub configuration_value: u8,
}

/// An interface descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceDescriptor {
    /// The interface number.
    pub interface_number: u8,
    /// The alternate setting.
    pub alternate_setting: u8,
    /// The endpoint count.
    pub num_endpoints: u8,
    /// The interface class.
    pub interface_class: u8,
}

/// An endpoint descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EndpointDescriptor {
    /// The endpoint address.
    pub endpoint_address: u8,
    /// The attributes.
    pub attributes: u8,
    /// The maximum packet size.
    pub max_packet_size: u16,
    /// The polling interval.
    pub interval: u8,
}

/// A generic descriptor view.
pub struct Descriptor<'a> {
    descriptor_type: u8,
    bytes: &'a [u8],
}

impl Descriptor<'_> {
    /// Returns the descriptor type.
    #[must_use]
    pub const fn descriptor_type(&self) -> u8 {
        self.descriptor_type
    }

    /// Returns the bytes.
    #[must_use]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

/// Iterates descriptors in a configuration.
pub struct DescriptorIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for DescriptorIter<'a> {
    type Item = Descriptor<'a>;

    /// Returns the next descriptor.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 2 > self.bytes.len() {
            return None;
        }
        let length = *self.bytes.get(self.offset)? as usize;
        if length < 2 || self.offset + length > self.bytes.len() {
            return None;
        }
        let descriptor_type = *self.bytes.get(self.offset + 1)?;
        let slice = self.bytes.get(self.offset..self.offset + length)?;
        self.offset += length;
        Some(Descriptor {
            descriptor_type,
            bytes: slice,
        })
    }
}

/// Creates a descriptor iterator.
#[must_use]
pub fn descriptor_iter(bytes: &[u8]) -> DescriptorIter<'_> {
    DescriptorIter { bytes, offset: 0 }
}

/// Parses a device descriptor.
#[must_use]
pub fn parse_device_descriptor(bytes: &[u8]) -> Option<DeviceDescriptor> {
    if bytes.len() < 18 {
        return None;
    }
    let mut vendor = [0_u8; 2];
    vendor.copy_from_slice(bytes.get(8..10)?);
    let mut product = [0_u8; 2];
    product.copy_from_slice(bytes.get(10..12)?);
    let mut usb = [0_u8; 2];
    usb.copy_from_slice(bytes.get(2..4)?);
    Some(DeviceDescriptor {
        bcd_usb: u16::from_le_bytes(usb),
        device_class: *bytes.get(4)?,
        id_vendor: u16::from_le_bytes(vendor),
        id_product: u16::from_le_bytes(product),
        num_configurations: *bytes.get(17)?,
    })
}

/// Parses a configuration header.
#[must_use]
pub fn parse_config_descriptor(bytes: &[u8]) -> Option<ConfigDescriptor> {
    if bytes.len() < 9 {
        return None;
    }
    let mut total = [0_u8; 2];
    total.copy_from_slice(bytes.get(2..4)?);
    Some(ConfigDescriptor {
        total_length: u16::from_le_bytes(total),
        num_interfaces: *bytes.get(4)?,
        configuration_value: *bytes.get(5)?,
    })
}

/// Parses an interface descriptor.
#[must_use]
pub fn parse_interface_descriptor(bytes: &[u8]) -> Option<InterfaceDescriptor> {
    if bytes.len() < 9 {
        return None;
    }
    Some(InterfaceDescriptor {
        interface_number: *bytes.get(2)?,
        alternate_setting: *bytes.get(3)?,
        num_endpoints: *bytes.get(4)?,
        interface_class: *bytes.get(5)?,
    })
}

/// Parses an endpoint descriptor.
#[must_use]
pub fn parse_endpoint_descriptor(bytes: &[u8]) -> Option<EndpointDescriptor> {
    if bytes.len() < 7 {
        return None;
    }
    let mut max_packet = [0_u8; 2];
    max_packet.copy_from_slice(bytes.get(4..6)?);
    Some(EndpointDescriptor {
        endpoint_address: *bytes.get(2)?,
        attributes: *bytes.get(3)?,
        max_packet_size: u16::from_le_bytes(max_packet),
        interval: *bytes.get(6)?,
    })
}
