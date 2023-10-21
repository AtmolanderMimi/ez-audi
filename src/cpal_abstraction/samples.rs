use std::{mem, io};

use cpal::{Sample, FromSample};

pub struct Samples<T: Sample> {
    samples: Vec<T>,
    metadata: SampleMetadata,
}

impl<T: Sample> Samples<T>
where i16: FromSample<T> {
    pub fn new(samples: Vec<T>, metadata: SampleMetadata) -> Samples<T> {
        Samples {
            samples,
            metadata,
        }
    }

    /// A destrutive way to take samples, can only be used once then only returns None
    pub fn samples(&mut self) -> Option<Vec<T>> {
        let array = mem::take(&mut self.samples);
        if array.len() == 0 {
            None
        } else {
            Some(array)
        }
    }

    pub fn metadata(&self) -> &SampleMetadata {
        &self.metadata
    }
}
pub trait SamplesTrait {
    // Transforms the samples into i16 samples
    fn get_samples(&mut self) -> Option<Vec<i16>>;
}

impl<T: Sample> SamplesTrait for Samples<T>
where i16: FromSample<T> {
    fn get_samples(&mut self) -> Option<Vec<i16>> {
        let samples = self.samples()?;

        let samples = samples.into_iter()
            .map(|s| s.to_sample())
            .collect::<Vec<_>>();

        Some(samples)
    }
}

/// Metadata about audio samples
pub struct SampleMetadata {
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    pub channels: u16,
    /// The number of samples per a mount of time
    pub sample_rate: u32,
    pub sample_type: SampleType,
}

impl SampleMetadata {
    pub fn new(channels: u16, sample_rate: u32, sample_type: SampleType) -> SampleMetadata {
        SampleMetadata { 
            channels,
            sample_rate,
            sample_type,
        }
    }
}

pub enum SampleType {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F16,
    F32,
}