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
pub struct Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_check() {
        assert_eq!(::std::mem::size_of::<Header>(), 0x40);
    }
}

impl From<crate::elf::Header> for Header {
    fn from(value: crate::elf::Header) -> Self {
        Self {
            e_ident: value.e_ident,
            e_type: value.e_type,
            e_machine: value.e_machine,
            e_version: value.e_version,
            e_entry: value.e_entry,
            e_phoff: value.e_phoff,
            e_shoff: value.e_shoff,
            e_flags: value.e_flags,
            e_ehsize: value.e_ehsize,
            e_phentsize: value.e_phentsize,
            e_phnum: value.e_phnum,
            e_shentsize: value.e_shentsize,
            e_shnum: value.e_shnum,
            e_shstrndx: value.e_shstrndx,
        }
    }
}

impl From<Header> for crate::elf::Header {
    fn from(value: Header) -> Self {
        Self {
            e_ident: value.e_ident,
            e_type: value.e_type,
            e_machine: value.e_machine,
            e_version: value.e_version,
            e_entry: value.e_entry,
            e_phoff: value.e_phoff,
            e_shoff: value.e_shoff,
            e_flags: value.e_flags,
            e_ehsize: value.e_ehsize,
            e_phentsize: value.e_phentsize,
            e_phnum: value.e_phnum,
            e_shentsize: value.e_shentsize,
            e_shnum: value.e_shnum,
            e_shstrndx: value.e_shstrndx,
        }
    }
}
