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
#[derive(Clone)]
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

#[derive(Clone)]
pub struct RelA {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

#[derive(Clone)]
pub struct RelR {
    pub r_offset: u64,
}

impl Rel {
    pub fn r_sym(&self) -> u32 {
        crate::elf::elf64::elf64_r_sym(self.r_info)
    }

    pub fn r_type(&self) -> u32 {
        crate::elf::elf64::elf64_r_type(self.r_info)
    }

    pub fn set_r_sym_and_r_type(&mut self, r_sym: u32, r_type: u32) {
        self.r_info = crate::elf::elf64::elf64_r_info(r_sym, r_type);
    }
}

impl RelA {
    pub fn r_sym(&self) -> u32 {
        crate::elf::elf64::elf64_r_sym(self.r_info)
    }

    pub fn r_type(&self) -> u32 {
        crate::elf::elf64::elf64_r_type(self.r_info)
    }

    pub fn set_r_sym_and_r_type(&mut self, r_sym: u32, r_type: u32) {
        self.r_info = crate::elf::elf64::elf64_r_info(r_sym, r_type);
    }
}
