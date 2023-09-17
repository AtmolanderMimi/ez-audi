use crate::errors::PlayError;

use std::time::Duration;

pub trait Audio {
    // Starts playing the audio from a certain duration
    fn play_from(&self, duration: Duration) -> Result<(), PlayError>;

    // Gives the duration of the audio source
    fn duration(&self) -> Duration;
    // Gives the at how many hertz is the audio source
    fn hertz(&self) -> u32;
    // Gives what is the hertz rate of the audio source
    fn bitrate(&self) -> u32;
}