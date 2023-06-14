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
pub struct Sym {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_size_check() {
        assert_eq!(::std::mem::size_of::<Sym>(), 0x10);
    }
}

impl TryFrom<crate::elf::Sym> for Sym {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::Sym) -> Result<Self, Self::Error> {
        Ok(Self {
            st_name: value.st_name,
            st_value: u32::try_from(value.st_value)?,
            st_size: u32::try_from(value.st_size)?,
            st_info: value.st_info,
            st_other: value.st_other,
            st_shndx: value.st_shndx,
        })
    }
}

impl From<Sym> for crate::elf::Sym {
    fn from(value: Sym) -> Self {
        Self {
            st_name: value.st_name,
            st_info: value.st_info,
            st_other: value.st_other,
            st_shndx: value.st_shndx,
            st_value: u64::from(value.st_value),
            st_size: u64::from(value.st_size),
        }
    }
}
