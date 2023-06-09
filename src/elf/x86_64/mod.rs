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

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Unwind information
        pub SHT_X86_64_UNWIND = 0x70000001;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Section flags
/// If an object file section does not have this flag set, then it may not hold
/// more than 2GB and can be freely referred to in objects using smaller code
/// models. Otherwise, only objects using larger code models can refer to them.
/// For example, a medium code model object can refer to data in a section that
/// sets this flag besides being able to refer to data in a section that does
/// not set it; likewise, a small code model object can refer only to code in a
/// section that does not set this flag.
pub const SHF_X86_64_LARGE: u32 = 0x10000000;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_X86_64_NONE = 0;
        pub R_X86_64_64 = 1;
        pub R_X86_64_PC32 = 2;
        pub R_X86_64_GOT32 = 3;
        pub R_X86_64_PLT32 = 4;
        pub R_X86_64_COPY = 5;
        pub R_X86_64_GLOB_DAT = 6;
        pub R_X86_64_JUMP_SLOT = 7;
        pub R_X86_64_RELATIVE = 8;
        pub R_X86_64_GOTPCREL = 9;
        pub R_X86_64_32 = 10;
        pub R_X86_64_32S = 11;
        pub R_X86_64_16 = 12;
        pub R_X86_64_PC16 = 13;
        pub R_X86_64_8 = 14;
        pub R_X86_64_PC8 = 15;
        pub R_X86_64_DTPMOD64 = 16;
        pub R_X86_64_DTPOFF64 = 17;
        pub R_X86_64_TPOFF64 = 18;
        pub R_X86_64_TLSGD = 19;
        pub R_X86_64_TLSLD = 20;
        pub R_X86_64_DTPOFF32 = 21;
        pub R_X86_64_GOTTPOFF = 22;
        pub R_X86_64_TPOFF32 = 23;
        pub R_X86_64_PC64 = 24;
        pub R_X86_64_GOTOFF64 = 25;
        pub R_X86_64_GOTPC32 = 26;
        pub R_X86_64_GOT64 = 27;
        pub R_X86_64_GOTPCREL64 = 28;
        pub R_X86_64_GOTPC64 = 29;
        pub R_X86_64_GOTPLT64 = 30;
        pub R_X86_64_PLTOFF64 = 31;
        pub R_X86_64_SIZE32 = 32;
        pub R_X86_64_SIZE64 = 33;
        pub R_X86_64_GOTPC32_TLSDESC = 34;
        pub R_X86_64_TLSDESC_CALL = 35;
        pub R_X86_64_TLSDESC = 36;
        pub R_X86_64_IRELATIVE = 37;
        pub R_X86_64_GOTPCRELX = 41;
        pub R_X86_64_REX_GOTPCRELX = 42;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
