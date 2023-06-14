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
pub const EF_AVR_ARCH_AVR1: u32 = 1;
pub const EF_AVR_ARCH_AVR2: u32 = 2;
pub const EF_AVR_ARCH_AVR25: u32 = 25;
pub const EF_AVR_ARCH_AVR3: u32 = 3;
pub const EF_AVR_ARCH_AVR31: u32 = 31;
pub const EF_AVR_ARCH_AVR35: u32 = 35;
pub const EF_AVR_ARCH_AVR4: u32 = 4;
pub const EF_AVR_ARCH_AVR5: u32 = 5;
pub const EF_AVR_ARCH_AVR51: u32 = 51;
pub const EF_AVR_ARCH_AVR6: u32 = 6;
pub const EF_AVR_ARCH_AVRTINY: u32 = 100;
pub const EF_AVR_ARCH_XMEGA1: u32 = 101;
pub const EF_AVR_ARCH_XMEGA2: u32 = 102;
pub const EF_AVR_ARCH_XMEGA3: u32 = 103;
pub const EF_AVR_ARCH_XMEGA4: u32 = 104;
pub const EF_AVR_ARCH_XMEGA5: u32 = 105;
pub const EF_AVR_ARCH_XMEGA6: u32 = 106;
pub const EF_AVR_ARCH_XMEGA7: u32 = 107;
/// EF_AVR_ARCH_xxx selection mask
pub const EF_AVR_ARCH_MASK: u32 = 0x7f;
/// The file is prepared for linker relaxation to be applied
pub const EF_AVR_LINKRELAX_PREPARED: u32 = 0x80;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_AVR_NONE = 0;
        pub R_AVR_32 = 1;
        pub R_AVR_7_PCREL = 2;
        pub R_AVR_13_PCREL = 3;
        pub R_AVR_16 = 4;
        pub R_AVR_16_PM = 5;
        pub R_AVR_LO8_LDI = 6;
        pub R_AVR_HI8_LDI = 7;
        pub R_AVR_HH8_LDI = 8;
        pub R_AVR_LO8_LDI_NEG = 9;
        pub R_AVR_HI8_LDI_NEG = 10;
        pub R_AVR_HH8_LDI_NEG = 11;
        pub R_AVR_LO8_LDI_PM = 12;
        pub R_AVR_HI8_LDI_PM = 13;
        pub R_AVR_HH8_LDI_PM = 14;
        pub R_AVR_LO8_LDI_PM_NEG = 15;
        pub R_AVR_HI8_LDI_PM_NEG = 16;
        pub R_AVR_HH8_LDI_PM_NEG = 17;
        pub R_AVR_CALL = 18;
        pub R_AVR_LDI = 19;
        pub R_AVR_6 = 20;
        pub R_AVR_6_ADIW = 21;
        pub R_AVR_MS8_LDI = 22;
        pub R_AVR_MS8_LDI_NEG = 23;
        pub R_AVR_LO8_LDI_GS = 24;
        pub R_AVR_HI8_LDI_GS = 25;
        pub R_AVR_8 = 26;
        pub R_AVR_8_LO8 = 27;
        pub R_AVR_8_HI8 = 28;
        pub R_AVR_8_HLO8 = 29;
        pub R_AVR_DIFF8 = 30;
        pub R_AVR_DIFF16 = 31;
        pub R_AVR_DIFF32 = 32;
        pub R_AVR_LDS_STS_16 = 33;
        pub R_AVR_PORT6 = 34;
        pub R_AVR_PORT5 = 35;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
