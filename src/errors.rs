use std::{io, fmt::Display, error};

use crate::cpal_abstraction::SampleType;

pub type Error<T> = Result<T, PlayError>;

#[derive(Debug)]
pub enum PlayError {
    TimeOutOfBounds,
    FileNotAccessible(io::Error),
    WrongFileType,
    DeviceIoError(String, Option<Box<dyn error::Error + 'static>>),
    DeviceDoesNotSupportAudioSettings(Vec<AudioSettings>, Option<Box<dyn error::Error + 'static>>),
    DeviceDoesNotExist{ name: String },
    Unsupported(String),
}

impl Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeOutOfBounds => f.write_str("the time specified is out of bounds"),
            Self::FileNotAccessible(_) => f.write_str("there was an error while trying to access the file"),
            Self::WrongFileType => f.write_str("file was of the wrong file type"),
            Self::DeviceDoesNotExist{ name: n } => f.write_str(&format!("the device '{n}' does not exist")),
            Self::DeviceIoError(c, _) => f.write_str(&format!("the device had an issue with io because {c}")),
            Self::DeviceDoesNotSupportAudioSettings(s, _) => f.write_str(&format!("the device had an issue with config because {s:?} is/are not supported")),
            Self::Unsupported(e) => f.write_str(&format!("ez_audi does not support '{}'", e)),
        }
    }
}

impl error::Error for PlayError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::TimeOutOfBounds => None,
            Self::FileNotAccessible(e) => Some(e),
            Self::WrongFileType => None,
            Self::DeviceDoesNotExist{ .. } => None,
            Self::DeviceIoError(_, s) => {
                match s {
                    Some(s) => Some(&**s.clone()),
                    None => None,
                }
            },
            Self::DeviceDoesNotSupportAudioSettings(_, s) => {
                match s {
                    Some(s) => Some(&**s.clone()),
                    None => None,
                }
            },
            Self::Unsupported(_) => None,
        }
    }
}

impl From<io::Error> for PlayError {
    fn from(value: io::Error) -> Self {
        Self::FileNotAccessible(value)
    }
}

#[derive(Debug, Clone)]
/// Used in PlayError::DeviceDoesNotSupportAudioSettings
pub enum AudioSettings {
    /// The type of the sample
    SampleType(Option<SampleType>),
    /// The hertz of the consuming of samples
    SampleRate(u32),
    /// The number of channels
    Channels(u32),
    /// If it is caused by the combinaison of more than one setting above,
    /// if two independently are unsupported than it should be reported as multiple
    /// enums
    Combinaison,
}

impl Display for AudioSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SampleType(st) => f.write_str(&format!("the sample type {:?}", st)),
            Self::SampleRate(r) => f.write_str(&format!("the sample rate of {:?}", r)),
            Self::Channels(c) => f.write_str(&format!("the channel count of {:?}", c)),
            Self::Combinaison => f.write_str("the combinaison of settings")
        }
    }
}