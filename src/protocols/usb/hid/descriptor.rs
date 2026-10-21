//! # HID report descriptors
//!
//! Parsing of the Human Interface Device (HID) report descriptor.
//!
//! A report descriptor is a sequence of items. A short item starts with
//! a prefix byte that names the size, the type and the tag of the item;
//! a long item starts with the prefix byte `0xFE` and carries its own
//! tag and data size.
//!
//! Global items set the state that applies to every following item
//! until another global item changes it, local items apply to the next
//! main item only, and main items describe the collections and the
//! fields of the report.

/// The maximum number of usages the parser keeps for one field.
pub const MAX_USAGES: usize = 8;

/// The maximum number of nested collections the parser keeps.
pub const MAX_COLLECTION_DEPTH: usize = 4;

/// The type of a report descriptor item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    /// A main item, which describes the structure of the report.
    Main,
    /// A global item, which applies to every following item.
    Global,
    /// A local item, which applies to the next main item only.
    Local,
    /// The reserved item type.
    Reserved,
}

impl ItemType {
    /// Returns the numeric item type.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Main => 0,
            Self::Global => 1,
            Self::Local => 2,
            Self::Reserved => 3,
        }
    }

    /// Creates an item type from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::Main,
            1 => Self::Global,
            2 => Self::Local,
            _ => Self::Reserved,
        }
    }
}

/// The kind of a main item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MainItemType {
    /// A collection, which opens a group of items.
    Collection,
    /// The end of a collection.
    EndCollection,
    /// An input field.
    Input,
    /// An output field.
    Output,
    /// A feature field.
    Feature,
}

impl MainItemType {
    /// Returns the numeric tag of the main item.
    ///
    /// The tag values are the ones the HID specification assigns to
    /// the main items.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Input => 8,
            Self::Output => 9,
            Self::Collection => 10,
            Self::Feature => 11,
            Self::EndCollection => 12,
        }
    }

    /// Creates a main item type from its numeric tag.
    #[must_use]
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            8 => Some(Self::Input),
            9 => Some(Self::Output),
            10 => Some(Self::Collection),
            11 => Some(Self::Feature),
            12 => Some(Self::EndCollection),
            _ => None,
        }
    }
}

/// The type of a collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CollectionType {
    /// A physical collection.
    Physical,
    /// An application collection.
    Application,
    /// A logical collection.
    Logical,
    /// The reserved collection type.
    Reserved,
}

impl CollectionType {
    /// Returns the numeric collection type.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Physical => 0,
            Self::Application => 1,
            Self::Logical => 2,
            Self::Reserved => 3,
        }
    }

    /// Creates a collection type from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x3 {
            0 => Self::Physical,
            1 => Self::Application,
            2 => Self::Logical,
            _ => Self::Reserved,
        }
    }
}

/// The kind of a data field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldKind {
    /// An input field, reported by the device.
    Input,
    /// An output field, sent to the device.
    Output,
    /// A feature field, which the host and the device both access.
    Feature,
}

impl FieldKind {
    /// Returns the numeric tag of the main item.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Input => 8,
            Self::Output => 9,
            Self::Feature => 11,
        }
    }

    /// Creates a field kind from its numeric tag.
    #[must_use]
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            8 => Some(Self::Input),
            9 => Some(Self::Output),
            11 => Some(Self::Feature),
            _ => None,
        }
    }
}

/// The flags of a data field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldFlags(u8);

impl FieldFlags {
    /// The field carries a constant value.
    pub const CONSTANT: Self = Self(0x01);
    /// The field carries a variable value.
    pub const VARIABLE: Self = Self(0x02);
    /// The field value is relative to the previous value.
    pub const RELATIVE: Self = Self(0x04);
    /// The field value wraps around at its maximum.
    pub const WRAP: Self = Self(0x08);
    /// The field value is not linear.
    pub const NON_LINEAR: Self = Self(0x10);
    /// The field does not prefer a null state.
    pub const NO_PREFER: Self = Self(0x20);
    /// The field value is linear.
    pub const LINEAR: Self = Self(0x40);

    /// Creates a flag set from its raw bits.
    #[must_use]
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits)
    }

    /// Returns the raw bits of the flag set.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns whether every flag of `other` is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns whether no flag is set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// A single item of a report descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    /// The type of the item.
    pub item_type: ItemType,
    /// The tag of the item.
    pub tag: u8,
    /// The size of the data of the item in bytes.
    pub size: u8,
    /// The raw data of the item.
    pub value: u32,
    /// Whether the item is a long item.
    pub long_item: bool,
}

impl Item {
    /// Returns the data of the item interpreted as a signed value.
    #[must_use]
    pub fn signed_value(&self) -> i32 {
        match self.size {
            1 => i32::from(i8::from_ne_bytes([
                u8::try_from(self.value & 0xFF).unwrap_or_default()
            ])),
            2 => i32::from(i16::from_ne_bytes([
                u8::try_from(self.value & 0xFF).unwrap_or_default(),
                u8::try_from((self.value >> 8) & 0xFF).unwrap_or_default(),
            ])),
            _ => i32::from_ne_bytes([
                u8::try_from(self.value & 0xFF).unwrap_or_default(),
                u8::try_from((self.value >> 8) & 0xFF).unwrap_or_default(),
                u8::try_from((self.value >> 16) & 0xFF).unwrap_or_default(),
                u8::try_from((self.value >> 24) & 0xFF).unwrap_or_default(),
            ]),
        }
    }
}

/// The collection a field belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CollectionScope {
    /// The type of the collection.
    pub collection_type: CollectionType,
    /// The usage page that was active when the collection opened.
    pub usage_page: u16,
    /// The first usage of the collection.
    pub usage: u16,
}

/// A data field of a report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    /// The kind of the field.
    pub kind: FieldKind,
    /// The identifier of the report the field belongs to.
    pub report_id: u8,
    /// The usage page of the field.
    pub usage_page: u16,
    /// The usages of the field.
    pub usages: [u16; MAX_USAGES],
    /// The number of valid usages.
    pub usage_count: u8,
    /// The lowest usage of the field, when a usage range was given.
    pub usage_minimum: u16,
    /// The highest usage of the field, when a usage range was given.
    pub usage_maximum: u16,
    /// The number of values the field reports.
    pub report_count: u16,
    /// The size of one value in bits.
    pub report_size: u8,
    /// The lowest value the field can report.
    pub logical_minimum: i32,
    /// The highest value the field can report.
    pub logical_maximum: i32,
    /// The lowest physical value of the field.
    pub physical_minimum: i32,
    /// The highest physical value of the field.
    pub physical_maximum: i32,
    /// The unit of the field.
    pub unit: u32,
    /// The exponent of the unit of the field.
    pub unit_exponent: i32,
    /// The designator index of the field.
    pub designator_index: u16,
    /// The lowest designator of the field.
    pub designator_minimum: u16,
    /// The highest designator of the field.
    pub designator_maximum: u16,
    /// The string index of the field.
    pub string_index: u16,
    /// The lowest string of the field.
    pub string_minimum: u16,
    /// The highest string of the field.
    pub string_maximum: u16,
    /// The flags of the field.
    pub flags: FieldFlags,
    /// The offset of the field in bits, counted from the start of the
    /// report.
    pub offset: u32,
    /// The collections the field belongs to, outermost first.
    pub collections: [CollectionScope; MAX_COLLECTION_DEPTH],
    /// The number of valid collections.
    pub collection_depth: u8,
}

impl Field {
    /// Returns whether the field reports a signed value.
    #[must_use]
    pub fn is_signed(&self) -> bool {
        self.logical_minimum < 0
    }

    /// Returns the usage at `index`, if the field has one.
    #[must_use]
    pub fn usage(&self, index: usize) -> Option<u16> {
        if index >= usize::from(self.usage_count) {
            return None;
        }
        self.usages.get(index).copied()
    }

    /// Returns the collection at `index`, if the field has one.
    #[must_use]
    pub fn collection(&self, index: usize) -> Option<CollectionScope> {
        if index >= usize::from(self.collection_depth) {
            return None;
        }
        self.collections.get(index).copied()
    }
}

/// The global state of the report descriptor parser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GlobalState {
    usage_page: u16,
    logical_minimum: i32,
    logical_maximum: i32,
    physical_minimum: i32,
    physical_maximum: i32,
    unit_exponent: i32,
    unit: u32,
    report_size: u8,
    report_id: u8,
    report_count: u16,
}

impl GlobalState {
    const fn new() -> Self {
        Self {
            usage_page: 0,
            logical_minimum: 0,
            logical_maximum: 0,
            physical_minimum: 0,
            physical_maximum: 0,
            unit_exponent: 0,
            unit: 0,
            report_size: 0,
            report_id: 0,
            report_count: 0,
        }
    }
}

/// The local state of the report descriptor parser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct LocalState {
    usages: [u16; MAX_USAGES],
    usage_count: u8,
    usage_minimum: u16,
    usage_maximum: u16,
    designator_index: u16,
    designator_minimum: u16,
    designator_maximum: u16,
    string_index: u16,
    string_minimum: u16,
    string_maximum: u16,
    delimiter: u8,
}

impl LocalState {
    const fn new() -> Self {
        Self {
            usages: [0; MAX_USAGES],
            usage_count: 0,
            usage_minimum: 0,
            usage_maximum: 0,
            designator_index: 0,
            designator_minimum: 0,
            designator_maximum: 0,
            string_index: 0,
            string_minimum: 0,
            string_maximum: 0,
            delimiter: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

/// A parsed report descriptor.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReportDescriptor<const N: usize> {
    items: [Item; N],
    len: usize,
}

impl<const N: usize> ReportDescriptor<N> {
    /// Parses a report descriptor.
    ///
    /// Returns `None` when the descriptor is malformed or holds more
    /// items than the descriptor can store.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let mut items = [Item {
            item_type: ItemType::Main,
            tag: 0,
            size: 0,
            value: 0,
            long_item: false,
        }; N];
        let mut len = 0usize;
        let mut offset = 0usize;
        while offset < bytes.len() {
            let prefix = *bytes.get(offset)?;
            offset += 1;
            let item = if prefix == 0xFE {
                let size = *bytes.get(offset)? as usize;
                offset += 1;
                let tag = *bytes.get(offset)?;
                offset += 1;
                let data = bytes.get(offset..offset.checked_add(size)?)?;
                offset += size;
                let mut value = 0_u32;
                for (index, byte) in data.iter().enumerate() {
                    let shift = u32::try_from(index).unwrap_or_default() * 8;
                    value |= u32::from(*byte) << shift;
                }
                Item {
                    item_type: ItemType::Reserved,
                    tag,
                    size: u8::try_from(size).unwrap_or_default(),
                    value,
                    long_item: true,
                }
            } else {
                let size = match prefix & 0x03 {
                    0 => 0usize,
                    1 => 1,
                    2 => 2,
                    _ => 4,
                };
                let data = bytes.get(offset..offset.checked_add(size)?)?;
                offset += size;
                let mut value = 0_u32;
                for (index, byte) in data.iter().enumerate() {
                    let shift = u32::try_from(index).unwrap_or_default() * 8;
                    value |= u32::from(*byte) << shift;
                }
                Item {
                    item_type: ItemType::from_raw((prefix >> 2) & 0x03),
                    tag: (prefix >> 4) & 0x0F,
                    size: u8::try_from(size).unwrap_or_default(),
                    value,
                    long_item: false,
                }
            };
            let slot = items.get_mut(len)?;
            *slot = item;
            len += 1;
        }
        Some(Self { items, len })
    }

    /// Returns the items of the descriptor.
    #[must_use]
    pub fn items(&self) -> &[Item] {
        self.items.get(..self.len).unwrap_or(&[])
    }

    /// Returns the number of items of the descriptor.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the descriptor holds no item.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns whether the descriptor declares report identifiers.
    #[must_use]
    pub fn has_report_ids(&self) -> bool {
        self.items()
            .iter()
            .any(|item| item.item_type == ItemType::Global && item.tag == 8 && !item.long_item)
    }

    /// Returns the fields of the descriptor.
    #[must_use]
    pub fn fields(&self) -> FieldIter<'_> {
        FieldIter {
            items: self.items(),
            index: 0,
            globals: GlobalState::new(),
            local: LocalState::new(),
            collections: [CollectionScope {
                collection_type: CollectionType::Reserved,
                usage_page: 0,
                usage: 0,
            }; MAX_COLLECTION_DEPTH],
            depth: 0,
            offset: 0,
            offset_report_id: 0,
        }
    }
}

/// An iterator over the fields of a report descriptor.
pub struct FieldIter<'a> {
    items: &'a [Item],
    index: usize,
    globals: GlobalState,
    local: LocalState,
    collections: [CollectionScope; MAX_COLLECTION_DEPTH],
    depth: u8,
    offset: u32,
    offset_report_id: u8,
}

impl FieldIter<'_> {
    /// Returns the number of collections that are currently open.
    #[must_use]
    pub fn collection_depth(&self) -> u8 {
        self.depth
    }
}

impl Iterator for FieldIter<'_> {
    type Item = Field;

    /// Returns the next field of the report descriptor.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.items.get(self.index) {
            self.index += 1;
            if item.long_item {
                continue;
            }
            match item.item_type {
                ItemType::Main => {
                    if let Some(field) = self.main_item(*item) {
                        return Some(field);
                    }
                }
                ItemType::Global => self.global_item(*item),
                ItemType::Local => self.local_item(*item),
                ItemType::Reserved => {}
            }
        }
        None
    }
}

impl FieldIter<'_> {
    fn main_item(&mut self, item: Item) -> Option<Field> {
        match item.tag {
            10 => {
                let usage = self.local.usages.first().copied().unwrap_or(0);
                let scope = CollectionScope {
                    collection_type: CollectionType::from_raw(
                        u8::try_from(item.value & 0x3).unwrap_or_default(),
                    ),
                    usage_page: self.globals.usage_page,
                    usage,
                };
                if let Some(slot) = self.collections.get_mut(usize::from(self.depth)) {
                    *slot = scope;
                }
                self.depth = self.depth.saturating_add(1);
                self.local.reset();
                None
            }
            12 => {
                if self.depth > 0 {
                    self.depth -= 1;
                }
                self.local.reset();
                None
            }
            _ => {
                let kind = FieldKind::from_raw(item.tag)?;
                let field = self.build_field(kind, item);
                self.local.reset();
                Some(field)
            }
        }
    }

    fn build_field(&mut self, kind: FieldKind, item: Item) -> Field {
        let report_id = self.globals.report_id;
        if report_id != self.offset_report_id {
            self.offset_report_id = report_id;
            self.offset = 0;
        }
        let mut collections = [CollectionScope {
            collection_type: CollectionType::Reserved,
            usage_page: 0,
            usage: 0,
        }; MAX_COLLECTION_DEPTH];
        if let (Some(slice), Some(source)) = (
            collections.get_mut(..usize::from(self.depth)),
            self.collections.get(..usize::from(self.depth)),
        ) {
            slice.copy_from_slice(source);
        }
        let field = Field {
            kind,
            report_id,
            usage_page: self.globals.usage_page,
            usages: self.local.usages,
            usage_count: self.local.usage_count,
            usage_minimum: self.local.usage_minimum,
            usage_maximum: self.local.usage_maximum,
            report_count: self.globals.report_count,
            report_size: self.globals.report_size,
            logical_minimum: self.globals.logical_minimum,
            logical_maximum: self.globals.logical_maximum,
            physical_minimum: self.globals.physical_minimum,
            physical_maximum: self.globals.physical_maximum,
            unit: self.globals.unit,
            unit_exponent: self.globals.unit_exponent,
            designator_index: self.local.designator_index,
            designator_minimum: self.local.designator_minimum,
            designator_maximum: self.local.designator_maximum,
            string_index: self.local.string_index,
            string_minimum: self.local.string_minimum,
            string_maximum: self.local.string_maximum,
            flags: FieldFlags::from_bits_truncate(
                u8::try_from(item.value & 0x7F).unwrap_or_default(),
            ),
            offset: self.offset,
            collections,
            collection_depth: self.depth,
        };
        self.offset += u32::from(self.globals.report_size) * u32::from(self.globals.report_count);
        field
    }

    fn global_item(&mut self, item: Item) {
        match item.tag {
            0 => self.globals.usage_page = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            1 => self.globals.logical_minimum = item.signed_value(),
            2 => self.globals.logical_maximum = item.signed_value(),
            3 => self.globals.physical_minimum = item.signed_value(),
            4 => self.globals.physical_maximum = item.signed_value(),
            5 => self.globals.unit_exponent = item.signed_value(),
            6 => self.globals.unit = item.value,
            7 => self.globals.report_size = u8::try_from(item.value & 0xFF).unwrap_or_default(),
            8 => self.globals.report_id = u8::try_from(item.value & 0xFF).unwrap_or_default(),
            9 => self.globals.report_count = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            _ => {}
        }
    }

    fn local_item(&mut self, item: Item) {
        match item.tag {
            0 => {
                let index = usize::from(self.local.usage_count);
                if let Some(slot) = self.local.usages.get_mut(index) {
                    *slot = u16::try_from(item.value & 0xFFFF).unwrap_or_default();
                }
                if self.local.usage_count < u8::try_from(MAX_USAGES).unwrap_or(u8::MAX) {
                    self.local.usage_count += 1;
                }
            }
            1 => self.local.usage_minimum = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            2 => self.local.usage_maximum = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            3 => {
                self.local.designator_index =
                    u16::try_from(item.value & 0xFFFF).unwrap_or_default();
            }
            4 => {
                self.local.designator_minimum =
                    u16::try_from(item.value & 0xFFFF).unwrap_or_default();
            }
            5 => {
                self.local.designator_maximum =
                    u16::try_from(item.value & 0xFFFF).unwrap_or_default();
            }
            6 => self.local.string_index = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            7 => self.local.string_minimum = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            8 => self.local.string_maximum = u16::try_from(item.value & 0xFFFF).unwrap_or_default(),
            9 => self.local.delimiter = u8::try_from(item.value & 0xFF).unwrap_or_default(),
            _ => {}
        }
    }
}
