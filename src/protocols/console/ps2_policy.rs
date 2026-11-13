//! # PS/2 Policy Protocol
//!
//! Abstracts platform-specific PS/2 keyboard and mouse initialization,
//! port reset, and LED indicators (Caps Lock, Num Lock, Scroll Lock).

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// EFI PS/2 Policy Protocol GUID: `{4df19259-dc71-4d46-bef1-357bb578c418}`.
pub const EFI_PS2_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4DF1_9259,
    0xDC71,
    0x4D46,
    [0xBE, 0xF1, 0x35, 0x7B, 0xB5, 0x78, 0xC4, 0x18],
);

/// PS/2 keyboard Scroll Lock LED flag.
pub const EFI_KEYBOARD_SCROLLLOCK: u8 = 0x01;
/// PS/2 keyboard Num Lock LED flag.
pub const EFI_KEYBOARD_NUMLOCK: u8 = 0x02;
/// PS/2 keyboard Caps Lock LED flag.
pub const EFI_KEYBOARD_CAPSLOCK: u8 = 0x04;

/// Platform-specific hardware initialization callback for PS/2 controllers.
pub type EfiPs2InitHardware = unsafe extern "efiapi" fn(handle: EfiHandle) -> EfiStatus;

/// EFI PS/2 Policy Protocol interface (`EFI_PS2_POLICY_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPs2PolicyProtocol {
    /// Keyboard LED light indicators (`EFI_KEYBOARD_*`).
    pub keyboard_light: u8,
    /// Platform hardware initialization hook.
    pub ps2_init_hardware: EfiPs2InitHardware,
}
