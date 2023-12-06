//!# Ez-Audi:
//!**A lightweight (to be cross-platform/wasm-compilable(?)) audio library based on cpal**
//!
//!## Features (as of now):
//!* Read and play LPcm WAVE (.wav) files
//!* Apply modifiers to the samples for Volume, Looping, etc..
//!* Control over the raw audio samples
//!* Get audio file metadata
//!
//!## Supports (as of now):
//!* Linux
//!* LPcm WAVE files
//!
//!## Get started
//!```no_run
//!use ez_audi::audio_types::WavAudio;
//!use ez_audi::public_traits::*;
//!
//!let wav_audio = WavAudio::new("yourfilehere.wav").unwrap();
//!
//!// Creates an audio player, keep it in scope to keep the audio playing
//!let player = wav_audio.play_on_default_output().unwrap();
//!
//!std::thread::sleep(std::time::Duration::from_secs(2));
//!```
//!
//!## What is left to do:
//!* Pretty much everything, don't use this in any serious projects unless you enjoy relying on unstable, buggy code
//!* The documentation is also very much lacking
//!* The projet is **very much still in development** if you are not intersted now, please come back later

mod audio_codecs;
mod cpal_abstraction;
mod wav;
mod errors;
mod traits;

use errors::Error;

pub use errors::PlayError;
pub use cpal_abstraction::{Device, Stream, SamplesPlayer};

pub use cpal_abstraction::samples::modifiers;

pub mod audio_types {
    //! Functions and structs for dealing with audio files and their audio_codecs

    use crate::wav;
    pub use wav::WavAudio;
    pub use wav::file_is_wav;
    use crate::audio_codecs;
    pub use audio_codecs::AudioCodec;
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
    pub use cpal_abstraction::{SamplesPlayerTrait, ModifierTrait, SamplesTrait};
}