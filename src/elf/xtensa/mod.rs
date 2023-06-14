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

// e_flags
pub const EF_XTENSA_MACH: u32 = 0x0000000f;
pub const EF_XTENSA_MACH_NONE: u32 = 0x00000000;
pub const EF_XTENSA_XT_INSN: u32 = 0x00000100;
pub const EF_XTENSA_XT_LIT: u32 = 0x00000200;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_XTENSA_NONE = 0;
        pub R_XTENSA_32 = 1;
        pub R_XTENSA_RTLD = 2;
        pub R_XTENSA_GLOB_DAT = 3;
        pub R_XTENSA_JMP_SLOT = 4;
        pub R_XTENSA_RELATIVE = 5;
        pub R_XTENSA_PLT = 6;
        pub R_XTENSA_OP0 = 8;
        pub R_XTENSA_OP1 = 9;
        pub R_XTENSA_OP2 = 10;
        pub R_XTENSA_ASM_EXPAND = 11;
        pub R_XTENSA_ASM_SIMPLIFY = 12;
        pub R_XTENSA_32_PCREL = 14;
        pub R_XTENSA_GNU_VTINHERIT = 15;
        pub R_XTENSA_GNU_VTENTRY = 16;
        pub R_XTENSA_DIFF8 = 17;
        pub R_XTENSA_DIFF16 = 18;
        pub R_XTENSA_DIFF32 = 19;
        pub R_XTENSA_SLOT0_OP = 20;
        pub R_XTENSA_SLOT1_OP = 21;
        pub R_XTENSA_SLOT2_OP = 22;
        pub R_XTENSA_SLOT3_OP = 23;
        pub R_XTENSA_SLOT4_OP = 24;
        pub R_XTENSA_SLOT5_OP = 25;
        pub R_XTENSA_SLOT6_OP = 26;
        pub R_XTENSA_SLOT7_OP = 27;
        pub R_XTENSA_SLOT8_OP = 28;
        pub R_XTENSA_SLOT9_OP = 29;
        pub R_XTENSA_SLOT10_OP = 30;
        pub R_XTENSA_SLOT11_OP = 31;
        pub R_XTENSA_SLOT12_OP = 32;
        pub R_XTENSA_SLOT13_OP = 33;
        pub R_XTENSA_SLOT14_OP = 34;
        pub R_XTENSA_SLOT0_ALT = 35;
        pub R_XTENSA_SLOT1_ALT = 36;
        pub R_XTENSA_SLOT2_ALT = 37;
        pub R_XTENSA_SLOT3_ALT = 38;
        pub R_XTENSA_SLOT4_ALT = 39;
        pub R_XTENSA_SLOT5_ALT = 40;
        pub R_XTENSA_SLOT6_ALT = 41;
        pub R_XTENSA_SLOT7_ALT = 42;
        pub R_XTENSA_SLOT8_ALT = 43;
        pub R_XTENSA_SLOT9_ALT = 44;
        pub R_XTENSA_SLOT10_ALT = 45;
        pub R_XTENSA_SLOT11_ALT = 46;
        pub R_XTENSA_SLOT12_ALT = 47;
        pub R_XTENSA_SLOT13_ALT = 48;
        pub R_XTENSA_SLOT14_ALT = 49;
        pub R_XTENSA_TLSDESC_FN = 50;
        pub R_XTENSA_TLSDESC_ARG = 51;
        pub R_XTENSA_TLS_DTPOFF = 52;
        pub R_XTENSA_TLS_TPOFF = 53;
        pub R_XTENSA_TLS_FUNC = 54;
        pub R_XTENSA_TLS_ARG = 55;
        pub R_XTENSA_TLS_CALL = 56;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
