mod audio_codecs;
mod cpal_abstraction;
mod wav;
mod errors;
mod traits;

pub use errors::Error;
pub use cpal_abstraction::{Device, Stream};

pub mod audio_types {
    use crate::wav;
    pub use wav::WavAudio;
    pub use wav::file_is_wav;
    use crate::audio_codecs;
    pub use audio_codecs::AudioCodec;
}

pub mod samples {
    use crate::cpal_abstraction;
    pub use cpal_abstraction::{Sample, SampleType};
}

pub mod public_traits {
    use crate::traits;
    pub use traits::AudioFileTrait;
    pub use crate::audio_codecs::AudioCodecTrait;
}