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
