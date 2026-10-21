//! # Human interface infrastructure forms
//!
//! Bounded parsing for the binary IFR (internal forms representation)
//! opcode stream carried in HII form packages. Each opcode begins with
//! a two-byte header: an opcode byte and a packed length/scope byte.
//!
//! The parser validates framing and scope nesting. It does not
//! interpret questions, evaluate expressions, or write configuration
//! data; those operations require the firmware HII database.

/// The IFR opcode header size in bytes.
pub const IFR_HEADER_SIZE: usize = 2;

/// The maximum encoded opcode length.
pub const IFR_MAX_LENGTH: usize = 0x7F;

/// The form opcode.
pub const IFR_FORM: u8 = 0x01;
/// The subtitle opcode.
pub const IFR_SUBTITLE: u8 = 0x02;
/// The text opcode.
pub const IFR_TEXT: u8 = 0x03;
/// The image opcode.
pub const IFR_IMAGE: u8 = 0x04;
/// The one-of opcode.
pub const IFR_ONE_OF: u8 = 0x05;
/// The checkbox opcode.
pub const IFR_CHECKBOX: u8 = 0x06;
/// The numeric opcode.
pub const IFR_NUMERIC: u8 = 0x07;
/// The password opcode.
pub const IFR_PASSWORD: u8 = 0x08;
/// The string opcode.
pub const IFR_STRING: u8 = 0x09;
/// The date opcode.
pub const IFR_DATE: u8 = 0x0A;
/// The time opcode.
pub const IFR_TIME: u8 = 0x0B;
/// The ordered-list opcode.
pub const IFR_ORDERED_LIST: u8 = 0x0C;
/// The variable-store opcode.
pub const IFR_VARSTORE: u8 = 0x0D;
/// The refresh opcode.
pub const IFR_REFRESH: u8 = 0x0F;
/// The disable-if opcode.
pub const IFR_DISABLE_IF: u8 = 0x10;
/// The form-set opcode.
pub const IFR_FORM_SET: u8 = 0x11;
/// The form-set identifier opcode.
pub const IFR_FORM_SET_ID: u8 = 0x12;
/// The form-map opcode.
pub const IFR_FORM_MAP: u8 = 0x13;
/// The get-current-value opcode.
pub const IFR_GET_CURRENT_VALUE: u8 = 0x14;
/// The get-default-value opcode.
pub const IFR_GET_DEFAULT_VALUE: u8 = 0x15;
/// The get-option-value opcode.
pub const IFR_GET_OPTION_VALUE: u8 = 0x16;
/// The one-of-option opcode.
pub const IFR_ONE_OF_OPTION: u8 = 0x17;
/// The suppress-if opcode.
pub const IFR_SUPPRESS_IF: u8 = 0x18;
/// The locked opcode.
pub const IFR_LOCKED: u8 = 0x19;
/// The action opcode.
pub const IFR_ACTION: u8 = 0x1A;
/// The reset-button opcode.
pub const IFR_RESET_BUTTON: u8 = 0x1B;
/// The form-set-changed opcode.
pub const IFR_FORM_SET_CHANGED: u8 = 0x1C;
/// The end opcode.
pub const IFR_END: u8 = 0x29;

/// An IFR decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfrError {
    /// Fewer than two bytes remained for an opcode header.
    TruncatedHeader,
    /// A declared opcode length was below the header size.
    InvalidLength,
    /// A declared opcode length exceeded the remaining input.
    OutOfBounds,
    /// An `END` opcode appeared without an open scope.
    UnexpectedEnd,
    /// A scope was not closed by an `END` opcode.
    MissingEnd,
}

impl core::fmt::Display for IfrError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::TruncatedHeader => "truncated IFR header",
            Self::InvalidLength => "invalid IFR opcode length",
            Self::OutOfBounds => "IFR opcode exceeds buffer",
            Self::UnexpectedEnd => "unexpected IFR end opcode",
            Self::MissingEnd => "IFR scope not closed",
        };
        formatter.write_str(text)
    }
}

/// A parsed IFR opcode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfrOp<'a> {
    /// The opcode byte.
    pub opcode: u8,
    /// The encoded total length including the header.
    pub length: u8,
    /// Whether the opcode opens a scope.
    pub scope: bool,
    /// The operation payload after the header.
    pub payload: &'a [u8],
}

/// Iterates IFR opcodes.
#[derive(Clone, Debug)]
pub struct IfrIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> IfrIter<'a> {
    /// Creates an iterator over an IFR stream.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    /// Returns the current byte offset.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.offset
    }
}

impl<'a> Iterator for IfrIter<'a> {
    type Item = Result<IfrOp<'a>, IfrError>;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.bytes.get(self.offset..)?;
        if remaining.is_empty() {
            return None;
        }
        if remaining.len() < IFR_HEADER_SIZE {
            self.offset = self.bytes.len();
            return Some(Err(IfrError::TruncatedHeader));
        }
        let opcode = remaining[0];
        let flags = remaining[1];
        let length = flags & 0x7F;
        let scope = flags & 0x80 != 0;
        let total = usize::from(length);
        if total < IFR_HEADER_SIZE {
            self.offset = self.bytes.len();
            return Some(Err(IfrError::InvalidLength));
        }
        let Some(payload) = remaining.get(IFR_HEADER_SIZE..total) else {
            self.offset = self.bytes.len();
            return Some(Err(IfrError::OutOfBounds));
        };
        self.offset += total;
        Some(Ok(IfrOp {
            opcode,
            length,
            scope,
            payload,
        }))
    }
}

/// Validates an IFR opcode stream including scope nesting.
///
/// Unknown opcodes are accepted as opaque operations, because vendors
/// may extend the opcode space. Framing errors, unbalanced scopes, and
/// stray `END` opcodes are rejected.
///
/// # Errors
///
/// Returns the first framing or scope defect encountered.
pub fn validate_ifr(bytes: &[u8]) -> Result<(), IfrError> {
    let mut depth = 0_usize;
    let mut saw_opcode = false;
    for item in IfrIter::new(bytes) {
        let op = item?;
        saw_opcode = true;
        if op.opcode == IFR_END {
            if op.scope {
                return Err(IfrError::UnexpectedEnd);
            }
            if depth == 0 {
                return Err(IfrError::UnexpectedEnd);
            }
            depth -= 1;
        } else if op.scope {
            depth = depth.checked_add(1).ok_or(IfrError::MissingEnd)?;
        }
    }
    if !saw_opcode {
        return Err(IfrError::TruncatedHeader);
    }
    if depth != 0 {
        return Err(IfrError::MissingEnd);
    }
    Ok(())
}

/// Returns the number of opcodes in a valid stream, or `None` when the
/// stream is malformed.
#[must_use]
pub fn ifr_opcode_count(bytes: &[u8]) -> Option<usize> {
    if validate_ifr(bytes).is_err() {
        return None;
    }
    let mut count = 0_usize;
    for item in IfrIter::new(bytes) {
        item.ok()?;
        count = count.checked_add(1)?;
    }
    Some(count)
}
