// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
