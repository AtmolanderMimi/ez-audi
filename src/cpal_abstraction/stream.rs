use cpal;
use cpal::traits::StreamTrait;

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

    /// Continues the audio from where it ended
    pub fn start(&self) {
        self.stream.play();
    }

    pub fn stop(&self) {
        self.stream.pause();
    }
}

impl From<cpal::Stream> for Stream {
    fn from(value: cpal::Stream) -> Self {
        Stream::new(value)
    }
}