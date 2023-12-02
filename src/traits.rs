use crate::audio_codecs::AudioCodec;
use crate::cpal_abstraction::{Device, SampleType, SamplesPlayerTrait};
use crate::errors::PlayError;

pub trait AudioFileTrait {
    // Starts playing the audio from a certain duration
    fn play(&self, device: Device) -> Result<Box<dyn SamplesPlayerTrait>, PlayError>;

    fn play_on_default_output(&self) -> Result<Box<dyn SamplesPlayerTrait>, PlayError> {
        let device = match Device::default_output() {
            Some(d) => d,
            None => return Err(PlayError::DeviceDoesNotExist { name: "default".to_string() })
        };

        self.play(device)
    }

    fn metadata(&self) -> Box<dyn AudioMetadataTrait>;
}

/// Trait implemented on all audio metadata
pub trait AudioMetadataTrait {
    /// The path to the file, may be None if the metadata is not about an audio file
    fn file_path(&self) -> Option<String>;
    /// The codec used to decode the audio
    fn audio_codec(&self) -> AudioCodec;
    /// The number of channels in the audio
    fn channels(&self) -> u32;
    // TODO: Is the fact ↓ true?
    /// The number of samples processed in one second (Hz)
    fn sample_rate(&self) -> u32;
    /// The underlying type of the samples, may be none if the codec does not use typical types
    fn sample_type(&self) -> Option<SampleType>;
}