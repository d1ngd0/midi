use core::fmt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    InvalidSubCommand(String),
    MidiInitError(midir::InitError),
    MidiPortInfoError(midir::PortInfoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSubCommand(msg) => write!(f, "{}", msg),
            Self::MidiInitError(err) => write!(f, "{}", err),
            Self::MidiPortInfoError(err) => write!(f, "{}", err),
        }
    }
}

impl From<midir::InitError> for Error {
    fn from(value: midir::InitError) -> Self {
        Error::MidiInitError(value)
    }
}

impl From<midir::PortInfoError> for Error {
    fn from(value: midir::PortInfoError) -> Self {
        Error::MidiPortInfoError(value)
    }
}
