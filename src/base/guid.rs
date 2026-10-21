//! # Globally unique identifiers
//!
//! The `GUID` type used by `UEFI` protocols, configuration
//! tables and variables to identify interfaces and data.
//!
//! The layout follows the specification: a 32-bit field, two
//! 16-bit fields and an 8-byte array. On the wire the first
//! three fields use little-endian encoding and the final
//! array is stored as bytes. The [`EfiGuid::to_bytes`] and
//! [`EfiGuid::from_bytes`] conversions implement that
//! mixed-endian representation explicitly so the result
//! does not depend on the host byte order.

/// A 128-bit globally unique identifier.
///
/// The fields match the `EFI_GUID` structure in the
/// specification. The struct is `#[repr(C)]` so its
/// size (16 bytes) and field offsets match the firmware
/// definition on every target.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiGuid {
    /// The first 32-bit component.
    pub data1: u32,
    /// The second 16-bit component.
    pub data2: u16,
    /// The third 16-bit component.
    pub data3: u16,
    /// The final 8-byte component.
    pub data4: [u8; 8],
}

impl EfiGuid {
    /// The all-zero identifier.
    pub const ZERO: Self = Self {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    /// Creates a `GUID` from its components.
    #[must_use]
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Returns whether the identifier is all zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.data1 == 0
            && self.data2 == 0
            && self.data3 == 0
            && self.data4[0] == 0
            && self.data4[1] == 0
            && self.data4[2] == 0
            && self.data4[3] == 0
            && self.data4[4] == 0
            && self.data4[5] == 0
            && self.data4[6] == 0
            && self.data4[7] == 0
    }

    /// Converts the identifier to its canonical byte form.
    ///
    /// The first three fields are encoded as little-endian
    /// and the final field is copied as bytes, matching the
    /// 16-byte representation used in text strings and in
    /// firmware data structures.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        let data1 = self.data1.to_le_bytes();
        let data2 = self.data2.to_le_bytes();
        let data3 = self.data3.to_le_bytes();
        [
            data1[0],
            data1[1],
            data1[2],
            data1[3],
            data2[0],
            data2[1],
            data3[0],
            data3[1],
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        ]
    }

    /// Creates an identifier from its canonical byte form.
    ///
    /// This is the inverse of [`EfiGuid::to_bytes`].
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_le_bytes([bytes[4], bytes[5]]),
            data3: u16::from_le_bytes([bytes[6], bytes[7]]),
            data4: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    /// Returns a pointer to the identifier bytes.
    ///
    /// The pointer refers to `self`, which must outlive every use of
    /// the returned pointer. The bytes are the canonical 16-byte
    /// representation in the host memory layout, which is the same as
    /// [`EfiGuid::to_bytes`] on little-endian firmware targets.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 16] {
        // SAFETY: `EfiGuid` is `#[repr(C)]` with no padding and exactly
        // 16 bytes of scalar fields, so reinterpreting a reference to it
        // as a 16-byte array is sound for every lifetime of `self`.
        unsafe { &*core::ptr::from_ref(self).cast::<[u8; 16]>() }
    }

    /// Writes the canonical byte form into `output`.
    ///
    /// Returns the number of bytes written, always `16`.
    pub fn write_bytes(&self, output: &mut [u8; 16]) -> usize {
        *output = self.to_bytes();
        16
    }

    /// Parses an identifier from its textual form.
    ///
    /// The accepted syntax is the standard registry form
    /// `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` where each `x` is a
    /// hexadecimal digit. Upper- and lower-case digits are accepted.
    /// Braces, a `urn:uuid:` prefix and a leading `\` are not accepted;
    /// callers that receive such strings must strip the decoration.
    ///
    /// # Errors
    ///
    /// Returns [`GuidParseError`] when the string does not match the
    /// required syntax.
    pub fn parse(text: &str) -> Result<Self, GuidParseError> {
        let bytes = text.as_bytes();
        if bytes.len() != 36 {
            return Err(GuidParseError::InvalidLength);
        }
        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            return Err(GuidParseError::InvalidSeparator);
        }
        let mut digits = [0_u8; 32];
        let mut index = 0;
        for (position, byte) in bytes.iter().enumerate() {
            if matches!(position, 8 | 13 | 18 | 23) {
                continue;
            }
            let value = hex_value(*byte).ok_or(GuidParseError::InvalidDigit)?;
            digits[index] = value;
            index += 1;
        }
        let mut value = [0_u8; 16];
        for (i, target) in value.iter_mut().enumerate() {
            *target = (digits[i * 2] << 4) | digits[i * 2 + 1];
        }
        Ok(Self {
            data1: u32::from_be_bytes([value[0], value[1], value[2], value[3]]),
            data2: u16::from_be_bytes([value[4], value[5]]),
            data3: u16::from_be_bytes([value[6], value[7]]),
            data4: [
                value[8], value[9], value[10], value[11], value[12], value[13], value[14],
                value[15],
            ],
        })
    }

    /// Formats the identifier into `output`.
    ///
    /// The output is the 36-character canonical textual form. Returns
    /// the number of bytes written, always `36`.
    ///
    /// # Panics
    ///
    /// Panics when `output` is shorter than 36 bytes. Use
    /// [`EfiGuid::try_format`] when the buffer size is not known to be
    /// sufficient.
    pub fn format_into(&self, output: &mut [u8]) -> usize {
        assert!(output.len() >= 36, "GUID output buffer too small");
        let hex = b"0123456789abcdef";
        let bytes = self.to_canonical_bytes();
        let mut index = 0;
        for (position, byte) in bytes.iter().enumerate() {
            if matches!(position, 4 | 6 | 8 | 10) {
                output[index] = b'-';
                index += 1;
            }
            output[index] = hex[usize::from(byte >> 4)];
            output[index + 1] = hex[usize::from(byte & 0x0f)];
            index += 2;
        }
        index
    }

    /// Formats the identifier into `output` if it is large enough.
    ///
    /// Returns `None` when `output` is shorter than 36 bytes.
    pub fn try_format(&self, output: &mut [u8]) -> Option<usize> {
        if output.len() < 36 {
            return None;
        }
        Some(self.format_into(output))
    }

    /// Returns the canonical 16-byte form with the textual field order.
    ///
    /// Unlike [`EfiGuid::to_bytes`], the first three fields use
    /// big-endian order so the result matches the textual representation
    /// read left to right. This layout is used only for display and
    /// hashing; firmware structures use [`EfiGuid::to_bytes`].
    #[must_use]
    pub const fn to_canonical_bytes(self) -> [u8; 16] {
        let data1 = self.data1.to_be_bytes();
        let data2 = self.data2.to_be_bytes();
        let data3 = self.data3.to_be_bytes();
        [
            data1[0],
            data1[1],
            data1[2],
            data1[3],
            data2[0],
            data2[1],
            data3[0],
            data3[1],
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        ]
    }
}

/// An error produced while parsing a textual `GUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GuidParseError {
    /// The input was not exactly 36 bytes long.
    InvalidLength,
    /// A dash separator was missing or in the wrong position.
    InvalidSeparator,
    /// A character was not a hexadecimal digit.
    InvalidDigit,
}

impl core::fmt::Display for GuidParseError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLength => "GUID text must be exactly 36 characters",
            Self::InvalidSeparator => "GUID text has a misplaced separator",
            Self::InvalidDigit => "GUID text contains a non-hexadecimal digit",
        })
    }
}

/// Returns the numeric value of one ASCII hexadecimal digit.
const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

impl core::fmt::Display for EfiGuid {
    /// Formats the identifier in canonical registry form.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.to_canonical_bytes();
        for (index, byte) in bytes.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                formatter.write_str("-")?;
            }
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl core::str::FromStr for EfiGuid {
    type Err = GuidParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl core::fmt::Debug for EfiGuid {
    /// Formats the identifier as its four components.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("EfiGuid")
            .field("data1", &self.data1)
            .field("data2", &self.data2)
            .field("data3", &self.data3)
            .field("data4", &self.data4)
            .finish()
    }
}
