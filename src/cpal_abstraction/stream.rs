use cpal;
use cpal::traits::StreamTrait;

use crate::errors::Error;

/// An audio stream, stops the stream when dropped
pub struct Stream {
    stream: cpal::Stream
}

impl Stream {
    fn new(stream: cpal::Stream) -> Stream {
        Stream {
            stream,
        }
    }

    /// Continues/Starts the audio from where it ended
    pub fn start(&self) -> Error<()> {
        match self.stream.play() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    /// Stops the audio until it is explicitly continued
    pub fn stop(&self) -> Error<()> {
        match self.stream.pause() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}

impl From<cpal::Stream> for Stream {
    fn from(value: cpal::Stream) -> Self {
        Stream::new(value)
    }
}