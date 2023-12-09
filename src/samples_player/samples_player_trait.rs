use crate::{Device, traits::AudioMetadataTrait, modifiers::ModifierTrait, errors::Error, PlayError};



/// Trait that implements the functionality of the SamplesPlayer struct
pub trait SamplesPlayerTrait {
    /// Returns the metadata of the samples
    fn metadata(&self) -> Box<dyn AudioMetadataTrait>;

    /// Adds a modifier
    fn add_modifier(&mut self, modifier: Box<dyn ModifierTrait>);

    /// Clears all modifiers and their effects
    fn clear_modifiers(&mut self);

    /// Starts/Continues the playing
    fn start(&self) -> Error<()>;

    /// Stops the playing
    fn stop(&self) -> Error<()>;

    /// Starts playing on a device
    fn play_on_device(&mut self, _device: Device) -> Error<()>;

    /// Starts playing on the default device of the default host
    fn play_on_default(&mut self) -> Error<()> {
        let default_output = match Device::default_output() {
            Some(o) => o,
            None => return Err(PlayError::DeviceDoesNotExist { name : "default".to_string() }),
        };

        self.play_on_device(default_output)
    }
}