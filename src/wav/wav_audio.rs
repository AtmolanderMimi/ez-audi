use std::io::{BufReader, Read};
use std::fs::File;
use std::ops::Deref;

use crate::errors::PlayError;
use crate::traits::AudioFileTrait;
use crate::SampleMetadata;
use crate::cpal_abstraction::{Samples, SampleType, SamplesTrait};
use crate::wav::utils;
use crate::wav::AudioFormat;
use crate::errors::Error;

const HEADER_SIZE: usize = 44;

#[derive(Debug, Clone)]
/// Info contained in the WAVE file header
pub struct WavAudioMetadata {
    /// The way the data is read
    audio_format: AudioFormat,
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    channels: u16,
    /// The number of samples per second (most likely)
    sample_rate: u32,
    // Byte rate = sample_rate * channels * bits_per_sample/8
    // Block align = channels * bits_per_sample/8
    /// The number of bits in a sample. BITS NOT BYTES.
    /// 8-bit sample are usigned values, whereas 16-bit are signed values
    bits_per_sample: u16,
}

impl WavAudioMetadata {
    /// Gets the metadata from the file's header. Assumes that the file is a WAVE file
    pub fn new(path: &str) -> Result<WavAudioMetadata, PlayError> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut header = [0u8; HEADER_SIZE];
        reader.read_exact(&mut header)?;

        let audio_format_value = u16::from_le_bytes(header[20..22].try_into().unwrap());
        let audio_format = match audio_format_value {
            1 => AudioFormat::LPcm,
            _ => return Err(PlayError::Unsuported("audio format other than LPCM".to_string())),
        };

        let channels = u16::from_le_bytes(header[22..24].try_into().unwrap());

        let sample_rate = u32::from_le_bytes(header[24..28].try_into().unwrap());

        let bits_per_sample = u16::from_le_bytes(header[34..36].try_into().unwrap());

        let metadata = WavAudioMetadata {
            audio_format,
            sample_rate,
            channels,
            bits_per_sample,
        };

        Ok(metadata)
    }

    /// Returns the audio format
    pub fn audio_format(&self) -> AudioFormat {
        self.audio_format.clone()
    }

    /// Returns the number of channels.
    /// 1 = mono, 2 = stereo, etc...
    pub fn channels(&self) -> u16 {
        self.channels.clone()
    }

    /// Returns the number of samples per second (Hz)
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate.clone()
    }

    /// Returns he number of bits in a sample. BITS NOT BYTES
    pub fn bits_per_sample(&self) -> u16 {
        self.bits_per_sample.clone()
    }

    /// Calculates the byte rate
    pub fn byte_rate(&self) -> u32 {
        (self.sample_rate * self.channels as u32 * self.bits_per_sample as u32) / 8
    }

    /// Calculates the block alignment
    pub fn block_align(&self) -> u16 {
        (self.channels * self.bits_per_sample) / 8
    }

    /// Returns the sample format based on the bits per sample
    pub fn sample_type(&self) -> SampleType {
        match self.bits_per_sample {
            8 => SampleType::U8,
            16 => SampleType::I16,
            _ => todo!("Unsupported")
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
/// A link to a WAVE file
pub struct WavAudio {
    // The path to the .wav file
    file_path: String,
    metadata: WavAudioMetadata,
}

impl WavAudio {
    pub fn new(path: &str) -> Error<WavAudio> {
        if !utils::file_is_wav(path)? {
            return Err(PlayError::WrongFileType);
        }
        
        let metadata = WavAudioMetadata::new(&path)?;
        
        let audio = WavAudio {
            file_path: path.to_string(),
            metadata,
        };

        Ok(audio)
    }

    pub fn get_samples(&self) -> Error<Box<dyn SamplesTrait>> {
        match self.metadata.sample_type() {
            SampleType::U8 => return Ok(Box::new(self.get_samples_u8()?)),
            SampleType::I16 => return Ok(Box::new(self.get_samples_i16()?)),
            _ => todo!("unsupported")
        }
    }

    fn get_samples_bytes(&self) -> Error<Vec<u8>> {
        let f = File::open(&self.file_path)?;
        let mut reader = BufReader::new(f);

        // Removes the header
        let mut header = [0u8; HEADER_SIZE];
        reader.read_exact(&mut header)?;

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        return Ok(bytes)
    }

    fn get_samples_u8(&self) -> Error<Samples<u8>> {
        let samples_bytes = self.get_samples_bytes()?;
        let metadata = self.metadata.clone().into();

        let sample_containter = Samples::new(samples_bytes, metadata);

        Ok(sample_containter)
    }

    fn get_samples_i16(&self) -> Error<Samples<i16>> {
        let bytes = self.get_samples_bytes()?;
        let metadata = self.metadata.clone().into();

        let mut samples_array = Vec::new();
        for i in 0..((bytes.len() / 2)) {
            let sample = i16::from_le_bytes([bytes[i*2], bytes[(i*2)+1]]);
            samples_array.push(sample);
        }

        let sample_containter = Samples::new(samples_array, metadata);

        Ok(sample_containter)
    }
}

impl AudioFileTrait for WavAudio {
    fn play(&self, device: crate::cpal_abstraction::Device) -> Result<crate::cpal_abstraction::Stream, PlayError> {
        let stream = self.get_samples()?.play_on_device(device);

        Ok(stream)
    }
}

impl Deref for WavAudio {
    type Target = WavAudioMetadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

impl From<WavAudioMetadata> for SampleMetadata {
    fn from(value: WavAudioMetadata) -> Self {
        let sample_type = value.sample_type();

        SampleMetadata::new(value.channels, value.sample_rate, sample_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_is_valid() {
        let meta = WavAudioMetadata::new("test_assets/9000.wav").unwrap();

        assert_eq!(meta.audio_format, AudioFormat::LPcm);

        assert_eq!(meta.channels, 1);

        assert_eq!(meta.sample_rate, 22050);

        assert_eq!(meta.bits_per_sample, 16);

        assert_eq!(meta.byte_rate(), 44100);

        assert_eq!(meta.block_align(), 2)
    }
}