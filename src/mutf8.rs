use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ContainedNulByte(),
    MissingContinuationByte(usize),
    InvalidSurrogatePair(usize),
    UnsupportedUtf8FourByteFormat(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ContainedNulByte() => write!(
                f,
                "Modified UTF-8 strings cannot contain a raw nul byte `\\0`!"
            ),
            Error::MissingContinuationByte(at_index) => write!(
                f,
                "Missing required continuation byte at index {}!",
                at_index
            ),
            Error::InvalidSurrogatePair(at_index) => write!(
                f,
                "Invalid surrogate pair at index {}",
                at_index
            ),
            Error::UnsupportedUtf8FourByteFormat(at_index) => write!(
                f,
                "Index {} is the start of a UTF-8 four-byte, this is not supported by Modified UTF-8",
                at_index
            ),
        }
    }
}

impl error::Error for Error {}

pub trait MUTF8 {
    fn from_mutf8(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

impl MUTF8 for String {
    fn from_mutf8(bytes: &[u8]) -> Result<Self> {
        // In most cases, the output bytes should be the same as the input bytes
        // An example when it won't be is for a nul byte, which is 2 bytes long
        // for MUTF-8
        let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut iter = bytes.iter();
        let mut current_index: usize = 0;

        macro_rules! commit_cont_byte {
            ($value:expr) => {
                if $value >= 0x80 && $value <= 0xBF {
                    result.push($value);
                    true
                } else {
                    false
                }
            };
        }

        macro_rules! commit_surrogate_pair {
            ($v:expr, $w:expr, $y:expr, $z:expr) => {
                // This is a straight rip of the algorithm listed in the JVM docs, I haven't tested
                // to see if this is accurate.
                let code_point = 0x10000
                    + (u32::from($v & 0x0F) << 16)
                    + (u32::from($w & 0x3F) << 10)
                    + (u32::from($y & 0x0F) << 6)
                    + u32::from($z & 0x3F);
                let byte0 = 0xF0 | ((code_point & 0x1C0000) >> 18) as u8;
                let byte1 = 0x80 | ((code_point & 0x3F000) >> 12) as u8;
                let byte2 = 0x80 | ((code_point & 0xFC0) >> 6) as u8;
                let byte3 = 0x80 | (code_point & 0x3F) as u8;
                result.push(byte0);
                result.push(byte1);
                result.push(byte2);
                result.push(byte3);
            };
        }

        macro_rules! next_byte {
            () => {{
                current_index += 1;
                iter.next()
            }};
        }

        while let Some(&byte0) = next_byte!() {
            match byte0 {
                0x00 => {
                    // Modified UTF-8 CANNOT contain raw nul bytes, they have to be
                    return Err(Error::ContainedNulByte());
                }
                0x01..=0x7F => result.push(byte0),
                0xC0..=0xDF => {
                    if let Some(&byte1) = next_byte!() {
                        // Java's Modified UTF-8 is weird, it uses `[0xC0, 0x80]` for a nul byte
                        // instead of just `\0`... I assume because the original JVM was written in
                        // C, idk :shrug:
                        if byte0 == 0xC0 && byte1 == 0x80 {
                            result.push(0);
                            continue;
                        } else {
                            result.push(byte0);

                            if commit_cont_byte!(byte1) {
                                continue;
                            }
                        }
                    }

                    // If we reach this point then something went wrong!
                    return Err(Error::MissingContinuationByte(current_index - 1));
                }
                0xE0..=0xEF => {
                    if let Some(&byte1) = next_byte!() {
                        // All the random byte ranges here don't come from the JVM docs
                        // They come from here: https://datatracker.ietf.org/doc/html/rfc3629#page-4
                        // Except the `(0xED, 0xA0..=0xAF)`, those do come from the JVM docs
                        if let Some(&byte2) = next_byte!() {
                            match (byte0, byte1) {
                                (0xE0, 0xA0..=0xBF)
                                | (0xE1..=0xEC, 0x80..=0xBF)
                                | (0xED, 0x80..=0x9F)
                                | (0xEE..=0xEF, 0x80..=0xBF) => {
                                    // These are valid UTF-8, just commit them
                                    result.push(byte0);
                                    result.push(byte1);
                                    result.push(byte2);
                                }
                                (0xED, 0xA0..=0xAF) => {
                                    // These are surrogate pairs...
                                    if let Some(&byte3) = next_byte!() {
                                        if byte3 != 0xED {
                                            return Err(Error::InvalidSurrogatePair(
                                                current_index - 1,
                                            ));
                                        }
                                        if let Some(&byte4) = next_byte!() {
                                            if !(byte4 >= 0xB0 && byte4 <= 0xBF) {
                                                return Err(Error::InvalidSurrogatePair(
                                                    current_index - 1,
                                                ));
                                            }

                                            if let Some(&byte5) = next_byte!() {
                                                if !(byte5 >= 0x80 && byte5 <= 0xBF) {
                                                    return Err(Error::InvalidSurrogatePair(
                                                        current_index - 1,
                                                    ));
                                                }

                                                commit_surrogate_pair!(byte1, byte2, byte4, byte5);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    // If we reach this point then something went wrong!
                    return Err(Error::InvalidSurrogatePair(current_index - 1));
                }
                _ => {
                    return Err(Error::UnsupportedUtf8FourByteFormat(current_index));
                }
            }
        }

        Ok(unsafe { String::from_utf8_unchecked(result) })
    }
}
