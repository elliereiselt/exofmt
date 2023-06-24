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

// NOTE: This file has been _specifically_ written to match how Android parses leb128.
//       I'm not sure if this is correct or not for any other leb128 usecase.
//       Additionally, I've only validated `decode_uleb128` is correct, nothing else.
// TODO: I... might have actually ruined this because it turned out I was parsing something incorrect 😞
//       Basically, I was parsing a section that wasn't leb128. Now, I parse it the way the ART runtime does...
//       But I'm not sure if that's what I want? At least what I want in a generic `leb128`...
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

    while shift < bit_size {
        reader.read_exact(&mut buffer)?;

        if shift < bit_size {
            result = result | (T::from(buffer[0] & 0x7F).unwrap() << shift);
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

    while shift < bit_size {
        reader.read_exact(&mut buffer)?;

        result = result | (T::from(buffer[0] & 0x7F).unwrap() << shift);

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
