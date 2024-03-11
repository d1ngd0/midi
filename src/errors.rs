use core::fmt;

use midir::{MidiInput, MidiOutput};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    InvalidSubCommand(String),
    InvalidInput(String),
    MidiInitError(midir::InitError),
    MidiPortInfoError(midir::PortInfoError),
    MidiConnectionErrorOutput(midir::ConnectError<midir::MidiOutput>),
    MidiConnectionErrorInput(midir::ConnectError<midir::MidiInput>),
    MidiSendError(midir::SendError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSubCommand(msg) => write!(f, "{}", msg),
            Self::InvalidInput(msg) => write!(f, "{}", msg),
            Self::MidiInitError(err) => write!(f, "{}", err),
            Self::MidiPortInfoError(err) => write!(f, "{}", err),
            Self::MidiConnectionErrorOutput(err) => write!(f, "{}", err),
            Self::MidiConnectionErrorInput(err) => write!(f, "{}", err),
            Self::MidiSendError(err) => write!(f, "{}", err),
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

impl From<midir::ConnectError<MidiOutput>> for Error {
    fn from(value: midir::ConnectError<MidiOutput>) -> Self {
        Error::MidiConnectionErrorOutput(value)
    }
}

impl From<midir::ConnectError<MidiInput>> for Error {
    fn from(value: midir::ConnectError<MidiInput>) -> Self {
        Error::MidiConnectionErrorInput(value)
    }
}

impl From<midir::SendError> for Error {
    fn from(value: midir::SendError) -> Self {
        Error::MidiSendError(value)
    }
}
