use std::cell::{RefCell, RefMut};
use std::io::{BufReader, Read, BufRead, Seek};
use std::fs::File;
use std::ops::Deref;

use crate::errors::PlayError;
use crate::traits::{AudioFileTrait, AudioMetadataTrait};
use crate::cpal_abstraction::SamplesMetadata;
use crate::samples_player::{SamplesPlayerTrait, SamplesPlayer, ExactSamplesPlayer};
use crate::cpal_abstraction::{Samples, SampleType};
use crate::wav::utils;
use crate::audio_codecs::{AudioCodec, AudioCodecTrait};
use crate::errors::Error;

const FMT_ID_END_BYTE: u8 = b' ';
/// The "fmt " block size if the format is LPCM, at least all we need of it
/// (does not include the "fmt " and "data" indentificator)
const FMT_BLOCK_SIZE: usize = 20;

/// Reads until and passes the "fmt " id
fn read_until_fmt_block_and_pass<T: BufRead + Seek>(mut reader: T) -> Result<(), PlayError> {
    reader.read_until(FMT_ID_END_BYTE, &mut Vec::new())?;

    Ok(())
}

/// Reads until and passes the "fmt " id
fn read_all_of_header<T: BufRead + Seek>(mut reader: T) -> Result<(), PlayError> {
    read_until_fmt_block_and_pass(&mut reader)?;

    // Reads until the start of the data indicator then passes it
    reader.read_until(b'd', &mut Vec::new())?;
    //reader.read_until(b'a', &mut Vec::new())?;
    reader.read_exact(&mut [0; 3])?;

    Ok(())
}

#[derive(Debug, Clone)]
/// Info contained in the WAVE file header
pub struct WavAudioMetadata {
    /// Where the file is
    file_path: Option<String>,
    /// The codec in which the data is read
    audio_codec: AudioCodec,
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
    pub fn build_from_reader(f: impl ReadSeek) -> Error<WavAudioMetadata> {
        let mut reader = BufReader::new(f);
        read_until_fmt_block_and_pass(&mut reader)?;

        let mut fmt_block = [0u8; FMT_BLOCK_SIZE];
        reader.read_exact(&mut fmt_block)?;

        let audio_codec_value = u16::from_le_bytes(fmt_block[4..6].try_into().unwrap());
        let audio_codec = match audio_codec_value {
            1 => AudioCodec::LPcm,
            v => return Err(PlayError::Unsupported(format!("audio format other than LPCM. Audio codec value of: {:?}", v))),
        };

        let channels = u16::from_le_bytes(fmt_block[6..8].try_into().unwrap());

        let sample_rate = u32::from_le_bytes(fmt_block[8..12].try_into().unwrap());

        let bits_per_sample = u16::from_le_bytes(fmt_block[18..20].try_into().unwrap());

        let metadata = WavAudioMetadata {
            file_path: None,
            audio_codec,
            sample_rate,
            channels,
            bits_per_sample,
        };

        Ok(metadata)
    }

    /// Gets the metadata from the file's header. Assumes that the file is a WAVE file
    pub fn build_from_path(path: &str) -> Error<WavAudioMetadata> {
        let f = File::open(path)?;
        
        let mut wav_audio = Self::build_from_reader(&f)?;
        wav_audio.set_file_path(path.to_string());
        Ok(wav_audio)
    }

    pub(self) fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path)
    }

    /// Returns the file path
    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    /// Returns the audio format
    pub fn audio_codec(&self) -> AudioCodec {
        self.audio_codec.clone()
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

    /// Returns the sample type based on the bits per sample
    pub fn sample_type(&self) -> SampleType {
        match self.bits_per_sample {
            8 => SampleType::U8,
            16 => SampleType::I16,
            n => todo!("Unsupported {:?} bits", n)
        }
    }
}

impl AudioMetadataTrait for WavAudioMetadata {
    fn file_path(&self) -> Option<String> {
        self.file_path()
    }

    fn audio_codec(&self) -> AudioCodec {
        self.audio_codec()
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

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

#[derive(Debug)]
#[non_exhaustive]
/// A link to a WAVE file
pub struct WavAudio<T: ReadSeek> {
    // The path to the .wav file
    data: RefCell<BufReader<T>>, // TODO: RefCell is not safe in parrallel enviroments
    metadata: WavAudioMetadata,
}

impl<T: ReadSeek> WavAudio<T> {
    /// Creates a new WavAudio and checks if the file is a valid WAVE file
    pub fn build_from_reader(data: T) -> Error<WavAudio<T>> {
        //FIXME: change utils to file
        //if !utils::file_is_wav(path)? {
        //    return Err(PlayError::WrongFileType);
        //}
        let mut data = BufReader::new(data);

        let metadata = WavAudioMetadata::build_from_reader(&mut data)?;
        data.rewind()?;
        
        let audio = WavAudio {
            data: RefCell::new(data),
            metadata,
        };

        Ok(audio)
    }

    /// Gets the the reader, with no issues at all
    fn get_file_buf_reader(&self) -> Error<RefMut<BufReader<T>>> {
        let mut reader = self.data.borrow_mut();
        reader.rewind()?;
        Ok(reader)
    } 

    /// Gets the samples byte by byte, used to pass into codecs
    pub fn get_samples_bytes(&self) -> Error<Vec<u8>> {
        let mut reader = self.get_file_buf_reader()?;

        // Removed the header
        read_all_of_header(&mut *reader)?;

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        return Ok(bytes)
    }

    /// Gets the sample bytes and puts it through the u8 version of the decoder
    fn get_samples_u8(&self) -> Error<Vec<u8>> {
        let samples_bytes = self.get_samples_bytes()?;

        self.audio_codec.bytes_to_u8_samples(&samples_bytes, &self.metadata)
    }

    /// Gets the sample bytess and puts it through the i16 version of the decoder
    fn get_samples_i16(&self) -> Error<Vec<i16>> {
        let samples_bytes = self.get_samples_bytes()?;

        self.audio_codec.bytes_to_i16_samples(&samples_bytes, &self.metadata)
    }
}

impl WavAudio<File> {
    /// Creates a new WavAudio and checks if the file is a valid WAVE file
    pub fn build_from_path(path: &str) -> Error<WavAudio<File>> {
        let file = File::open(path)?;

        let audio: WavAudio<File> = WavAudio::<File>::build_from_reader(file)?;

        Ok(audio)
    }
}

impl<T: ReadSeek> AudioFileTrait for WavAudio<T> {
    fn get_samples(&self) -> Error<Box<dyn crate::cpal_abstraction::SamplesTrait>> {
        match self.metadata.sample_type() {
            SampleType::U8 => {
                let samples = self.get_samples_u8()?;

                let samples_struct = Samples::new(samples, self.metadata.clone().into());
                return Ok(Box::new(samples_struct));
            },
            SampleType::I16 => {
                let samples = self.get_samples_i16()?;

                let samples_struct = Samples::new(samples, self.metadata.clone().into());
                return Ok(Box::new(samples_struct));
            },
            _ => return Err(PlayError::Unsupported(format!("unsupported sample type {:?} for WAVE", self.sample_type())))
        }
    }

    fn make_player(&self, is_exact: bool) -> Error<Box<dyn SamplesPlayerTrait>> {
        match self.metadata.sample_type() {
            SampleType::U8 => {
                let samples = self.get_samples_u8()?;

                let samples_struct = Samples::new(samples, self.metadata.clone().into());

                let samples_player = match is_exact {
                    true => Box::new(SamplesPlayer::new(samples_struct)) as Box<dyn SamplesPlayerTrait>,
                    false => Box::new(ExactSamplesPlayer::new(samples_struct)),
                };

                return Ok(samples_player);
            },
            SampleType::I16 => {
                let samples = self.get_samples_i16()?;
                
                let samples_struct = Samples::new(samples, self.metadata.clone().into());

                let samples_player = match is_exact {
                    true => Box::new(ExactSamplesPlayer::new(samples_struct)) as Box<dyn SamplesPlayerTrait>,
                    false => Box::new(SamplesPlayer::new(samples_struct)),
                };

                return Ok(samples_player);
            },
            _ => return Err(PlayError::Unsupported(format!("unsupported sample type {:?} for WAVE", self.sample_type())))
        }
    }

    fn play(&self, device: crate::cpal_abstraction::Device, is_exact: bool) -> Error<Box<dyn SamplesPlayerTrait>> {
        let mut player = self.make_player(is_exact)?;
        player.play_on_device(device)?;

        Ok(player)
    }

    fn metadata(&self) -> Box<dyn AudioMetadataTrait> {
        Box::new(self.metadata.clone())
    }
}

impl<T: ReadSeek> Deref for WavAudio<T> {
    type Target = WavAudioMetadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

impl From<WavAudioMetadata> for SamplesMetadata {
    fn from(value: WavAudioMetadata) -> Self {
        let sample_type = value.sample_type();

        SamplesMetadata::new(value.channels, value.sample_rate, sample_type)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn metadata_is_valid() {
        let meta = WavAudioMetadata::build_from_path("test_assets/helium.wav").unwrap();

        assert_eq!(meta.audio_codec, AudioCodec::LPcm);

        assert_eq!(meta.channels, 2);

        assert_eq!(meta.sample_rate, 48000);

        assert_eq!(meta.bits_per_sample, 16);

        assert_eq!(meta.byte_rate(), 192000);

        assert_eq!(meta.block_align(), 4)
    }

    fn get_bytes(mut file: File) -> Vec<u8> {
        let metadata = file.metadata().expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        buffer
    } 

    #[test]
    fn generics_dont_implode() {
        let file = File::open("test_assets/helium.wav").unwrap();
        let bytes = get_bytes(file);

        // Thank the lord (I am not religious, but there is no way this is not a miracle)
        let _: WavAudio<Cursor<Vec<u8>>> = WavAudio::build_from_reader(Cursor::new(bytes)).unwrap();
    }
}