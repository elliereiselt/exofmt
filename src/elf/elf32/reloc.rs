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
    pub r_offset: u32,
    pub r_info: u32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct RelA {
    pub r_offset: u32,
    pub r_info: u32,
    pub r_addend: i32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct RelR {
    pub r_offset: u32,
}

pub const fn elf32_r_sym(i: u32) -> u32 {
    i >> 8
}

pub const fn elf32_r_type(i: u32) -> u8 {
    (i & 0xff) as u8
}

pub fn elf32_r_info(r_sym: u32, r_type: u8) -> u32 {
    (r_sym << 8) | u32::from(r_type)
}

fn convert_elf32_r_info_to_elf64(r_info: u32) -> u64 {
    let r_sym = elf32_r_sym(r_info);
    let r_type = elf32_r_type(r_info);
    crate::elf::elf64::elf64_r_info(r_sym, u32::from(r_type))
}

fn try_convert_elf64_r_info_to_elf32(r_info: u64) -> Result<u32, std::num::TryFromIntError> {
    let r_sym = crate::elf::elf64::elf64_r_sym(r_info);
    let r_type = u8::try_from(crate::elf::elf64::elf64_r_type(r_info))?;
    Ok(elf32_r_info(r_sym, r_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_size_check() {
        assert_eq!(::std::mem::size_of::<Rel>(), 0x8);
    }

    #[test]
    fn rela_size_check() {
        assert_eq!(::std::mem::size_of::<RelA>(), 0xc);
    }

    #[test]
    fn relr_size_check() {
        assert_eq!(::std::mem::size_of::<RelR>(), 0x4);
    }
}

impl TryFrom<crate::elf::Rel> for Rel {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::Rel) -> Result<Self, Self::Error> {
        Ok(Self {
            r_offset: u32::try_from(value.r_offset)?,
            r_info: try_convert_elf64_r_info_to_elf32(value.r_info)?,
        })
    }
}

impl From<Rel> for crate::elf::Rel {
    fn from(value: Rel) -> Self {
        Self {
            r_offset: u64::from(value.r_offset),
            r_info: convert_elf32_r_info_to_elf64(value.r_info),
        }
    }
}

impl TryFrom<crate::elf::RelA> for RelA {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::RelA) -> Result<Self, Self::Error> {
        Ok(Self {
            r_offset: u32::try_from(value.r_offset)?,
            r_info: try_convert_elf64_r_info_to_elf32(value.r_info)?,
            r_addend: i32::try_from(value.r_info)?,
        })
    }
}

impl From<RelA> for crate::elf::RelA {
    fn from(value: RelA) -> Self {
        Self {
            r_offset: u64::from(value.r_offset),
            r_info: convert_elf32_r_info_to_elf64(value.r_info),
            r_addend: i64::from(value.r_addend),
        }
    }
}

impl TryFrom<crate::elf::RelR> for RelR {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::RelR) -> Result<Self, Self::Error> {
        Ok(Self {
            r_offset: u32::try_from(value.r_offset)?,
        })
    }
}

impl From<RelR> for crate::elf::RelR {
    fn from(value: RelR) -> Self {
        Self {
            r_offset: u64::from(value.r_offset),
        }
    }
}
