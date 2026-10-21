//! # TCP configuration
//!
//! Configuration helpers.

/// A TCP configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TcpConfig {
    /// The station port.
    pub station_port: u16,
}
