use crate::{audio_codecs::{LPcmMetadata, BytesToSamples}, errors::Error, cpal_abstraction::{Samples, SampleMetadata}};

#[derive(Debug, Clone, PartialEq)]
pub enum AudioFormat {
    LPcm,
}

impl AudioFormat {
    pub fn bytes_to_u8_samples(&self, bytes: &Vec<u8>, metadata: impl Into<SampleMetadata>) -> Error<Samples<u8>> {
        match self {
            Self::LPcm => LPcmMetadata::bytes_to_u8_samples(bytes, metadata)
        }
    }

    pub fn bytes_to_i16_samples(&self, bytes: &Vec<u8>, metadata: impl Into<SampleMetadata>) -> Error<Samples<i16>> {
        match self {
            Self::LPcm => LPcmMetadata::bytes_to_i16_samples(bytes, metadata)
        }
    }
}

