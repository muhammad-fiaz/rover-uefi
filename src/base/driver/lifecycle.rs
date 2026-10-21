//! # Driver lifecycle
//!
//! Native driver lifecycle trait and error type.

use crate::base::driver::device::Device;

/// A driver failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DriverError {
    /// The device is not supported.
    Unsupported,
    /// A resource is missing.
    MissingResource,
    /// The device refused the operation.
    Rejected,
    /// The operation timed out.
    Timeout,
}

impl core::fmt::Display for DriverError {
    /// Formats the error without allocation.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::Unsupported => "unsupported device",
            Self::MissingResource => "missing resource",
            Self::Rejected => "operation rejected",
            Self::Timeout => "operation timed out",
        };
        formatter.write_str(text)
    }
}

/// A native driver lifecycle.
///
/// Implementations must release resources acquired during `start`
/// when `stop` is called. The trait does not acquire hardware
/// resources itself.
pub trait Driver {
    /// Probes whether this driver supports the device.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError::Unsupported`] when the device cannot be
    /// driven.
    fn probe(&self, device: &Device) -> Result<(), DriverError>;

    /// Starts the device after a successful probe.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError::MissingResource`] when required
    /// resources are absent, or [`DriverError::Rejected`] when the
    /// device refuses to start.
    fn start(&mut self, device: &Device) -> Result<(), DriverError>;

    /// Stops the device and releases resources.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError::Timeout`] when shutdown times out.
    fn stop(&mut self, device: &Device) -> Result<(), DriverError>;

    /// Resets the device.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError::Rejected`] when reset is refused.
    fn reset(&mut self, device: &Device) -> Result<(), DriverError>;
}
