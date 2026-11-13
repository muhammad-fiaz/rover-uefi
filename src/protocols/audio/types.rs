//! # Audio Types and Enumerations
//!
//! Sampling rates, bit depths, channel configurations, and endpoint ports
//! for High Definition Audio (HDA) and USB Audio controllers.

use crate::base::EfiStatus;

/// Standard PCM audio sampling rates in Hz.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSampleRate {
    /// 8.0 kHz.
    Rate8kHz = 8_000,
    /// 11.025 kHz.
    Rate11kHz = 11_025,
    /// 16.0 kHz.
    Rate16kHz = 16_000,
    /// 22.05 kHz.
    Rate22kHz = 22_050,
    /// 32.0 kHz.
    Rate32kHz = 32_000,
    /// 44.1 kHz (CD Audio).
    Rate44kHz = 44_100,
    /// 48.0 kHz (DVD / Standard Broadcast).
    Rate48kHz = 48_000,
    /// 88.2 kHz.
    Rate88kHz = 88_200,
    /// 96.0 kHz (High Resolution Studio).
    Rate96kHz = 96_000,
    /// 192.0 kHz (Audiophile High Definition).
    Rate192kHz = 192_000,
}

/// Bit resolution per audio sample.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioBitsPerSample {
    /// 8-bit unsigned PCM.
    Bits8 = 8,
    /// 16-bit signed PCM.
    Bits16 = 16,
    /// 20-bit signed PCM.
    Bits20 = 20,
    /// 24-bit signed PCM.
    Bits24 = 24,
    /// 32-bit signed PCM or IEEE float.
    Bits32 = 32,
}

/// Channel layout and speaker arrangement.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioChannels {
    /// 1 channel (Monaural).
    Mono = 1,
    /// 2 channels (Stereo L/R).
    Stereo = 2,
    /// 6 channels (5.1 Surround Sound).
    Surround51 = 6,
    /// 8 channels (7.1 Surround Sound).
    Surround71 = 8,
}

/// Hardware endpoint port connector.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioPortType {
    /// Analog Line Out (green 3.5mm jack).
    LineOut = 0,
    /// Internal Chassis Speaker.
    Speaker = 1,
    /// Headphone jack.
    Headphones = 2,
    /// Digital Optical/Coaxial SPDIF.
    Spdif = 3,
    /// Microphone input.
    Microphone = 4,
    /// Analog Line In (blue 3.5mm jack).
    LineIn = 5,
}

/// Asynchronous audio playback/capture completion callback.
pub type EfiAudioIoCallback = unsafe extern "efiapi" fn(context: *mut ()) -> EfiStatus;
