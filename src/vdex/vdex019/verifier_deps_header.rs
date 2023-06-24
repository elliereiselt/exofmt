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
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerifierDepsHeader {
    pub magic: [u8; 4],
    pub verifier_deps_version: [u8; 4],
    pub dex_section_version: [u8; 4],
    pub number_of_dex_files: u32,
    pub verifier_deps_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_deps_header_size_check() {
        assert_eq!(::std::mem::size_of::<VerifierDepsHeader>(), 0x14);
    }
}

impl VerifierDepsHeader {
    pub fn has_dex_section(&self) -> bool {
        self.dex_section_version != crate::vdex::vdex019::DEX_SECTION_VERSION_EMPTY
    }
}
