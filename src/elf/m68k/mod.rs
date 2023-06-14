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

option_stringable_consts_block! {
    const stringable: u32 {
        /// No reloc
        pub R_68K_NONE = 0;
        /// Direct 32 bit
        pub R_68K_32 = 1;
        /// Direct 16 bit
        pub R_68K_16 = 2;
        /// Direct 8 bit
        pub R_68K_8 = 3;
        /// PC relative 32 bit
        pub R_68K_PC32 = 4;
        /// PC relative 16 bit
        pub R_68K_PC16 = 5;
        /// PC relative 8 bit
        pub R_68K_PC8 = 6;
        /// 32 bit PC relative GOT entry
        pub R_68K_GOTPCREL32 = 7;
        /// 16 bit PC relative GOT entry
        pub R_68K_GOTPCREL16 = 8;
        /// 8 bit PC relative GOT entry
        pub R_68K_GOTPCREL8 = 9;
        /// 32 bit GOT offset
        pub R_68K_GOTOFF32 = 10;
        /// 16 bit GOT offset
        pub R_68K_GOTOFF16 = 11;
        /// 8 bit GOT offset
        pub R_68K_GOTOFF8 = 12;
        /// 32 bit PC relative PLT address
        pub R_68K_PLT32 = 13;
        /// 16 bit PC relative PLT address
        pub R_68K_PLT16 = 14;
        /// 8 bit PC relative PLT address
        pub R_68K_PLT8 = 15;
        /// 32 bit PLT offset
        pub R_68K_PLTOFF32 = 16;
        /// 16 bit PLT offset
        pub R_68K_PLTOFF16 = 17;
        /// 8 bit PLT offset
        pub R_68K_PLTOFF8 = 18;
        /// Copy symbol at runtime
        pub R_68K_COPY = 19;
        /// Create GOT entry
        pub R_68K_GLOB_DAT = 20;
        /// Create PLT entry
        pub R_68K_JMP_SLOT = 21;
        /// Adjust by program base
        pub R_68K_RELATIVE = 22;
        /// These are GNU extensions to enable C++ vtable garbage collection.
        pub R_68K_GNU_VTINHERIT = 23;
        pub R_68K_GNU_VTENTRY = 24;
        /// TLS static relocations.
        pub R_68K_TLS_GD32 = 25;
        pub R_68K_TLS_GD16 = 26;
        pub R_68K_TLS_GD8 = 27;
        pub R_68K_TLS_LDM32 = 28;
        pub R_68K_TLS_LDM16 = 29;
        pub R_68K_TLS_LDM8 = 30;
        pub R_68K_TLS_LDO32 = 31;
        pub R_68K_TLS_LDO16 = 32;
        pub R_68K_TLS_LDO8 = 33;
        pub R_68K_TLS_IE32 = 34;
        pub R_68K_TLS_IE16 = 35;
        pub R_68K_TLS_IE8 = 36;
        pub R_68K_TLS_LE32 = 37;
        pub R_68K_TLS_LE16 = 38;
        pub R_68K_TLS_LE8 = 39;
        pub R_68K_TLS_DTPMOD32 = 40;
        pub R_68K_TLS_DTPREL32 = 41;
        pub R_68K_TLS_TPREL32 = 42;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
