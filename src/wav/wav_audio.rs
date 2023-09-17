use std::io::{BufReader, Read, self};
use std::fs::File;

use crate::audio_codec::Audio;
use crate::wav::utils;
use crate::wav::AudioFormat;

#[derive(Debug, Clone)]
/// Info contained in the WAVE file header
struct WavAudioMetadata {
    /// The way the data is read
    pub audio_format: AudioFormat,
    /// Numbers of channels: mono = 1, Stereo = 2, etc...
    pub channels: u16,
    /// The number of samples per a mount of time TODO: what amount of time?
    pub sample_rate: u32,
    // Byte rate = sample_rate * channels * bits_per_sample/8
    // Block align = channels * bits_per_sample/8
    /// The number of bits in a sample. BITS NOT BYTES.
    /// 8-bit sample are usigned values, whereas 16-bit are signed values
    pub bits_per_sample: u16,
}

impl WavAudioMetadata {
    /// Gets the metadata from the file's header. Assumes that the file is a WAVE file
    pub fn new(path: &str) -> Result<WavAudioMetadata, io::Error> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut header = [0u8; 44];
        reader.read_exact(&mut header)?;

        let audio_format_value = u16::from_le_bytes(header[20..22].try_into().unwrap());
        let audio_format = match audio_format_value {
            1 => AudioFormat::Pcm,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported or invalid WAVE audio format")),
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

    /// Calculates the byte rate
    pub fn byte_rate(&self) -> u32 {
        (self.sample_rate * self.channels as u32 * self.bits_per_sample as u32) / 8
    }

    /// Calculates the block alignment
    pub fn block_align(&self) -> u16 {
        (self.channels * self.bits_per_sample) / 8
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
    pub fn new(path: &str) -> Result<WavAudio, io::Error> {
        if !utils::file_is_wav(path)? {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Expected a WAVE file"));
        }

        let metadata = WavAudioMetadata::new(&path)?;

        let audio = WavAudio {
            file_path: path.to_string(),
            metadata,
        };

        Ok(audio)
    }
}

// TODO:
//impl AudioCodec for WavAudio {
//    fn play_from(&self, duration: std::time::Duration) -> Result<(), crate::errors::PlayError> {
//        let f = File::open(self.file_path)
//            .expect("File path should be valid since it was verified in the build process");
//        let reader = BufReader::new(f);
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_is_valid() {
        let meta = WavAudioMetadata::new("test_assets/9000.wav").unwrap();

        assert_eq!(meta.audio_format, AudioFormat::Pcm);

        assert_eq!(meta.channels, 1);

        assert_eq!(meta.sample_rate, 22050);

        assert_eq!(meta.bits_per_sample, 16);

        assert_eq!(meta.byte_rate(), 44100);

        assert_eq!(meta.block_align(), 2)
    }
}