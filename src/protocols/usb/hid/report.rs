//! # HID reports
//!
//! Decoding of Human Interface Device (HID) input reports.
//!
//! A report is decoded with the help of a parsed report descriptor. The
//! descriptor tells the decoder how many bits every field occupies, in
//! which order the fields appear in the report and whether a value is
//! signed.

use crate::protocols::usb::hid::descriptor::{Field, FieldKind, MAX_USAGES, ReportDescriptor};

/// The maximum number of values one field can report.
pub const MAX_FIELD_VALUES: usize = 8;

/// The value of a decoded field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldValue {
    /// A field that reports a single value.
    Scalar(i64),
    /// A field that reports an array of values.
    Array {
        /// The values of the field.
        values: [i64; MAX_FIELD_VALUES],
        /// The number of valid values.
        count: u8,
    },
}

impl FieldValue {
    /// Returns the value at `index`, if the field has one.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<i64> {
        match self {
            Self::Scalar(value) => {
                if index == 0 {
                    Some(*value)
                } else {
                    None
                }
            }
            Self::Array { values, count } => {
                if index >= usize::from(*count) {
                    return None;
                }
                values.get(index).copied()
            }
        }
    }

    /// Returns the number of values of the field.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Scalar(_) => 1,
            Self::Array { count, .. } => usize::from(*count),
        }
    }

    /// Returns whether the field reports no value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A field of a report together with its decoded value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecodedField {
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
    /// The offset of the field in bits, counted from the start of the
    /// report.
    pub offset: u32,
    /// The size of one value in bits.
    pub report_size: u8,
    /// Whether the values of the field are signed.
    pub signed: bool,
    /// The decoded value of the field.
    pub value: FieldValue,
}

impl DecodedField {
    /// Returns the usage at `index`, if the field has one.
    #[must_use]
    pub fn usage(&self, index: usize) -> Option<u16> {
        if index >= usize::from(self.usage_count) {
            return None;
        }
        self.usages.get(index).copied()
    }
}

/// An iterator over the decoded fields of a report.
pub struct ReportIter<'a, const N: usize> {
    descriptor: &'a ReportDescriptor<N>,
    report: &'a [u8],
    fields: crate::protocols::usb::hid::descriptor::FieldIter<'a>,
    report_id: Option<u8>,
    /// The offset of the report data in bits. It skips the leading
    /// report identifier byte when the descriptor declares identifiers.
    data_offset: u32,
}

impl<'a, const N: usize> ReportIter<'a, N> {
    /// Returns the identifier of the report, if the descriptor declares
    /// report identifiers.
    #[must_use]
    pub fn report_id(&self) -> Option<u8> {
        self.report_id
    }

    /// Returns the descriptor the report is decoded with.
    #[must_use]
    pub fn descriptor(&self) -> &'a ReportDescriptor<N> {
        self.descriptor
    }
}

impl<const N: usize> ReportDescriptor<N> {
    /// Decodes an input report.
    ///
    /// When the descriptor declares report identifiers, the first byte
    /// of the report holds the identifier and only the fields of that
    /// report are decoded.
    #[must_use]
    pub fn parse_report<'a>(&'a self, report: &'a [u8]) -> ReportIter<'a, N> {
        let has_report_ids = self.has_report_ids();
        let report_id = if has_report_ids {
            report.first().copied()
        } else {
            None
        };
        ReportIter {
            descriptor: self,
            report,
            fields: self.fields(),
            report_id,
            data_offset: if has_report_ids { 8 } else { 0 },
        }
    }
}

impl<const N: usize> Iterator for ReportIter<'_, N> {
    type Item = DecodedField;

    /// Returns the next decoded field of the report.
    fn next(&mut self) -> Option<Self::Item> {
        for field in self.fields.by_ref() {
            if self
                .report_id
                .is_some_and(|report_id| field.report_id != report_id)
            {
                continue;
            }
            let Some(value) = decode_field(self.report, &field, self.data_offset) else {
                continue;
            };
            return Some(DecodedField {
                kind: field.kind,
                report_id: field.report_id,
                usage_page: field.usage_page,
                usages: field.usages,
                usage_count: field.usage_count,
                offset: field.offset,
                report_size: field.report_size,
                signed: field.is_signed(),
                value,
            });
        }
        None
    }
}

/// Decodes the value of one field from a report.
fn decode_field(report: &[u8], field: &Field, data_offset: u32) -> Option<FieldValue> {
    let size = u32::from(field.report_size);
    let count = u32::from(field.report_count);
    if size == 0 || count == 0 {
        return None;
    }
    let signed = field.is_signed();
    if count == 1 {
        let raw = read_bits(report, field.offset.checked_add(data_offset)?, size)?;
        let value = if signed {
            signed_value(raw, size)
        } else {
            unsigned_value(raw)
        };
        return Some(FieldValue::Scalar(value));
    }
    let mut values = [0_i64; MAX_FIELD_VALUES];
    let mut valid = 0_u8;
    for index in 0..count {
        if index >= u32::try_from(MAX_FIELD_VALUES).unwrap_or(u32::MAX) {
            break;
        }
        let offset = field
            .offset
            .checked_add(data_offset)?
            .checked_add(index.checked_mul(size)?)?;
        let raw = read_bits(report, offset, size)?;
        let value = if signed {
            signed_value(raw, size)
        } else {
            unsigned_value(raw)
        };
        if let Some(slot) = values.get_mut(index as usize) {
            *slot = value;
        }
        valid = u8::try_from(index + 1).unwrap_or(u8::MAX);
    }
    Some(FieldValue::Array {
        values,
        count: valid,
    })
}

/// Converts an unsigned value of a report field into a signed integer.
fn unsigned_value(value: u64) -> i64 {
    i64::try_from(value & 0xFFFF_FFFF).unwrap_or(0)
}

/// Converts a raw value of `size` bits into a signed integer.
fn signed_value(value: u64, size: u32) -> i64 {
    let bits = size.clamp(1, 32);
    let masked = value & ((1_u64 << bits) - 1);
    let sign = 1_u64 << (bits - 1);
    if masked & sign == 0 {
        i64::try_from(masked).unwrap_or(0)
    } else {
        let magnitude = i64::try_from(masked & !sign).unwrap_or(0);
        magnitude - (1_i64 << (bits - 1))
    }
}

/// Reads `count` bits from a report, starting at `bit_offset`.
///
/// The first bit of a report is the least significant bit of the first
/// byte.
fn read_bits(report: &[u8], bit_offset: u32, count: u32) -> Option<u64> {
    let mut value = 0_u64;
    for index in 0..count {
        let bit = bit_offset.checked_add(index)?;
        let byte_index = usize::try_from(bit / 8).ok()?;
        let shift = bit % 8;
        let byte = report.get(byte_index)?;
        let bit_value = (byte >> shift) & 1;
        value |= u64::from(bit_value) << index;
    }
    Some(value)
}
