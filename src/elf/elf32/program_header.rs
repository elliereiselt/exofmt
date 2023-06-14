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
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_header_size_check() {
        assert_eq!(::std::mem::size_of::<ProgramHeader>(), 0x20);
    }
}

impl TryFrom<crate::elf::ProgramHeader> for ProgramHeader {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::ProgramHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            p_type: value.p_type,
            p_flags: value.p_flags.bits(),
            p_offset: u32::try_from(value.p_offset)?,
            p_vaddr: u32::try_from(value.p_vaddr)?,
            p_paddr: u32::try_from(value.p_paddr)?,
            p_filesz: u32::try_from(value.p_filesz)?,
            p_memsz: u32::try_from(value.p_memsz)?,
            p_align: u32::try_from(value.p_align)?,
        })
    }
}

impl From<ProgramHeader> for crate::elf::ProgramHeader {
    fn from(value: ProgramHeader) -> Self {
        Self {
            p_type: value.p_type,
            p_flags: crate::elf::PFlags::from_bits_retain(value.p_flags),
            p_offset: u64::from(value.p_offset),
            p_vaddr: u64::from(value.p_vaddr),
            p_paddr: u64::from(value.p_paddr),
            p_filesz: u64::from(value.p_filesz),
            p_memsz: u64::from(value.p_memsz),
            p_align: u64::from(value.p_align),
        }
    }
}
