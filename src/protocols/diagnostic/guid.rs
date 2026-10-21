//! # Diagnostic identifiers
//!
//! Official timer, watchdog and status-code `GUID` constants.

use crate::base::EfiGuid;

/// The timer architecture protocol `GUID`.
pub const TIMER_ARCH_GUID: EfiGuid = EfiGuid::new(
    0x26BA_CCB3,
    0x6F42,
    0x11D4,
    [0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// The watchdog timer architecture protocol `GUID`.
pub const WATCHDOG_TIMER_ARCH_GUID: EfiGuid = EfiGuid::new(
    0x665E_3FF5,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The status-code runtime protocol `GUID`.
pub const STATUS_CODE_RUNTIME_GUID: EfiGuid = EfiGuid::new(
    0xD2B2_B828,
    0x0826,
    0x48A7,
    [0xB3, 0xDF, 0x98, 0x3C, 0x00, 0x60, 0x24, 0xF0],
);
