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

use crate::stringable_consts_blocks::stringable_consts_block;
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VDexSectionHeader {
    pub section_kind: u32,
    pub section_offset: u32,
    pub section_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_deps_header_size_check() {
        assert_eq!(::std::mem::size_of::<VDexSectionHeader>(), 0xc);
    }
}

stringable_consts_block! {
    const stringable: u32 {
        pub VDEX_SECTION_CHECKSUM = 0;
        pub VDEX_SECTION_DEX_FILE = 1;
        pub VDEX_SECTION_VERIFIER_DEPS = 2;
        pub VDEX_SECTION_TYPE_LOOKUP_TABLE = 3;
    }

    const ignore: u32 {}

    pub fn vdex_section_to_str(value: u32) -> &'static str {
        match value {
            _unknown => "VDEX_SECTION_UNKNOWN",
        }
    }
}
