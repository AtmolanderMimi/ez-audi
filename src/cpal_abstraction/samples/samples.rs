use cpal::{self, Sample as CpalSampleTrait};

use crate::{traits::AudioMetadataTrait, audio_codecs::AudioCodec};

use super::{SampleType, Sample};

#[derive(Debug, Clone)]
/// A sample container of LPcm samples ready to be send to audio streams
pub struct Samples<T: Sample> {
    pub samples: Vec<T>,
    pub metadata: SamplesMetadata,
}

impl<T: Sample> Samples<T> {
    pub fn new(samples: Vec<T>, metadata: SamplesMetadata) -> Samples<T> {
        Samples {
            samples,
            metadata,
        }
    }
}

/// This is used to be able to store Samples Struct of multiple generic type in Box
pub trait SamplesTrait {
    /// Consumes the samples and plays on the specified device
    fn into_f32_samples(&self) -> Samples<f32>;

    fn metadata(&self) -> Box<dyn AudioMetadataTrait>;
}

impl<T: Sample> SamplesTrait for Samples<T>
where f32: cpal::FromSample<T>  {
    fn into_f32_samples(&self) -> Samples<f32> {
        let f32_samples = self.samples.clone().into_iter().map(|s| s.to_sample()).collect();

        Samples::new(f32_samples, self.metadata.clone())  
    }

    fn metadata(&self) -> Box<dyn AudioMetadataTrait> {
        Box::new(self.metadata.clone())
    }
}

impl Samples<f32> {
    pub fn into_t_samples<T: Sample + cpal::FromSample<f32>>(&self) -> Samples<T> {
        let samples = self.samples.clone().into_iter()
            .map(|s| s.to_sample())
            .collect();

        Samples::new(samples, self.metadata.clone())
    }
}

/// Metadata about audio samples
#[derive(Debug, Clone)]
pub struct SamplesMetadata {
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    pub channels: u16,
    /// The number of samples per a mount of time
    pub sample_rate: u32,
    pub sample_type: SampleType,
}

impl SamplesMetadata {
    pub fn new(channels: u16, sample_rate: u32, sample_type: SampleType) -> SamplesMetadata {
        SamplesMetadata { 
            channels,
            sample_rate,
            sample_type,
        }
    }
}

impl AudioMetadataTrait for SamplesMetadata {
    fn file_path(&self) -> Option<String> {
        None
    }

    // TODO: Is it right to say that the samples are in LPcm since don't have to got through any other codec? 
    fn audio_codec(&self) -> crate::audio_codecs::AudioCodec {
        AudioCodec::LPcm
    }

    fn channels(&self) -> u32 {
        self.channels as u32
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn sample_type(&self) -> Option<SampleType> {
        Some(self.sample_type.clone())
    }
}

impl From<&SamplesMetadata> for cpal::SupportedStreamConfig {
    fn from(value: &SamplesMetadata) -> Self {
        let channels = value.channels;
        let sample_rate = cpal::SampleRate(value.sample_rate);
        let sample_type = value.sample_type.clone().into();

        cpal::SupportedStreamConfig::new(channels, sample_rate, cpal::SupportedBufferSize::Unknown, sample_type)
    }
}