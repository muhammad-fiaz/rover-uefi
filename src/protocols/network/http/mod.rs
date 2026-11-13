//! # HTTP
//!
//! HTTP protocol access.

pub mod config;
pub mod message;
pub mod protocol;
pub mod utilities;

pub use protocol::{EfiHttpProtocol, Http};
pub use utilities::{
    EFI_HTTP_UTILITIES_PROTOCOL_GUID, EfiHttpHeader, EfiHttpUtilitiesProtocol, EfiHttpUtilsBuild,
    EfiHttpUtilsParse, HttpUtilities, format_header_line, parse_header_line, parse_raw_headers,
};
