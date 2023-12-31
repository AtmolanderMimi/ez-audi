use crate::Error;
use crate::audio_codecs::AudioCodec;
use crate::cpal_abstraction::{Device, SampleType, SamplesTrait};
use crate::samples_player::SamplesPlayerTrait;
use crate::errors::PlayError;

/// Trait implemented on every AudioFile structs, that handles playback
pub trait AudioFileTrait {
    /// Gets the file's samles
    fn get_samples(&self) -> Error<Box<dyn SamplesTrait>>;

    /// Creates a SamplesPlayer with the samples of the file, can be exact or not (exact tends to be very slower)
    fn make_player(&self, is_exact: bool) -> Error<Box<dyn SamplesPlayerTrait>>;

    /// Starts playing the audio from a certain duration
    fn play(&self, device: Device, is_exact: bool) -> Error<Box<dyn SamplesPlayerTrait>>;

    /// Plays on the default output of the default host
    fn play_on_default_output(&self, is_exact: bool) -> Error<Box<dyn SamplesPlayerTrait>> {
        let device = match Device::default_output() {
            Some(d) => d,
            None => return Err(PlayError::DeviceDoesNotExist { name: "default".to_string() })
        };

        self.play(device, is_exact)
    }

    /// Returns the file's metadata
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
    /// The number of samples prssocessed in one second (Hz)
    fn sample_rate(&self) -> u32;
    /// The underlying type of the samples, may be none if the codec does not use typical types
    fn sample_type(&self) -> Option<SampleType>;
}