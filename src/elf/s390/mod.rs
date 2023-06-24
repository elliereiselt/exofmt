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

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_S390_HIGH_GPRS = 0x300;
        pub NT_S390_TIMER = 0x301;
        pub NT_S390_TODCMP = 0x302;
        pub NT_S390_TODPREG = 0x303;
        pub NT_S390_CTRS = 0x304;
        pub NT_S390_PREFIX = 0x305;
        pub NT_S390_LAST_BREAK = 0x306;
        pub NT_S390_SYSTEM_CALL = 0x307;
        pub NT_S390_TDB = 0x308;
        pub NT_S390_VXRS_LOW = 0x309;
        pub NT_S390_VXRS_HIGH = 0x30a;
        pub NT_S390_GS_CB = 0x30b;
        pub NT_S390_GS_BC = 0x30c;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_S390_NONE = 0;
        pub R_S390_8 = 1;
        pub R_S390_12 = 2;
        pub R_S390_16 = 3;
        pub R_S390_32 = 4;
        pub R_S390_PC32 = 5;
        pub R_S390_GOT12 = 6;
        pub R_S390_GOT32 = 7;
        pub R_S390_PLT32 = 8;
        pub R_S390_COPY = 9;
        pub R_S390_GLOB_DAT = 10;
        pub R_S390_JMP_SLOT = 11;
        pub R_S390_RELATIVE = 12;
        pub R_S390_GOTOFF = 13;
        pub R_S390_GOTPC = 14;
        pub R_S390_GOT16 = 15;
        pub R_S390_PC16 = 16;
        pub R_S390_PC16DBL = 17;
        pub R_S390_PLT16DBL = 18;
        pub R_S390_PC32DBL = 19;
        pub R_S390_PLT32DBL = 20;
        pub R_S390_GOTPCDBL = 21;
        pub R_S390_64 = 22;
        pub R_S390_PC64 = 23;
        pub R_S390_GOT64 = 24;
        pub R_S390_PLT64 = 25;
        pub R_S390_GOTENT = 26;
        pub R_S390_GOTOFF16 = 27;
        pub R_S390_GOTOFF64 = 28;
        pub R_S390_GOTPLT12 = 29;
        pub R_S390_GOTPLT16 = 30;
        pub R_S390_GOTPLT32 = 31;
        pub R_S390_GOTPLT64 = 32;
        pub R_S390_GOTPLTENT = 33;
        pub R_S390_PLTOFF16 = 34;
        pub R_S390_PLTOFF32 = 35;
        pub R_S390_PLTOFF64 = 36;
        pub R_S390_TLS_LOAD = 37;
        pub R_S390_TLS_GDCALL = 38;
        pub R_S390_TLS_LDCALL = 39;
        pub R_S390_TLS_GD32 = 40;
        pub R_S390_TLS_GD64 = 41;
        pub R_S390_TLS_GOTIE12 = 42;
        pub R_S390_TLS_GOTIE32 = 43;
        pub R_S390_TLS_GOTIE64 = 44;
        pub R_S390_TLS_LDM32 = 45;
        pub R_S390_TLS_LDM64 = 46;
        pub R_S390_TLS_IE32 = 47;
        pub R_S390_TLS_IE64 = 48;
        pub R_S390_TLS_IEENT = 49;
        pub R_S390_TLS_LE32 = 50;
        pub R_S390_TLS_LE64 = 51;
        pub R_S390_TLS_LDO32 = 52;
        pub R_S390_TLS_LDO64 = 53;
        pub R_S390_TLS_DTPMOD = 54;
        pub R_S390_TLS_DTPOFF = 55;
        pub R_S390_TLS_TPOFF = 56;
        pub R_S390_20 = 57;
        pub R_S390_GOT20 = 58;
        pub R_S390_GOTPLT20 = 59;
        pub R_S390_TLS_GOTIE20 = 60;
        pub R_S390_IRELATIVE = 61;
        pub R_S390_PC12DBL = 62;
        pub R_S390_PLT12DBL = 63;
        pub R_S390_PC24DBL = 64;
        pub R_S390_PLT24DBL = 65;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
