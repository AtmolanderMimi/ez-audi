use crate::{errors::Error, traits::AudioMetadataTrait};

use super::AudioCodecTrait;

impl AudioCodecTrait for LPcm {
    fn bytes_to_u8_samples(&self, bytes: &Vec<u8>, _metadata: &dyn AudioMetadataTrait) -> Error<Vec<u8>> {
        Ok(bytes.clone())
    }

    fn bytes_to_i16_samples(&self, bytes: &Vec<u8>, _metadata: &dyn AudioMetadataTrait) -> Error<Vec<i16>> {
        let mut samples_array = Vec::new();
        for i in 0..((bytes.len() / 2)) {
            let sample = i16::from_le_bytes([bytes[i*2], bytes[(i*2)+1]]);
            samples_array.push(sample);
        }

        Ok(samples_array)
    }
}

/// Linear Pulse Modulation *Thighy* struct, contains all the methods to interpret bytes formated via LPcm into samples
#[derive(Debug, Clone, Default)]
pub struct LPcm;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{wav::WavAudio, traits::AudioFileTrait};

    fn wav_audio_u8() -> WavAudio<std::fs::File> {
        WavAudio::build_from_path("test_assets/u8-stereo-lpcm.wav").unwrap()
    }
    
    fn wav_audio_i16() -> WavAudio<std::fs::File> {
        WavAudio::build_from_path("test_assets/i16-stereo-lpcm.wav").unwrap()
    }

    #[test]
    fn lpcm_u8_works() {
        let wav = wav_audio_u8();
        let bytes = wav.get_samples_bytes().unwrap();
        for i in 0..100 {
            print!("{:?}, ", bytes[i]);
        }

        let samples = LPcm::default().bytes_to_u8_samples(&bytes, wav.metadata().as_ref()).unwrap();
        assert_eq!(samples[0], 182u8);

        assert_eq!(samples[1], 22);

        assert_eq!(samples[2], 2);

        assert_eq!(samples[64], 129);

        assert_eq!(samples[128], 128);
    }

    #[test]
    fn lpcm_i16_works() {
        let wav = wav_audio_i16();
        let bytes = wav.get_samples_bytes().unwrap();

        let samples = LPcm::default().bytes_to_i16_samples(&bytes, wav.metadata().as_ref()).unwrap();
        assert_eq!(samples[0], 11628i16);

        assert_eq!(samples[1], 4);

        assert_eq!(samples[2], 83);

        assert_eq!(samples[128], -50);
        
        assert_eq!(samples[68444], -26);
    }
}