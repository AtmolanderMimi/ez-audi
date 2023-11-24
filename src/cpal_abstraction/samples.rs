use cpal;

use super::{Device, Stream};

pub trait Sample: cpal::SizedSample + std::marker::Send + 'static {}

impl<T: cpal::SizedSample + std::marker::Send + 'static> Sample for T {}

#[derive(Debug, Clone)]
pub struct Samples<T: Sample> {
    samples: Vec<T>,
    metadata: SampleMetadata,
}

impl<T: Sample> Samples<T> {
    pub fn new(samples: Vec<T>, metadata: SampleMetadata) -> Samples<T> {
        Samples { samples, metadata }
    }

    /// Gets the samples, but then destroys the struct
    #[doc(hidden)]
    pub fn samples(self) -> Vec<T> {
        self.samples
    }

    pub fn metadata(&self) -> &SampleMetadata {
        &self.metadata
    }
}
pub trait SamplesTrait {
    /// Consumes the samples and plays on the specified device
    fn play_on_device(&self, device: Device) -> Stream;

    fn metadata(&self) -> SampleMetadata;
}

impl<T: Sample> SamplesTrait for Samples<T> {
    fn play_on_device(&self, device: Device) -> Stream {
        // TODO: Cloneing here is not a good idea
        device.play(self.clone())
    }

    fn metadata(&self) -> SampleMetadata {
        self.metadata.clone()
    }
}

/// Metadata about audio samples
#[derive(Debug, Clone)]
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

impl From<&SampleMetadata> for cpal::SupportedStreamConfig {
    fn from(value: &SampleMetadata) -> Self {
        let channels = value.channels;
        let sample_rate = cpal::SampleRate(value.sample_rate);
        let sample_type = value.sample_type.clone().into();

        cpal::SupportedStreamConfig::new(
            channels,
            sample_rate,
            cpal::SupportedBufferSize::Unknown,
            sample_type,
        )
    }
}

#[derive(Debug, Clone)]
pub enum SampleType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl From<SampleType> for cpal::SampleFormat {
    fn from(value: SampleType) -> Self {
        match value {
            SampleType::U8 => cpal::SampleFormat::U8,
            SampleType::U16 => cpal::SampleFormat::U16,
            SampleType::U32 => cpal::SampleFormat::U32,
            SampleType::U64 => cpal::SampleFormat::U64,

            SampleType::I8 => cpal::SampleFormat::I8,
            SampleType::I16 => cpal::SampleFormat::I16,
            SampleType::I32 => cpal::SampleFormat::I32,
            SampleType::I64 => cpal::SampleFormat::I64,

            SampleType::F32 => cpal::SampleFormat::F32,
            SampleType::F64 => cpal::SampleFormat::F64,
        }
    }
}
