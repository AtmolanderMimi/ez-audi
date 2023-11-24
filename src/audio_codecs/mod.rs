mod l_pcm;
pub use l_pcm::*;

use crate::errors::{PlayError, Error};

use crate::cpal_abstraction::{SampleMetadata, Samples};
pub trait BytesToSamples {
    fn bytes_to_u8_samples(_bytes: &Vec<u8>, _metadata: impl Into<SampleMetadata>) -> Error<Samples<u8>> {
        Err(PlayError::Unsupported("Bytes to u8 samples is not supported for the audio codec".to_string()))
    }

    fn bytes_to_i16_samples(_bytes: &Vec<u8>, _metadata: impl Into<SampleMetadata>) -> Error<Samples<i16>> {
        Err(PlayError::Unsupported("Bytes to u8 samples is not supported for the audio codec".to_string()))
    }
}