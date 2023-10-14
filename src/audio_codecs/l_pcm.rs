use crate::{audio_codec, wav::WavAudioMetadata, errors::PlayError};

#[derive(Debug, Clone)]
struct LPcmMetadata {
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
}

impl From<WavAudioMetadata> for LPcmMetadata {
    fn from(value: WavAudioMetadata) -> Self {
        LPcmMetadata::new(value.channels, value.sample_rate, value.bits_per_sample)
    }
}

///// Sends the pcm samples to the system to be played
///// 
///// # Panic
///// The samples are expected to be of the size expressed in the metadata, if not this will **panic**.
//pub fn play_l_pcm_from_until(samples: &[u8], metadata: &LPcmMetadata) -> Result<(), PlayError> {
//    enum Mode {
//        HeightBits,
//        SixteenBits,
//    }
//
//    if metadata.bits_per_sample % 8 != 0 {
//        todo!("Support all sample bit sizes that are not byte aligned")   
//    }
//
//    let mode = match metadata.bits_per_sample {
//        8 => Mode::HeightBits,
//        16 => Mode::SixteenBits,
//        _ => todo!("Support all sample bit sizes")
//    };
//
//    // This part assumes that the samples are byte aligned
//    let byte_per_sample = metadata.bits_per_sample/8;
//    
//    let channels = Vec::with_capacity(metadata.channels as usize);
//    for i in 0..metadata.channels {
//        let amplitude = match mode {
//            Mode::HeightBits => u8::from_le_bytes([samples[i as usize]]) as i32,
//            Mode::SixteenBits => {
//                let offset = (i*2) as usize;
//                let bytes: [u8; 2] = [*samples.get(offset).unwrap(), *samples.get(offset + 1).unwrap()];
//                u16::from_le_bytes(bytes) as i32
//            }
//        };
//
//        
//    }
//}