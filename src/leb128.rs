// TODO: I've copied and pasted this from an over year old Rust project I did when still learning rust
//       It should probably be cleaned up... somehow.
use crate::Error;
use num::{NumCast, PrimInt, Signed, Unsigned};
use std::io::Read;
use std::mem::size_of;

type Result<T> = std::result::Result<T, Error>;

pub fn encode_uleb128<T>(mut value: T) -> Vec<u8>
where
    T: PrimInt + Unsigned + NumCast,
{
    let mut result: Vec<u8> = Vec::new();

    loop {
        let byte = (value & T::from(0x7F).unwrap()).to_u8().unwrap();

        value = value >> 7;

        if value == T::zero() {
            result.push(byte);
            break;
        } else {
            result.push(byte | 0x80);
        }
    }

    result
}

pub fn encode_sleb128<T>(mut value: T) -> Vec<u8>
where
    T: PrimInt + Signed + NumCast,
{
    let mut result: Vec<u8> = Vec::new();

    loop {
        let byte = (value & T::from(0x7F).unwrap()).to_u8().unwrap();

        value = value >> 7;

        if (value == T::zero() && (byte & 0x80 == 0)) || (value == -T::one() && (byte & 0x80 == 0))
        {
            result.push(byte);
            break;
        } else {
            result.push(byte | 0x80);
        }
    }

    result
}

pub fn decode_uleb128<T, R>(reader: &mut R) -> Result<T>
where
    T: PrimInt + Unsigned + NumCast,
    R: Read,
{
    let mut result: T = T::zero();
    let mut buffer = [0u8; 1];
    let mut shift = 0;

    let bit_size = size_of::<T>() * 8;

    loop {
        reader.read_exact(&mut buffer)?;

        if shift <= bit_size {
            result = result | (T::from(buffer[0] & 0x7F).unwrap() << shift);
        } else {
            return Err(Error::Malformed(format!(
                "uleb128 bytes overflowed for type `{}`",
                std::any::type_name::<T>(),
            )));
        }

        if (buffer[0] & 0x80) == 0 {
            break;
        }

        shift += 7;
    }

    Ok(result)
}

pub fn decode_sleb128<T, R>(reader: &mut R) -> Result<T>
where
    T: PrimInt + Signed + NumCast,
    R: Read,
{
    let mut result: T = T::zero();
    let mut buffer = [0u8; 1];
    let mut shift = 0;

    let bit_size = size_of::<T>() * 8;

    loop {
        reader.read_exact(&mut buffer)?;

        if shift <= bit_size {
            result = result | (T::from(buffer[0] & 0x7F).unwrap() << shift);
        } else {
            return Err(Error::Malformed(format!(
                "sleb128 bytes overflowed for type `{}`",
                std::any::type_name::<T>(),
            )));
        }

        shift += 7;

        if (buffer[0] & 0x80) == 0 {
            break;
        }
    }

    if (shift < bit_size) && (buffer[0] & 0x40) != 0 {
        result = result | (!T::zero() << shift);
    }

    Ok(result)
}
