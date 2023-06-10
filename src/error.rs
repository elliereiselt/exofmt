use crate::mutf8;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ScrollError(scroll::Error),
    MUTF8Error(mutf8::Error),

    InvalidMagicNumber(u64),
    Malformed(String),
    InvalidArguments(String),
    TooManyArrayItems(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IOError(io_error) => Some(io_error),
            Error::ScrollError(scroll_error) => Some(scroll_error),
            Error::MUTF8Error(mutf8_error) => Some(mutf8_error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(io_error) => write!(f, "{}", io_error),
            Error::ScrollError(scroll_error) => write!(f, "{}", scroll_error),
            Error::MUTF8Error(mutf8_error) => write!(f, "{}", mutf8_error),
            Error::InvalidMagicNumber(value) => write!(f, "Invalid magic number: 0x{:x}", *value),
            Error::Malformed(message) => write!(f, "Malformed: {}", message),
            Error::InvalidArguments(message) => write!(f, "Invalid arguments: {}", message),
            Error::TooManyArrayItems(message) => write!(f, "Too many items: {}", message),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<scroll::Error> for Error {
    fn from(value: scroll::Error) -> Self {
        Self::ScrollError(value)
    }
}

impl From<mutf8::Error> for Error {
    fn from(value: mutf8::Error) -> Self {
        Self::MUTF8Error(value)
    }
}