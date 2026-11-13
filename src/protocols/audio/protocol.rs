//! # Audio I/O Protocol
//!
//! Provides pre-boot audio playback, sound generation, volume control,
//! and codec configuration across Intel HD Audio and USB Audio hardware.

use super::types::{AudioBitsPerSample, AudioChannels, AudioSampleRate, EfiAudioIoCallback};
use crate::abi::types::UIntN;
use crate::base::EfiStatus;

/// Queries the bitmask of available physical audio output channels.
pub type EfiAudioIoGetOutputs =
    unsafe extern "efiapi" fn(this: *mut EfiAudioIoProtocol, output_mask: *mut u32) -> EfiStatus;

/// Configures playback sample rate, bit resolution, channels, and volume.
pub type EfiAudioIoSetupPlayback = unsafe extern "efiapi" fn(
    this: *mut EfiAudioIoProtocol,
    output_index: u32,
    volume: u8,
    sample_rate: AudioSampleRate,
    bits_per_sample: AudioBitsPerSample,
    channels: AudioChannels,
) -> EfiStatus;

/// Starts synchronous audio playback of a PCM sample buffer.
pub type EfiAudioIoStartPlayback = unsafe extern "efiapi" fn(
    this: *mut EfiAudioIoProtocol,
    data: *const u8,
    data_length: UIntN,
    position: UIntN,
) -> EfiStatus;

/// Starts non-blocking asynchronous audio playback.
pub type EfiAudioIoStartPlaybackAsync = unsafe extern "efiapi" fn(
    this: *mut EfiAudioIoProtocol,
    data: *const u8,
    data_length: UIntN,
    position: UIntN,
    callback: EfiAudioIoCallback,
    context: *mut (),
) -> EfiStatus;

/// Immediately terminates current audio playback and mutes output.
pub type EfiAudioIoStopPlayback =
    unsafe extern "efiapi" fn(this: *mut EfiAudioIoProtocol) -> EfiStatus;

/// EFI Audio I/O Protocol interface (`EFI_AUDIO_IO_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiAudioIoProtocol {
    /// Queries available output ports.
    pub get_outputs: EfiAudioIoGetOutputs,
    /// Configures playback format and volume.
    pub setup_playback: EfiAudioIoSetupPlayback,
    /// Starts synchronous playback.
    pub start_playback: EfiAudioIoStartPlayback,
    /// Starts asynchronous playback.
    pub start_playback_async: EfiAudioIoStartPlaybackAsync,
    /// Stops active playback.
    pub stop_playback: EfiAudioIoStopPlayback,
}
