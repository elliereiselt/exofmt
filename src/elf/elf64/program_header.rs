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
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_header_size_check() {
        assert_eq!(::std::mem::size_of::<ProgramHeader>(), 0x38);
    }
}

impl From<crate::elf::ProgramHeader> for ProgramHeader {
    fn from(value: crate::elf::ProgramHeader) -> Self {
        Self {
            p_type: value.p_type,
            p_flags: value.p_flags.bits(),
            p_offset: value.p_offset,
            p_vaddr: value.p_vaddr,
            p_paddr: value.p_paddr,
            p_filesz: value.p_filesz,
            p_memsz: value.p_memsz,
            p_align: value.p_align,
        }
    }
}

impl From<ProgramHeader> for crate::elf::ProgramHeader {
    fn from(value: ProgramHeader) -> Self {
        Self {
            p_type: value.p_type,
            p_flags: crate::elf::PFlags::from_bits_retain(value.p_flags),
            p_offset: value.p_offset,
            p_vaddr: value.p_vaddr,
            p_paddr: value.p_paddr,
            p_filesz: value.p_filesz,
            p_memsz: value.p_memsz,
            p_align: value.p_align,
        }
    }
}
