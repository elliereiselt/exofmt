use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct CompressionHeader {
    pub ch_type: u32,
    pub ch_reserved: u32,
    pub ch_size: u64,
    pub ch_addralign: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_header_size_check() {
        assert_eq!(::std::mem::size_of::<CompressionHeader>(), 0x18);
    }
}

impl From<crate::elf::CompressionHeader> for CompressionHeader {
    fn from(value: crate::elf::CompressionHeader) -> Self {
        Self {
            ch_type: value.ch_type,
            ch_reserved: value.ch_reserved,
            ch_size: value.ch_size,
            ch_addralign: value.ch_addralign,
        }
    }
}

impl From<CompressionHeader> for crate::elf::CompressionHeader {
    fn from(value: CompressionHeader) -> Self {
        Self {
            ch_type: value.ch_type,
            ch_reserved: value.ch_reserved,
            ch_size: value.ch_size,
            ch_addralign: value.ch_addralign,
        }
    }
}
