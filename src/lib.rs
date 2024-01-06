//! # Ez-Audi
//! **A lightweight (to be cross-platform/wasm-compilable(?)) audio playback library based on cpal**
//! 
//! ## Features (as of now):
//! * Read and play LPcm WAVE (.wav) files
//! * Apply modifiers to the samples for Volume, Looping, etc..
//! * Control over the raw audio samples
//! * Get audio file metadata
//! 
//! ## Supports (as of now):
//! * Linux
//! * LPcm WAVE files
//! 
//! ## Get started
//! ```no_run
//! use ez_audi::audio_files::WavAudio;
//! use ez_audi::public_traits::*;
//! 
//! let wav_audio = WavAudio::build_from_path("test_assets/u8-stereo-lpcm.wav").unwrap();
//! 
//! // Creates an audio player, keep it in scope to keep the audio playing
//! let player = wav_audio.play_on_default_output(false).unwrap();
//! 
//! std::thread::sleep(std::time::Duration::from_secs(2));
//! ```
//! 
//! ## What is left to do:
//! * Pretty much everything, don't use this in any serious projects unless you enjoy relying on unstable, buggy code
//! * The documentation is also very much lacking

mod audio_codecs;
mod cpal_abstraction;
mod wav;
mod errors;
mod traits;

use errors::Error;

pub use errors::PlayError;
pub use cpal_abstraction::{Device, Stream};

pub mod samples_player;
pub use samples_player::SamplesPlayer;
pub use samples_player::modifiers;

pub mod audio_files {
    //! Functions and structs for dealing with audio files and their audio_codecs

    use crate::wav;
    pub use wav::WavAudio;
    pub use wav::file_is_wav;
    use crate::audio_codecs;
    pub use audio_codecs::{AudioCodec, AudioCodecTrait};
}

pub mod samples {
    //! Functions and structs for closely working with samples 

    use crate::cpal_abstraction;
    pub use cpal_abstraction::{Sample, IntermediateSampleType, Samples, SampleType};
}

pub mod public_traits {
    //! A collection of most of the traits you need to work with this library,
    //! note that the traits are also present in the modules where they are relevent

    use crate::traits;
    pub use traits::{AudioFileTrait, AudioMetadataTrait};
    pub use crate::audio_codecs::AudioCodecTrait;
    use crate::cpal_abstraction;
    pub use cpal_abstraction::SamplesTrait;
    use crate::samples_player;
    pub use samples_player::SamplesPlayerTrait;
    use crate::modifiers;
    pub use modifiers::ModifierTrait;
}