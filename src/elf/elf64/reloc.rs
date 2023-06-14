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
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct RelA {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct RelR {
    pub r_offset: u64,
}

pub const fn elf64_r_sym(i: u64) -> u32 {
    (i >> 32) as u32
}

pub const fn elf64_r_type(i: u64) -> u32 {
    (i & 0xffffffff) as u32
}

pub fn elf64_r_info(r_sym: u32, r_type: u32) -> u64 {
    (u64::from(r_sym) << 32) | u64::from(r_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_size_check() {
        assert_eq!(::std::mem::size_of::<Rel>(), 0x10);
    }

    #[test]
    fn rela_size_check() {
        assert_eq!(::std::mem::size_of::<RelA>(), 0x18);
    }

    #[test]
    fn relr_size_check() {
        assert_eq!(::std::mem::size_of::<RelR>(), 0x8);
    }
}

impl From<crate::elf::Rel> for Rel {
    fn from(value: crate::elf::Rel) -> Self {
        Self {
            r_offset: value.r_offset,
            r_info: value.r_info,
        }
    }
}

impl From<Rel> for crate::elf::Rel {
    fn from(value: Rel) -> Self {
        Self {
            r_offset: value.r_offset,
            r_info: value.r_info,
        }
    }
}

impl From<crate::elf::RelA> for RelA {
    fn from(value: crate::elf::RelA) -> Self {
        Self {
            r_offset: value.r_offset,
            r_info: value.r_info,
            r_addend: value.r_addend,
        }
    }
}

impl From<RelA> for crate::elf::RelA {
    fn from(value: RelA) -> Self {
        Self {
            r_offset: value.r_offset,
            r_info: value.r_info,
            r_addend: value.r_addend,
        }
    }
}

impl From<crate::elf::RelR> for RelR {
    fn from(value: crate::elf::RelR) -> Self {
        Self {
            r_offset: value.r_offset,
        }
    }
}

impl From<RelR> for crate::elf::RelR {
    fn from(value: RelR) -> Self {
        Self {
            r_offset: value.r_offset,
        }
    }
}
