mod errors;
pub mod wav;
pub use errors::PlayError;
pub mod traits;
// TODO: Rethink Interface design
pub mod cpal_abstraction;
use cpal_abstraction::{SampleMetadata, SampleType};

mod audio_codecs;
