//! # UEFI Regular Expression Protocol
//!
//! Provides services to match Unicode (UCS-2 / UTF-16) strings against regular
//! expression patterns, as defined in UEFI Specification 2.5+ (Section 35.1).
//! Includes standard syntax identifiers (POSIX Extended, Perl, ECMA-262), the
//! `EFI_REGULAR_EXPRESSION_PROTOCOL` ABI layout, safe client wrapper, and an
//! independent `#![no_std]` regular expression matching engine.

use crate::abi::types::{Bool, EFI_FALSE, EFI_TRUE, UIntN};
use crate::base::{EfiGuid, EfiStatus};

/// Regular Expression Protocol `GUID` (`EFI_REGULAR_EXPRESSION_PROTOCOL_GUID`).
pub const EFI_REGULAR_EXPRESSION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB3F7_9D9A,
    0x436C,
    0xDC11,
    [0xB0, 0x52, 0xCD, 0x85, 0xDF, 0x52, 0x4C, 0xE6],
);

/// POSIX Extended Regular Expression (`ERE`) Syntax Type `GUID`.
pub const EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID: EfiGuid = EfiGuid::new(
    0x5F05_B20F,
    0x4A56,
    0xC231,
    [0xFA, 0x0B, 0xA7, 0xB1, 0xF1, 0x10, 0x04, 0x1D],
);

/// Perl Regular Expression Syntax Type `GUID`.
pub const EFI_REGEX_SYNTAX_TYPE_PERL_GUID: EfiGuid = EfiGuid::new(
    0x63E6_0A51,
    0x497D,
    0xD427,
    [0xC4, 0xA5, 0xB8, 0xAB, 0xDC, 0x3A, 0xAE, 0xB6],
);

/// ECMA 262 Regular Expression Syntax Type `GUID`.
pub const EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID: EfiGuid = EfiGuid::new(
    0x9A47_3A4A,
    0x4CEB,
    0xB95A,
    [0x41, 0x5E, 0x5B, 0xA0, 0xBC, 0x63, 0x9B, 0x2E],
);

/// A captured sub-expression span within the matched string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiRegexCapture {
    /// Pointer to the start of the captured sub-expression in the matched string.
    pub capture_ptr: *const u16,
    /// Length of the captured sub-expression in UTF-16 code units.
    pub length: UIntN,
}

/// Function signature for `EFI_REGULAR_EXPRESSION_PROTOCOL.MatchString`.
pub type EfiRegularExpressionMatch = unsafe extern "efiapi" fn(
    this: *mut EfiRegularExpressionProtocol,
    string: *const u16,
    pattern: *const u16,
    syntax_type: *const EfiGuid,
    result: *mut Bool,
    captures: *mut *mut EfiRegexCapture,
    captures_count: *mut UIntN,
) -> EfiStatus;

/// Function signature for `EFI_REGULAR_EXPRESSION_PROTOCOL.GetInfo`.
pub type EfiRegularExpressionGetInfo = unsafe extern "efiapi" fn(
    this: *mut EfiRegularExpressionProtocol,
    regex_syntax_type_list_size: *mut UIntN,
    regex_syntax_type_list: *mut EfiGuid,
) -> EfiStatus;

/// Raw interface structure for the UEFI Regular Expression Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiRegularExpressionProtocol {
    /// Matches a null-terminated UTF-16 string against a regular expression pattern.
    pub match_string: EfiRegularExpressionMatch,
    /// Returns supported regular expression syntax types.
    pub get_info: EfiRegularExpressionGetInfo,
}

/// Safe idiomatic client wrapper for the UEFI Regular Expression Protocol.
#[derive(Debug)]
pub struct RegularExpression<'a> {
    protocol: &'a EfiRegularExpressionProtocol,
}

impl<'a> RegularExpression<'a> {
    /// Regular Expression Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_REGULAR_EXPRESSION_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiRegularExpressionProtocol) -> Self {
        Self { protocol }
    }

    /// Obtains the list of supported regex syntax types.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small to receive all GUIDs.
    /// Returns `EfiStatus::DEVICE_ERROR` on firmware subsystem errors.
    pub fn get_supported_syntax_types(&self, buffer: &mut [EfiGuid]) -> Result<usize, EfiStatus> {
        let mut size = core::mem::size_of_val(buffer);
        let status = unsafe {
            (self.protocol.get_info)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut size,
                buffer.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(size / core::mem::size_of::<EfiGuid>())
        } else {
            Err(status)
        }
    }

    /// Tests if a null-terminated UTF-16 string matches a pattern.
    ///
    /// # Safety
    ///
    /// `string` and `pattern` must point to valid null-terminated UTF-16 character buffers.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if string or pattern pointers are invalid.
    /// Returns `EfiStatus::UNSUPPORTED` if the specified syntax type is not supported.
    pub unsafe fn matches(
        &self,
        string: *const u16,
        pattern: *const u16,
        syntax: Option<&EfiGuid>,
    ) -> Result<bool, EfiStatus> {
        let mut result = EFI_FALSE;
        let mut count: UIntN = 0;
        let syntax_ptr = match syntax {
            Some(g) => core::ptr::from_ref::<EfiGuid>(g),
            None => core::ptr::null(),
        };

        let status = unsafe {
            (self.protocol.match_string)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                string,
                pattern,
                syntax_ptr,
                &raw mut result,
                core::ptr::null_mut(),
                &raw mut count,
            )
        };

        if status.is_success() {
            Ok(result == EFI_TRUE)
        } else {
            Err(status)
        }
    }
}

// --- Native `#![no_std]` Regular Expression Engine ---

/// Match token parsed from a regex pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegexToken {
    /// Exact character match.
    Literal(u16),
    /// Any single character ('.').
    Any,
    /// Character class: matches if character is in inclusive range `[start, end]`.
    Range(u16, u16),
    /// Inverted range `[^start-end]`.
    NotRange(u16, u16),
    /// Digit `\d` (`0`..=`9`).
    Digit,
    /// Whitespace `\s` (space, tab, newline, carriage return).
    Whitespace,
    /// Word character `\w` (`a`..=`z`, `A`..=`Z`, `0`..=`9`, `_`).
    WordChar,
}

/// Quantifier modifier for a token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quantifier {
    /// Exactly once.
    Once,
    /// Zero or more times (`*`).
    ZeroOrMore,
    /// One or more times (`+`).
    OneOrMore,
    /// Zero or one time (`?`).
    ZeroOrOne,
}

/// An element in a compiled regular expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegexElement {
    token: RegexToken,
    quantifier: Quantifier,
}

impl RegexToken {
    fn matches(self, c: u16) -> bool {
        match self {
            Self::Literal(lit) => c == lit,
            Self::Any => true,
            Self::Range(low, high) => c >= low && c <= high,
            Self::NotRange(low, high) => c < low || c > high,
            Self::Digit => c >= u16::from(b'0') && c <= u16::from(b'9'),
            Self::Whitespace => {
                c == u16::from(b' ')
                    || c == u16::from(b'\t')
                    || c == u16::from(b'\n')
                    || c == u16::from(b'\r')
            }
            Self::WordChar => {
                (c >= u16::from(b'a') && c <= u16::from(b'z'))
                    || (c >= u16::from(b'A') && c <= u16::from(b'Z'))
                    || (c >= u16::from(b'0') && c <= u16::from(b'9'))
                    || c == u16::from(b'_')
            }
        }
    }
}

/// Maximum number of compiled regex tokens in bounded `#![no_std]` evaluation.
pub const MAX_REGEX_ELEMENTS: usize = 64;

/// Compiled bounded regular expression pattern.
#[derive(Clone, Copy, Debug)]
pub struct CompiledRegex {
    elements: [Option<RegexElement>; MAX_REGEX_ELEMENTS],
    element_count: usize,
    anchored_start: bool,
    anchored_end: bool,
}

impl CompiledRegex {
    /// Compiles a UTF-16 pattern into a bounded regex structure.
    #[must_use]
    pub fn compile(pattern: &[u16]) -> Option<Self> {
        let mut elements = [None; MAX_REGEX_ELEMENTS];
        let mut count = 0;
        let mut anchored_start = false;
        let mut anchored_end = false;

        let mut idx = 0;
        let len = pattern.len();

        if idx < len && pattern[idx] == u16::from(b'^') {
            anchored_start = true;
            idx += 1;
        }

        while idx < len {
            let ch = pattern[idx];
            if ch == u16::from(b'$') && idx + 1 == len {
                anchored_end = true;
                break;
            }

            let token = match ch {
                c if c == u16::from(b'.') => RegexToken::Any,
                c if c == u16::from(b'\\') => {
                    idx += 1;
                    if idx >= len {
                        return None;
                    }
                    match pattern[idx] {
                        esc if esc == u16::from(b'd') => RegexToken::Digit,
                        esc if esc == u16::from(b's') => RegexToken::Whitespace,
                        esc if esc == u16::from(b'w') => RegexToken::WordChar,
                        other => RegexToken::Literal(other),
                    }
                }
                c if c == u16::from(b'[') => {
                    idx += 1;
                    let inverted = if idx < len && pattern[idx] == u16::from(b'^') {
                        idx += 1;
                        true
                    } else {
                        false
                    };
                    if idx + 2 >= len {
                        return None;
                    }
                    let low = pattern[idx];
                    idx += 1;
                    if pattern[idx] != u16::from(b'-') {
                        return None;
                    }
                    idx += 1;
                    let high = pattern[idx];
                    idx += 1;
                    if idx >= len || pattern[idx] != u16::from(b']') {
                        return None;
                    }
                    if inverted {
                        RegexToken::NotRange(low, high)
                    } else {
                        RegexToken::Range(low, high)
                    }
                }
                literal => RegexToken::Literal(literal),
            };

            idx += 1;

            // Check following quantifier
            let quantifier = if idx < len {
                match pattern[idx] {
                    q if q == u16::from(b'*') => {
                        idx += 1;
                        Quantifier::ZeroOrMore
                    }
                    q if q == u16::from(b'+') => {
                        idx += 1;
                        Quantifier::OneOrMore
                    }
                    q if q == u16::from(b'?') => {
                        idx += 1;
                        Quantifier::ZeroOrOne
                    }
                    _ => Quantifier::Once,
                }
            } else {
                Quantifier::Once
            };

            if count >= MAX_REGEX_ELEMENTS {
                return None;
            }
            elements[count] = Some(RegexElement { token, quantifier });
            count += 1;
        }

        Some(Self {
            elements,
            element_count: count,
            anchored_start,
            anchored_end,
        })
    }

    /// Tests if the input UTF-16 slice matches this compiled regular expression.
    #[must_use]
    pub fn is_match(&self, input: &[u16]) -> bool {
        if self.anchored_start {
            self.match_at(input, 0)
        } else {
            for start in 0..=input.len() {
                if self.match_at(input, start) {
                    return true;
                }
            }
            false
        }
    }

    fn match_at(&self, input: &[u16], start: usize) -> bool {
        self.match_recursive(input, start, 0)
    }

    fn match_recursive(&self, input: &[u16], in_pos: usize, elem_idx: usize) -> bool {
        if elem_idx >= self.element_count {
            if self.anchored_end {
                return in_pos == input.len();
            }
            return true;
        }

        let Some(element) = self.elements[elem_idx] else {
            return false;
        };

        match element.quantifier {
            Quantifier::Once => {
                if in_pos < input.len() && element.token.matches(input[in_pos]) {
                    self.match_recursive(input, in_pos + 1, elem_idx + 1)
                } else {
                    false
                }
            }
            Quantifier::ZeroOrOne => {
                // Try matching 1 instance first (greedy)
                if in_pos < input.len()
                    && element.token.matches(input[in_pos])
                    && self.match_recursive(input, in_pos + 1, elem_idx + 1)
                {
                    return true;
                }
                // Fallback: match 0 instances
                self.match_recursive(input, in_pos, elem_idx + 1)
            }
            Quantifier::OneOrMore => {
                if in_pos >= input.len() || !element.token.matches(input[in_pos]) {
                    return false;
                }
                // Matched at least one; advance and treat remainder as ZeroOrMore
                let next_elem = RegexElement {
                    token: element.token,
                    quantifier: Quantifier::ZeroOrMore,
                };
                let mut sub_elements = self.elements;
                sub_elements[elem_idx] = Some(next_elem);
                let sub_regex = Self {
                    elements: sub_elements,
                    element_count: self.element_count,
                    anchored_start: self.anchored_start,
                    anchored_end: self.anchored_end,
                };
                sub_regex.match_recursive(input, in_pos + 1, elem_idx)
            }
            Quantifier::ZeroOrMore => {
                // Find how many consecutive characters match
                let mut max_match = 0;
                while in_pos + max_match < input.len()
                    && element.token.matches(input[in_pos + max_match])
                {
                    max_match += 1;
                }

                // Greedy backoff: from max_match down to 0
                for count in (0..=max_match).rev() {
                    if self.match_recursive(input, in_pos + count, elem_idx + 1) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

/// Evaluates whether `input` matches `pattern` using the built-in `#![no_std]` regex engine.
#[must_use]
pub fn regex_match(input: &[u16], pattern: &[u16]) -> bool {
    match CompiledRegex::compile(pattern) {
        Some(compiled) => compiled.is_match(input),
        None => false,
    }
}

/// Evaluates whether an ASCII / UTF-8 string matches a pattern.
#[must_use]
pub fn regex_match_str(input: &str, pattern: &str) -> bool {
    let mut in_buf = [0u16; 128];
    let mut pat_buf = [0u16; 64];

    let in_chars: &mut [u16] = if input.len() <= in_buf.len() {
        for (i, b) in input.bytes().enumerate() {
            in_buf[i] = u16::from(b);
        }
        &mut in_buf[..input.len()]
    } else {
        return false;
    };

    let pat_chars: &mut [u16] = if pattern.len() <= pat_buf.len() {
        for (i, b) in pattern.bytes().enumerate() {
            pat_buf[i] = u16::from(b);
        }
        &mut pat_buf[..pattern.len()]
    } else {
        return false;
    };

    regex_match(in_chars, pat_chars)
}
