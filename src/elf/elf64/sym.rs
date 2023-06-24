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
pub struct Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_size_check() {
        assert_eq!(::std::mem::size_of::<Sym>(), 0x18);
    }
}

impl From<crate::elf::Sym> for Sym {
    fn from(value: crate::elf::Sym) -> Self {
        Self {
            st_name: value.st_name,
            st_info: value.st_info,
            st_other: value.st_other,
            st_shndx: value.st_shndx,
            st_value: value.st_value,
            st_size: value.st_size,
        }
    }
}

impl From<Sym> for crate::elf::Sym {
    fn from(value: Sym) -> Self {
        Self {
            st_name: value.st_name,
            st_info: value.st_info,
            st_other: value.st_other,
            st_shndx: value.st_shndx,
            st_value: value.st_value,
            st_size: value.st_size,
        }
    }
}
