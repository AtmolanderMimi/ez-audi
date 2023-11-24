use crate::cpal_abstraction::{Device, Stream};
use crate::errors::PlayError;

pub trait AudioFileTrait {
    // Starts playing the audio from a certain duration
    fn play(&self, device: Device) -> Result<Stream, PlayError>;

    fn play_on_default_output(&self) -> Result<Stream, PlayError> {
        let device = match Device::default_output() {
            Some(d) => d,
            None => {
                return Err(PlayError::DeviceDoesNotExist {
                    name: "default".to_string(),
                })
            }
        };

        self.play(device)
    }
}
