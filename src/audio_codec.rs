use crate::cpal_abstraction::{Device, Stream};
use crate::errors::PlayError;

use std::time::Duration;
use std::io;

pub trait PlayableTrait {
    // Starts playing the audio from a certain duration
    fn play(&self, device: Device) -> Result<Stream, io::Error>;

    fn play_on_default_output(&self) -> Result<Stream, io::Error> {
        let device = Device::default_output().unwrap_or(todo!("Better error"));
        self.play(device)
    }
}