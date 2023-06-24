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
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_header_size_check() {
        assert_eq!(::std::mem::size_of::<SectionHeader>(), 0x40);
    }
}

impl From<crate::elf::SectionHeader> for SectionHeader {
    fn from(value: crate::elf::SectionHeader) -> Self {
        Self {
            sh_name: value.sh_name,
            sh_type: value.sh_type,
            sh_flags: value.sh_flags.bits(),
            sh_addr: value.sh_addr,
            sh_offset: value.sh_offset,
            sh_size: value.sh_size,
            sh_link: value.sh_link,
            sh_info: value.sh_info,
            sh_addralign: value.sh_addralign,
            sh_entsize: value.sh_entsize,
        }
    }
}

impl From<SectionHeader> for crate::elf::SectionHeader {
    fn from(value: SectionHeader) -> Self {
        Self {
            sh_name: value.sh_name,
            sh_type: value.sh_type,
            sh_flags: crate::elf::SHFlags::from_bits_retain(value.sh_flags),
            sh_addr: value.sh_addr,
            sh_offset: value.sh_offset,
            sh_size: value.sh_size,
            sh_link: value.sh_link,
            sh_info: value.sh_info,
            sh_addralign: value.sh_addralign,
            sh_entsize: value.sh_entsize,
        }
    }
}
