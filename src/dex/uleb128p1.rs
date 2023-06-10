use crate::error::Error;
use crate::leb128;
use std::io::Read;

type Result<T> = std::result::Result<T, Error>;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum uleb128p1 {
    Positive(u32),
    Negative,
}

impl uleb128p1 {
    pub fn encode(self) -> Vec<u8> {
        match self {
            Self::Positive(value) => leb128::encode_uleb128(value + 1),
            Self::Negative => leb128::encode_uleb128(0u32),
        }
    }

    pub fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let result = leb128::decode_uleb128::<u32, R>(reader)?;

        if result == 0 {
            Ok(Self::Negative)
        } else {
            Ok(Self::Positive(result - 1))
        }
    }
}
