//! HTTP tests.

use core::mem::size_of;

use rover_uefi::protocols::network::guid::HTTP_GUID;
use rover_uefi::protocols::network::http::config::HttpVersion;
use rover_uefi::protocols::network::http::message::validate_header_bounds;
use rover_uefi::protocols::network::http::protocol::EfiHttpProtocol;

#[test]
fn header_bounds() {
    assert!(validate_header_bounds(10, 20));
    assert!(validate_header_bounds(20, 20));
    assert!(validate_header_bounds(0, 0));
    assert!(!validate_header_bounds(30, 20));
    assert!(!validate_header_bounds(1, 0));
}

#[test]
fn http_version_values() {
    assert_eq!(HttpVersion::V10.to_raw(), 0);
    assert_eq!(HttpVersion::V11.to_raw(), 1);
    assert_eq!(HttpVersion::from_raw(0), HttpVersion::V10);
    assert_eq!(HttpVersion::from_raw(1), HttpVersion::V11);
    assert_eq!(HttpVersion::from_raw(2).to_raw(), 2);
}

#[test]
fn http_protocol_layout() {
    assert_eq!(
        size_of::<EfiHttpProtocol>(),
        6 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        HTTP_GUID.to_bytes(),
        [
            0x9B, 0xB2, 0x59, 0x7A, 0x0B, 0x91, 0x71, 0x41, 0x82, 0x42, 0xA8, 0x5A, 0x0D, 0xF2,
            0x5B, 0x5B
        ]
    );
    assert!(!HTTP_GUID.is_zero());
}

use rover_uefi::protocols::network::http::message::{
    HttpError, content_length, find_header, parse_request, parse_response,
};

#[test]
fn request_parsing() {
    let input = b"GET /index.html HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\n\r\nhello";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(request.method, b"GET");
    assert_eq!(request.target, b"/index.html");
    assert_eq!(request.version_minor, 1);
    assert_eq!(&input[request.body_offset..], b"hello");
    assert_eq!(request.headers.clone().count(), 2);
    assert_eq!(
        find_header(&request.headers, b"host"),
        Some(b"example.com".as_slice())
    );
    assert_eq!(content_length(&request.headers), Ok(Some(5)));
}

#[test]
fn header_names_are_case_insensitive() {
    let input = b"POST /x HTTP/1.0\r\ncontent-length: 0\r\nX-Trace:   spaced  \r\n\r\n";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(request.version_minor, 0);
    assert_eq!(content_length(&request.headers), Ok(Some(0)));
    assert_eq!(
        find_header(&request.headers, b"x-trace"),
        Some(b"spaced".as_slice())
    );
}

#[test]
fn response_parsing() {
    let input = b"HTTP/1.1 404 Not Found\r\nServer: rover\r\n\r\nbody";
    let Ok(response) = parse_response(input) else {
        panic!("the response must parse")
    };
    assert_eq!(response.status, 404);
    assert_eq!(response.reason, b"Not Found");
    assert_eq!(response.version_minor, 1);
    assert_eq!(&input[response.body_offset..], b"body");
    assert_eq!(
        find_header(&response.headers, b"server"),
        Some(b"rover".as_slice())
    );
    assert_eq!(content_length(&response.headers), Ok(None));
}

#[test]
fn response_allows_an_empty_reason() {
    let input = b"HTTP/1.0 204\r\n\r\n";
    let Ok(response) = parse_response(input) else {
        panic!("the response must parse")
    };
    assert_eq!(response.status, 204);
    assert_eq!(response.reason, b"");
    assert_eq!(response.headers.count(), 0);
    assert_eq!(response.body_offset, input.len());
}

#[test]
fn malformed_start_lines_are_rejected() {
    assert_eq!(
        parse_request(b"GET\r\n\r\n"),
        Err(HttpError::InvalidStartLine)
    );
    assert_eq!(
        parse_request(b"GET  HTTP/1.1\r\n\r\n"),
        Err(HttpError::InvalidStartLine)
    );
    assert_eq!(
        parse_request(b"GET / HTTP/1.1 extra\r\n\r\n"),
        Err(HttpError::InvalidStartLine)
    );
    assert_eq!(
        parse_request(b"GET / HTTP/2.0\r\n\r\n"),
        Err(HttpError::UnsupportedVersion)
    );
    assert_eq!(
        parse_response(b"HTTP/1.1 20 OK\r\n\r\n"),
        Err(HttpError::InvalidStartLine)
    );
    assert_eq!(
        parse_response(b"HTTP/1.1 abc OK\r\n\r\n"),
        Err(HttpError::InvalidStartLine)
    );
}

#[test]
fn truncated_messages_are_rejected() {
    assert_eq!(parse_request(b""), Err(HttpError::Truncated));
    assert_eq!(parse_request(b"GET / HTTP/1.1"), Err(HttpError::Truncated));
    assert_eq!(
        parse_request(b"GET / HTTP/1.1\r\nHost: x"),
        Err(HttpError::Truncated)
    );
    assert_eq!(
        parse_response(b"HTTP/1.1 200 OK\r\n"),
        Err(HttpError::Truncated)
    );
}

#[test]
fn malformed_headers_are_rejected() {
    assert_eq!(
        parse_request(b"GET / HTTP/1.1\r\nNoColon\r\n\r\n"),
        Err(HttpError::InvalidHeader)
    );
    assert_eq!(
        parse_request(b"GET / HTTP/1.1\r\n: value\r\n\r\n"),
        Err(HttpError::InvalidHeader)
    );
    assert_eq!(
        parse_request(b"GET / HTTP/1.1\r\nBad Name: value\r\n\r\n"),
        Err(HttpError::InvalidHeader)
    );
}

#[test]
fn header_count_is_bounded() {
    let mut input = Vec::new();
    input.extend_from_slice(b"GET / HTTP/1.1\r\n");
    for index in 0..65 {
        input.extend_from_slice(format!("X-Pad-{index}: 1\r\n").as_bytes());
    }
    input.extend_from_slice(b"\r\n");
    assert_eq!(parse_request(&input), Err(HttpError::TooManyHeaders));
}

#[test]
fn content_length_rejects_bad_values() {
    let input = b"POST / HTTP/1.1\r\nContent-Length: 12a\r\n\r\n";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(
        content_length(&request.headers),
        Err(HttpError::InvalidContentLength)
    );

    let input = b"POST / HTTP/1.1\r\nContent-Length: 99999999999999999999999999\r\n\r\n";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(
        content_length(&request.headers),
        Err(HttpError::InvalidContentLength)
    );

    let input = b"POST / HTTP/1.1\r\nContent-Length:\r\n\r\n";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(
        content_length(&request.headers),
        Err(HttpError::InvalidContentLength)
    );
}

#[test]
fn bare_line_feeds_are_accepted() {
    let input = b"GET / HTTP/1.1\nHost: x\n\n";
    let Ok(request) = parse_request(input) else {
        panic!("the request must parse")
    };
    assert_eq!(
        find_header(&request.headers, b"host"),
        Some(b"x".as_slice())
    );
    assert_eq!(request.body_offset, input.len());
}
