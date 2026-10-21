//! # Timer and watchdog protocols
//!
//! The firmware interfaces for periodic timers.

use crate::base::EfiStatus;

/// The timer architecture protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTimerArchProtocol {
    /// Registers a handler.
    pub register_handler: unsafe extern "efiapi" fn(*mut EfiTimerArchProtocol, usize) -> EfiStatus,
    /// Sets the timer period.
    pub set_timer_period: unsafe extern "efiapi" fn(*mut EfiTimerArchProtocol, u64) -> EfiStatus,
    /// Gets the timer period.
    pub get_timer_period:
        unsafe extern "efiapi" fn(*mut EfiTimerArchProtocol, *mut u64) -> EfiStatus,
    /// Generates a soft interrupt.
    pub generate_soft_interrupt: unsafe extern "efiapi" fn(*mut EfiTimerArchProtocol) -> EfiStatus,
}

/// The watchdog timer architecture protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiWatchdogTimerArchProtocol {
    /// Registers a handler.
    pub register_handler:
        unsafe extern "efiapi" fn(*mut EfiWatchdogTimerArchProtocol, usize) -> EfiStatus,
    /// Sets the timer period.
    pub set_timer_period:
        unsafe extern "efiapi" fn(*mut EfiWatchdogTimerArchProtocol, u64) -> EfiStatus,
    /// Gets the timer period.
    pub get_timer_period:
        unsafe extern "efiapi" fn(*mut EfiWatchdogTimerArchProtocol, *mut u64) -> EfiStatus,
}

/// A borrowed timer view.
pub struct TimerArch<'a> {
    protocol: &'a EfiTimerArchProtocol,
}

impl<'a> TimerArch<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiTimerArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiTimerArchProtocol {
        self.protocol
    }

    /// Gets the timer period in 100-ns units.
    ///
    /// A zero period means the timer is disabled.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_timer_period(&self) -> Result<u64, EfiStatus> {
        let mut period: u64 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_timer_period)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut period,
            )
        };
        if status.is_success() {
            Ok(period)
        } else {
            Err(status)
        }
    }

    /// Sets the timer period in 100-ns units.
    #[must_use]
    pub fn set_timer_period(&self, period: u64) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_timer_period)(core::ptr::from_ref(self.protocol).cast_mut(), period)
        }
    }

    /// Generates a soft interrupt.
    #[must_use]
    pub fn generate_soft_interrupt(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.generate_soft_interrupt)(core::ptr::from_ref(self.protocol).cast_mut())
        }
    }
}

impl core::fmt::Debug for TimerArch<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("TimerArch")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A borrowed watchdog view.
pub struct WatchdogTimerArch<'a> {
    protocol: &'a EfiWatchdogTimerArchProtocol,
}

impl<'a> WatchdogTimerArch<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiWatchdogTimerArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiWatchdogTimerArchProtocol {
        self.protocol
    }

    /// Gets the timer period in 100-ns units.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the request fails.
    pub fn get_timer_period(&self) -> Result<u64, EfiStatus> {
        let mut period: u64 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.get_timer_period)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut period,
            )
        };
        if status.is_success() {
            Ok(period)
        } else {
            Err(status)
        }
    }

    /// Sets the timer period in 100-ns units.
    #[must_use]
    pub fn set_timer_period(&self, period: u64) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_timer_period)(core::ptr::from_ref(self.protocol).cast_mut(), period)
        }
    }
}

impl core::fmt::Debug for WatchdogTimerArch<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("WatchdogTimerArch")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
