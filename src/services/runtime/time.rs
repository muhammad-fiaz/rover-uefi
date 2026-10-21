//! # Time
//!
//! Wall-clock time and wakeup alarm services.
//!
//! The safe methods use stack locals for firmware
//! output parameters. Time values are not
//! validated by the wrapper beyond the firmware
//! status. The timezone is in minutes from UTC,
//! with an unspecified value meaning unknown.
//! Daylight flags use the constants in
//! `base::time`. Capabilities describe the
//! resolution, accuracy, and zeroing behavior
//! of the real-time clock.

use crate::abi::types::Bool;
use crate::base::{EfiStatus, EfiTime, EfiTimeCapabilities};
use crate::services::runtime::RuntimeServices;

impl RuntimeServices<'_> {
    /// Returns the current time.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// time cannot be retrieved.
    pub fn get_time(&self) -> Result<EfiTime, EfiStatus> {
        let mut time = EfiTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            pad1: 0,
            nanosecond: 0,
            timezone: 0,
            daylight: 0,
            pad2: 0,
        };
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe { (self.table.get_time)(&raw mut time, core::ptr::null_mut()) };
        if status.is_success() {
            Ok(time)
        } else {
            Err(status)
        }
    }

    /// Returns the time and capabilities.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// time cannot be retrieved.
    pub fn get_time_with_capabilities(&self) -> Result<(EfiTime, EfiTimeCapabilities), EfiStatus> {
        let mut time = EfiTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            pad1: 0,
            nanosecond: 0,
            timezone: 0,
            daylight: 0,
            pad2: 0,
        };
        let mut capabilities = EfiTimeCapabilities {
            resolution: 0,
            accuracy: 0,
            sets_to_zero: 0,
        };
        // SAFETY: Both stack locals are valid for their out parameters.
        let status = unsafe { (self.table.get_time)(&raw mut time, &raw mut capabilities) };
        if status.is_success() {
            Ok((time, capabilities))
        } else {
            Err(status)
        }
    }

    /// Sets the current time.
    #[must_use]
    pub fn set_time(&self, time: &EfiTime) -> EfiStatus {
        // SAFETY: The reference is valid for the call and firmware only reads the time.
        unsafe { (self.table.set_time)(core::ptr::from_ref(time).cast_mut()) }
    }

    /// Returns the wakeup alarm setting.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// alarm cannot be retrieved.
    pub fn get_wakeup_time(&self) -> Result<(Bool, Bool, EfiTime), EfiStatus> {
        let mut enabled: Bool = 0;
        let mut pending: Bool = 0;
        let mut time = EfiTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            pad1: 0,
            nanosecond: 0,
            timezone: 0,
            daylight: 0,
            pad2: 0,
        };
        // SAFETY: All stack locals are valid for their out parameters.
        let status = unsafe {
            (self.table.get_wakeup_time)(&raw mut enabled, &raw mut pending, &raw mut time)
        };
        if status.is_success() {
            Ok((enabled, pending, time))
        } else {
            Err(status)
        }
    }

    /// Sets the wakeup alarm.
    ///
    /// When `enable` is false, the time may be
    /// `None` and the alarm is disabled.
    #[must_use]
    pub fn set_wakeup_time(&self, enable: Bool, time: Option<&EfiTime>) -> EfiStatus {
        let time_ptr = match time {
            Some(value) => core::ptr::from_ref(value).cast_mut(),
            None => core::ptr::null_mut(),
        };
        // SAFETY: The reference (when present) is valid for the call.
        unsafe { (self.table.set_wakeup_time)(enable, time_ptr) }
    }
}
