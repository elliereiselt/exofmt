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

use crate::Error;
use scroll::Endian;
use scroll::IOread;
use std::io::BufReader;
use std::io::Seek;

type Result<T> = std::result::Result<T, Error>;

use crate::dex::Header as DexHeader;

pub struct Header {
    pub header: DexHeader,
    pub feature_flags: u32,
    /// Position in the CDex that the debug info table data starts
    pub debug_info_offsets_position: u32,
    /// Offset into the debug info table data where the lookup table is
    pub debug_info_offsets_table_offset: u32,
    /// Base offset of where debug info starts in the CDex file
    pub debug_info_base: u32,
    /// Range start of the shared data section owned by the CDex file
    pub owned_data_begin: u32,
    /// Range end of the shared data section owned by the CDex file
    pub owned_data_end: u32,
}

pub const MAGIC: [u8; 4] = [0x63, 0x64, 0x65, 0x78];

pub const FEATURE_FLAG_DEFAULT_METHODS: u32 = 0x01;

impl Header {
    pub fn read_from_buf_reader<TRead: IOread<Endian> + Seek>(
        reader: &mut BufReader<TRead>,
    ) -> Result<Header> {
        let header = crate::dex::Header::read_from_buf_reader(reader)?;

        let endianness = if header.endian_tag == crate::dex::REVERSE_ENDIAN_CONSTANT_BYTES {
            Endian::Little
        } else if header.endian_tag == crate::dex::ENDIAN_CONSTANT_BYTES {
            Endian::Big
        } else {
            return Err(Error::Malformed(format!(
                "Dex file `endian_tag` field malformed - found `{:02x?}` but expected `{:02x?}` or `{:02x?}`",
                header.endian_tag, crate::dex::ENDIAN_CONSTANT_BYTES, crate::dex::REVERSE_ENDIAN_CONSTANT_BYTES,
            )));
        };

        let feature_flags = reader.ioread_with::<u32>(endianness)?;
        let debug_info_offsets_position = reader.ioread_with::<u32>(endianness)?;
        let debug_info_offsets_table_offset = reader.ioread_with::<u32>(endianness)?;
        let debug_info_base = reader.ioread_with::<u32>(endianness)?;
        let owned_data_begin = reader.ioread_with::<u32>(endianness)?;
        let owned_data_end = reader.ioread_with::<u32>(endianness)?;

        Ok(Header {
            header,
            feature_flags,
            debug_info_offsets_position,
            debug_info_offsets_table_offset,
            debug_info_base,
            owned_data_begin,
            owned_data_end,
        })
    }
}
