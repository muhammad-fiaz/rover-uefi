//! # Diagnostics
//!
//! Timer, watchdog and status-code reporting.

pub mod guid;
pub mod status_code;
pub mod timer;
pub mod types;

pub use guid::{STATUS_CODE_RUNTIME_GUID, TIMER_ARCH_GUID, WATCHDOG_TIMER_ARCH_GUID};
pub use status_code::{EfiStatusCodeProtocol, StatusCode};
pub use timer::{EfiTimerArchProtocol, EfiWatchdogTimerArchProtocol, TimerArch, WatchdogTimerArch};
