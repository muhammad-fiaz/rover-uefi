//! # HTTP messages
//!
//! Bounded parsing of HTTP/1.0 and HTTP/1.1 request and response
//! messages.
//!
//! The parser borrows the input, allocates nothing, and validates the
//! start line and every header field before returning a view. Header
//! names are compared case-insensitively, as the protocol requires.
//! Message bodies are not interpreted; the caller receives the offset
//! at which the body starts.

/// The maximum number of header fields a message may carry.
pub const MAX_HEADER_FIELDS: usize = 64;

/// The minimum length of a header line, in bytes.
pub const MIN_HEADER_LINE: usize = 2;

/// A reason a message could not be parsed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HttpError {
    /// The message ends before its start line or header block.
    Truncated,
    /// The start line does not follow the request-line grammar.
    InvalidStartLine,
    /// The protocol version is not HTTP/1.0 or HTTP/1.1.
    UnsupportedVersion,
    /// A header field is malformed.
    InvalidHeader,
    /// The message carries more fields than [`MAX_HEADER_FIELDS`].
    TooManyHeaders,
    /// A `Content-Length` value is not a valid decimal length.
    InvalidContentLength,
}

/// One header field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HttpHeader<'a> {
    /// The field name, without surrounding whitespace.
    pub name: &'a [u8],
    /// The field value, without leading or trailing whitespace.
    pub value: &'a [u8],
}

impl HttpHeader<'_> {
    /// Returns whether the field name matches `name` case-insensitively.
    #[must_use]
    pub fn name_matches(&self, name: &[u8]) -> bool {
        ascii_eq_ignore_case(self.name, name)
    }
}

/// An iterator over the header fields of a message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpHeaders<'a> {
    bytes: &'a [u8],
    offset: usize,
    end: usize,
}

impl<'a> Iterator for HttpHeaders<'a> {
    type Item = HttpHeader<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.end {
            return None;
        }
        let (line, next) = next_line(self.bytes, self.offset)?;
        if next > self.end {
            return None;
        }
        self.offset = next;
        parse_header_line(line)
    }
}

/// A parsed HTTP request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    /// The method token, for example `GET`.
    pub method: &'a [u8],
    /// The request target.
    pub target: &'a [u8],
    /// The minor version, `0` for HTTP/1.0 and `1` for HTTP/1.1.
    pub version_minor: u8,
    /// The header fields of the request.
    pub headers: HttpHeaders<'a>,
    /// The offset of the message body in the input.
    pub body_offset: usize,
}

/// A parsed HTTP response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse<'a> {
    /// The status code.
    pub status: u16,
    /// The minor version, `0` for HTTP/1.0 and `1` for HTTP/1.1.
    pub version_minor: u8,
    /// The reason phrase, which may be empty.
    pub reason: &'a [u8],
    /// The header fields of the response.
    pub headers: HttpHeaders<'a>,
    /// The offset of the message body in the input.
    pub body_offset: usize,
}

/// Parses an HTTP request.
///
/// # Errors
///
/// Returns [`HttpError::Truncated`] when the message has no complete
/// header block, [`HttpError::InvalidStartLine`] for a malformed
/// request line, [`HttpError::UnsupportedVersion`] for a version other
/// than HTTP/1.0 or HTTP/1.1, [`HttpError::InvalidHeader`] for a
/// malformed field, and [`HttpError::TooManyHeaders`] when the field
/// count exceeds [`MAX_HEADER_FIELDS`].
pub fn parse_request(input: &[u8]) -> Result<HttpRequest<'_>, HttpError> {
    let (line, offset) = next_line(input, 0).ok_or(HttpError::Truncated)?;
    let mut parts = line.split(|byte| *byte == b' ');
    let method = parts.next().ok_or(HttpError::InvalidStartLine)?;
    let target = parts.next().ok_or(HttpError::InvalidStartLine)?;
    let version = parts.next().ok_or(HttpError::InvalidStartLine)?;
    if parts.next().is_some() {
        return Err(HttpError::InvalidStartLine);
    }
    if method.is_empty() || !method.iter().all(|byte| is_token_byte(*byte)) {
        return Err(HttpError::InvalidStartLine);
    }
    if target.is_empty() {
        return Err(HttpError::InvalidStartLine);
    }
    let version_minor = parse_version(version)?;
    let (headers, body_offset) = parse_headers(input, offset)?;
    Ok(HttpRequest {
        method,
        target,
        version_minor,
        headers,
        body_offset,
    })
}

/// Parses an HTTP response.
///
/// # Errors
///
/// Returns the same errors as [`parse_request`], plus
/// [`HttpError::InvalidStartLine`] for a status line without a
/// three-digit status code.
pub fn parse_response(input: &[u8]) -> Result<HttpResponse<'_>, HttpError> {
    let (line, offset) = next_line(input, 0).ok_or(HttpError::Truncated)?;
    let mut parts = line.splitn(3, |byte| *byte == b' ');
    let version = parts.next().ok_or(HttpError::InvalidStartLine)?;
    let status = parts.next().ok_or(HttpError::InvalidStartLine)?;
    let reason = parts.next().unwrap_or(&[]);
    let version_minor = parse_version(version)?;
    let status = parse_status_code(status)?;
    let (headers, body_offset) = parse_headers(input, offset)?;
    Ok(HttpResponse {
        status,
        version_minor,
        reason,
        headers,
        body_offset,
    })
}

/// Returns the value of the first field named `name`.
///
/// The comparison is case-insensitive. The iterator is cloned, so
/// the caller keeps ownership of the header view.
#[must_use]
pub fn find_header<'a>(headers: &HttpHeaders<'a>, name: &[u8]) -> Option<&'a [u8]> {
    headers
        .clone()
        .into_iter()
        .find(|header| header.name_matches(name))
        .map(|header| header.value)
}

/// Returns the `Content-Length` of a message, if present.
///
/// # Errors
///
/// Returns [`HttpError::InvalidContentLength`] when the field is not a
/// decimal number or does not fit into a `usize`.
pub fn content_length(headers: &HttpHeaders<'_>) -> Result<Option<usize>, HttpError> {
    let Some(value) = find_header(headers, b"content-length") else {
        return Ok(None);
    };
    if value.is_empty() {
        return Err(HttpError::InvalidContentLength);
    }
    let mut length = 0_usize;
    for byte in value {
        let digit = byte.wrapping_sub(b'0');
        if digit > 9 {
            return Err(HttpError::InvalidContentLength);
        }
        length = length
            .checked_mul(10)
            .and_then(|value| value.checked_add(usize::from(digit)))
            .ok_or(HttpError::InvalidContentLength)?;
    }
    Ok(Some(length))
}

/// Returns whether a header block fits within a buffer.
#[must_use]
pub fn validate_header_bounds(headers_byte_count: usize, buffer_len: usize) -> bool {
    headers_byte_count <= buffer_len
}

/// Parses the header block that starts at `offset`.
fn parse_headers(input: &[u8], offset: usize) -> Result<(HttpHeaders<'_>, usize), HttpError> {
    let mut cursor = offset;
    let mut count = 0_usize;
    loop {
        let line_start = cursor;
        let (line, next) = next_line(input, cursor).ok_or(HttpError::Truncated)?;
        cursor = next;
        if line.is_empty() {
            return Ok((
                HttpHeaders {
                    bytes: input,
                    offset,
                    end: line_start,
                },
                cursor,
            ));
        }
        count = count.checked_add(1).ok_or(HttpError::TooManyHeaders)?;
        if count > MAX_HEADER_FIELDS {
            return Err(HttpError::TooManyHeaders);
        }
        if line.len() < MIN_HEADER_LINE || parse_header_line(line).is_none() {
            return Err(HttpError::InvalidHeader);
        }
    }
}

/// Parses one `name: value` field.
fn parse_header_line(line: &[u8]) -> Option<HttpHeader<'_>> {
    let colon = line.iter().position(|byte| *byte == b':')?;
    let name = trim_trailing(line.get(..colon)?);
    if name.is_empty() || !name.iter().all(|byte| is_token_byte(*byte)) {
        return None;
    }
    let value = trim(line.get(colon.checked_add(1)?..)?);
    Some(HttpHeader { name, value })
}

/// Parses `HTTP/1.0` or `HTTP/1.1` and returns the minor version.
fn parse_version(version: &[u8]) -> Result<u8, HttpError> {
    match version {
        b"HTTP/1.0" => Ok(0),
        b"HTTP/1.1" => Ok(1),
        _ => Err(HttpError::UnsupportedVersion),
    }
}

/// Parses a three-digit status code.
fn parse_status_code(status: &[u8]) -> Result<u16, HttpError> {
    if status.len() != 3 || !status.iter().all(u8::is_ascii_digit) {
        return Err(HttpError::InvalidStartLine);
    }
    let mut value = 0_u16;
    for byte in status {
        value = value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u16::from(byte.wrapping_sub(b'0'))))
            .ok_or(HttpError::InvalidStartLine)?;
    }
    Ok(value)
}

/// Returns the line starting at `offset` without its terminator, and
/// the offset just past the terminator.
fn next_line(input: &[u8], offset: usize) -> Option<(&[u8], usize)> {
    let rest = input.get(offset..)?;
    let newline = rest.iter().position(|byte| *byte == b'\n')?;
    let mut line = rest.get(..newline)?;
    if line.last().copied() == Some(b'\r') {
        line = line.get(..line.len().saturating_sub(1))?;
    }
    Some((line, offset.checked_add(newline)?.checked_add(1)?))
}

/// Trims leading and trailing spaces and tabs.
fn trim(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|byte| *byte != b' ' && *byte != b'\t')
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|byte| *byte != b' ' && *byte != b'\t')
        .map_or(start, |index| index.saturating_add(1));
    bytes.get(start..end).unwrap_or(&[])
}

/// Trims trailing spaces and tabs.
fn trim_trailing(bytes: &[u8]) -> &[u8] {
    let end = bytes
        .iter()
        .rposition(|byte| *byte != b' ' && *byte != b'\t')
        .map_or(0, |index| index.saturating_add(1));
    bytes.get(..end).unwrap_or(&[])
}

/// Returns whether `byte` may appear in a token or a header name.
fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

/// Compares two byte slices ignoring ASCII case.
fn ascii_eq_ignore_case(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
}
