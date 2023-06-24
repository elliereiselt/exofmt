/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::mutf8;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ScrollError(scroll::Error),
    MUTF8Error(mutf8::Error),

    InvalidMagicNumber(Vec<u8>),
    InvalidVersionNumber(String),
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
            Error::InvalidMagicNumber(value) => write!(f, "Invalid magic number: 0x{:x?}", *value),
            Error::InvalidVersionNumber(expected_version) => {
                write!(f, "Invalid version number: expected {}", expected_version)
            }
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
