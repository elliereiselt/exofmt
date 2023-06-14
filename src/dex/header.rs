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
    pub endian_tag: u32,
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

pub const ENDIAN_CONSTANT: u32 = 0x12345678;
pub const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;

pub const ENDIAN_CONSTANT_BYTES: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
pub const REVERSE_ENDIAN_CONSTANT_BYTES: [u8; 4] = [0x78, 0x56, 0x34, 0x12];
