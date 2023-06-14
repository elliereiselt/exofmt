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
        /// Like bss, but not immutable.
        pub PT_OPENBSD_MUTABLE = 0x65a3dbe5;
        /// Fill with random data.
        pub PT_OPENBSD_RANDOMIZE = 0x65a3dbe6;
        /// Program does W^X violations.
        pub PT_OPENBSD_WXNEEDED = 0x65a3dbe7;
        /// Section for boot arguments.
        pub PT_OPENBSD_BOOTDATA = 0x65a41be6;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// OpenBSD core note types.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_OPENBSD_PROCINFO = 10;
        pub NT_OPENBSD_AUXV = 11;
        pub NT_OPENBSD_REGS = 20;
        pub NT_OPENBSD_FPREGS = 21;
        pub NT_OPENBSD_XFPREGS = 22;
        pub NT_OPENBSD_WCOOKIE = 23;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}
