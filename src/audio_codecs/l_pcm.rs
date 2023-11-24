use crate::{
    cpal_abstraction::{SampleMetadata, Samples},
    errors::Error,
};

use super::BytesToSamples;

impl BytesToSamples for LPcm {
    fn bytes_to_u8_samples(
        bytes: &Vec<u8>,
        metadata: impl Into<SampleMetadata>,
    ) -> Error<Samples<u8>> {
        Ok(Samples::new(bytes.clone(), metadata.into()))
    }

    fn bytes_to_i16_samples(
        bytes: &Vec<u8>,
        metadata: impl Into<SampleMetadata>,
    ) -> Error<Samples<i16>> {
        let mut samples_array = Vec::new();
        for i in 0..(bytes.len() / 2) {
            let sample = i16::from_le_bytes([bytes[i * 2], bytes[(i * 2) + 1]]);
            samples_array.push(sample);
        }

        Ok(Samples::new(samples_array, metadata.into()))
    }
}

pub struct LPcm;
