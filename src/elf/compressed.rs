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
use crate::stringable_consts_blocks::stringable_consts_block;
use std::borrow::Cow;

pub struct CompressionHeader {
    /// Compression format
    pub ch_type: u32,
    /// Reserved.
    pub ch_reserved: u32,
    /// Uncompressed data size
    pub ch_size: u64,
    /// Uncompressed data alignment
    pub ch_addralign: u64,
}

pub struct CompressedSection<'a> {
    pub header: CompressionHeader,
    pub bytes: Cow<'a, [u8]>,
}

// Compressed types
stringable_consts_block! {
    const stringable: u32 {
        /// ZLIB/DEFLATE algorithm.
        pub COMPRESS_ZLIB = 1;
        /// Zstandard algorithm
        pub COMPRESS_ZSTD = 2;
    }

    const ignore: u32 {
        /// Start of OS-specific.
        pub COMPRESS_LOOS = 0x60000000;
        /// End of OS-specific.
        pub COMPRESS_HIOS = 0x6fffffff;
        /// Start of processor-specific.
        pub COMPRESS_LOPROC = 0x70000000;
        /// End of processor-specific.
        pub COMPRESS_HIPROC = 0x7fffffff;
    }

    pub fn compress_to_str(value: u32) -> &'static str {
        match value {
            unknown => {
                if unknown >= COMPRESS_LOOS && unknown <= COMPRESS_HIOS {
                    "COMPRESS_UNKNOWN_OS_RANGE"
                } else if unknown >= COMPRESS_LOPROC && unknown <= COMPRESS_HIPROC {
                    "COMPRESS_UNKNOWN_PROC_RANGE"
                } else {
                    "COMPRESS_UNKNOWN"
                }
            }
        }
    }
}
