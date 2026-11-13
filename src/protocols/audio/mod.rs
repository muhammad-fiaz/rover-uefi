//! # Audio
//!
//! Audio playback, sound synthesis, Intel HD Audio and USB Audio interfaces.

pub mod guid;
pub mod protocol;
pub mod types;

pub use guid::EFI_AUDIO_IO_PROTOCOL_GUID;
pub use protocol::{
    EfiAudioIoGetOutputs, EfiAudioIoProtocol, EfiAudioIoSetupPlayback, EfiAudioIoStartPlayback,
    EfiAudioIoStartPlaybackAsync, EfiAudioIoStopPlayback,
};
pub use types::{
    AudioBitsPerSample, AudioChannels, AudioPortType, AudioSampleRate, EfiAudioIoCallback,
};
