//! # Firmware Management Protocol (FMP) Dependency Expressions
//!
//! Provides parsing, validation, and execution of capsule dependency expressions
//! (`EFI_FIRMWARE_IMAGE_DEP`) defined in the UEFI 2.8+ specification.
//!
//! Evaluates complex pre-update firmware dependencies (such as requiring a minimum
//! EC or ME version prior to flashing a system BIOS capsule) with a pure `#![no_std]`,
//! zero-allocation stack machine.

#![allow(clippy::cast_possible_truncation, clippy::too_many_lines)]

use crate::base::EfiGuid;

// Standard FMP Dependency Expression Opcodes
/// Pushes a 16-byte `ImageTypeId` GUID onto the stack.
pub const EFI_FMP_DEP_PUSH_GUID: u8 = 0x00;
/// Pushes a 32-bit integer version number onto the stack.
pub const EFI_FMP_DEP_PUSH_VERSION: u8 = 0x01;
/// Pushes a null-terminated UTF-8/ASCII version string onto the stack.
pub const EFI_FMP_DEP_VERSION_STR: u8 = 0x02;
/// Evaluates logical AND of top two boolean values.
pub const EFI_FMP_DEP_AND: u8 = 0x03;
/// Evaluates logical OR of top two boolean values.
pub const EFI_FMP_DEP_OR: u8 = 0x04;
/// Inverts the top boolean value.
pub const EFI_FMP_DEP_NOT: u8 = 0x05;
/// Pushes boolean `true` onto the stack.
pub const EFI_FMP_DEP_TRUE: u8 = 0x06;
/// Pushes boolean `false` onto the stack.
pub const EFI_FMP_DEP_FALSE: u8 = 0x07;
/// Evaluates equality between two versions or two booleans.
pub const EFI_FMP_DEP_EQ: u8 = 0x08;
/// Evaluates greater-than (`lhs > rhs`).
pub const EFI_FMP_DEP_GT: u8 = 0x09;
/// Evaluates greater-than-or-equal (`lhs >= rhs`).
pub const EFI_FMP_DEP_GTE: u8 = 0x0A;
/// Evaluates less-than (`lhs < rhs`).
pub const EFI_FMP_DEP_LT: u8 = 0x0B;
/// Evaluates less-than-or-equal (`lhs <= rhs`).
pub const EFI_FMP_DEP_LTE: u8 = 0x0C;
/// Terminates dependency expression evaluation.
pub const EFI_FMP_DEP_END: u8 = 0x0D;
/// Declares the total length in bytes of the dependency expression.
pub const EFI_FMP_DEP_DECLARE_LENGTH: u8 = 0x0E;

// Last Attempt Status vendor failure code ranges (0x1000 - 0x1FFF)
/// Minimum value for FMP reserved failure codes.
pub const LAST_ATTEMPT_STATUS_FMP_RESERVED_MIN: u32 = 0x1000;
/// Maximum value for FMP reserved failure codes.
pub const LAST_ATTEMPT_STATUS_FMP_RESERVED_MAX: u32 = 0x1FFF;
/// Driver-specific minimum failure code.
pub const LAST_ATTEMPT_STATUS_DRIVER_MIN: u32 = 0x1000;
/// Driver-specific maximum failure code.
pub const LAST_ATTEMPT_STATUS_DRIVER_MAX: u32 = 0x107F;
/// Dependency library minimum failure code.
pub const LAST_ATTEMPT_STATUS_DEPENDENCY_LIB_MIN: u32 = 0x1080;
/// Dependency library maximum failure code.
pub const LAST_ATTEMPT_STATUS_DEPENDENCY_LIB_MAX: u32 = 0x109F;
/// Dependency check minimum failure code.
pub const LAST_ATTEMPT_STATUS_DEPENDENCY_CHECK_MIN: u32 = 0x10A0;
/// Dependency check maximum failure code.
pub const LAST_ATTEMPT_STATUS_DEPENDENCY_CHECK_MAX: u32 = 0x10BF;

/// Maximum evaluation stack depth.
pub const MAX_FMP_DEPEX_STACK_DEPTH: usize = 32;

/// Value types held on the dependency expression evaluator stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmpDepexValue {
    /// Boolean logic value.
    Bool(bool),
    /// 32-bit numeric firmware version.
    Version(u32),
    /// Image type identifier GUID.
    Guid(EfiGuid),
}

/// Errors occurring during dependency expression parsing or evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmpDepexError {
    /// Expression terminated unexpectedly while parsing instruction payload.
    UnexpectedEnd,
    /// Unrecognized dependency opcode encountered.
    InvalidOpcode(u8),
    /// Evaluator stack exceeded maximum depth capacity.
    StackOverflow,
    /// Stack underflow occurred during operation evaluation.
    StackUnderflow,
    /// Operands on top of stack were of incompatible types.
    TypeMismatch,
    /// Expression declared length field was invalid.
    InvalidDeclaredLength,
    /// Expression did not conclude with `EFI_FMP_DEP_END`.
    MissingEndOpcode,
    /// Evaluation completed with extra values remaining on stack.
    UnconsumedStackValues,
    /// Referenced image type GUID was not found in the platform.
    ImageNotFound,
}

/// Validates the structure and syntax of an FMP dependency expression.
///
/// Returns the total byte length of the expression including the terminating `EFI_FMP_DEP_END`.
///
/// # Errors
/// Returns [`FmpDepexError`] if the bytecode contains illegal opcodes, invalid lengths,
/// or does not properly conclude with an `EFI_FMP_DEP_END`.
pub fn validate_fmp_depex(expr: &[u8]) -> Result<usize, FmpDepexError> {
    let mut offset = 0;
    let len = expr.len();

    // Check optional declared length prefix
    if offset < len && expr[offset] == EFI_FMP_DEP_DECLARE_LENGTH {
        offset += 1;
        if offset + 4 > len {
            return Err(FmpDepexError::UnexpectedEnd);
        }
        let mut d_len_raw = [0_u8; 4];
        d_len_raw.copy_from_slice(&expr[offset..offset + 4]);
        let declared = u32::from_le_bytes(d_len_raw) as usize;
        offset += 4;
        if declared > len {
            return Err(FmpDepexError::InvalidDeclaredLength);
        }
    }

    while offset < len {
        let opcode = expr[offset];
        offset += 1;

        match opcode {
            EFI_FMP_DEP_PUSH_GUID => {
                if offset + 16 > len {
                    return Err(FmpDepexError::UnexpectedEnd);
                }
                offset += 16;
            }
            EFI_FMP_DEP_PUSH_VERSION => {
                if offset + 4 > len {
                    return Err(FmpDepexError::UnexpectedEnd);
                }
                offset += 4;
            }
            EFI_FMP_DEP_VERSION_STR => {
                // Read until null terminator
                let mut found_null = false;
                while offset < len {
                    if expr[offset] == 0 {
                        found_null = true;
                        offset += 1;
                        break;
                    }
                    offset += 1;
                }
                if !found_null {
                    return Err(FmpDepexError::UnexpectedEnd);
                }
            }
            EFI_FMP_DEP_AND | EFI_FMP_DEP_OR | EFI_FMP_DEP_NOT | EFI_FMP_DEP_TRUE
            | EFI_FMP_DEP_FALSE | EFI_FMP_DEP_EQ | EFI_FMP_DEP_GT | EFI_FMP_DEP_GTE
            | EFI_FMP_DEP_LT | EFI_FMP_DEP_LTE => {}
            EFI_FMP_DEP_END => return Ok(offset),
            other => return Err(FmpDepexError::InvalidOpcode(other)),
        }
    }

    Err(FmpDepexError::MissingEndOpcode)
}

/// Evaluates an FMP capsule dependency expression against system image versions.
///
/// `version_resolver` is invoked when an `EFI_FMP_DEP_PUSH_GUID` is resolved to determine
/// the currently installed version for that image type.
///
/// # Errors
/// Returns [`FmpDepexError`] on malformed bytecodes, type mismatches, or unresolved images.
pub fn evaluate_fmp_depex<F>(expr: &[u8], version_resolver: F) -> Result<bool, FmpDepexError>
where
    F: Fn(&EfiGuid) -> Option<u32>,
{
    let mut stack = [FmpDepexValue::Bool(false); MAX_FMP_DEPEX_STACK_DEPTH];
    let mut sp = 0;

    let mut offset = 0;
    let len = expr.len();

    // Check optional declared length
    if offset < len && expr[offset] == EFI_FMP_DEP_DECLARE_LENGTH {
        offset += 1;
        if offset + 4 > len {
            return Err(FmpDepexError::UnexpectedEnd);
        }
        offset += 4;
    }

    while offset < len {
        let opcode = expr[offset];
        offset += 1;

        match opcode {
            EFI_FMP_DEP_PUSH_GUID => {
                if offset + 16 > len {
                    return Err(FmpDepexError::UnexpectedEnd);
                }
                let mut guid_bytes = [0_u8; 16];
                guid_bytes.copy_from_slice(&expr[offset..offset + 16]);
                offset += 16;
                let guid = EfiGuid::from_bytes(guid_bytes);

                let ver = version_resolver(&guid).ok_or(FmpDepexError::ImageNotFound)?;
                if sp >= MAX_FMP_DEPEX_STACK_DEPTH {
                    return Err(FmpDepexError::StackOverflow);
                }
                stack[sp] = FmpDepexValue::Version(ver);
                sp += 1;
            }
            EFI_FMP_DEP_PUSH_VERSION => {
                if offset + 4 > len {
                    return Err(FmpDepexError::UnexpectedEnd);
                }
                let mut ver_raw = [0_u8; 4];
                ver_raw.copy_from_slice(&expr[offset..offset + 4]);
                offset += 4;
                let ver = u32::from_le_bytes(ver_raw);

                if sp >= MAX_FMP_DEPEX_STACK_DEPTH {
                    return Err(FmpDepexError::StackOverflow);
                }
                stack[sp] = FmpDepexValue::Version(ver);
                sp += 1;
            }
            EFI_FMP_DEP_TRUE => {
                if sp >= MAX_FMP_DEPEX_STACK_DEPTH {
                    return Err(FmpDepexError::StackOverflow);
                }
                stack[sp] = FmpDepexValue::Bool(true);
                sp += 1;
            }
            EFI_FMP_DEP_FALSE => {
                if sp >= MAX_FMP_DEPEX_STACK_DEPTH {
                    return Err(FmpDepexError::StackOverflow);
                }
                stack[sp] = FmpDepexValue::Bool(false);
                sp += 1;
            }
            EFI_FMP_DEP_NOT => {
                if sp == 0 {
                    return Err(FmpDepexError::StackUnderflow);
                }
                match stack[sp - 1] {
                    FmpDepexValue::Bool(b) => {
                        stack[sp - 1] = FmpDepexValue::Bool(!b);
                    }
                    _ => return Err(FmpDepexError::TypeMismatch),
                }
            }
            EFI_FMP_DEP_AND => {
                if sp < 2 {
                    return Err(FmpDepexError::StackUnderflow);
                }
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                sp -= 2;
                match (lhs, rhs) {
                    (FmpDepexValue::Bool(a), FmpDepexValue::Bool(b)) => {
                        stack[sp] = FmpDepexValue::Bool(a && b);
                        sp += 1;
                    }
                    _ => return Err(FmpDepexError::TypeMismatch),
                }
            }
            EFI_FMP_DEP_OR => {
                if sp < 2 {
                    return Err(FmpDepexError::StackUnderflow);
                }
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                sp -= 2;
                match (lhs, rhs) {
                    (FmpDepexValue::Bool(a), FmpDepexValue::Bool(b)) => {
                        stack[sp] = FmpDepexValue::Bool(a || b);
                        sp += 1;
                    }
                    _ => return Err(FmpDepexError::TypeMismatch),
                }
            }
            EFI_FMP_DEP_EQ | EFI_FMP_DEP_GT | EFI_FMP_DEP_GTE | EFI_FMP_DEP_LT
            | EFI_FMP_DEP_LTE => {
                if sp < 2 {
                    return Err(FmpDepexError::StackUnderflow);
                }
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                sp -= 2;
                match (lhs, rhs) {
                    (FmpDepexValue::Version(a), FmpDepexValue::Version(b)) => {
                        let res = match opcode {
                            EFI_FMP_DEP_EQ => a == b,
                            EFI_FMP_DEP_GT => a > b,
                            EFI_FMP_DEP_GTE => a >= b,
                            EFI_FMP_DEP_LT => a < b,
                            EFI_FMP_DEP_LTE => a <= b,
                            _ => unreachable!(),
                        };
                        stack[sp] = FmpDepexValue::Bool(res);
                        sp += 1;
                    }
                    (FmpDepexValue::Bool(a), FmpDepexValue::Bool(b))
                        if opcode == EFI_FMP_DEP_EQ =>
                    {
                        stack[sp] = FmpDepexValue::Bool(a == b);
                        sp += 1;
                    }
                    _ => return Err(FmpDepexError::TypeMismatch),
                }
            }
            EFI_FMP_DEP_END => {
                if sp != 1 {
                    return Err(FmpDepexError::UnconsumedStackValues);
                }
                return match stack[0] {
                    FmpDepexValue::Bool(b) => Ok(b),
                    _ => Err(FmpDepexError::TypeMismatch),
                };
            }
            other => return Err(FmpDepexError::InvalidOpcode(other)),
        }
    }

    Err(FmpDepexError::MissingEndOpcode)
}
