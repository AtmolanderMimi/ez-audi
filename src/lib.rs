mod audio_codecs;
mod cpal_abstraction;
mod wav;
mod errors;
mod traits;

pub use errors::Error;
pub use cpal_abstraction::{Device, Stream, SamplesPlayer};

pub use cpal_abstraction::samples::modifiers;

pub mod audio_types {
    use crate::wav;
    pub use wav::WavAudio;
    pub use wav::file_is_wav;
    use crate::audio_codecs;
    pub use audio_codecs::AudioCodec;
}

pub mod samples {
    use crate::cpal_abstraction;
    pub use cpal_abstraction::{Sample, IntermediateSampleType, Samples, SampleType};
}

pub mod public_traits {
    use crate::traits;
    pub use traits::{AudioFileTrait, AudioMetadataTrait};
    pub use crate::audio_codecs::AudioCodecTrait;
    use crate::cpal_abstraction;
    pub use cpal_abstraction::{SamplesPlayerTrait, ModifierTrait, SamplesTrait};
}