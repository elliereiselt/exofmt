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
use std::io::Read;
use std::io::Seek;

type Result<T> = std::result::Result<T, Error>;

#[repr(C)]
pub struct Header {
    /// Magic number for the Dex file in the format of `dex\nVER\0` where `VER` is a value between `000` and `999`
    ///
    /// Example: `dex\n039\0`
    pub magic: [u8; 8],
    /// adler32 checksum of the rest of the file (everything but magic and this field); used to detect file corruption
    pub checksum: u32,
    /// SHA-1 signature (hash) of the rest of the file (everything but magic, checksum, and this field); used to uniquely identify files
    pub signature: [u8; 20],
    /// Size of the entire file (including the header), in bytes
    pub file_size: u32,
    /// Size of the found header in bytes. This allows for at least a limited amount of backwards/forwards compatibility without invalidating the format.
    pub header_size: u32,
    /// Endianness tag. 0x12345678 for big endian, 0x78563412 for little endian
    ///
    /// This field applies to ALL fields, including `checksum`, `file_size`, and `header_size`
    ///
    /// NOTE: Dex files will almost always be little endian. The only scenarios this may be untrue is with `.odex` but this too should be unlikely nowadays
    pub endian_tag: [u8; 4],
    /// Size of the link section, or `0` if this file isn't statically linked
    pub link_size: u32,
    /// Offset from the start of the file to the link section, or `0` if `link_size == 0`. The offset, if non-zero, should be to an offset into the `link_data` section
    pub link_offset: u32,
    /// Offset from the start of the file to the map list. The offset, which must be non-zero, should be to an offset into the `data` section
    pub map_offset: u32,
    /// Count of strings in the string identifiers list
    pub string_ids_size: u32,
    /// Offset from the start of the file to the string identifiers list, or `0` if `string_ids_size == 0`. The offset, if non-zero, should be to the start of the `string_ids` section
    pub string_ids_offset: u32,
    /// Count of elements in the type identifiers list, at most `65535`
    pub type_ids_size: u32,
    /// Offset from the start of the file to the type identifiers list, or `0` if `type_ids_size == 0`
    pub type_ids_offset: u32,
    /// Count of elements in the prototype identifiers list, at most `65535`
    pub proto_ids_size: u32,
    /// Offset from the start of the file to the prototype identifiers list, or `0` if `proto_ids_size == 0`. The offset, if non-zero, should be to the start of the `proto_ids` section
    pub proto_ids_offset: u32,
    /// Count of elements in the field identifiers list
    pub field_ids_size: u32,
    /// Offset from the start of the file to the field identifiers list, or `0` if `field_ids_size == 0`. The offset, if non-zero, should be to the start of the `field_ids` section
    pub field_ids_offset: u32,
    /// Count of elements in the method identifiers list
    pub method_ids_size: u32,
    /// Offset from the start of the file to the method identifiers list, or `0` if `method_ids_size == 0`. The offset, if non-zero, should be to the start of the `method_ids` section
    pub method_ids_offset: u32,
    /// Count of elements in the class definitions list
    pub class_defs_size: u32,
    /// Offset from the start of the file to the class definitions list, or `0` if `class_defs_size == 0`. The offset, if non-zero, should be to the start of the `class_defs` section
    pub class_defs_offset: u32,
    /// Size of data section in bytes. Must be an even multiple of `::std::mem::size_of::<u32>()`
    pub data_size: u32,
    /// Offset from the start of the file to the start of the data section
    pub data_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_check() {
        assert_eq!(::std::mem::size_of::<Header>(), 0x70);
    }
}

pub const MAGIC: [u8; 4] = [0x64, 0x65, 0x78, 0x0a];

pub const ENDIAN_CONSTANT: u32 = 0x12345678;
pub const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;

pub const ENDIAN_CONSTANT_BYTES: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
pub const REVERSE_ENDIAN_CONSTANT_BYTES: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

impl Header {
    pub fn read_from_buf_reader<TRead: IOread<Endian> + Seek>(
        reader: &mut BufReader<TRead>,
    ) -> Result<Header> {
        let mut magic = [0u8; 8];
        let mut checksum = [0u8; 4];
        let mut signature = [0u8; 20];
        let mut file_size = [0u8; 4];
        let mut header_size = [0u8; 4];
        let mut endian_tag = [0u8; 4];

        reader.read_exact(&mut magic)?;

        // I don't bother to validate the Dex version number but maybe we should... TBD
        // We allow `dex` or `cdex`, the caller should double check this...
        if (magic[0..4] != MAGIC && magic[0..4] != crate::dex::cdex::MAGIC) || magic[7] != 0 {
            return Err(Error::InvalidMagicNumber(magic.to_vec()));
        }

        reader.read_exact(&mut checksum)?;
        reader.read_exact(&mut signature)?;
        reader.read_exact(&mut file_size)?;
        reader.read_exact(&mut header_size)?;
        reader.read_exact(&mut endian_tag)?;

        let endianness = if endian_tag == REVERSE_ENDIAN_CONSTANT_BYTES {
            Endian::Little
        } else if endian_tag == ENDIAN_CONSTANT_BYTES {
            Endian::Big
        } else {
            return Err(Error::Malformed(format!(
                "Dex file `endian_tag` field malformed - found `{:02x?}` but expected `{:02x?}` or `{:02x?}`",
                endian_tag, ENDIAN_CONSTANT_BYTES, REVERSE_ENDIAN_CONSTANT_BYTES,
            )));
        };

        let checksum = if endianness.is_little() {
            u32::from_le_bytes(checksum)
        } else {
            u32::from_be_bytes(checksum)
        };
        let file_size = if endianness.is_little() {
            u32::from_le_bytes(file_size)
        } else {
            u32::from_be_bytes(file_size)
        };
        let header_size = if endianness.is_little() {
            u32::from_le_bytes(header_size)
        } else {
            u32::from_be_bytes(header_size)
        };
        let link_size = reader.ioread_with::<u32>(endianness)?;
        let link_offset = reader.ioread_with::<u32>(endianness)?;
        let map_offset = reader.ioread_with::<u32>(endianness)?;
        let string_ids_size = reader.ioread_with::<u32>(endianness)?;
        let string_ids_offset = reader.ioread_with::<u32>(endianness)?;
        let type_ids_size = reader.ioread_with::<u32>(endianness)?;
        let type_ids_offset = reader.ioread_with::<u32>(endianness)?;
        let proto_ids_size = reader.ioread_with::<u32>(endianness)?;
        let proto_ids_offset = reader.ioread_with::<u32>(endianness)?;
        let field_ids_size = reader.ioread_with::<u32>(endianness)?;
        let field_ids_offset = reader.ioread_with::<u32>(endianness)?;
        let method_ids_size = reader.ioread_with::<u32>(endianness)?;
        let method_ids_offset = reader.ioread_with::<u32>(endianness)?;
        let class_defs_size = reader.ioread_with::<u32>(endianness)?;
        let class_defs_offset = reader.ioread_with::<u32>(endianness)?;
        let data_size = reader.ioread_with::<u32>(endianness)?;
        let data_offset = reader.ioread_with::<u32>(endianness)?;

        Ok(Header {
            magic,
            checksum,
            signature,
            file_size,
            header_size,
            endian_tag,
            link_size,
            link_offset,
            map_offset,
            string_ids_size,
            string_ids_offset,
            type_ids_size,
            type_ids_offset,
            proto_ids_size,
            proto_ids_offset,
            field_ids_size,
            field_ids_offset,
            method_ids_size,
            method_ids_offset,
            class_defs_size,
            class_defs_offset,
            data_size,
            data_offset,
        })
    }
}
