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
