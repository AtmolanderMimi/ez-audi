use std::{mem, io};

use cpal::Sample;

pub struct Samples<T>
where T: Sample {
    samples: Vec<T>,
    metadata: SampleMetadata,
}

impl<T: Sample> Samples<T> {
    pub fn new(samples: Vec<T>, metadata: SampleMetadata) -> Samples<T> {
        Samples {
            samples,
            metadata,
        }
    }

    pub fn samples(mut self) -> Vec<T> {
        mem::take(&mut self.samples)
    }

    pub fn metadata(&self) -> &SampleMetadata {
        &self.metadata
    }
}

/// Metadata about audio samples
pub struct SampleMetadata {
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    pub channels: u16,
    /// The number of samples per a mount of time
    pub sample_rate: u32,
}

impl SampleMetadata {
    pub fn new(channels: u16, sample_rate: u32) -> SampleMetadata {
        SampleMetadata { 
            channels,
            sample_rate,
        }
    }
}

pub trait GetSamples<T> {
    type SampleType: Sample;

    fn get_samples(&self) -> Result<Samples<Self::SampleType>, io::Error>;
}