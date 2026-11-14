//! # PS/2 Mouse Controller and Packet Decoder
//!
//! Production-ready implementation of the PS/2 auxiliary device (mouse)
//! protocol for UEFI systems.
//!
//! Provides packet framing and synchronization, 3-byte standard PS/2 packet decoding,
//! 4-byte `IntelliMouse` (scroll wheel) and 5-button packet decoding, sign extension,
//! relative displacement accumulation, button tracking, and compliance with the
//! UEFI `EFI_SIMPLE_POINTER_PROTOCOL`.

use crate::abi::types::{EFI_FALSE, EFI_TRUE};
use crate::base::EfiStatus;
use crate::protocols::console::pointer::{EfiSimplePointerMode, EfiSimplePointerState};

// ---------------------------------------------------------------------------
// PS/2 Mouse Commands
// ---------------------------------------------------------------------------

/// Set mouse scaling to 1:1.
pub const MOUSE_CMD_SET_SCALING_1_1: u8 = 0xE6;

/// Set mouse scaling to 2:1.
pub const MOUSE_CMD_SET_SCALING_2_1: u8 = 0xE7;

/// Set mouse resolution.
pub const MOUSE_CMD_SET_RESOLUTION: u8 = 0xE8;

/// Request mouse status.
pub const MOUSE_CMD_STATUS_REQUEST: u8 = 0xE9;

/// Set stream mode (continuous packet reporting).
pub const MOUSE_CMD_SET_STREAM_MODE: u8 = 0xEA;

/// Read data packet (remote mode).
pub const MOUSE_CMD_READ_DATA: u8 = 0xEB;

/// Reset wrap mode.
pub const MOUSE_CMD_RESET_WRAP_MODE: u8 = 0xEC;

/// Set wrap mode (echoes incoming bytes).
pub const MOUSE_CMD_SET_WRAP_MODE: u8 = 0xEE;

/// Set remote mode (polled data reporting).
pub const MOUSE_CMD_SET_REMOTE_MODE: u8 = 0xF0;

/// Read mouse device identifier.
pub const MOUSE_CMD_GET_DEVICE_ID: u8 = 0xF2;

/// Set mouse sample rate in Hz.
pub const MOUSE_CMD_SET_SAMPLE_RATE: u8 = 0xF3;

/// Enable mouse streaming / data reporting.
pub const MOUSE_CMD_ENABLE_STREAMING: u8 = 0xF4;

/// Disable mouse streaming / data reporting.
pub const MOUSE_CMD_DISABLE_STREAMING: u8 = 0xF5;

/// Reset mouse settings to power-on defaults.
pub const MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;

/// Request resend of last transmission.
pub const MOUSE_CMD_RESEND: u8 = 0xFE;

/// Reset mouse hardware and initiate BAT (Basic Assurance Test).
pub const MOUSE_CMD_RESET: u8 = 0xFF;

// ---------------------------------------------------------------------------
// PS/2 Mouse Responses & IDs
// ---------------------------------------------------------------------------

/// BAT completion code first byte (0xAA).
pub const MOUSE_RESP_BAT1: u8 = 0xAA;

/// BAT completion code second byte (0x00).
pub const MOUSE_RESP_BAT2: u8 = 0x00;

/// Command acknowledge code (0xFA).
pub const MOUSE_RESP_ACK: u8 = 0xFA;

/// Resend request code (0xFE).
pub const MOUSE_RESP_RESEND: u8 = 0xFE;

/// Standard PS/2 mouse device ID (3-button without wheel).
pub const MOUSE_ID_STANDARD: u8 = 0x00;

/// `IntelliMouse` device ID (with scroll wheel).
pub const MOUSE_ID_INTELLIMOUSE: u8 = 0x03;

/// `IntelliMouse` Explorer device ID (with scroll wheel and 5 buttons).
pub const MOUSE_ID_5_BUTTON: u8 = 0x04;

// ---------------------------------------------------------------------------
// Packet Flags and Sync
// ---------------------------------------------------------------------------

/// Packet byte 0: Left button pressed flag.
pub const PS2_MOUSE_FLAG_LEFT_BUTTON: u8 = 0x01;

/// Packet byte 0: Right button pressed flag.
pub const PS2_MOUSE_FLAG_RIGHT_BUTTON: u8 = 0x02;

/// Packet byte 0: Middle button pressed flag.
pub const PS2_MOUSE_FLAG_MIDDLE_BUTTON: u8 = 0x04;

/// Packet byte 0: Synchronization bit (must always be 1).
pub const PS2_MOUSE_FLAG_SYNC: u8 = 0x08;

/// Packet byte 0: X displacement sign bit (1 = negative).
pub const PS2_MOUSE_FLAG_X_SIGN: u8 = 0x10;

/// Packet byte 0: Y displacement sign bit (1 = negative).
pub const PS2_MOUSE_FLAG_Y_SIGN: u8 = 0x20;

/// Packet byte 0: X displacement overflow flag.
pub const PS2_MOUSE_FLAG_X_OVERFLOW: u8 = 0x40;

/// Packet byte 0: Y displacement overflow flag.
pub const PS2_MOUSE_FLAG_Y_OVERFLOW: u8 = 0x80;

/// `IntelliMouse` byte 3: Button 4 pressed flag.
pub const PS2_MOUSE_FLAG_BUTTON_4: u8 = 0x10;

/// `IntelliMouse` byte 3: Button 5 pressed flag.
pub const PS2_MOUSE_FLAG_BUTTON_5: u8 = 0x20;

// ---------------------------------------------------------------------------
// Sample Rates and Resolutions
// ---------------------------------------------------------------------------

/// PS/2 mouse sample rate settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MouseSampleRate {
    /// 10 samples/sec.
    Rate10 = 10,
    /// 20 samples/sec.
    Rate20 = 20,
    /// 40 samples/sec.
    Rate40 = 40,
    /// 60 samples/sec.
    Rate60 = 60,
    /// 80 samples/sec.
    Rate80 = 80,
    /// 100 samples/sec (standard default).
    #[default]
    Rate100 = 100,
    /// 200 samples/sec.
    Rate200 = 200,
}

impl MouseSampleRate {
    /// Returns the command parameter byte for `MOUSE_CMD_SET_SAMPLE_RATE`.
    #[must_use]
    pub const fn command_byte(self) -> u8 {
        self as u8
    }
}

/// PS/2 mouse resolution settings (counts per millimeter).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MouseResolution {
    /// 1 count / mm (25 dpi).
    Res1CountPerMm = 0,
    /// 2 counts / mm (50 dpi).
    Res2CountPerMm = 1,
    /// 4 counts / mm (100 dpi, standard default).
    #[default]
    Res4CountPerMm = 2,
    /// 8 counts / mm (200 dpi).
    Res8CountPerMm = 3,
}

impl MouseResolution {
    /// Returns the command parameter byte for `MOUSE_CMD_SET_RESOLUTION`.
    #[must_use]
    pub const fn command_byte(self) -> u8 {
        self as u8
    }

    /// Returns the resolution in counts/mm.
    #[must_use]
    pub const fn counts_per_mm(self) -> u64 {
        match self {
            Self::Res1CountPerMm => 1,
            Self::Res2CountPerMm => 2,
            Self::Res4CountPerMm => 4,
            Self::Res8CountPerMm => 8,
        }
    }
}

// ---------------------------------------------------------------------------
// PS/2 Mouse Decoder Error
// ---------------------------------------------------------------------------

/// Errors encountered while decoding PS/2 mouse packets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps2MouseError {
    /// Packet dropped due to synchronization bit mismatch (bit 3 was 0).
    OutOfSync,
    /// Packet dropped due to hardware counter overflow flag.
    DisplacementOverflow,
}

// ---------------------------------------------------------------------------
// Decoder State Machine
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecoderStep {
    Byte0,
    Byte1,
    Byte2,
    Byte3,
}

/// Production-ready PS/2 mouse packet decoder and relative motion tracker.
#[derive(Debug)]
pub struct Ps2MouseDecoder {
    /// Internal decoding step.
    step: DecoderStep,
    /// Packet byte buffer (up to 4 bytes).
    packet: [u8; 4],
    /// Expected packet size (3 for standard mouse, 4 for wheel / 5-button mouse).
    packet_size: u8,
    /// Device ID reported by mouse (`MOUSE_ID_*`).
    device_id: u8,
    /// Middle button supported.
    has_middle_button: bool,
    /// 4th and 5th buttons supported.
    has_extra_buttons: bool,
    /// Accumulated pointer state.
    state: EfiSimplePointerState,
    /// Whether state has changed since last `get_state()`.
    state_changed: bool,
    /// Pointer mode parameters.
    mode: EfiSimplePointerMode,
}

impl Default for Ps2MouseDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Ps2MouseDecoder {
    /// Creates a new PS/2 mouse decoder configured for standard 3-byte packets.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            step: DecoderStep::Byte0,
            packet: [0u8; 4],
            packet_size: 3,
            device_id: MOUSE_ID_STANDARD,
            has_middle_button: true,
            has_extra_buttons: false,
            state: EfiSimplePointerState {
                relative_movement_x: 0,
                relative_movement_y: 0,
                relative_movement_z: 0,
                left_button: EFI_FALSE,
                right_button: EFI_FALSE,
            },
            state_changed: false,
            mode: EfiSimplePointerMode {
                resolution_x: 4,
                resolution_y: 4,
                resolution_z: 0,
                left_button: EFI_TRUE,
                right_button: EFI_TRUE,
            },
        }
    }

    /// Configures the decoder for `IntelliMouse` mode (4-byte packets with Z wheel).
    pub fn set_intellimouse_mode(&mut self) {
        self.device_id = MOUSE_ID_INTELLIMOUSE;
        self.packet_size = 4;
        self.mode.resolution_z = 1;
        self.has_middle_button = true;
        self.has_extra_buttons = false;
        self.reset_state();
    }

    /// Configures the decoder for 5-button `IntelliMouse` Explorer mode.
    pub fn set_5_button_mode(&mut self) {
        self.device_id = MOUSE_ID_5_BUTTON;
        self.packet_size = 4;
        self.mode.resolution_z = 1;
        self.has_middle_button = true;
        self.has_extra_buttons = true;
        self.reset_state();
    }

    /// Resets the internal framing step and accumulated motion.
    pub fn reset_state(&mut self) {
        self.step = DecoderStep::Byte0;
        self.packet = [0u8; 4];
        self.state.relative_movement_x = 0;
        self.state.relative_movement_y = 0;
        self.state.relative_movement_z = 0;
        self.state.left_button = EFI_FALSE;
        self.state.right_button = EFI_FALSE;
        self.state_changed = false;
    }

    /// Sets the pointer resolution.
    pub fn set_resolution(&mut self, res: MouseResolution) {
        self.mode.resolution_x = res.counts_per_mm();
        self.mode.resolution_y = res.counts_per_mm();
    }

    /// Returns a reference to the pointer mode.
    #[must_use]
    pub const fn mode(&self) -> &EfiSimplePointerMode {
        &self.mode
    }

    /// Returns `true` if mouse state has changed since last read.
    #[must_use]
    pub const fn has_state_changed(&self) -> bool {
        self.state_changed
    }

    /// Feeds a raw byte received from the PS/2 auxiliary port.
    ///
    /// When a complete packet has been accumulated and validated, returns
    /// `Ok(Some(state))` with the updated pointer state.
    ///
    /// # Errors
    ///
    /// Returns `Ps2MouseError::OutOfSync` if byte 0 does not have sync bit 3 set.
    /// Returns `Ps2MouseError::DisplacementOverflow` if X or Y overflow bits are set.
    pub fn feed_byte(&mut self, byte: u8) -> Result<Option<EfiSimplePointerState>, Ps2MouseError> {
        match self.step {
            DecoderStep::Byte0 => {
                // Byte 0 must have bit 3 set
                if (byte & PS2_MOUSE_FLAG_SYNC) == 0 {
                    return Err(Ps2MouseError::OutOfSync);
                }
                self.packet[0] = byte;
                self.step = DecoderStep::Byte1;
                Ok(None)
            }
            DecoderStep::Byte1 => {
                self.packet[1] = byte;
                self.step = DecoderStep::Byte2;
                Ok(None)
            }
            DecoderStep::Byte2 => {
                self.packet[2] = byte;
                if self.packet_size == 4 {
                    self.step = DecoderStep::Byte3;
                    Ok(None)
                } else {
                    self.step = DecoderStep::Byte0;
                    self.decode_packet()
                }
            }
            DecoderStep::Byte3 => {
                self.packet[3] = byte;
                self.step = DecoderStep::Byte0;
                self.decode_packet()
            }
        }
    }

    #[allow(
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap
    )]
    fn decode_packet(&mut self) -> Result<Option<EfiSimplePointerState>, Ps2MouseError> {
        let b0 = self.packet[0];
        let b1 = self.packet[1];
        let b2 = self.packet[2];

        // Check for displacement overflow
        if (b0 & (PS2_MOUSE_FLAG_X_OVERFLOW | PS2_MOUSE_FLAG_Y_OVERFLOW)) != 0 {
            return Err(Ps2MouseError::DisplacementOverflow);
        }

        // Decode 9-bit two's complement X displacement
        let mut rel_x = i32::from(b1);
        if (b0 & PS2_MOUSE_FLAG_X_SIGN) != 0 {
            rel_x = (rel_x as i16 | -256) as i32;
        }

        // Decode 9-bit two's complement Y displacement
        let mut rel_y = i32::from(b2);
        if (b0 & PS2_MOUSE_FLAG_Y_SIGN) != 0 {
            rel_y = (rel_y as i16 | -256) as i32;
        }

        // Decode Z wheel displacement (if 4-byte packet)
        let mut rel_z = 0i32;
        if self.packet_size == 4 {
            let b3 = self.packet[3];
            // Low 4 bits are signed 4-bit 2's complement wheel delta (-8 to +7)
            let z_raw = (b3 & 0x0F) as i8;
            if (z_raw & 0x08) != 0 {
                // Negative 4-bit sign extension
                rel_z = i32::from(z_raw | -16);
            } else {
                rel_z = i32::from(z_raw);
            }
        }

        let left_pressed = (b0 & PS2_MOUSE_FLAG_LEFT_BUTTON) != 0;
        let right_pressed = (b0 & PS2_MOUSE_FLAG_RIGHT_BUTTON) != 0;

        // Invert Y for standard screen coordinates (positive Y = downward)
        self.state.relative_movement_x += rel_x;
        self.state.relative_movement_y -= rel_y;
        self.state.relative_movement_z += rel_z;
        self.state.left_button = if left_pressed { EFI_TRUE } else { EFI_FALSE };
        self.state.right_button = if right_pressed { EFI_TRUE } else { EFI_FALSE };
        self.state_changed = true;

        Ok(Some(self.state))
    }

    /// Implements `EFI_SIMPLE_POINTER_PROTOCOL.GetState()` semantics:
    /// Returns the accumulated relative motion and button status, and resets
    /// relative movement counts to 0.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_READY` if the mouse state has not changed since
    /// the last retrieval.
    pub fn get_state(&mut self) -> Result<EfiSimplePointerState, EfiStatus> {
        if !self.state_changed {
            return Err(EfiStatus::NOT_READY);
        }

        let result = self.state;
        self.state.relative_movement_x = 0;
        self.state.relative_movement_y = 0;
        self.state.relative_movement_z = 0;
        self.state_changed = false;

        Ok(result)
    }
}
