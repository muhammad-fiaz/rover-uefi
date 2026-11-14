//! # Diagnostics
//!
//! Timer, watchdog and status-code reporting.

pub mod guid;
pub mod metronome;
pub mod performance_measurement;
pub mod status_code;
pub mod timer;
pub mod timestamp;
pub mod types;

pub use guid::{STATUS_CODE_RUNTIME_GUID, TIMER_ARCH_GUID, WATCHDOG_TIMER_ARCH_GUID};
pub use metronome::{
    EfiMetronomeArchProtocol, EfiMetronomeWaitForTick, METRONOME_ARCH_PROTOCOL_GUID, MetronomeArch,
};
pub use performance_measurement::{
    CreatePerformanceMeasurement, EfiPerformanceExProtocol, EfiPerformanceMeasurementProtocol,
    EfiPerformanceProtocol, GaugeDataEntry, GaugeDataEntryEx, PERFORMANCE_EX_PROTOCOL_GUID,
    PERFORMANCE_MEASUREMENT_PROTOCOL_GUID, PERFORMANCE_PROTOCOL_GUID, PERFORMANCE_STRING_SIZE,
    PerfMeasurementAttribute, Performance, PerformanceEndGauge, PerformanceEndGaugeEx,
    PerformanceEx, PerformanceGetGauge, PerformanceGetGaugeEx, PerformanceMeasurement,
    PerformanceStartGauge, PerformanceStartGaugeEx, SMM_PERFORMANCE_EX_PROTOCOL_GUID,
    SMM_PERFORMANCE_MEASUREMENT_PROTOCOL_GUID, SMM_PERFORMANCE_PROTOCOL_GUID,
};
pub use status_code::{EfiStatusCodeProtocol, StatusCode};
pub use timer::{EfiTimerArchProtocol, EfiWatchdogTimerArchProtocol, TimerArch, WatchdogTimerArch};
pub use timestamp::{
    EfiTimestampGet, EfiTimestampGetProperties, EfiTimestampProperties, EfiTimestampProtocol,
    TIMESTAMP_PROTOCOL_GUID, TimestampProtocol,
};
