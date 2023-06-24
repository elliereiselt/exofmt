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

// e_flags
pub const EF_LOONGARCH_ABI_SOFT_FLOAT: u32 = 0x1;
pub const EF_LOONGARCH_ABI_SINGLE_FLOAT: u32 = 0x2;
pub const EF_LOONGARCH_ABI_DOUBLE_FLOAT: u32 = 0x3;
pub const EF_LOONGARCH_ABI_MODIFIER_MASK: u32 = 0x7;
pub const EF_LOONGARCH_OBJABI_V0: u32 = 0x0;
pub const EF_LOONGARCH_OBJABI_V1: u32 = 0x40;
pub const EF_LOONGARCH_OBJABI_MASK: u32 = 0xC0;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_LOONGARCH_NONE = 0;
        pub R_LOONGARCH_32 = 1;
        pub R_LOONGARCH_64 = 2;
        pub R_LOONGARCH_RELATIVE = 3;
        pub R_LOONGARCH_COPY = 4;
        pub R_LOONGARCH_JUMP_SLOT = 5;
        pub R_LOONGARCH_TLS_DTPMOD32 = 6;
        pub R_LOONGARCH_TLS_DTPMOD64 = 7;
        pub R_LOONGARCH_TLS_DTPREL32 = 8;
        pub R_LOONGARCH_TLS_DTPREL64 = 9;
        pub R_LOONGARCH_TLS_TPREL32 = 10;
        pub R_LOONGARCH_TLS_TPREL64 = 11;
        pub R_LOONGARCH_IRELATIVE = 12;
        pub R_LOONGARCH_MARK_LA = 20;
        pub R_LOONGARCH_MARK_PCREL = 21;
        pub R_LOONGARCH_SOP_PUSH_PCREL = 22;
        pub R_LOONGARCH_SOP_PUSH_ABSOLUTE = 23;
        pub R_LOONGARCH_SOP_PUSH_DUP = 24;
        pub R_LOONGARCH_SOP_PUSH_GPREL = 25;
        pub R_LOONGARCH_SOP_PUSH_TLS_TPREL = 26;
        pub R_LOONGARCH_SOP_PUSH_TLS_GOT = 27;
        pub R_LOONGARCH_SOP_PUSH_TLS_GD = 28;
        pub R_LOONGARCH_SOP_PUSH_PLT_PCREL = 29;
        pub R_LOONGARCH_SOP_ASSERT = 30;
        pub R_LOONGARCH_SOP_NOT = 31;
        pub R_LOONGARCH_SOP_SUB = 32;
        pub R_LOONGARCH_SOP_SL = 33;
        pub R_LOONGARCH_SOP_SR = 34;
        pub R_LOONGARCH_SOP_ADD = 35;
        pub R_LOONGARCH_SOP_AND = 36;
        pub R_LOONGARCH_SOP_IF_ELSE = 37;
        pub R_LOONGARCH_SOP_POP_32_S_10_5 = 38;
        pub R_LOONGARCH_SOP_POP_32_U_10_12 = 39;
        pub R_LOONGARCH_SOP_POP_32_S_10_12 = 40;
        pub R_LOONGARCH_SOP_POP_32_S_10_16 = 41;
        pub R_LOONGARCH_SOP_POP_32_S_10_16_S2 = 42;
        pub R_LOONGARCH_SOP_POP_32_S_5_20 = 43;
        pub R_LOONGARCH_SOP_POP_32_S_0_5_10_16_S2 = 44;
        pub R_LOONGARCH_SOP_POP_32_S_0_10_10_16_S2 = 45;
        pub R_LOONGARCH_SOP_POP_32_U = 46;
        pub R_LOONGARCH_ADD8 = 47;
        pub R_LOONGARCH_ADD16 = 48;
        pub R_LOONGARCH_ADD24 = 49;
        pub R_LOONGARCH_ADD32 = 50;
        pub R_LOONGARCH_ADD64 = 51;
        pub R_LOONGARCH_SUB8 = 52;
        pub R_LOONGARCH_SUB16 = 53;
        pub R_LOONGARCH_SUB24 = 54;
        pub R_LOONGARCH_SUB32 = 55;
        pub R_LOONGARCH_SUB64 = 56;
        pub R_LOONGARCH_GNU_VTINHERIT = 57;
        pub R_LOONGARCH_GNU_VTENTRY = 58;
        pub R_LOONGARCH_B16 = 64;
        pub R_LOONGARCH_B21 = 65;
        pub R_LOONGARCH_B26 = 66;
        pub R_LOONGARCH_ABS_HI20 = 67;
        pub R_LOONGARCH_ABS_LO12 = 68;
        pub R_LOONGARCH_ABS64_LO20 = 69;
        pub R_LOONGARCH_ABS64_HI12 = 70;
        pub R_LOONGARCH_PCALA_HI20 = 71;
        pub R_LOONGARCH_PCALA_LO12 = 72;
        pub R_LOONGARCH_PCALA64_LO20 = 73;
        pub R_LOONGARCH_PCALA64_HI12 = 74;
        pub R_LOONGARCH_GOT_PC_HI20 = 75;
        pub R_LOONGARCH_GOT_PC_LO12 = 76;
        pub R_LOONGARCH_GOT64_PC_LO20 = 77;
        pub R_LOONGARCH_GOT64_PC_HI12 = 78;
        pub R_LOONGARCH_GOT_HI20 = 79;
        pub R_LOONGARCH_GOT_LO12 = 80;
        pub R_LOONGARCH_GOT64_LO20 = 81;
        pub R_LOONGARCH_GOT64_HI12 = 82;
        pub R_LOONGARCH_TLS_LE_HI20 = 83;
        pub R_LOONGARCH_TLS_LE_LO12 = 84;
        pub R_LOONGARCH_TLS_LE64_LO20 = 85;
        pub R_LOONGARCH_TLS_LE64_HI12 = 86;
        pub R_LOONGARCH_TLS_IE_PC_HI20 = 87;
        pub R_LOONGARCH_TLS_IE_PC_LO12 = 88;
        pub R_LOONGARCH_TLS_IE64_PC_LO20 = 89;
        pub R_LOONGARCH_TLS_IE64_PC_HI12 = 90;
        pub R_LOONGARCH_TLS_IE_HI20 = 91;
        pub R_LOONGARCH_TLS_IE_LO12 = 92;
        pub R_LOONGARCH_TLS_IE64_LO20 = 93;
        pub R_LOONGARCH_TLS_IE64_HI12 = 94;
        pub R_LOONGARCH_TLS_LD_PC_HI20 = 95;
        pub R_LOONGARCH_TLS_LD_HI20 = 96;
        pub R_LOONGARCH_TLS_GD_PC_HI20 = 97;
        pub R_LOONGARCH_TLS_GD_HI20 = 98;
        pub R_LOONGARCH_32_PCREL = 99;
        pub R_LOONGARCH_RELAX = 100;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
