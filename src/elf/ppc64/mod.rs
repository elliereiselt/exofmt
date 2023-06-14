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
/// Unspecified or not using any features affected by the differences
pub const EF_PPC64_ABI_UNSPECIFIED: u32 = 0;
/// Original ABI using function descriptors
pub const EF_PPC64_ABI_ORIGINAL: u32 = 1;
/// Revised ABI without function descriptors
pub const EF_PPC64_ABI_REVISED: u32 = 2;
pub const EF_PPC64_ABI: u32 = 3;

// st_other
pub const STO_PPC64_LOCAL_BIT: u8 = 5;
pub const STO_PPC64_LOCAL_MASK: u8 = 7 << STO_PPC64_LOCAL_BIT;

pub fn sto_decode_local_entry_offset(st_other: u8) -> i64 {
    let st_other = u64::from(st_other & STO_PPC64_LOCAL_MASK) >> STO_PPC64_LOCAL_BIT;
    ((1 << st_other) >> 2) << 2
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_PPC64_NONE = 0;
        pub R_PPC64_ADDR32 = 1;
        pub R_PPC64_ADDR24 = 2;
        pub R_PPC64_ADDR16 = 3;
        pub R_PPC64_ADDR16_LO = 4;
        pub R_PPC64_ADDR16_HI = 5;
        pub R_PPC64_ADDR16_HA = 6;
        pub R_PPC64_ADDR14 = 7;
        pub R_PPC64_ADDR14_BRTAKEN = 8;
        pub R_PPC64_ADDR14_BRNTAKEN = 9;
        pub R_PPC64_REL24 = 10;
        pub R_PPC64_REL14 = 11;
        pub R_PPC64_REL14_BRTAKEN = 12;
        pub R_PPC64_REL14_BRNTAKEN = 13;
        pub R_PPC64_GOT16 = 14;
        pub R_PPC64_GOT16_LO = 15;
        pub R_PPC64_GOT16_HI = 16;
        pub R_PPC64_GOT16_HA = 17;
        pub R_PPC64_COPY = 19;
        pub R_PPC64_GLOB_DAT = 20;
        pub R_PPC64_JMP_SLOT = 21;
        pub R_PPC64_RELATIVE = 22;
        pub R_PPC64_REL32 = 26;
        pub R_PPC64_ADDR64 = 38;
        pub R_PPC64_ADDR16_HIGHER = 39;
        pub R_PPC64_ADDR16_HIGHERA = 40;
        pub R_PPC64_ADDR16_HIGHEST = 41;
        pub R_PPC64_ADDR16_HIGHESTA = 42;
        pub R_PPC64_REL64 = 44;
        pub R_PPC64_TOC16 = 47;
        pub R_PPC64_TOC16_LO = 48;
        pub R_PPC64_TOC16_HI = 49;
        pub R_PPC64_TOC16_HA = 50;
        pub R_PPC64_TOC = 51;
        pub R_PPC64_ADDR16_DS = 56;
        pub R_PPC64_ADDR16_LO_DS = 57;
        pub R_PPC64_GOT16_DS = 58;
        pub R_PPC64_GOT16_LO_DS = 59;
        pub R_PPC64_TOC16_DS = 63;
        pub R_PPC64_TOC16_LO_DS = 64;
        pub R_PPC64_TLS = 67;
        pub R_PPC64_DTPMOD64 = 68;
        pub R_PPC64_TPREL16 = 69;
        pub R_PPC64_TPREL16_LO = 70;
        pub R_PPC64_TPREL16_HI = 71;
        pub R_PPC64_TPREL16_HA = 72;
        pub R_PPC64_TPREL64 = 73;
        pub R_PPC64_DTPREL16 = 74;
        pub R_PPC64_DTPREL16_LO = 75;
        pub R_PPC64_DTPREL16_HI = 76;
        pub R_PPC64_DTPREL16_HA = 77;
        pub R_PPC64_DTPREL64 = 78;
        pub R_PPC64_GOT_TLSGD16 = 79;
        pub R_PPC64_GOT_TLSGD16_LO = 80;
        pub R_PPC64_GOT_TLSGD16_HI = 81;
        pub R_PPC64_GOT_TLSGD16_HA = 82;
        pub R_PPC64_GOT_TLSLD16 = 83;
        pub R_PPC64_GOT_TLSLD16_LO = 84;
        pub R_PPC64_GOT_TLSLD16_HI = 85;
        pub R_PPC64_GOT_TLSLD16_HA = 86;
        pub R_PPC64_GOT_TPREL16_DS = 87;
        pub R_PPC64_GOT_TPREL16_LO_DS = 88;
        pub R_PPC64_GOT_TPREL16_HI = 89;
        pub R_PPC64_GOT_TPREL16_HA = 90;
        pub R_PPC64_GOT_DTPREL16_DS = 91;
        pub R_PPC64_GOT_DTPREL16_LO_DS = 92;
        pub R_PPC64_GOT_DTPREL16_HI = 93;
        pub R_PPC64_GOT_DTPREL16_HA = 94;
        pub R_PPC64_TPREL16_DS = 95;
        pub R_PPC64_TPREL16_LO_DS = 96;
        pub R_PPC64_TPREL16_HIGHER = 97;
        pub R_PPC64_TPREL16_HIGHERA = 98;
        pub R_PPC64_TPREL16_HIGHEST = 99;
        pub R_PPC64_TPREL16_HIGHESTA = 100;
        pub R_PPC64_DTPREL16_DS = 101;
        pub R_PPC64_DTPREL16_LO_DS = 102;
        pub R_PPC64_DTPREL16_HIGHER = 103;
        pub R_PPC64_DTPREL16_HIGHERA = 104;
        pub R_PPC64_DTPREL16_HIGHEST = 105;
        pub R_PPC64_DTPREL16_HIGHESTA = 106;
        pub R_PPC64_TLSGD = 107;
        pub R_PPC64_TLSLD = 108;
        pub R_PPC64_ADDR16_HIGH = 110;
        pub R_PPC64_ADDR16_HIGHA = 111;
        pub R_PPC64_TPREL16_HIGH = 112;
        pub R_PPC64_TPREL16_HIGHA = 113;
        pub R_PPC64_DTPREL16_HIGH = 114;
        pub R_PPC64_DTPREL16_HIGHA = 115;
        pub R_PPC64_REL24_NOTOC = 116;
        pub R_PPC64_PCREL_OPT = 123;
        pub R_PPC64_PCREL34 = 132;
        pub R_PPC64_GOT_PCREL34 = 133;
        pub R_PPC64_TPREL34 = 146;
        pub R_PPC64_DTPREL34 = 147;
        pub R_PPC64_GOT_TLSGD_PCREL34 = 148;
        pub R_PPC64_GOT_TLSLD_PCREL34 = 149;
        pub R_PPC64_GOT_TPREL_PCREL34 = 150;
        pub R_PPC64_IRELATIVE = 248;
        pub R_PPC64_REL16 = 249;
        pub R_PPC64_REL16_LO = 250;
        pub R_PPC64_REL16_HI = 251;
        pub R_PPC64_REL16_HA = 252;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        /// Address of 32 bytes before the first glink lazy resolver stub.
        pub DT_PPC64_GLINK = 0x70000000;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
