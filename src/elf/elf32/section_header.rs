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
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_header_size_check() {
        assert_eq!(::std::mem::size_of::<SectionHeader>(), 0x28);
    }
}

impl TryFrom<crate::elf::SectionHeader> for SectionHeader {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::SectionHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            sh_name: value.sh_name,
            sh_type: value.sh_type,
            sh_flags: u32::try_from(value.sh_flags.bits())?,
            sh_addr: u32::try_from(value.sh_addr)?,
            sh_offset: u32::try_from(value.sh_offset)?,
            sh_size: u32::try_from(value.sh_size)?,
            sh_link: value.sh_link,
            sh_info: value.sh_info,
            sh_addralign: u32::try_from(value.sh_addralign)?,
            sh_entsize: u32::try_from(value.sh_entsize)?,
        })
    }
}

impl From<SectionHeader> for crate::elf::SectionHeader {
    fn from(value: SectionHeader) -> Self {
        Self {
            sh_name: value.sh_name,
            sh_type: value.sh_type,
            sh_flags: crate::elf::SHFlags::from_bits_retain(u64::from(value.sh_flags)),
            sh_addr: u64::from(value.sh_addr),
            sh_offset: u64::from(value.sh_offset),
            sh_size: u64::from(value.sh_size),
            sh_link: value.sh_link,
            sh_info: value.sh_info,
            sh_addralign: u64::from(value.sh_addralign),
            sh_entsize: u64::from(value.sh_entsize),
        }
    }
}
