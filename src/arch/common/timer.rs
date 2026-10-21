//! # Timer abstraction
//!
//! A hardware independent description of a monotonic time source.
//!
//! Firmware rarely programs a timer directly. It asks for the
//! current time, arms a one-shot deadline and cancels it again.
//! The [`Timer`] trait captures exactly that surface, so the
//! architecture modules only have to describe how a deadline is
//! programmed.
//!
//! The module also provides the arithmetic that surrounds a
//! periodic tick source: converting between nanoseconds and
//! clock ticks, and accumulating the number of ticks that
//! elapsed over an arbitrary span of time. All of the arithmetic
//! is overflow safe; it saturates instead of wrapping.

/// The number of nanoseconds in a microsecond.
pub const NANOS_PER_MICRO: u64 = 1_000;

/// The number of nanoseconds in a millisecond.
pub const NANOS_PER_MILLI: u64 = 1_000_000;

/// The number of nanoseconds in a second.
pub const NANOS_PER_SEC: u64 = 1_000_000_000;

/// The number of nanoseconds in a minute.
pub const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SEC;

/// The number of nanoseconds in an hour.
pub const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;

/// The number of milliseconds in a second.
pub const MILLIS_PER_SEC: u64 = 1_000;

/// Converts a count of milliseconds to nanoseconds.
///
/// The conversion saturates at [`u64::MAX`] instead of
/// wrapping around.
#[must_use]
pub const fn millis_to_nanos(millis: u64) -> u64 {
    millis.saturating_mul(NANOS_PER_MILLI)
}

/// Converts a count of nanoseconds to milliseconds.
///
/// The conversion truncates towards zero.
#[must_use]
pub const fn nanos_to_millis(nanos: u64) -> u64 {
    nanos / NANOS_PER_MILLI
}

/// Converts a count of seconds to nanoseconds.
///
/// The conversion saturates at [`u64::MAX`] instead of
/// wrapping around.
#[must_use]
pub const fn secs_to_nanos(secs: u64) -> u64 {
    secs.saturating_mul(NANOS_PER_SEC)
}

/// Converts a count of nanoseconds to seconds.
///
/// The conversion truncates towards zero.
#[must_use]
pub const fn nanos_to_secs(nanos: u64) -> u64 {
    nanos / NANOS_PER_SEC
}

/// Converts a duration in nanoseconds to a count of timer ticks.
///
/// `frequency_hz` is the rate at which the time source
/// increments. A frequency of zero has no meaningful conversion
/// and yields zero ticks. The conversion saturates at
/// [`u64::MAX`] instead of wrapping around.
#[must_use]
pub const fn nanos_to_ticks(nanos: u64, frequency_hz: u64) -> u64 {
    let whole = (nanos / NANOS_PER_SEC).saturating_mul(frequency_hz);
    let fraction = (nanos % NANOS_PER_SEC).saturating_mul(frequency_hz) / NANOS_PER_SEC;
    whole.saturating_add(fraction)
}

/// Converts a count of timer ticks to a duration in nanoseconds.
///
/// `frequency_hz` is the rate at which the time source
/// increments. A frequency of zero has no meaningful conversion
/// and yields zero nanoseconds. The conversion saturates at
/// [`u64::MAX`] instead of wrapping around.
#[must_use]
pub const fn ticks_to_nanos(ticks: u64, frequency_hz: u64) -> u64 {
    if frequency_hz == 0 {
        return 0;
    }
    let whole = (ticks / frequency_hz).saturating_mul(NANOS_PER_SEC);
    let fraction = (ticks % frequency_hz).saturating_mul(NANOS_PER_SEC) / frequency_hz;
    whole.saturating_add(fraction)
}

/// The reason a timer operation could not be completed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimerError {
    /// The time source cannot arm a deadline.
    Unsupported,
    /// A deadline is already armed.
    AlreadyArmed,
    /// No deadline is armed.
    NotArmed,
    /// The requested period is not usable.
    InvalidPeriod,
}

/// A monotonic time source with an optional one-shot deadline.
///
/// The trait describes the behaviour that the architecture
/// modules implement: reading the current time, arming a
/// deadline and cancelling it again.
pub trait Timer {
    /// Returns the current value of the time source.
    ///
    /// The value is a monotonically increasing count of
    /// nanoseconds since an unspecified origin.
    #[must_use]
    fn now_nanos(&self) -> u64;

    /// Arms a one-shot deadline at the given time.
    ///
    /// The deadline is expressed in the same time base as
    /// [`Timer::now_nanos`].
    ///
    /// # Errors
    ///
    /// Returns [`TimerError::Unsupported`] when the time source
    /// cannot arm a deadline and [`TimerError::AlreadyArmed`]
    /// when a deadline is still pending.
    fn set_deadline(&mut self, deadline_nanos: u64) -> Result<(), TimerError>;

    /// Cancels the armed deadline.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError::NotArmed`] when no deadline is
    /// pending.
    fn cancel(&mut self) -> Result<(), TimerError>;
}

/// A periodic tick accumulator.
///
/// The accumulator tracks the next tick deadline of a periodic
/// time source. Every call to [`TickAccumulator::advance`]
/// reports how many tick deadlines have passed since the
/// previous call and moves the deadline forward. The arithmetic
/// is overflow safe, so a long sleep cannot wrap the deadline
/// back into the past.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TickAccumulator {
    period_nanos: u64,
    next_deadline_nanos: u64,
}

impl TickAccumulator {
    /// Creates an accumulator for a periodic source.
    ///
    /// The first tick deadline is `start_nanos` plus one
    /// period. A period of zero disables the accumulator; every
    /// subsequent [`TickAccumulator::advance`] reports zero
    /// ticks.
    #[must_use]
    pub const fn new(period_nanos: u64, start_nanos: u64) -> Self {
        Self {
            period_nanos,
            next_deadline_nanos: start_nanos.saturating_add(period_nanos),
        }
    }

    /// Returns the period of the tick source.
    #[must_use]
    pub const fn period_nanos(&self) -> u64 {
        self.period_nanos
    }

    /// Returns the next tick deadline.
    #[must_use]
    pub const fn next_deadline_nanos(&self) -> u64 {
        self.next_deadline_nanos
    }

    /// Returns the number of whole periods in an elapsed time.
    ///
    /// A period of zero has no whole periods and yields zero.
    #[must_use]
    pub fn ticks_in(elapsed_nanos: u64, period_nanos: u64) -> u64 {
        elapsed_nanos.checked_div(period_nanos).unwrap_or_default()
    }

    /// Reports the number of tick deadlines that have passed.
    ///
    /// The deadline is moved forward so that it lies strictly
    /// after `now_nanos`. When the time source slept for many
    /// periods, every missed deadline is reported, which lets
    /// the caller resynchronise instead of losing ticks.
    pub fn advance(&mut self, now_nanos: u64) -> u64 {
        if self.period_nanos == 0 {
            return 0;
        }
        if now_nanos < self.next_deadline_nanos {
            return 0;
        }
        let elapsed = now_nanos - self.next_deadline_nanos;
        let periods = elapsed / self.period_nanos;
        let ticks = periods.saturating_add(1);
        let step = self.period_nanos.saturating_mul(ticks);
        self.next_deadline_nanos = self.next_deadline_nanos.saturating_add(step);
        ticks
    }

    /// Changes the period of the tick source.
    ///
    /// The next deadline is recomputed from the current
    /// deadline and the new period.
    pub fn set_period(&mut self, period_nanos: u64) {
        self.period_nanos = period_nanos;
    }
}

/// A monotonic timer built on a tick counter.
///
/// The timer reads the raw counter through a function pointer
/// and converts it to nanoseconds with the configured
/// frequency. The deadline is tracked in nanoseconds so that
/// the same value can be compared against
/// [`Timer::now_nanos`].
#[derive(Clone, Copy)]
pub struct MonotonicTimer {
    frequency_hz: u64,
    read_ticks: fn() -> u64,
    deadline_nanos: Option<u64>,
}

impl core::fmt::Debug for MonotonicTimer {
    /// Formats the timer without reading the counter.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("MonotonicTimer")
            .field("frequency_hz", &self.frequency_hz)
            .field("read_ticks", &self.read_ticks)
            .field("deadline_nanos", &self.deadline_nanos)
            .finish()
    }
}

impl MonotonicTimer {
    /// Creates a timer that reads its counter through the given
    /// function pointer.
    ///
    /// The function pointer must return a monotonically
    /// increasing tick count.
    #[must_use]
    pub const fn new(frequency_hz: u64, read_ticks: fn() -> u64) -> Self {
        Self {
            frequency_hz,
            read_ticks,
            deadline_nanos: None,
        }
    }

    /// Returns the frequency of the underlying tick counter.
    #[must_use]
    pub const fn frequency_hz(&self) -> u64 {
        self.frequency_hz
    }

    /// Returns the armed deadline, if any.
    #[must_use]
    pub const fn deadline_nanos(&self) -> Option<u64> {
        self.deadline_nanos
    }

    /// Returns whether a deadline is armed.
    #[must_use]
    pub const fn is_armed(&self) -> bool {
        self.deadline_nanos.is_some()
    }
}

impl Timer for MonotonicTimer {
    fn now_nanos(&self) -> u64 {
        ticks_to_nanos((self.read_ticks)(), self.frequency_hz)
    }

    fn set_deadline(&mut self, deadline_nanos: u64) -> Result<(), TimerError> {
        if self.frequency_hz == 0 {
            return Err(TimerError::Unsupported);
        }
        if self.deadline_nanos.is_some() {
            return Err(TimerError::AlreadyArmed);
        }
        self.deadline_nanos = Some(deadline_nanos);
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), TimerError> {
        match self.deadline_nanos.take() {
            Some(_) => Ok(()),
            None => Err(TimerError::NotArmed),
        }
    }
}
