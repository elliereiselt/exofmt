pub mod common;
pub mod elf32;
pub mod elf64;

pub mod aarch64;
pub mod amdgpu;
pub mod android;
pub mod arc;
pub mod arm;
pub mod avr;
pub mod bpf;
pub mod c6000;
pub mod csky;
pub mod freebsd;
pub mod gnu;
pub mod hexagon;
pub mod lanai;
pub mod llvm;
pub mod loongarch;
pub mod m68k;
pub mod mips;
pub mod msp430;
pub mod netbsd;
pub mod openbsd;
pub mod ppc;
pub mod ppc64;
pub mod riscv;
pub mod s390;
pub mod sparc;
pub mod sunw;
pub mod ve;
pub mod x86;
pub mod x86_64;
pub mod xcore;
pub mod xtensa;

mod header;
pub use header::*;
mod program_header;
pub use program_header::*;
mod section_header;
pub use section_header::*;
mod strtab;
pub use strtab::*;
mod sym;
pub use sym::*;
mod note;
pub use note::*;
mod reloc;
pub use reloc::*;
mod dynamic;
pub use dynamic::*;
mod compressed;
pub use compressed::*;
mod hash;
pub use hash::*;

use crate::Error;
use scroll::{Endian, IOread};
use std::borrow::Cow;
use std::io::{Seek, SeekFrom};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ElfClass {
    Elf32,
    Elf64,
}

pub struct ElfIdent {
    pub class: ElfClass,
    pub endianness: Endian,
}

pub fn get_elf_ident<TRead: IOread<scroll::Endian> + Seek>(reader: &mut TRead) -> Result<ElfIdent> {
    let reset_position = reader.seek(SeekFrom::Current(0))?;

    reader.seek(SeekFrom::Start(0))?;

    let magic_number = reader.ioread_with::<u32>(scroll::BE)?;

    if magic_number != 0x7f454c46 {
        // Reset the position to prevent any potential issues with parsing after calling this function...
        reader.seek(SeekFrom::Start(reset_position))?;

        Err(Error::InvalidMagicNumber(u64::from(magic_number)))
    } else {
        let class = match reader.ioread_with::<u8>(scroll::BE)? {
            1 => ElfClass::Elf32,
            2 => ElfClass::Elf64,
            unknown => {
                return Err(Error::Malformed(format!(
                "Invalid `e_ident[EI_CLASS]` value `{}`, expected `1` for 32-bit or `2` for 64-bit",
                unknown
            )))
            }
        };
        let endianness = match reader.ioread_with::<u8>(scroll::BE)? {
            1 => scroll::LE,
            2 => scroll::BE,
            unknown => {
                return Err(Error::Malformed(format!(
                    "Invalid `e_ident[EI_DATA]` value `{}`, expected `1` for little endian or `2` for big endian",
                    unknown
                )))
            }
        };

        // Reset the position to prevent any potential issues with parsing after calling this function...
        reader.seek(SeekFrom::Start(reset_position))?;

        Ok(ElfIdent { class, endianness })
    }
}

pub trait Reader<'a> {
    fn read_header(&mut self) -> Result<Header>;
    fn read_program_headers(
        &mut self,
        e_phoff: u64,
        e_phentsize: u16,
        e_phnum: u16,
    ) -> Result<Vec<ProgramHeader>>;
    fn read_section_headers(
        &mut self,
        e_shoff: u64,
        e_shentsize: u16,
        e_shnum: u16,
    ) -> Result<Vec<SectionHeader>>;

    fn get_program_bytes(&mut self, program_header: &ProgramHeader) -> Result<Cow<'a, [u8]>>;

    fn read_dynamic_program(&mut self, program_header: &ProgramHeader) -> Result<Cow<'a, [Dyn]>>;

    fn get_section_bytes(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [u8]>>;

    fn read_str_table_section(&mut self, section_header: &SectionHeader) -> Result<StrTab<'a>>;
    fn read_sym_table_section(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [Sym]>>;
    fn read_note_section(&mut self, section_header: &SectionHeader) -> Result<Note<'a>>;
    fn read_compressed_section(
        &mut self,
        section_header: &SectionHeader,
    ) -> Result<CompressedSection<'a>>;
    fn read_dynamic_section(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [Dyn]>>;
    fn read_hash_section(&mut self, section_header: &SectionHeader) -> Result<Hash<'a>>;
    fn read_rel_section(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [Rel]>>;
    fn read_rela_section(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [RelA]>>;
    fn read_relr_section(&mut self, section_header: &SectionHeader) -> Result<Cow<'a, [RelR]>>;

    fn get_bytes(&mut self, table_offset: u64, table_size: u64) -> Result<Cow<'a, [u8]>>;

    fn read_str_table(&mut self, table_offset: u64, table_size: u64) -> Result<StrTab<'a>>;
    fn read_sym_table(&mut self, table_offset: u64, table_size: u64) -> Result<Cow<'a, [Sym]>>;
}

macro_rules! validate_section_header_overflow {
    ($section_header:expr, $stream_len:expr) => {{
        let offset = $section_header.sh_offset as u64;
        let size = $section_header.sh_size;
        let (end, overflow) = offset.overflowing_add(size as u64);

        if end >= $stream_len || overflow {
            Err(Error::Malformed(format!(
                "Section offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                offset, size, $stream_len
            )))
        } else {
            Ok(())
        }
    }};
}

macro_rules! validate_program_header_overflow {
    ($program_header:expr, $stream_len:expr) => {{
        assert!($program_header.p_offset != 0);

        let offset = $program_header.p_offset as u64;
        let size = $program_header.p_filesz;
        let (end, overflow) = offset.overflowing_add(size as u64);

        if end >= $stream_len || overflow {
            Err(Error::Malformed(format!(
                "Program offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                offset, size, $stream_len
            )))
        } else {
            Ok(())
        }
    }};
}

macro_rules! validate_offset_overflow {
    ($slice_offset:expr, $slice_size:expr, $stream_len:expr) => {{
        assert!($slice_offset != 0);

        let offset = $slice_offset;
        let size = $slice_size;
        let (end, overflow) = offset.overflowing_add(size);

        if end >= $stream_len || overflow {
            Err(Error::Malformed(format!(
                "Bytes offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                offset, size, $stream_len
            )))
        } else {
            Ok(())
        }
    }};
}

macro_rules! validate_section_header_sh_flags_and_size {
    ($calling_function_name:literal, $section_header:expr, $expected_sh_flag:expr, $expected_sh_flag_str:literal, $stream_len:expr) => {
        if !$section_header.sh_flags.contains($expected_sh_flag) {
            Err(Error::InvalidArguments(format!(
                "Invalid `section_header` passed to `{}`, `sh_flags` must have `{}` flag",
                $calling_function_name, $expected_sh_flag_str,
            )))
        } else {
            crate::elf::validate_section_header_overflow!($section_header, $stream_len)
        }
    };
}

macro_rules! validate_section_header_sh_type_and_size {
    ($calling_function_name:literal, $section_header:expr, $expected_sh_type:expr, $expected_sh_type_str:literal, $stream_len:expr) => {
        if $section_header.sh_type != $expected_sh_type {
            Err(Error::InvalidArguments(format!(
                "Invalid `section_header` passed to `{}`, expected `sh_type` of `{}` but found `{}`",
                $calling_function_name,
                $expected_sh_type_str,
                crate::elf::sht_to_str($section_header.sh_type)
            )))
        } else {
            crate::elf::validate_section_header_overflow!($section_header, $stream_len)
        }
    };
}

macro_rules! validate_program_header_p_type_and_size {
    ($calling_function_name:literal, $program_header:expr, $expected_p_type:expr, $expected_p_type_str:literal, $stream_len:expr) => {
        if $program_header.p_type != $expected_p_type {
            Err(Error::InvalidArguments(format!(
                "Invalid `program_header` passed to `{}`, expected `sh_type` of `{}` but found `{}`",
                $calling_function_name,
                $expected_p_type_str,
                crate::elf::pt_to_str($program_header.p_type)
            )))
        } else {
            crate::elf::validate_program_header_overflow!($program_header, $stream_len)
        }
    };
}

// TODO: For future proofing, it might be good to at least do a `debug_assert!($section_header.sh_entsize == entsize)`...
macro_rules! io_read_section_as_array {
    ($reader:expr, $endianness:expr, $section_header:expr, $read_index_type:ty, $insert_index_type:ty) => {{
        let entsize = ::std::mem::size_of::<$read_index_type>();
        let length = $section_header.sh_size / (entsize as u64);

        if let Ok(length) = usize::try_from(length) {
            let mut result: Vec<$insert_index_type> = Vec::with_capacity(length);

            for _ in 0..length {
                let value = $reader.ioread_with::<$read_index_type>($endianness)?;
                result.push(<$insert_index_type>::from(value));
            }

            Ok(result)
        } else {
            Err(Error::TooManyArrayItems(format!(
                "Attempted to parse array with `{}` items, more items than can fit in `usize` (`{}` max)",
                length,
                usize::MAX,
            )))
        }
    }};
}

macro_rules! io_read_program_as_array {
    ($reader:expr, $endianness:expr, $program_header:expr, $read_index_type:ty, $insert_index_type:ty) => {{
        let entsize = ::std::mem::size_of::<$read_index_type>();
        let length = $program_header.p_filesz / (entsize as u64);

        if let Ok(length) = usize::try_from(length) {
            let mut result: Vec<$insert_index_type> = Vec::with_capacity(length);

            for _ in 0..length {
                let value = $reader.ioread_with::<$read_index_type>($endianness)?;
                result.push(<$insert_index_type>::from(value));
            }

            Ok(result)
        } else {
            Err(Error::TooManyArrayItems(format!(
                "Attempted to parse array with `{}` items, more items than can fit in `usize` (`{}` max)",
                length,
                usize::MAX,
            )))
        }
    }};
}

macro_rules! io_read_as_array {
    ($reader:expr, $endianness:expr, $slice_size:expr, $read_index_type:ty, $insert_index_type:ty) => {{
        let entsize = ::std::mem::size_of::<$read_index_type>();
        let length = $slice_size / (entsize as u64);

        if let Ok(length) = usize::try_from(length) {
            let mut result: Vec<$insert_index_type> = Vec::with_capacity(length);

            for _ in 0..length {
                let value = $reader.ioread_with::<$read_index_type>($endianness)?;
                result.push(<$insert_index_type>::from(value));
            }

            Ok(result)
        } else {
            Err(Error::TooManyArrayItems(format!(
                "Attempted to parse array with `{}` items, more items than can fit in `usize` (`{}` max)",
                length,
                usize::MAX,
            )))
        }
    }};
}

pub(crate) use io_read_as_array;
pub(crate) use io_read_program_as_array;
pub(crate) use io_read_section_as_array;
pub(crate) use validate_offset_overflow;
pub(crate) use validate_program_header_overflow;
pub(crate) use validate_program_header_p_type_and_size;
pub(crate) use validate_section_header_overflow;
pub(crate) use validate_section_header_sh_flags_and_size;
pub(crate) use validate_section_header_sh_type_and_size;

macro_rules! elf_io_reader_impl {
    (
        $Header:ty,
        $ProgramHeader:ty,
        $SectionHeader:ty,
        $Sym:ty,
        $CompressionHeader:ty,
        $Dyn:ty,
        $Rel:ty,
        $RelA:ty,
        $RelR:ty
    ) => {
        use scroll::{Endian, IOread};
        use std::borrow::Cow;
        use std::io::{Seek, SeekFrom};

        use crate::Error;

        type Result<T> = std::result::Result<T, Error>;

        pub struct IoReader<'a, TRead: IOread<Endian> + Seek> {
            pub reader: &'a mut TRead,
            pub endianness: Endian,
            pub stream_len: u64,
        }

        impl<'a, TRead: IOread<Endian> + Seek> IoReader<'a, TRead> {
            pub fn new(reader: &'a mut TRead, endianness: Endian) -> Result<Self> {
                let stream_len = reader.seek(SeekFrom::End(0))?;
                reader.seek(SeekFrom::Start(0))?;
                Ok(Self {
                    reader,
                    endianness,
                    stream_len,
                })
            }

            pub fn read_header(&mut self) -> Result<crate::elf::Header> {
                self.reader.seek(SeekFrom::Start(0))?;
                Ok(self.reader.ioread_with::<$Header>(self.endianness)?.into())
            }

            pub fn read_program_headers(
                &mut self,
                e_phoff: u64,
                e_phentsize: u16,
                e_phnum: u16,
            ) -> Result<Vec<crate::elf::ProgramHeader>> {
                self.reader.seek(SeekFrom::Start(e_phoff))?;

                let mut result: Vec<crate::elf::ProgramHeader> = Vec::with_capacity(e_phnum as usize);

                for phidx in 0..e_phnum {
                    let offset: u64 = e_phoff + ((phidx as u64) * (e_phentsize as u64));

                    result.push(self.read_program_header(offset)?)
                }

                Ok(result)
            }

            pub fn read_program_header(&mut self, offset: u64) -> Result<crate::elf::ProgramHeader> {
                self.reader.seek(SeekFrom::Start(offset))?;

                let program_header = self.reader.ioread_with::<$ProgramHeader>(self.endianness)?;

                Ok(program_header.into())
            }

            pub fn read_section_headers(
                &mut self,
                e_shoff: u64,
                e_shentsize: u16,
                e_shnum: u16,
            ) -> Result<Vec<crate::elf::SectionHeader>> {
                self.reader.seek(SeekFrom::Start(e_shoff))?;

                let mut result: Vec<crate::elf::SectionHeader> = Vec::with_capacity(e_shnum as usize);

                for shidx in 0..e_shnum {
                    let offset: u64 = e_shoff + ((shidx as u64) * (e_shentsize as u64));

                    result.push(self.read_section_header(offset)?)
                }

                Ok(result)
            }

            pub fn read_section_header(&mut self, offset: u64) -> Result<crate::elf::SectionHeader> {
                self.reader.seek(SeekFrom::Start(offset))?;

                let section_header = self.reader.ioread_with::<$SectionHeader>(self.endianness)?;

                Ok(section_header.into())
            }

            // Program parsing
            pub fn get_program_bytes(&mut self, program_header: &crate::elf::ProgramHeader) -> Result<Cow<'static, [u8]>> {
                assert!(program_header.p_offset != 0);

                let offset = program_header.p_offset as u64;
                let size = program_header.p_filesz;
                let (end, overflow) = offset.overflowing_add(size as u64);

                if end >= self.stream_len || overflow {
                    Err(Error::Malformed(format!(
                        "Program offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                        offset, size, self.stream_len
                    )))
                } else {
                    self.reader.seek(SeekFrom::Start(offset))?;

                    if let Ok(size) = usize::try_from(size) {
                        let mut result: Vec<u8> = Vec::with_capacity(size);
                        // Resize is a "safe" way to set length to what the capacity is
                        result.resize(size, 0);
                        self.reader.read_exact(&mut result)?;
                        Ok(Cow::Owned(result))
                    } else {
                        Err(Error::TooManyArrayItems(format!(
                            "Attempted to load the bytes of a program with a size of `{}`, `usize` can only hold `{}` bytes", size, usize::MAX
                        )))
                    }
                }
            }

            pub fn read_dynamic_program(&mut self, program_header: &crate::elf::ProgramHeader) -> Result<Cow<'static, [crate::elf::Dyn]>> {
                crate::elf::validate_program_header_p_type_and_size!(
                    "read_dynamic_program",
                    program_header,
                    crate::elf::PT_DYNAMIC,
                    "PT_DYNAMIC",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(program_header.p_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_program_as_array!(
                    self.reader,
                    self.endianness,
                    program_header,
                    $Dyn,
                    crate::elf::Dyn
                )?))
            }

            // Section parsing
            pub fn get_section_bytes(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [u8]>> {
                let offset = section_header.sh_offset as u64;
                let size = section_header.sh_size;
                let (end, overflow) = offset.overflowing_add(size as u64);

                if end >= self.stream_len || overflow {
                    Err(Error::Malformed(format!(
                        "Section offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                        offset, size, self.stream_len
                    )))
                } else {
                    self.reader.seek(SeekFrom::Start(offset))?;

                    if let Ok(size) = usize::try_from(size) {
                        let mut result: Vec<u8> = Vec::with_capacity(size);
                        // Resize is a "safe" way to set length to what the capacity is
                        result.resize(size, 0);
                        self.reader.read_exact(&mut result)?;
                        Ok(Cow::Owned(result))
                    } else {
                        Err(Error::TooManyArrayItems(format!(
                            "Attempted to load the bytes of a section with a size of `{}`, `usize` can only hold `{}` bytes", size, usize::MAX
                        )))
                    }
                }
            }

            pub fn read_str_table_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<crate::elf::StrTab<'static>> {
                if section_header.sh_type != crate::elf::SHT_STRTAB {
                    return Err(Error::InvalidArguments(format!(
                        "Invalid `section_header` passed to `read_str_table_section`, expected `sh_type` of `SHT_STRTAB` but found `{}`",
                        crate::elf::sht_to_str(section_header.sh_type)
                    )));
                }

                let strtab_bytes = self.get_section_bytes(&section_header)?;
                Ok(crate::elf::StrTab::<'static>::parse(strtab_bytes, 0)?)
            }

            pub fn read_hash_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<crate::elf::Hash<'static>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_hash_section",
                    section_header,
                    crate::elf::SHT_HASH,
                    "SHT_HASH",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                let hash_header = self.reader.ioread_with::<crate::elf::common::HashHeader>(self.endianness)?;
                let mut buckets: Vec<u32> = Vec::with_capacity(hash_header.n_buckets as usize);
                let mut chains: Vec<u32> = Vec::with_capacity(hash_header.n_chains as usize);

                for _ in 0..hash_header.n_buckets {
                    buckets.push(self.reader.ioread_with::<u32>(self.endianness)?);
                }

                for _ in 0..hash_header.n_chains {
                    chains.push(self.reader.ioread_with::<u32>(self.endianness)?);
                }

                Ok(crate::elf::Hash {
                    buckets: Cow::Owned(buckets),
                    chains: Cow::Owned(chains),
                })
            }

            pub fn read_note_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<crate::elf::Note<'static>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_section_note",
                    section_header,
                    crate::elf::SHT_NOTE,
                    "SHT_NOTE",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                let note_header = self.reader.ioread_with::<crate::elf::common::NoteHeader>(self.endianness)?;
                // I'm doing `- 1` to get rid of the unneeded nul character
                let n_name_len = note_header.n_namesz as usize - 1;
                let mut n_name_bytes: Vec<u8> = Vec::with_capacity(n_name_len);
                n_name_bytes.resize(n_name_len, 0);
                let mut n_desc: Vec<u8> = Vec::with_capacity(note_header.n_descsz as usize);
                n_desc.resize(note_header.n_descsz as usize, 0);

                // Since I'm ignoring the nul character for `n_name_bytes`, we need to read that nul into something...
                let mut nul_char = [0u8; 1];

                self.reader.read_exact(&mut n_name_bytes)?;
                self.reader.read_exact(&mut nul_char)?;
                self.reader.read_exact(&mut n_desc)?;

                let n_name = match String::from_utf8(n_name_bytes) {
                    Ok(string) => string,
                    Err(utf8_error) => {
                        return Err(Error::Malformed(format!(
                            "Invalid note name as offset {}, {}",
                            section_header.sh_offset, utf8_error
                        )))
                    }
                };

                Ok(crate::elf::Note {
                    n_type: note_header.n_type,
                    n_name: Cow::Owned(n_name),
                    n_desc: Cow::Owned(n_desc),
                })
            }

            pub fn read_sym_table_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [crate::elf::Sym]>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_sym_table_section",
                    section_header,
                    crate::elf::SHT_SYMTAB,
                    "SHT_SYMTAB",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_section_as_array!(
                    self.reader,
                    self.endianness,
                    section_header,
                    $Sym,
                    crate::elf::Sym
                )?))
            }

            pub fn read_compressed_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<crate::elf::CompressedSection<'static>> {
                crate::elf::validate_section_header_sh_flags_and_size!(
                    "read_compressed_section",
                    section_header,
                    crate::elf::SHFlags::COMPRESSED,
                    "SHF_COMRESSED",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                let compression_header = self
                    .reader
                    .ioread_with::<$CompressionHeader>(self.endianness)?;

                if let Ok(sh_size) = usize::try_from(section_header.sh_size) {
                    let compressed_bytes_length = sh_size - ::std::mem::size_of::<$CompressionHeader>();
                    let mut compressed_bytes: Vec<u8> = Vec::with_capacity(compressed_bytes_length);
                    compressed_bytes.resize(compressed_bytes_length, 0);

                    self.reader.read_exact(&mut compressed_bytes)?;

                    Ok(crate::elf::CompressedSection {
                        header: compression_header.into(),
                        bytes: Cow::Owned(compressed_bytes),
                    })
                } else {
                    Err(Error::TooManyArrayItems(format!(
                        "Compressed section had {} of compressed bytes, more than max value of `usize` ({})",
                        section_header.sh_size,
                        usize::MAX,
                    )))
                }
            }

            pub fn read_dynamic_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [crate::elf::Dyn]>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_dynamic_section",
                    section_header,
                    crate::elf::SHT_DYNAMIC,
                    "SHT_DYNAMIC",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_section_as_array!(
                    self.reader,
                    self.endianness,
                    section_header,
                    $Dyn,
                    crate::elf::Dyn
                )?))
            }

            pub fn read_rel_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [crate::elf::Rel]>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_rel_section",
                    section_header,
                    crate::elf::SHT_REL,
                    "SHT_REL",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_section_as_array!(
                    self.reader,
                    self.endianness,
                    section_header,
                    $Rel,
                    crate::elf::Rel
                )?))
            }

            pub fn read_rela_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [crate::elf::RelA]>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_rela_section",
                    section_header,
                    crate::elf::SHT_RELA,
                    "SHT_RELA",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_section_as_array!(
                    self.reader,
                    self.endianness,
                    section_header,
                    $RelA,
                    crate::elf::RelA
                )?))
            }

            pub fn read_relr_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<Cow<'static, [crate::elf::RelR]>> {
                crate::elf::validate_section_header_sh_type_and_size!(
                    "read_relr_section",
                    section_header,
                    crate::elf::SHT_RELR,
                    "SHT_RELR",
                    self.stream_len
                )?;

                self.reader
                    .seek(SeekFrom::Start(section_header.sh_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_section_as_array!(
                    self.reader,
                    self.endianness,
                    section_header,
                    $RelR,
                    crate::elf::RelR
                )?))
            }

            pub fn get_bytes(
                &mut self,
                table_offset: u64,
                table_size: u64,
            ) -> Result<Cow<'static, [u8]>> {
                let offset = table_offset;
                let size = table_size;
                let (end, overflow) = offset.overflowing_add(size);

                if end >= self.stream_len || overflow {
                    Err(Error::Malformed(format!(
                        "Bytes offset of `{}` + size of `{}` is out of bounds for `{}` bytes",
                        offset, size, self.stream_len
                    )))
                } else {
                    self.reader.seek(SeekFrom::Start(offset))?;

                    if let Ok(size) = usize::try_from(size) {
                        let mut result: Vec<u8> = Vec::with_capacity(size);
                        // Resize is a "safe" way to set length to what the capacity is
                        result.resize(size, 0);
                        self.reader.read_exact(&mut result)?;
                        Ok(Cow::Owned(result))
                    } else {
                        Err(Error::TooManyArrayItems(format!(
                            "Attempted to load the bytes with a size of `{}`, `usize` can only hold `{}` bytes", size, usize::MAX
                        )))
                    }
                }
            }

            pub fn read_str_table(&mut self, table_offset: u64, table_size: u64) -> Result<crate::elf::StrTab<'static>> {
                let strtab_bytes = self.get_bytes(table_offset, table_size)?;
                Ok(crate::elf::StrTab::<'static>::parse(strtab_bytes, 0)?)
            }

            pub fn read_sym_table(&mut self, table_offset: u64, table_size: u64) -> Result<Cow<'static, [crate::elf::Sym]>> {
                crate::elf::validate_offset_overflow!(table_offset, table_size, self.stream_len)?;

                self.reader
                    .seek(SeekFrom::Start(table_offset))?;

                Ok(Cow::Owned(crate::elf::io_read_as_array!(
                    self.reader,
                    self.endianness,
                    table_size,
                    $Sym,
                    crate::elf::Sym
                )?))
            }
        }

        impl<'a, TRead: IOread<Endian> + Seek> crate::elf::Reader<'static> for IoReader<'a, TRead> {
            fn read_header(&mut self) -> Result<crate::elf::Header> {
                self.read_header()
            }

            fn read_program_headers(
                &mut self,
                e_phoff: u64,
                e_phentsize: u16,
                e_phnum: u16,
            ) -> Result<Vec<crate::elf::ProgramHeader>> {
                self.read_program_headers(e_phoff, e_phentsize, e_phnum)
            }

            fn read_section_headers(
                &mut self,
                e_shoff: u64,
                e_shentsize: u16,
                e_shnum: u16,
            ) -> Result<Vec<crate::elf::SectionHeader>> {
                self.read_section_headers(e_shoff, e_shentsize, e_shnum)
            }

            fn get_program_bytes(&mut self, program_header: &crate::elf::ProgramHeader) -> Result<Cow<'static, [u8]>> {
                self.get_program_bytes(program_header)
            }

            fn read_dynamic_program(&mut self, program_header: &crate::elf::ProgramHeader) -> Result<Cow<'static, [crate::elf::Dyn]>> {
                self.read_dynamic_program(program_header)
            }

            fn get_section_bytes(&mut self, section_header: &crate::elf::SectionHeader) -> Result<Cow<'static, [u8]>> {
                self.get_section_bytes(section_header)
            }

            fn read_str_table_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<crate::elf::StrTab<'static>> {
                self.read_str_table_section(section_header)
            }

            fn read_sym_table_section(&mut self, section_header: &crate::elf::SectionHeader)
                -> Result<Cow<'static, [crate::elf::Sym]>> {
                self.read_sym_table_section(section_header)
            }

            fn read_note_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<crate::elf::Note<'static>> {
                self.read_note_section(section_header)
            }

            fn read_compressed_section(
                &mut self,
                section_header: &crate::elf::SectionHeader,
            ) -> Result<crate::elf::CompressedSection<'static>> {
                self.read_compressed_section(section_header)
            }

            fn read_dynamic_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<Cow<'static, [crate::elf::Dyn]>> {
                self.read_dynamic_section(section_header)
            }

            fn read_hash_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<crate::elf::Hash<'static>> {
                self.read_hash_section(section_header)
            }

            fn read_rel_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<Cow<'static, [crate::elf::Rel]>> {
                self.read_rel_section(section_header)
            }

            fn read_rela_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<Cow<'static, [crate::elf::RelA]>> {
                self.read_rela_section(section_header)
            }

            fn read_relr_section(&mut self, section_header: &crate::elf::SectionHeader) -> Result<Cow<'static, [crate::elf::RelR]>> {
                self.read_relr_section(section_header)
            }

            fn get_bytes(&mut self, table_offset: u64, table_size: u64) -> Result<Cow<'static, [u8]>> {
                self.get_bytes(table_offset, table_size)
            }

            fn read_str_table(&mut self, table_offset: u64, table_size: u64) -> Result<crate::elf::StrTab<'static>> {
                self.read_str_table(table_offset, table_size)
            }

            fn read_sym_table(&mut self, table_offset: u64, table_size: u64) -> Result<Cow<'static, [crate::elf::Sym]>> {
                self.read_sym_table(table_offset, table_size)
            }
        }
    };
}

pub(crate) use elf_io_reader_impl;
