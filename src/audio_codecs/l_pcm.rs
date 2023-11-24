use crate::{wav::WavAudioMetadata, errors::Error, cpal_abstraction::{Samples, SampleMetadata}, SampleType};

use super::BytesToSamples;

impl BytesToSamples for LPcmMetadata {
    fn bytes_to_u8_samples(bytes: &Vec<u8>, metadata: impl Into<SampleMetadata>) -> Error<Samples<u8>> {
        Ok(Samples::new(bytes.clone(), metadata.into())) 
    }

    fn bytes_to_i16_samples(bytes: &Vec<u8>, metadata: impl Into<SampleMetadata>) -> Error<Samples<i16>> {
        let mut samples_array = Vec::new();
        for i in 0..((bytes.len() / 2)) {
            let sample = i16::from_le_bytes([bytes[i*2], bytes[(i*2)+1]]);
            samples_array.push(sample);
        }

        Ok(Samples::new(samples_array, metadata.into()))
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Metadata about a collection of LPcm Samples and how to read them
pub struct LPcmMetadata {
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    pub channels: u16,
    /// The number of samples per a mount of time
    pub sample_rate: u32,
    // Byte rate = sample_rate * channels * bits_per_sample/8
    // Block align = channels * bits_per_sample/8
    /// The number of bits in a sample. BITS NOT BYTES.
    /// 8-bit sample are usigned values, whereas 16-bit are signed values
    pub bits_per_sample: u16,
}

impl LPcmMetadata {
    pub fn new(channels: u16, sample_rate: u32, bits_per_sample: u16) -> LPcmMetadata {
        LPcmMetadata {
            channels,
            sample_rate,
            bits_per_sample,
        }
    }

    /// Calculates the byte rate
    pub fn byte_rate(&self) -> u32 {
        (self.sample_rate * self.channels as u32 * self.bits_per_sample as u32) / 8
    }

    /// Calculates the block alignment
    pub fn block_align(&self) -> u16 {
        (self.channels * self.bits_per_sample) / 8
    }

    pub fn sample_type(&self) -> Option<SampleType> {
        match self.bits_per_sample {
            8 => Some(SampleType::U8),
            16 => Some(SampleType::I16),
            other => todo!("Sample bytes was not recognised in LPCM: {}", other), // TODO: Add more
        }
    }
}

impl From<WavAudioMetadata> for LPcmMetadata {
    fn from(value: WavAudioMetadata) -> Self {
        LPcmMetadata::new(value.channels(), value.sample_rate(), value.bits_per_sample())
    }
}

impl From<LPcmMetadata> for SampleMetadata {
    fn from(value: LPcmMetadata) -> Self {
        SampleMetadata::new(value.channels, value.sample_rate, value.sample_type().unwrap()) // TODO: Unwrap here
    }
}