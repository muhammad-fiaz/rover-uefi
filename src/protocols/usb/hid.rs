//! # USB Human Interface Device class
//!
//! Constants, descriptors and a bounded parser for the `HID` class
//! report descriptor defined by the USB Device Class Definition for
//! Human Interface Devices.
//!
//! The parser is allocation-free. It walks the item stream and records
//! each main item as a [`ReportField`] in a fixed-capacity table. The
//! caller decides whether to ignore a report descriptor that exceeds the
//! table capacity; an oversized descriptor is reported as
//! [`HidError::TooManyFields`] rather than truncated silently.
//!
//! The parser does not interpret usages into human-readable names and
//! does not implement a keyboard or mouse state machine. It resolves the
//! structural information a driver needs to locate a field inside an
//! input, output or feature report.

use crate::base::bytes::{read_u16_le, read_u8};

/// The `HID` descriptor type in a configuration descriptor.
pub const HID_DESCRIPTOR_TYPE: u8 = 0x21;

/// The `HID` report descriptor type.
pub const HID_REPORT_DESCRIPTOR_TYPE: u8 = 0x22;

/// The `HID` physical descriptor type.
pub const HID_PHYSICAL_DESCRIPTOR_TYPE: u8 = 0x23;

/// The class request to retrieve a report.
pub const HID_REQUEST_GET_REPORT: u8 = 0x01;

/// The class request to retrieve an idle rate.
pub const HID_REQUEST_GET_IDLE: u8 = 0x02;

/// The class request to retrieve the protocol.
pub const HID_REQUEST_GET_PROTOCOL: u8 = 0x03;

/// The class request to set a report.
pub const HID_REQUEST_SET_REPORT: u8 = 0x09;

/// The class request to set an idle rate.
pub const HID_REQUEST_SET_IDLE: u8 = 0x0A;

/// The class request to set the protocol.
pub const HID_REQUEST_SET_PROTOCOL: u8 = 0x0B;

/// The boot protocol.
pub const HID_PROTOCOL_BOOT: u8 = 0;

/// The report protocol.
pub const HID_PROTOCOL_REPORT: u8 = 1;

/// The request type for a class `IN` request to the interface.
pub const HID_REQUEST_TYPE_IN: u8 = 0xA1;

/// The request type for a class `OUT` request to the interface.
pub const HID_REQUEST_TYPE_OUT: u8 = 0x21;

/// A `HID` class descriptor.
///
/// The descriptor is six bytes in its original form and nine bytes when
/// it declares a report descriptor. The optional trailing fields are
/// ignored by this view because the length is supplied by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HidDescriptor {
    /// The descriptor length in bytes.
    pub length: u8,
    /// The descriptor type, always [`HID_DESCRIPTOR_TYPE`].
    pub descriptor_type: u8,
    /// The `HID` specification release in binary-coded decimal.
    pub bcd_hid: u16,
    /// The country code.
    pub country_code: u8,
    /// The number of class descriptors that follow.
    pub class_descriptor_count: u8,
    /// The type of the first class descriptor.
    pub report_descriptor_type: u8,
    /// The length of the first class descriptor.
    pub report_descriptor_length: u16,
}

impl HidDescriptor {
    /// Parses a `HID` class descriptor.
    ///
    /// Returns `None` when the input is shorter than the required
    /// fields or when the declared type is not [`HID_DESCRIPTOR_TYPE`].
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let length = read_u8(bytes, 0)?;
        let descriptor_type = read_u8(bytes, 1)?;
        if descriptor_type != HID_DESCRIPTOR_TYPE || length < 6 {
            return None;
        }
        let declared = usize::from(length);
        if bytes.len() < declared {
            return None;
        }
        let has_report = length >= 9 && bytes.len() >= 9;
        Some(Self {
            length,
            descriptor_type,
            bcd_hid: read_u16_le(bytes, 2)?,
            country_code: read_u8(bytes, 4)?,
            class_descriptor_count: read_u8(bytes, 5)?,
            report_descriptor_type: if has_report {
                read_u8(bytes, 6)?
            } else {
                0
            },
            report_descriptor_length: if has_report {
                read_u16_le(bytes, 7)?
            } else {
                0
            },
        })
    }
}

/// The kind of report a field belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReportType {
    /// An input report sent by the device.
    Input = 0,
    /// An output report sent to the device.
    Output = 1,
    /// A feature report read or written on demand.
    Feature = 2,
}

impl ReportType {
    /// Returns the report type as a stable index.
    ///
    /// The values are `0` for input, `1` for output and `2` for
    /// feature, matching the order used by the report tables.
    #[must_use]
    pub const fn as_index(self) -> usize {
        match self {
            Self::Input => 0,
            Self::Output => 1,
            Self::Feature => 2,
        }
    }
}

/// A field declared by a main item of a report descriptor.
///
/// `bit_offset` is the bit position of the first bit of the field inside
/// the report identified by `report_id`. The offset is recomputed for
/// each report identifier and report type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReportField {
    /// The usage page of the field.
    pub usage_page: u16,
    /// The first usage of the field, or zero when none was declared.
    pub usage: u16,
    /// The report identifier, or zero when the descriptor uses no
    /// report identifiers.
    pub report_id: u8,
    /// The size of one field element in bits.
    pub report_size: u8,
    /// The number of elements in the field.
    pub report_count: u16,
    /// The logical minimum of the field.
    pub logical_minimum: i32,
    /// The logical maximum of the field.
    pub logical_maximum: i32,
    /// The bit offset inside the report.
    pub bit_offset: u32,
    /// The kind of report the field belongs to.
    pub report_type: ReportType,
    /// The raw main-item flags.
    ///
    /// Bit semantics follow the specification: `1` constant, `2`
    /// variable, `4` relative, `8` wrap, `16` non-linear, `32` no
    /// preferred state, `64` null state, `128` buffered bytes.
    pub flags: u8,
}

impl ReportField {
    /// Returns whether the field is constant padding.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Returns whether the field is an array rather than a variable.
    #[must_use]
    pub const fn is_array(&self) -> bool {
        self.flags & 0x02 == 0
    }

    /// Returns whether the field reports relative movement.
    #[must_use]
    pub const fn is_relative(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Returns the total size of the field in bits.
    ///
    /// Returns `None` on overflow.
    #[must_use]
    pub fn total_bits(&self) -> Option<u32> {
        u32::from(self.report_size).checked_mul(u32::from(self.report_count))
    }
}

/// The maximum number of report fields retained by a parsed descriptor.
pub const MAX_REPORT_FIELDS: usize = 64;

/// Errors reported while parsing a report descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HidError {
    /// The descriptor ended in the middle of an item.
    Truncated,
    /// The descriptor contains an item with an invalid prefix.
    InvalidItem,
    /// The descriptor declares more fields than the parser retains.
    TooManyFields,
    /// A push or pop item unbalanced the global state stack.
    GlobalStackUnderflow,
    /// A report identifier outside `0..=255` was declared.
    InvalidReportId,
    /// The nested collection depth exceeds the supported limit.
    CollectionDepth,
}

/// The maximum nesting depth of collections.
pub const MAX_COLLECTION_DEPTH: usize = 16;

/// A parsed `HID` report descriptor.
///
/// The structure keeps the report fields and the total bit length of
/// each `(report_id, report_type)` pair. The report lengths are exposed
/// through [`ReportDescriptor::report_bits`].
#[derive(Clone)]
pub struct ReportDescriptor {
    fields: [ReportField; MAX_REPORT_FIELDS],
    field_count: usize,
    input_bits: [u32; 256],
    output_bits: [u32; 256],
    feature_bits: [u32; 256],
    has_report_ids: bool,
}

impl ReportDescriptor {
    /// Parses a report descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`HidError`] when the descriptor is truncated, contains an
    /// invalid item, nests collections too deeply, or declares more
    /// fields than [`MAX_REPORT_FIELDS`].
    pub fn parse(bytes: &[u8]) -> Result<Self, HidError> {
        let mut parser = Parser::new();
        parser.run(bytes)?;
        Ok(parser.finish())
    }

    /// Returns the parsed fields.
    #[must_use]
    pub fn fields(&self) -> &[ReportField] {
        &self.fields[..self.field_count]
    }

    /// Returns the number of parsed fields.
    #[must_use]
    pub const fn field_count(&self) -> usize {
        self.field_count
    }

    /// Returns whether the descriptor uses report identifiers.
    #[must_use]
    pub const fn has_report_ids(&self) -> bool {
        self.has_report_ids
    }

    /// Returns the total bit length of a report.
    ///
    /// `report_id` is zero when the descriptor does not use identifiers.
    #[must_use]
    pub fn report_bits(&self, report_id: u8, report_type: ReportType) -> u32 {
        let table = match report_type {
            ReportType::Input => &self.input_bits,
            ReportType::Output => &self.output_bits,
            ReportType::Feature => &self.feature_bits,
        };
        table[usize::from(report_id)]
    }

    /// Returns the total byte length of a report.
    ///
    /// The byte length rounds up to a whole byte because reports are
    /// transferred as byte strings.
    #[must_use]
    pub fn report_bytes(&self, report_id: u8, report_type: ReportType) -> usize {
        usize::try_from(self.report_bits(report_id, report_type).div_ceil(8)).unwrap_or(usize::MAX)
    }

    /// Returns the first field that matches a usage page and usage.
    #[must_use]
    pub fn find_field(&self, usage_page: u16, usage: u16) -> Option<&ReportField> {
        self.fields()
            .iter()
            .find(|field| field.usage_page == usage_page && field.usage == usage)
    }
}

impl core::fmt::Debug for ReportDescriptor {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ReportDescriptor")
            .field("fields", &self.fields())
            .field("has_report_ids", &self.has_report_ids)
            .finish()
    }
}

/// The current global item state.
#[derive(Clone, Copy)]
struct GlobalState {
    usage_page: u16,
    logical_minimum: i32,
    logical_maximum: i32,
    report_size: u8,
    report_count: u16,
    report_id: u8,
}

impl GlobalState {
    const fn new() -> Self {
        Self {
            usage_page: 0,
            logical_minimum: 0,
            logical_maximum: 0,
            report_size: 0,
            report_count: 0,
            report_id: 0,
        }
    }
}

struct Parser {
    fields: [ReportField; MAX_REPORT_FIELDS],
    field_count: usize,
    input_bits: [u32; 256],
    output_bits: [u32; 256],
    feature_bits: [u32; 256],
    has_report_ids: bool,
    global: GlobalState,
    global_stack: [GlobalState; 8],
    global_depth: usize,
    usages: [u16; 16],
    usage_count: usize,
    usage_page_override: Option<u16>,
    collection_depth: usize,
}

impl Parser {
    fn new() -> Self {
        Self {
            fields: [ReportField {
                usage_page: 0,
                usage: 0,
                report_id: 0,
                report_size: 0,
                report_count: 0,
                logical_minimum: 0,
                logical_maximum: 0,
                bit_offset: 0,
                report_type: ReportType::Input,
                flags: 0,
            }; MAX_REPORT_FIELDS],
            field_count: 0,
            input_bits: [0; 256],
            output_bits: [0; 256],
            feature_bits: [0; 256],
            has_report_ids: false,
            global: GlobalState::new(),
            global_stack: [GlobalState::new(); 8],
            global_depth: 0,
            usages: [0; 16],
            usage_count: 0,
            usage_page_override: None,
            collection_depth: 0,
        }
    }

    fn run(&mut self, bytes: &[u8]) -> Result<(), HidError> {
        let mut index = 0;
        while index < bytes.len() {
            let prefix = bytes[index];
            if prefix == 0xFE {
                // Long item: bDataSize, bLongItemTag, data.
                let size = *bytes.get(index + 1).ok_or(HidError::Truncated)?;
                let end = index
                    .checked_add(3)
                    .and_then(|value| value.checked_add(usize::from(size)))
                    .ok_or(HidError::Truncated)?;
                if end > bytes.len() {
                    return Err(HidError::Truncated);
                }
                index = end;
                continue;
            }
            let size_code = usize::from(prefix & 0x03);
            let size = match size_code {
                0 => 0,
                1 | 2 => size_code,
                3 => 4,
                _ => return Err(HidError::InvalidItem),
            };
            let item_type = (prefix >> 2) & 0x03;
            let tag = prefix >> 4;
            let start = index + 1;
            let end = start.checked_add(size).ok_or(HidError::Truncated)?;
            if end > bytes.len() {
                return Err(HidError::Truncated);
            }
            let data = &bytes[start..end];
            index = end;

            match item_type {
                0 => self.main_item(tag, data)?,
                1 => self.global_item(tag, data)?,
                2 => self.local_item(tag, data),
                _ => {}
            }
        }
        Ok(())
    }

    fn main_item(&mut self, tag: u8, data: &[u8]) -> Result<(), HidError> {
        match tag {
            0x8 => self.add_field(ReportType::Input, data)?,
            0x9 => self.add_field(ReportType::Output, data)?,
            0xB => self.add_field(ReportType::Feature, data)?,
            0xA => {
                if self.collection_depth >= MAX_COLLECTION_DEPTH {
                    return Err(HidError::CollectionDepth);
                }
                self.collection_depth += 1;
                self.usage_count = 0;
            }
            0xC => {
                if self.collection_depth == 0 {
                    return Err(HidError::InvalidItem);
                }
                self.collection_depth -= 1;
                self.usage_count = 0;
            }
            _ => {}
        }
        Ok(())
    }

    fn add_field(&mut self, report_type: ReportType, data: &[u8]) -> Result<(), HidError> {
        let flags = data.first().copied().unwrap_or(0);
        let usage = self.usages.first().copied().unwrap_or(0);
        let usage_page = self.usage_page_override.unwrap_or(self.global.usage_page);
        let report_id = self.global.report_id;
        let total = u32::from(self.global.report_size)
            .checked_mul(u32::from(self.global.report_count))
            .ok_or(HidError::InvalidItem)?;
        let offset = {
            let table = match report_type {
                ReportType::Input => &mut self.input_bits,
                ReportType::Output => &mut self.output_bits,
                ReportType::Feature => &mut self.feature_bits,
            };
            let offset = table[usize::from(report_id)];
            table[usize::from(report_id)] = offset.checked_add(total).ok_or(HidError::InvalidItem)?;
            offset
        };
        if self.field_count >= MAX_REPORT_FIELDS {
            return Err(HidError::TooManyFields);
        }
        self.fields[self.field_count] = ReportField {
            usage_page,
            usage,
            report_id,
            report_size: self.global.report_size,
            report_count: self.global.report_count,
            logical_minimum: self.global.logical_minimum,
            logical_maximum: self.global.logical_maximum,
            bit_offset: offset,
            report_type,
            flags,
        };
        self.field_count += 1;
        self.usage_count = 0;
        self.usage_page_override = None;
        Ok(())
    }

    fn global_item(&mut self, tag: u8, data: &[u8]) -> Result<(), HidError> {
        match tag {
            0x0 => {
                self.global.usage_page = u16::try_from(extract_unsigned(data)).unwrap_or(0);
            }
            0x1 => self.global.logical_minimum = extract_signed(data),
            0x2 => self.global.logical_maximum = extract_signed(data),
            0x7 => self.global.report_size = u8::try_from(extract_unsigned(data)).unwrap_or(0),
            0x8 => {
                let id = u8::try_from(extract_unsigned(data)).map_err(|_| HidError::InvalidReportId)?;
                self.global.report_id = id;
                self.has_report_ids = true;
            }
            0x9 => self.global.report_count = u16::try_from(extract_unsigned(data)).unwrap_or(0),
            0xA => {
                if self.global_depth >= self.global_stack.len() {
                    return Err(HidError::GlobalStackUnderflow);
                }
                self.global_stack[self.global_depth] = self.global;
                self.global_depth += 1;
            }
            0xB => {
                if self.global_depth == 0 {
                    return Err(HidError::GlobalStackUnderflow);
                }
                self.global_depth -= 1;
                self.global = self.global_stack[self.global_depth];
            }
            _ => {}
        }
        Ok(())
    }

    fn local_item(&mut self, tag: u8, data: &[u8]) {
        if tag != 0x0 {
            return;
        }
        let raw = extract_unsigned(data);
        if data.len() == 4 {
            // Extended usage: the high half carries an explicit page.
            if self.usage_count < self.usages.len() {
                self.usages[self.usage_count] = u16::try_from(raw & 0xFFFF).unwrap_or(0);
                self.usage_count += 1;
            }
            self.usage_page_override = Some(u16::try_from(raw >> 16).unwrap_or(0));
        } else if self.usage_count < self.usages.len() {
            self.usages[self.usage_count] = u16::try_from(raw).unwrap_or(0);
            self.usage_count += 1;
        }
    }

    fn finish(self) -> ReportDescriptor {
        ReportDescriptor {
            fields: self.fields,
            field_count: self.field_count,
            input_bits: self.input_bits,
            output_bits: self.output_bits,
            feature_bits: self.feature_bits,
            has_report_ids: self.has_report_ids,
        }
    }
}

/// Extracts the unsigned value of an item's data bytes.
fn extract_unsigned(data: &[u8]) -> u32 {
    match data.len() {
        0 => 0,
        1 => u32::from(data[0]),
        2 => u32::from(u16::from_le_bytes([data[0], data[1]])),
        4 => u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        _ => 0,
    }
}

/// Extracts the signed value of an item's data bytes.
fn extract_signed(data: &[u8]) -> i32 {
    match data.len() {
        0 => 0,
        1 => i32::from(data[0] as i8),
        2 => i32::from(i16::from_le_bytes([data[0], data[1]])),
        4 => i32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{HidDescriptor, HidError, ReportDescriptor, ReportType};

    /// A boot keyboard report descriptor.
    const KEYBOARD: &[u8] = &[
        0x05, 0x01, // Usage page (Generic Desktop)
        0x09, 0x06, // Usage (Keyboard)
        0xA1, 0x01, // Collection (Application)
        0x05, 0x07, // Usage page (Keyboard)
        0x19, 0xE0, // Usage minimum (224)
        0x29, 0xE7, // Usage maximum (231)
        0x15, 0x00, // Logical minimum (0)
        0x25, 0x01, // Logical maximum (1)
        0x75, 0x01, // Report size (1)
        0x95, 0x08, // Report count (8)
        0x81, 0x02, // Input (Data, Variable, Absolute)
        0x95, 0x01, // Report count (1)
        0x75, 0x08, // Report size (8)
        0x81, 0x01, // Input (Constant)
        0x95, 0x06, // Report count (6)
        0x75, 0x08, // Report size (8)
        0x15, 0x00, // Logical minimum (0)
        0x25, 0x65, // Logical maximum (101)
        0x05, 0x07, // Usage page (Keyboard)
        0x19, 0x00, // Usage minimum (0)
        0x29, 0x65, // Usage maximum (101)
        0x81, 0x00, // Input (Data, Array)
        0xC0, // End collection
    ];

    #[test]
    fn parses_keyboard_descriptor() {
        let descriptor = ReportDescriptor::parse(KEYBOARD).expect("keyboard descriptor");
        assert_eq!(descriptor.field_count(), 3);
        assert!(!descriptor.has_report_ids());
        // 8 modifier bits + 8 reserved bits + 6 key slots.
        assert_eq!(descriptor.report_bits(0, ReportType::Input), 64);
        assert_eq!(descriptor.report_bytes(0, ReportType::Input), 8);
    }

    #[test]
    fn field_layout() {
        let descriptor = ReportDescriptor::parse(KEYBOARD).expect("keyboard descriptor");
        let fields = descriptor.fields();
        assert_eq!(fields[0].bit_offset, 0);
        assert_eq!(fields[0].report_size, 1);
        assert_eq!(fields[0].report_count, 8);
        assert!(fields[0].is_array());
        assert_eq!(fields[1].bit_offset, 8);
        assert!(fields[1].is_constant());
        assert_eq!(fields[2].bit_offset, 16);
        assert_eq!(fields[2].report_count, 6);
    }

    #[test]
    fn rejects_truncated_descriptor() {
        assert_eq!(ReportDescriptor::parse(&[0x05]), Err(HidError::Truncated));
        assert_eq!(
            ReportDescriptor::parse(&[0x06, 0x01]),
            Err(HidError::Truncated)
        );
    }

    #[test]
    fn rejects_unbalanced_collections() {
        assert_eq!(ReportDescriptor::parse(&[0xC0]), Err(HidError::InvalidItem));
    }

    #[test]
    fn parses_class_descriptor() {
        let bytes = [0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22, 0x3F, 0x00];
        let descriptor = HidDescriptor::parse(&bytes).expect("descriptor");
        assert_eq!(descriptor.length, 9);
        assert_eq!(descriptor.bcd_hid, 0x0111);
        assert_eq!(descriptor.class_descriptor_count, 1);
        assert_eq!(descriptor.report_descriptor_type, 0x22);
        assert_eq!(descriptor.report_descriptor_length, 0x3F);
    }

    #[test]
    fn rejects_wrong_descriptor_type() {
        let bytes = [0x09, 0x22, 0x11, 0x01, 0x00, 0x01, 0x22, 0x3F, 0x00];
        assert!(HidDescriptor::parse(&bytes).is_none());
    }
}
