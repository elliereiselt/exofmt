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
use crate::stringable_consts_blocks::stringable_consts_block;
use std::borrow::Cow;

pub struct Note<'a> {
    pub n_type: u32,
    pub n_name: Cow<'a, str>,
    pub n_desc: Cow<'a, [u8]>,
}

// Note section types

// Generic note types
stringable_consts_block! {
    const stringable: u32 {
        /// A version string
        pub NT_VERSION = 1;
        /// An architecture string
        pub NT_ARCH = 2;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> &'static str {
        match value {
            _unknown => "NT_UNKNOWN"
        }
    }
}

// Core note types (types for ET_CORE)
stringable_consts_block! {
    const stringable: u32 {
        pub NT_PRSTATUS = 1;
        pub NT_FPREGSET = 2;
        pub NT_PRPSINFO = 3;
        pub NT_TASKSTRUCT = 4;
        pub NT_AUXV = 6;
        pub NT_PSTATUS = 10;
        pub NT_FPREGS = 12;
        pub NT_PSINFO = 13;
        pub NT_LWPSTATUS = 16;
        pub NT_LWPSINFO = 17;
        pub NT_WIN32PSTATUS = 18;
        pub NT_FILE = 0x46494c45;
        pub NT_PRXFPREG = 0x46e62b7f;
        pub NT_SIGINFO = 0x53494749;
    }

    const ignore: u32 {}

    pub fn core_nt_to_str(value: u32) -> &'static str {
        match value {
            _unknown => "NT_UNKNOWN"
        }
    }
}
