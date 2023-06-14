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

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_SPARC_NONE = 0;
        pub R_SPARC_8 = 1;
        pub R_SPARC_16 = 2;
        pub R_SPARC_32 = 3;
        pub R_SPARC_DISP8 = 4;
        pub R_SPARC_DISP16 = 5;
        pub R_SPARC_DISP32 = 6;
        pub R_SPARC_WDISP30 = 7;
        pub R_SPARC_WDISP22 = 8;
        pub R_SPARC_HI22 = 9;
        pub R_SPARC_22 = 10;
        pub R_SPARC_13 = 11;
        pub R_SPARC_LO10 = 12;
        pub R_SPARC_GOT10 = 13;
        pub R_SPARC_GOT13 = 14;
        pub R_SPARC_GOT22 = 15;
        pub R_SPARC_PC10 = 16;
        pub R_SPARC_PC22 = 17;
        pub R_SPARC_WPLT30 = 18;
        pub R_SPARC_COPY = 19;
        pub R_SPARC_GLOB_DAT = 20;
        pub R_SPARC_JMP_SLOT = 21;
        pub R_SPARC_RELATIVE = 22;
        pub R_SPARC_UA32 = 23;
        pub R_SPARC_PLT32 = 24;
        pub R_SPARC_HIPLT22 = 25;
        pub R_SPARC_LOPLT10 = 26;
        pub R_SPARC_PCPLT32 = 27;
        pub R_SPARC_PCPLT22 = 28;
        pub R_SPARC_PCPLT10 = 29;
        pub R_SPARC_10 = 30;
        pub R_SPARC_11 = 31;
        pub R_SPARC_64 = 32;
        pub R_SPARC_OLO10 = 33;
        pub R_SPARC_HH22 = 34;
        pub R_SPARC_HM10 = 35;
        pub R_SPARC_LM22 = 36;
        pub R_SPARC_PC_HH22 = 37;
        pub R_SPARC_PC_HM10 = 38;
        pub R_SPARC_PC_LM22 = 39;
        pub R_SPARC_WDISP16 = 40;
        pub R_SPARC_WDISP19 = 41;
        pub R_SPARC_7 = 43;
        pub R_SPARC_5 = 44;
        pub R_SPARC_6 = 45;
        pub R_SPARC_DISP64 = 46;
        pub R_SPARC_PLT64 = 47;
        pub R_SPARC_HIX22 = 48;
        pub R_SPARC_LOX10 = 49;
        pub R_SPARC_H44 = 50;
        pub R_SPARC_M44 = 51;
        pub R_SPARC_L44 = 52;
        pub R_SPARC_REGISTER = 53;
        pub R_SPARC_UA64 = 54;
        pub R_SPARC_UA16 = 55;
        pub R_SPARC_TLS_GD_HI22 = 56;
        pub R_SPARC_TLS_GD_LO10 = 57;
        pub R_SPARC_TLS_GD_ADD = 58;
        pub R_SPARC_TLS_GD_CALL = 59;
        pub R_SPARC_TLS_LDM_HI22 = 60;
        pub R_SPARC_TLS_LDM_LO10 = 61;
        pub R_SPARC_TLS_LDM_ADD = 62;
        pub R_SPARC_TLS_LDM_CALL = 63;
        pub R_SPARC_TLS_LDO_HIX22 = 64;
        pub R_SPARC_TLS_LDO_LOX10 = 65;
        pub R_SPARC_TLS_LDO_ADD = 66;
        pub R_SPARC_TLS_IE_HI22 = 67;
        pub R_SPARC_TLS_IE_LO10 = 68;
        pub R_SPARC_TLS_IE_LD = 69;
        pub R_SPARC_TLS_IE_LDX = 70;
        pub R_SPARC_TLS_IE_ADD = 71;
        pub R_SPARC_TLS_LE_HIX22 = 72;
        pub R_SPARC_TLS_LE_LOX10 = 73;
        pub R_SPARC_TLS_DTPMOD32 = 74;
        pub R_SPARC_TLS_DTPMOD64 = 75;
        pub R_SPARC_TLS_DTPOFF32 = 76;
        pub R_SPARC_TLS_DTPOFF64 = 77;
        pub R_SPARC_TLS_TPOFF32 = 78;
        pub R_SPARC_TLS_TPOFF64 = 79;
        pub R_SPARC_GOTDATA_HIX22 = 80;
        pub R_SPARC_GOTDATA_LOX10 = 81;
        pub R_SPARC_GOTDATA_OP_HIX22 = 82;
        pub R_SPARC_GOTDATA_OP_LOX10 = 83;
        pub R_SPARC_GOTDATA_OP = 84;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
