use std::{io, fmt::Display, error};

pub type Error<T> = Result<T, PlayError>;

#[derive(Debug)]
pub enum PlayError {
    TimeOutOfBounds,
    FileNotAccessible(io::Error),
    WrongFileType,
    DeviceDoesNotSupportAudioSettings,
    DeviceDoesNotExist{ name: String },
    Unsuported(String),
}

impl Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeOutOfBounds => f.write_str("the time specified is out of bounds"),
            Self::FileNotAccessible(_) => f.write_str("there was an error while trying to access the file"),
            Self::WrongFileType => f.write_str("file was of the wrong file type"),
            Self::DeviceDoesNotExist{ name: n } => f.write_str(&format!("the device '{n}' does not exist")),
            Self::DeviceDoesNotSupportAudioSettings => f.write_str("the device does not support the settings of the audio file"),
            Self::Unsuported(e) => f.write_str(&format!("ez_audi does not support '{}'", e)),
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
            Self::DeviceDoesNotSupportAudioSettings => None,
            Self::Unsuported(_) => None,
        }
    }
}

impl From<io::Error> for PlayError {
    fn from(value: io::Error) -> Self {
        Self::FileNotAccessible(value)
    }
}