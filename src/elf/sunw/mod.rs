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

use crate::stringable_consts_blocks::option_stringable_consts_block;

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Contains stack unwind tables
        pub PT_SUNW_EH_FRAME = 0x6474e550;
        /// Contains stack unwind tables
        pub PT_SUNW_UNWIND = 0x6464e550;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section flags
/// Solaris equivalent of SHF_GNU_RETAIN
pub const SHF_SUNW_NODISCARD: u32 = 0x00100000;

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        /// Shared object to load before self
        pub DT_AUXILIARY = 0x7FFFFFFD;
        /// Same as DT_NEEDED
        pub DT_USED = 0x7FFFFFFE;
        /// Shared object to get values from
        pub DT_FILTER = 0x7FFFFFFF;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
