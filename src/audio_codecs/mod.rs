mod l_pcm;
pub use l_pcm::*;

use crate::errors::{PlayError, Error};

use crate::traits::AudioMetadataTrait;

/// The trait implemented on all audio decoders, allows to decode bytes into LPcm samples
pub trait AudioCodecTrait {
    /// Transforms bytes into u8 samples
    fn bytes_to_u8_samples(&self, _bytes: &Vec<u8>, _metadata: &dyn AudioMetadataTrait) -> Error<Vec<u8>> {
        Err(PlayError::Unsupported("Bytes to u8 samples is not supported for the audio codec".to_string()))
    }

    /// Transforms bytes into i16 samples
    fn bytes_to_i16_samples(&self, _bytes: &Vec<u8>, _metadata: &dyn AudioMetadataTrait) -> Error<Vec<i16>> {
        Err(PlayError::Unsupported("Bytes to i16 samples is not supported for the audio codec".to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Enumeration of all the audio codecs, allows static dispatch on decoding rather than using
/// a trait object
pub enum AudioCodec {
    LPcm,
}

impl AudioCodecTrait for AudioCodec {
    fn bytes_to_u8_samples(&self, bytes: &Vec<u8>, metadata: &dyn AudioMetadataTrait) -> Error<Vec<u8>> {
        match self {
            AudioCodec::LPcm => LPcm.bytes_to_u8_samples(bytes, metadata)
        }
    }

    fn bytes_to_i16_samples(&self, bytes: &Vec<u8>, metadata: &dyn AudioMetadataTrait) -> Error<Vec<i16>> {
        match self {
            AudioCodec::LPcm => LPcm.bytes_to_i16_samples(bytes, metadata)
        }
    }
}