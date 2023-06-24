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

use crate::vdex::vdex027::VDexSectionHeader;
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VDexFileHeader {
    pub magic: [u8; 4],
    pub vdex_version: [u8; 4],
    pub number_of_sections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_deps_header_size_check() {
        assert_eq!(::std::mem::size_of::<VDexFileHeader>(), 0xc);
    }
}

impl VDexFileHeader {
    /// Get the offset for a section header index relative to the start of the VDexFileHeader
    pub fn get_vdex_section_header_offset(&self, index: u32) -> u64 {
        (std::mem::size_of::<VDexFileHeader>()
            + (index as usize * std::mem::size_of::<VDexSectionHeader>())) as u64
    }
}
