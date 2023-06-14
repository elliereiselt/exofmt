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

// NetBSD core note types.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_NETBSDCORE_PROCINFO = 1;
        pub NT_NETBSDCORE_AUXV = 2;
        pub NT_NETBSDCORE_LWPSTATUS = 24;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}
