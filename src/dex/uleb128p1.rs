// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
}
