use std::io::{BufReader, Read, BufRead};
use std::fs::File;
use std::ops::Deref;

use crate::errors::PlayError;
use crate::traits::{AudioFileTrait, AudioMetadataTrait};
use crate::cpal_abstraction::SampleMetadata;
use crate::cpal_abstraction::{Samples, SampleType, SamplesTrait};
use crate::wav::utils;
use crate::audio_codecs::{AudioCodec, AudioCodecTrait};
use crate::errors::Error;

const FMT_ID_END_BYTE: u8 = 32;
/// The block size (without the "fmt " id)
const FMT_BLOCK_SIZE: usize = 24;

/// Reads until and passes the "fmt " id
fn read_until_fmt_block_and_pass(reader: &mut BufReader<File>) -> Result<(), PlayError> {
    reader.read_until(FMT_ID_END_BYTE, &mut Vec::new())?;

    Ok(())
}

#[derive(Debug, Clone)]
/// Info contained in the WAVE file header
pub struct WavAudioMetadata {
    /// Where the file is
    file_path: String,
    /// The way the data is read
    audio_format: AudioCodec,
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
        read_until_fmt_block_and_pass(&mut reader)?;

        let mut fmt_block = [0u8; FMT_BLOCK_SIZE];
        reader.read_exact(&mut fmt_block)?;

        let audio_format_value = u16::from_le_bytes(fmt_block[4..6].try_into().unwrap());
        let audio_format = match audio_format_value {
            1 => AudioCodec::LPcm,
            _ => return Err(PlayError::Unsupported("audio format other than LPCM".to_string())),
        };

        let channels = u16::from_le_bytes(fmt_block[6..8].try_into().unwrap());

        let sample_rate = u32::from_le_bytes(fmt_block[8..12].try_into().unwrap());

        let bits_per_sample = u16::from_le_bytes(fmt_block[18..20].try_into().unwrap());

        let metadata = WavAudioMetadata {
            file_path: path.to_string(),
            audio_format,
            sample_rate,
            channels,
            bits_per_sample,
        };

        Ok(metadata)
    }

    /// Returns the file path
    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }

    /// Returns the audio format
    pub fn audio_format(&self) -> AudioCodec {
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

impl AudioMetadataTrait for WavAudioMetadata {
    fn file_path(&self) -> Option<String> {
        Some(self.file_path())
    }

    fn audio_codec(&self) -> AudioCodec {
        self.audio_format()
    }

    fn channels(&self) -> u32 {
        self.channels() as u32
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate()
    }

    fn sample_type(&self) -> Option<SampleType> {
        Some(self.sample_type())
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
        read_until_fmt_block_and_pass(&mut reader)?;

        let mut header = [0u8; FMT_BLOCK_SIZE];
        reader.read_exact(&mut header)?;

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        return Ok(bytes)
    }

    fn get_samples_u8(&self) -> Error<Samples<u8>> {
        let samples_bytes = self.get_samples_bytes()?;

        self.audio_format.bytes_to_u8_samples(&samples_bytes, self.metadata.clone())
    }

    fn get_samples_i16(&self) -> Error<Samples<i16>> {
        let samples_bytes = self.get_samples_bytes()?;

        self.audio_format.bytes_to_i16_samples(&samples_bytes, self.metadata.clone())
    }
}

impl AudioFileTrait for WavAudio {
    fn play(&self, device: crate::cpal_abstraction::Device) -> Result<crate::cpal_abstraction::Stream, PlayError> {
        let stream = self.get_samples()?.play_on_device(device);

        Ok(stream)
    }

    fn metadata(&self) -> Box<dyn AudioMetadataTrait> {
        Box::new(self.metadata.clone())
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

        assert_eq!(meta.audio_format, AudioCodec::LPcm);

        assert_eq!(meta.channels, 1);

        assert_eq!(meta.sample_rate, 22050);

        assert_eq!(meta.bits_per_sample, 16);

        assert_eq!(meta.byte_rate(), 44100);

        assert_eq!(meta.block_align(), 2)
    }
}