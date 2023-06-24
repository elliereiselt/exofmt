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

// st_other
option_stringable_consts_block! {
    const stringable: u8 {
        /// Symbol may follow different calling convention than base PCS.
        pub STO_AARCH64_VARIANT_PCS = 0x80;
    }

    const ignore: u8 {}

    pub fn sto_to_str(value: u8) -> Option<&'static str>;
}

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// MTE memory tag segment type
        pub PT_AARCH64_MEMTAG_MTE = 0x70000002;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Special aarch64-specific sections for MTE support; as described in:
        /// https://github.com/ARM-software/abi-aa/blob/main/memtagabielf64/memtagabielf64.rst#7section-types
        pub SHT_AARCH64_MEMTAG_GLOBALS_STATIC = 0x70000007;
        pub SHT_AARCH64_MEMTAG_GLOBALS_DYNAMIC = 0x70000008;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_AARCH64_NONE = 0;
        pub R_AARCH64_ABS64 = 0x101;
        pub R_AARCH64_ABS32 = 0x102;
        pub R_AARCH64_ABS16 = 0x103;
        pub R_AARCH64_PREL64 = 0x104;
        pub R_AARCH64_PREL32 = 0x105;
        pub R_AARCH64_PREL16 = 0x106;
        pub R_AARCH64_MOVW_UABS_G0 = 0x107;
        pub R_AARCH64_MOVW_UABS_G0_NC = 0x108;
        pub R_AARCH64_MOVW_UABS_G1 = 0x109;
        pub R_AARCH64_MOVW_UABS_G1_NC = 0x10a;
        pub R_AARCH64_MOVW_UABS_G2 = 0x10b;
        pub R_AARCH64_MOVW_UABS_G2_NC = 0x10c;
        pub R_AARCH64_MOVW_UABS_G3 = 0x10d;
        pub R_AARCH64_MOVW_SABS_G0 = 0x10e;
        pub R_AARCH64_MOVW_SABS_G1 = 0x10f;
        pub R_AARCH64_MOVW_SABS_G2 = 0x110;
        pub R_AARCH64_LD_PREL_LO19 = 0x111;
        pub R_AARCH64_ADR_PREL_LO21 = 0x112;
        pub R_AARCH64_ADR_PREL_PG_HI21 = 0x113;
        pub R_AARCH64_ADR_PREL_PG_HI21_NC = 0x114;
        pub R_AARCH64_ADD_ABS_LO12_NC = 0x115;
        pub R_AARCH64_LDST8_ABS_LO12_NC = 0x116;
        pub R_AARCH64_TSTBR14 = 0x117;
        pub R_AARCH64_CONDBR19 = 0x118;
        pub R_AARCH64_JUMP26 = 0x11a;
        pub R_AARCH64_CALL26 = 0x11b;
        pub R_AARCH64_LDST16_ABS_LO12_NC = 0x11c;
        pub R_AARCH64_LDST32_ABS_LO12_NC = 0x11d;
        pub R_AARCH64_LDST64_ABS_LO12_NC = 0x11e;
        pub R_AARCH64_MOVW_PREL_G0 = 0x11f;
        pub R_AARCH64_MOVW_PREL_G0_NC = 0x120;
        pub R_AARCH64_MOVW_PREL_G1 = 0x121;
        pub R_AARCH64_MOVW_PREL_G1_NC = 0x122;
        pub R_AARCH64_MOVW_PREL_G2 = 0x123;
        pub R_AARCH64_MOVW_PREL_G2_NC = 0x124;
        pub R_AARCH64_MOVW_PREL_G3 = 0x125;
        pub R_AARCH64_LDST128_ABS_LO12_NC = 0x12b;
        pub R_AARCH64_MOVW_GOTOFF_G0 = 0x12c;
        pub R_AARCH64_MOVW_GOTOFF_G0_NC = 0x12d;
        pub R_AARCH64_MOVW_GOTOFF_G1 = 0x12e;
        pub R_AARCH64_MOVW_GOTOFF_G1_NC = 0x12f;
        pub R_AARCH64_MOVW_GOTOFF_G2 = 0x130;
        pub R_AARCH64_MOVW_GOTOFF_G2_NC = 0x131;
        pub R_AARCH64_MOVW_GOTOFF_G3 = 0x132;
        pub R_AARCH64_GOTREL64 = 0x133;
        pub R_AARCH64_GOTREL32 = 0x134;
        pub R_AARCH64_GOT_LD_PREL19 = 0x135;
        pub R_AARCH64_LD64_GOTOFF_LO15 = 0x136;
        pub R_AARCH64_ADR_GOT_PAGE = 0x137;
        pub R_AARCH64_LD64_GOT_LO12_NC = 0x138;
        pub R_AARCH64_LD64_GOTPAGE_LO15 = 0x139;
        pub R_AARCH64_PLT32 = 0x13a;
        pub R_AARCH64_TLSGD_ADR_PREL21 = 0x200;
        pub R_AARCH64_TLSGD_ADR_PAGE21 = 0x201;
        pub R_AARCH64_TLSGD_ADD_LO12_NC = 0x202;
        pub R_AARCH64_TLSGD_MOVW_G1 = 0x203;
        pub R_AARCH64_TLSGD_MOVW_G0_NC = 0x204;
        pub R_AARCH64_TLSLD_ADR_PREL21 = 0x205;
        pub R_AARCH64_TLSLD_ADR_PAGE21 = 0x206;
        pub R_AARCH64_TLSLD_ADD_LO12_NC = 0x207;
        pub R_AARCH64_TLSLD_MOVW_G1 = 0x208;
        pub R_AARCH64_TLSLD_MOVW_G0_NC = 0x209;
        pub R_AARCH64_TLSLD_LD_PREL19 = 0x20a;
        pub R_AARCH64_TLSLD_MOVW_DTPREL_G2 = 0x20b;
        pub R_AARCH64_TLSLD_MOVW_DTPREL_G1 = 0x20c;
        pub R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC = 0x20d;
        pub R_AARCH64_TLSLD_MOVW_DTPREL_G0 = 0x20e;
        pub R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC = 0x20f;
        pub R_AARCH64_TLSLD_ADD_DTPREL_HI12 = 0x210;
        pub R_AARCH64_TLSLD_ADD_DTPREL_LO12 = 0x211;
        pub R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC = 0x212;
        pub R_AARCH64_TLSLD_LDST8_DTPREL_LO12 = 0x213;
        pub R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC = 0x214;
        pub R_AARCH64_TLSLD_LDST16_DTPREL_LO12 = 0x215;
        pub R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC = 0x216;
        pub R_AARCH64_TLSLD_LDST32_DTPREL_LO12 = 0x217;
        pub R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC = 0x218;
        pub R_AARCH64_TLSLD_LDST64_DTPREL_LO12 = 0x219;
        pub R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC = 0x21a;
        pub R_AARCH64_TLSIE_MOVW_GOTTPREL_G1 = 0x21b;
        pub R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC = 0x21c;
        pub R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21 = 0x21d;
        pub R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC = 0x21e;
        pub R_AARCH64_TLSIE_LD_GOTTPREL_PREL19 = 0x21f;
        pub R_AARCH64_TLSLE_MOVW_TPREL_G2 = 0x220;
        pub R_AARCH64_TLSLE_MOVW_TPREL_G1 = 0x221;
        pub R_AARCH64_TLSLE_MOVW_TPREL_G1_NC = 0x222;
        pub R_AARCH64_TLSLE_MOVW_TPREL_G0 = 0x223;
        pub R_AARCH64_TLSLE_MOVW_TPREL_G0_NC = 0x224;
        pub R_AARCH64_TLSLE_ADD_TPREL_HI12 = 0x225;
        pub R_AARCH64_TLSLE_ADD_TPREL_LO12 = 0x226;
        pub R_AARCH64_TLSLE_ADD_TPREL_LO12_NC = 0x227;
        pub R_AARCH64_TLSLE_LDST8_TPREL_LO12 = 0x228;
        pub R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC = 0x229;
        pub R_AARCH64_TLSLE_LDST16_TPREL_LO12 = 0x22a;
        pub R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC = 0x22b;
        pub R_AARCH64_TLSLE_LDST32_TPREL_LO12 = 0x22c;
        pub R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC = 0x22d;
        pub R_AARCH64_TLSLE_LDST64_TPREL_LO12 = 0x22e;
        pub R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC = 0x22f;
        pub R_AARCH64_TLSDESC_LD_PREL19 = 0x230;
        pub R_AARCH64_TLSDESC_ADR_PREL21 = 0x231;
        pub R_AARCH64_TLSDESC_ADR_PAGE21 = 0x232;
        pub R_AARCH64_TLSDESC_LD64_LO12 = 0x233;
        pub R_AARCH64_TLSDESC_ADD_LO12 = 0x234;
        pub R_AARCH64_TLSDESC_OFF_G1 = 0x235;
        pub R_AARCH64_TLSDESC_OFF_G0_NC = 0x236;
        pub R_AARCH64_TLSDESC_LDR = 0x237;
        pub R_AARCH64_TLSDESC_ADD = 0x238;
        pub R_AARCH64_TLSDESC_CALL = 0x239;
        pub R_AARCH64_TLSLE_LDST128_TPREL_LO12 = 0x23a;
        pub R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC = 0x23b;
        pub R_AARCH64_TLSLD_LDST128_DTPREL_LO12 = 0x23c;
        pub R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC = 0x23d;
        // Dynamic relocations start
        pub R_AARCH64_COPY = 0x400;
        pub R_AARCH64_GLOB_DAT = 0x401;
        pub R_AARCH64_JUMP_SLOT = 0x402;
        pub R_AARCH64_RELATIVE = 0x403;
        // 0x404 and 0x405 are now R_AARCH64_TLS_IMPDEF1 and R_AARCH64_TLS_IMPDEF2
        // We follow GNU and define TLS_IMPDEF1 as TLS_DTPMOD64 and TLS_IMPDEF2 as
        // TLS_DTPREL64
        pub R_AARCH64_TLS_DTPMOD64 = 0x404;
        pub R_AARCH64_TLS_DTPREL64 = 0x405;
        pub R_AARCH64_TLS_TPREL64 = 0x406;
        pub R_AARCH64_TLSDESC = 0x407;
        pub R_AARCH64_IRELATIVE = 0x408;

        // pub R_AARCH64_P32_NONE = 0;
        pub R_AARCH64_P32_ABS32 = 0x001;
        pub R_AARCH64_P32_ABS16 = 0x002;
        pub R_AARCH64_P32_PREL32 = 0x003;
        pub R_AARCH64_P32_PREL16 = 0x004;
        pub R_AARCH64_P32_MOVW_UABS_G0 = 0x005;
        pub R_AARCH64_P32_MOVW_UABS_G0_NC = 0x006;
        pub R_AARCH64_P32_MOVW_UABS_G1 = 0x007;
        pub R_AARCH64_P32_MOVW_SABS_G0 = 0x008;
        pub R_AARCH64_P32_LD_PREL_LO19 = 0x009;
        pub R_AARCH64_P32_ADR_PREL_LO21 = 0x00a;
        pub R_AARCH64_P32_ADR_PREL_PG_HI21 = 0x00b;
        pub R_AARCH64_P32_ADD_ABS_LO12_NC = 0x00c;
        pub R_AARCH64_P32_LDST8_ABS_LO12_NC = 0x00d;
        pub R_AARCH64_P32_LDST16_ABS_LO12_NC = 0x00e;
        pub R_AARCH64_P32_LDST32_ABS_LO12_NC = 0x00f;
        pub R_AARCH64_P32_LDST64_ABS_LO12_NC = 0x010;
        pub R_AARCH64_P32_LDST128_ABS_LO12_NC = 0x011;
        pub R_AARCH64_P32_TSTBR14 = 0x012;
        pub R_AARCH64_P32_CONDBR19 = 0x013;
        pub R_AARCH64_P32_JUMP26 = 0x014;
        pub R_AARCH64_P32_CALL26 = 0x015;
        pub R_AARCH64_P32_MOVW_PREL_G0 = 0x016;
        pub R_AARCH64_P32_MOVW_PREL_G0_NC = 0x017;
        pub R_AARCH64_P32_MOVW_PREL_G1 = 0x018;
        pub R_AARCH64_P32_GOT_LD_PREL19 = 0x019;
        pub R_AARCH64_P32_ADR_GOT_PAGE = 0x01a;
        pub R_AARCH64_P32_LD32_GOT_LO12_NC = 0x01b;
        pub R_AARCH64_P32_LD32_GOTPAGE_LO14 = 0x01c;
        pub R_AARCH64_P32_PLT32 = 0x01d;
        pub R_AARCH64_P32_TLSGD_ADR_PREL21 = 0x050;
        pub R_AARCH64_P32_TLSGD_ADR_PAGE21 = 0x051;
        pub R_AARCH64_P32_TLSGD_ADD_LO12_NC = 0x052;
        pub R_AARCH64_P32_TLSLD_ADR_PREL21 = 0x053;
        pub R_AARCH64_P32_TLSLD_ADR_PAGE21 = 0x054;
        pub R_AARCH64_P32_TLSLD_ADD_LO12_NC = 0x055;
        pub R_AARCH64_P32_TLSLD_LD_PREL19 = 0x056;
        pub R_AARCH64_P32_TLSLD_MOVW_DTPREL_G1 = 0x057;
        pub R_AARCH64_P32_TLSLD_MOVW_DTPREL_G0 = 0x058;
        pub R_AARCH64_P32_TLSLD_MOVW_DTPREL_G0_NC = 0x059;
        pub R_AARCH64_P32_TLSLD_ADD_DTPREL_HI12 = 0x05a;
        pub R_AARCH64_P32_TLSLD_ADD_DTPREL_LO12 = 0x05b;
        pub R_AARCH64_P32_TLSLD_ADD_DTPREL_LO12_NC = 0x05c;
        pub R_AARCH64_P32_TLSLD_LDST8_DTPREL_LO12 = 0x05d;
        pub R_AARCH64_P32_TLSLD_LDST8_DTPREL_LO12_NC = 0x05e;
        pub R_AARCH64_P32_TLSLD_LDST16_DTPREL_LO12 = 0x05f;
        pub R_AARCH64_P32_TLSLD_LDST16_DTPREL_LO12_NC = 0x060;
        pub R_AARCH64_P32_TLSLD_LDST32_DTPREL_LO12 = 0x061;
        pub R_AARCH64_P32_TLSLD_LDST32_DTPREL_LO12_NC = 0x062;
        pub R_AARCH64_P32_TLSLD_LDST64_DTPREL_LO12 = 0x063;
        pub R_AARCH64_P32_TLSLD_LDST64_DTPREL_LO12_NC = 0x064;
        pub R_AARCH64_P32_TLSLD_LDST128_DTPREL_LO12 = 0x065;
        pub R_AARCH64_P32_TLSLD_LDST128_DTPREL_LO12_NC = 0x066;
        pub R_AARCH64_P32_TLSIE_ADR_GOTTPREL_PAGE21 = 0x067;
        pub R_AARCH64_P32_TLSIE_LD32_GOTTPREL_LO12_NC = 0x068;
        pub R_AARCH64_P32_TLSIE_LD_GOTTPREL_PREL19 = 0x069;
        pub R_AARCH64_P32_TLSLE_MOVW_TPREL_G1 = 0x06a;
        pub R_AARCH64_P32_TLSLE_MOVW_TPREL_G0 = 0x06b;
        pub R_AARCH64_P32_TLSLE_MOVW_TPREL_G0_NC = 0x06c;
        pub R_AARCH64_P32_TLSLE_ADD_TPREL_HI12 = 0x06d;
        pub R_AARCH64_P32_TLSLE_ADD_TPREL_LO12 = 0x06e;
        pub R_AARCH64_P32_TLSLE_ADD_TPREL_LO12_NC = 0x06f;
        pub R_AARCH64_P32_TLSLE_LDST8_TPREL_LO12 = 0x070;
        pub R_AARCH64_P32_TLSLE_LDST8_TPREL_LO12_NC = 0x071;
        pub R_AARCH64_P32_TLSLE_LDST16_TPREL_LO12 = 0x072;
        pub R_AARCH64_P32_TLSLE_LDST16_TPREL_LO12_NC = 0x073;
        pub R_AARCH64_P32_TLSLE_LDST32_TPREL_LO12 = 0x074;
        pub R_AARCH64_P32_TLSLE_LDST32_TPREL_LO12_NC = 0x075;
        pub R_AARCH64_P32_TLSLE_LDST64_TPREL_LO12 = 0x076;
        pub R_AARCH64_P32_TLSLE_LDST64_TPREL_LO12_NC = 0x077;
        pub R_AARCH64_P32_TLSLE_LDST128_TPREL_LO12 = 0x078;
        pub R_AARCH64_P32_TLSLE_LDST128_TPREL_LO12_NC = 0x079;
        pub R_AARCH64_P32_TLSDESC_LD_PREL19 = 0x07a;
        pub R_AARCH64_P32_TLSDESC_ADR_PREL21 = 0x07b;
        pub R_AARCH64_P32_TLSDESC_ADR_PAGE21 = 0x07c;
        pub R_AARCH64_P32_TLSDESC_LD32_LO12 = 0x07d;
        pub R_AARCH64_P32_TLSDESC_ADD_LO12 = 0x07e;
        pub R_AARCH64_P32_TLSDESC_CALL = 0x07f;
        // Dynamic relocations start
        pub R_AARCH64_P32_COPY = 0x0b4;
        pub R_AARCH64_P32_GLOB_DAT = 0x0b5;
        pub R_AARCH64_P32_JUMP_SLOT = 0x0b6;
        pub R_AARCH64_P32_RELATIVE = 0x0b7;
        pub R_AARCH64_P32_TLS_DTPREL = 0x0b8;
        pub R_AARCH64_P32_TLS_DTPMOD = 0x0b9;
        pub R_AARCH64_P32_TLS_TPREL = 0x0ba;
        pub R_AARCH64_P32_TLSDESC = 0x0bb;
        pub R_AARCH64_P32_IRELATIVE = 0x0bc;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        pub DT_AARCH64_BTI_PLT = 0x70000001;
        pub DT_AARCH64_PAC_PLT = 0x70000003;
        pub DT_AARCH64_VARIANT_PCS = 0x70000005;
        pub DT_AARCH64_MEMTAG_MODE = 0x70000009;
        pub DT_AARCH64_MEMTAG_HEAP = 0x7000000b;
        pub DT_AARCH64_MEMTAG_STACK = 0x7000000c;
        pub DT_AARCH64_MEMTAG_GLOBALS = 0x7000000d;
        pub DT_AARCH64_MEMTAG_GLOBALSSZ = 0x7000000f;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
