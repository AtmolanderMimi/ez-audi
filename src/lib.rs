mod audio_codecs;
mod cpal_abstraction;
mod wav;
mod errors;
mod traits;

pub use errors::Error;
pub use cpal_abstraction::{Device, Stream, SampleType};

pub mod audio_types {
    use crate::wav;
    pub use wav::WavAudio;
    pub use wav::file_is_wav;
}

pub mod samples {
    use crate::cpal_abstraction;
    pub use cpal_abstraction::{Sample, SampleType};
}

pub mod public_traits {
    use crate::traits;
    pub use traits::AudioFileTrait;
}