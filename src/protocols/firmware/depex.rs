//! # Firmware dependency expressions
//!
//! Bounded parsing and structural validation for the binary dependency
//! expressions stored in firmware files. A dependency expression is a
//! postfix boolean program over protocol GUID availability.
//!
//! The parser validates opcode framing, GUID operands, and stack
//! balance. It does not evaluate the expression, because evaluation
//! depends on the firmware dispatcher's protocol database.

use crate::base::EfiGuid;

/// The `BEFORE` opcode.
pub const DEPEX_BEFORE: u8 = 0x00;
/// The `AFTER` opcode.
pub const DEPEX_AFTER: u8 = 0x01;
/// The `PUSH` opcode.
pub const DEPEX_PUSH: u8 = 0x02;
/// The `AND` opcode.
pub const DEPEX_AND: u8 = 0x03;
/// The `OR` opcode.
pub const DEPEX_OR: u8 = 0x04;
/// The `NOT` opcode.
pub const DEPEX_NOT: u8 = 0x05;
/// The `TRUE` opcode.
pub const DEPEX_TRUE: u8 = 0x06;
/// The `FALSE` opcode.
pub const DEPEX_FALSE: u8 = 0x07;
/// The `END` opcode.
pub const DEPEX_END: u8 = 0x08;
/// The `SOR` opcode.
pub const DEPEX_SOR: u8 = 0x09;

/// A dependency expression decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepexError {
    /// An opcode requiring a GUID operand ended early.
    TruncatedGuid,
    /// An unknown opcode was encountered.
    UnknownOpcode(u8),
    /// The expression ended before the `END` opcode.
    MissingEnd,
    /// The expression ended without a complete result.
    IncompleteExpression,
    /// An operator consumed more operands than were available.
    StackUnderflow,
    /// The `END` opcode was reached with more than one result.
    UnconsumedOperands,
    /// Non-zero bytes followed the `END` opcode.
    TrailingData,
}

impl core::fmt::Display for DepexError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TruncatedGuid => formatter.write_str("truncated dependency GUID"),
            Self::UnknownOpcode(opcode) => {
                write!(formatter, "unknown dependency opcode {opcode:#04x}")
            }
            Self::MissingEnd => formatter.write_str("dependency expression has no end"),
            Self::IncompleteExpression => formatter.write_str("incomplete dependency expression"),
            Self::StackUnderflow => formatter.write_str("dependency stack underflow"),
            Self::UnconsumedOperands => formatter.write_str("dependency operands left unconsumed"),
            Self::TrailingData => formatter.write_str("trailing dependency expression data"),
        }
    }
}

/// A decoded dependency token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepexToken {
    /// Execute the driver before the referenced protocol is available.
    Before(EfiGuid),
    /// Execute the driver after the referenced protocol is available.
    After(EfiGuid),
    /// Test whether the referenced protocol is available.
    Push(EfiGuid),
    /// Boolean conjunction.
    And,
    /// Boolean disjunction.
    Or,
    /// Boolean negation.
    Not,
    /// A constant true value.
    True,
    /// A constant false value.
    False,
    /// Schedule the driver on request.
    ScheduleOnRequest,
    /// End of the expression.
    End,
}

/// Iterates decoded dependency tokens.
#[derive(Clone, Debug)]
pub struct DepexIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    finished: bool,
}

impl<'a> DepexIter<'a> {
    /// Creates an iterator over `bytes`.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            finished: false,
        }
    }

    /// Returns the offset after the last consumed token.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.offset
    }

    /// Reads a GUID operand at the current offset.
    fn read_guid(&mut self) -> Option<EfiGuid> {
        let end = self.offset.checked_add(16)?;
        let slice = self.bytes.get(self.offset..end)?;
        let mut raw = [0_u8; 16];
        raw.copy_from_slice(slice);
        self.offset = end;
        Some(EfiGuid::from_bytes(raw))
    }
}

impl Iterator for DepexIter<'_> {
    type Item = Result<DepexToken, DepexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let Some(&opcode) = self.bytes.get(self.offset) else {
            self.finished = true;
            return None;
        };
        self.offset += 1;
        let token = match opcode {
            DEPEX_BEFORE => match self.read_guid() {
                Some(guid) => Ok(DepexToken::Before(guid)),
                None => Err(DepexError::TruncatedGuid),
            },
            DEPEX_AFTER => match self.read_guid() {
                Some(guid) => Ok(DepexToken::After(guid)),
                None => Err(DepexError::TruncatedGuid),
            },
            DEPEX_PUSH => match self.read_guid() {
                Some(guid) => Ok(DepexToken::Push(guid)),
                None => Err(DepexError::TruncatedGuid),
            },
            DEPEX_AND => Ok(DepexToken::And),
            DEPEX_OR => Ok(DepexToken::Or),
            DEPEX_NOT => Ok(DepexToken::Not),
            DEPEX_TRUE => Ok(DepexToken::True),
            DEPEX_FALSE => Ok(DepexToken::False),
            DEPEX_SOR => Ok(DepexToken::ScheduleOnRequest),
            DEPEX_END => {
                self.finished = true;
                Ok(DepexToken::End)
            }
            other => {
                self.finished = true;
                Err(DepexError::UnknownOpcode(other))
            }
        };
        Some(token)
    }
}

/// Validates a dependency expression.
///
/// Checks opcode framing, GUID operands, postfix stack balance, and
/// requires the expression to end with the `END` opcode followed only
/// by zero padding.
///
/// # Errors
///
/// Returns the first structural defect encountered.
pub fn validate_dependency_expression(bytes: &[u8]) -> Result<(), DepexError> {
    let mut depth = 0_usize;
    let mut ended = false;
    let mut iterator = DepexIter::new(bytes);
    for token in iterator.by_ref() {
        match token? {
            DepexToken::Before(_)
            | DepexToken::After(_)
            | DepexToken::Push(_)
            | DepexToken::True
            | DepexToken::False
            | DepexToken::ScheduleOnRequest => depth += 1,
            DepexToken::And | DepexToken::Or => {
                if depth < 2 {
                    return Err(DepexError::StackUnderflow);
                }
                depth -= 2;
                depth += 1;
            }
            DepexToken::Not => {
                if depth == 0 {
                    return Err(DepexError::StackUnderflow);
                }
            }
            DepexToken::End => {
                ended = true;
                break;
            }
        }
    }
    if !ended {
        return Err(DepexError::MissingEnd);
    }
    if depth != 1 {
        return Err(if depth == 0 {
            DepexError::IncompleteExpression
        } else {
            DepexError::UnconsumedOperands
        });
    }
    match bytes.get(iterator.position()..) {
        Some(trailing) => {
            if trailing.iter().any(|byte| *byte != 0) {
                Err(DepexError::TrailingData)
            } else {
                Ok(())
            }
        }
        None => Err(DepexError::MissingEnd),
    }
}
