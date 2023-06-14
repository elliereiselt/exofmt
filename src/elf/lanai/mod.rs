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

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        /// No relocation
        pub R_LANAI_NONE = 0;
        /// 21-bit symbol relocation
        pub R_LANAI_21 = 1;
        /// 21-bit symbol relocation with last two bits masked to 0
        pub R_LANAI_21_F = 2;
        /// 25-bit branch targets
        pub R_LANAI_25 = 3;
        /// General 32-bit relocation
        pub R_LANAI_32 = 4;
        /// Upper 16-bits of a symbolic relocation
        pub R_LANAI_HI16 = 5;
        /// Lower 16-bits of a symbolic relocation
        pub R_LANAI_LO16 = 6;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
