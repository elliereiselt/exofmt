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
pub const EF_CSKY_801: u32 = 0xa;
pub const EF_CSKY_802: u32 = 0x10;
pub const EF_CSKY_803: u32 = 0x9;
pub const EF_CSKY_805: u32 = 0x11;
pub const EF_CSKY_807: u32 = 0x6;
pub const EF_CSKY_810: u32 = 0x8;
pub const EF_CSKY_860: u32 = 0xb;
pub const EF_CSKY_800: u32 = 0x1f;
pub const EF_CSKY_FLOAT: u32 = 0x2000;
pub const EF_CSKY_DSP: u32 = 0x4000;
pub const EF_CSKY_ABIV2: u32 = 0x20000000;
pub const EF_CSKY_EFV1: u32 = 0x1000000;
pub const EF_CSKY_EFV2: u32 = 0x2000000;
pub const EF_CSKY_EFV3: u32 = 0x3000000;

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        pub SHT_CSKY_ATTRIBUTES = 0x70000001;
    }

    const ignore: u32 {

    }

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_CKCORE_NONE = 0;
        pub R_CKCORE_ADDR32 = 1;
        pub R_CKCORE_PCREL_IMM8_4 = 2;
        pub R_CKCORE_PCREL_IMM11_2 = 3;
        pub R_CKCORE_PCREL_IMM4_2 = 4;
        pub R_CKCORE_PCREL32 = 5;
        pub R_CKCORE_PCREL_JSR_IMM11_2 = 6;
        pub R_CKCORE_GNU_VTINHERIT = 7;
        pub R_CKCORE_GNU_VTENTRY = 8;
        pub R_CKCORE_RELATIVE = 9;
        pub R_CKCORE_COPY = 10;
        pub R_CKCORE_GLOB_DAT = 11;
        pub R_CKCORE_JUMP_SLOT = 12;
        pub R_CKCORE_GOTOFF = 13;
        pub R_CKCORE_GOTPC = 14;
        pub R_CKCORE_GOT32 = 15;
        pub R_CKCORE_PLT32 = 16;
        pub R_CKCORE_ADDRGOT = 17;
        pub R_CKCORE_ADDRPLT = 18;
        pub R_CKCORE_PCREL_IMM26_2 = 19;
        pub R_CKCORE_PCREL_IMM16_2 = 20;
        pub R_CKCORE_PCREL_IMM16_4 = 21;
        pub R_CKCORE_PCREL_IMM10_2 = 22;
        pub R_CKCORE_PCREL_IMM10_4 = 23;
        pub R_CKCORE_ADDR_HI16 = 24;
        pub R_CKCORE_ADDR_LO16 = 25;
        pub R_CKCORE_GOTPC_HI16 = 26;
        pub R_CKCORE_GOTPC_LO16 = 27;
        pub R_CKCORE_GOTOFF_HI16 = 28;
        pub R_CKCORE_GOTOFF_LO16 = 29;
        pub R_CKCORE_GOT12 = 30;
        pub R_CKCORE_GOT_HI16 = 31;
        pub R_CKCORE_GOT_LO16 = 32;
        pub R_CKCORE_PLT12 = 33;
        pub R_CKCORE_PLT_HI16 = 34;
        pub R_CKCORE_PLT_LO16 = 35;
        pub R_CKCORE_ADDRGOT_HI16 = 36;
        pub R_CKCORE_ADDRGOT_LO16 = 37;
        pub R_CKCORE_ADDRPLT_HI16 = 38;
        pub R_CKCORE_ADDRPLT_LO16 = 39;
        pub R_CKCORE_PCREL_JSR_IMM26_2 = 40;
        pub R_CKCORE_TOFFSET_LO16 = 41;
        pub R_CKCORE_DOFFSET_LO16 = 42;
        pub R_CKCORE_PCREL_IMM18_2 = 43;
        pub R_CKCORE_DOFFSET_IMM18 = 44;
        pub R_CKCORE_DOFFSET_IMM18_2 = 45;
        pub R_CKCORE_DOFFSET_IMM18_4 = 46;
        pub R_CKCORE_GOTOFF_IMM18 = 47;
        pub R_CKCORE_GOT_IMM18_4 = 48;
        pub R_CKCORE_PLT_IMM18_4 = 49;
        pub R_CKCORE_PCREL_IMM7_4 = 50;
        pub R_CKCORE_TLS_LE32 = 51;
        pub R_CKCORE_TLS_IE32 = 52;
        pub R_CKCORE_TLS_GD32 = 53;
        pub R_CKCORE_TLS_LDM32 = 54;
        pub R_CKCORE_TLS_LDO32 = 55;
        pub R_CKCORE_TLS_DTPMOD32 = 56;
        pub R_CKCORE_TLS_DTPOFF32 = 57;
        pub R_CKCORE_TLS_TPOFF32 = 58;
        pub R_CKCORE_PCREL_FLRW_IMM8_4 = 59;
        pub R_CKCORE_NOJSRI = 60;
        pub R_CKCORE_CALLGRAPH = 61;
        pub R_CKCORE_IRELATIVE = 62;
        pub R_CKCORE_PCREL_BLOOP_IMM4_4 = 63;
        pub R_CKCORE_PCREL_BLOOP_IMM12_4 = 64;
        pub R_CKCORE_PCREL_VLRW_IMM12_1 = 65;
        pub R_CKCORE_PCREL_VLRW_IMM12_2 = 66;
        pub R_CKCORE_PCREL_VLRW_IMM12_4 = 67;
        pub R_CKCORE_PCREL_VLRW_IMM12_8 = 68;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
