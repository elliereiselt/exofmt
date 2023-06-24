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
pub struct Dyn {
    // This was originally `i64`, I've replaced it with `u64` as it's just easier to work with
    pub d_tag: u64,
    pub d_val: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_size_check() {
        assert_eq!(::std::mem::size_of::<Dyn>(), 0x10);
    }
}

impl From<crate::elf::Dyn> for Dyn {
    fn from(value: crate::elf::Dyn) -> Self {
        Self {
            d_tag: value.d_tag,
            d_val: value.d_val,
        }
    }
}

impl From<Dyn> for crate::elf::Dyn {
    fn from(value: Dyn) -> Self {
        Self {
            d_tag: value.d_tag,
            d_val: value.d_val,
        }
    }
}
