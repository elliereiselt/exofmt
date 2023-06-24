/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
