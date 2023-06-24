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

use crate::leb128;
use crate::Error;
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

    pub fn to_u32(&self) -> u32 {
        match self {
            uleb128p1::Positive(value) => *value,
            uleb128p1::Negative => 0xffffffff,
        }
    }
}
