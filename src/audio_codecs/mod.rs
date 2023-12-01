mod l_pcm;
pub use l_pcm::*;

use crate::errors::{PlayError, Error};

use crate::cpal_abstraction::{SamplesMetadata, Samples};

/// The trait implemented on all audio decoders
pub trait AudioCodecTrait {
    fn bytes_to_u8_samples(&self, _bytes: &Vec<u8>, _metadata: impl Into<SamplesMetadata>) -> Error<Samples<u8>> {
        Err(PlayError::Unsupported("Bytes to u8 samples is not supported for the audio codec".to_string()))
    }

    fn bytes_to_i16_samples(&self, _bytes: &Vec<u8>, _metadata: impl Into<SamplesMetadata>) -> Error<Samples<i16>> {
        Err(PlayError::Unsupported("Bytes to i16 samples is not supported for the audio codec".to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioCodec {
    LPcm,
}

impl AudioCodecTrait for AudioCodec {
    fn bytes_to_u8_samples(&self, bytes: &Vec<u8>, metadata: impl Into<SamplesMetadata>) -> Error<Samples<u8>> {
        match self {
            AudioCodec::LPcm => LPcm.bytes_to_u8_samples(bytes, metadata)
        }
    }

    fn bytes_to_i16_samples(&self, bytes: &Vec<u8>, metadata: impl Into<SamplesMetadata>) -> Error<Samples<i16>> {
        match self {
            AudioCodec::LPcm => LPcm.bytes_to_i16_samples(bytes, metadata)
        }
    }
}