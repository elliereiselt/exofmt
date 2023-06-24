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
pub const EF_RISCV_RVC: u32 = 0x0001;
pub const EF_RISCV_FLOAT_ABI: u32 = 0x0006;
pub const EF_RISCV_FLOAT_ABI_SOFT: u32 = 0x0000;
pub const EF_RISCV_FLOAT_ABI_SINGLE: u32 = 0x0002;
pub const EF_RISCV_FLOAT_ABI_DOUBLE: u32 = 0x0004;
pub const EF_RISCV_FLOAT_ABI_QUAD: u32 = 0x0006;
pub const EF_RISCV_RVE: u32 = 0x0008;
pub const EF_RISCV_TSO: u32 = 0x0010;

// st_other
/// Symbol may follow different calling convention than the standard calling convention
pub const STO_RISCV_VARIANT_CC: u8 = 0x80;

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// RISCV program header types.
        pub PT_RISCV_ATTRIBUTES = 0x70000003;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        pub SHT_RISCV_ATTRIBUTES = 0x70000003;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_RISCV_NONE = 0;
        pub R_RISCV_32 = 1;
        pub R_RISCV_64 = 2;
        pub R_RISCV_RELATIVE = 3;
        pub R_RISCV_COPY = 4;
        pub R_RISCV_JUMP_SLOT = 5;
        pub R_RISCV_TLS_DTPMOD32 = 6;
        pub R_RISCV_TLS_DTPMOD64 = 7;
        pub R_RISCV_TLS_DTPREL32 = 8;
        pub R_RISCV_TLS_DTPREL64 = 9;
        pub R_RISCV_TLS_TPREL32 = 10;
        pub R_RISCV_TLS_TPREL64 = 11;
        pub R_RISCV_BRANCH = 16;
        pub R_RISCV_JAL = 17;
        pub R_RISCV_CALL = 18;
        pub R_RISCV_CALL_PLT = 19;
        pub R_RISCV_GOT_HI20 = 20;
        pub R_RISCV_TLS_GOT_HI20 = 21;
        pub R_RISCV_TLS_GD_HI20 = 22;
        pub R_RISCV_PCREL_HI20 = 23;
        pub R_RISCV_PCREL_LO12_I = 24;
        pub R_RISCV_PCREL_LO12_S = 25;
        pub R_RISCV_HI20 = 26;
        pub R_RISCV_LO12_I = 27;
        pub R_RISCV_LO12_S = 28;
        pub R_RISCV_TPREL_HI20 = 29;
        pub R_RISCV_TPREL_LO12_I = 30;
        pub R_RISCV_TPREL_LO12_S = 31;
        pub R_RISCV_TPREL_ADD = 32;
        pub R_RISCV_ADD8 = 33;
        pub R_RISCV_ADD16 = 34;
        pub R_RISCV_ADD32 = 35;
        pub R_RISCV_ADD64 = 36;
        pub R_RISCV_SUB8 = 37;
        pub R_RISCV_SUB16 = 38;
        pub R_RISCV_SUB32 = 39;
        pub R_RISCV_SUB64 = 40;
        pub R_RISCV_GNU_VTINHERIT = 41;
        pub R_RISCV_GNU_VTENTRY = 42;
        pub R_RISCV_ALIGN = 43;
        pub R_RISCV_RVC_BRANCH = 44;
        pub R_RISCV_RVC_JUMP = 45;
        pub R_RISCV_RVC_LUI = 46;
        pub R_RISCV_RELAX = 51;
        pub R_RISCV_SUB6 = 52;
        pub R_RISCV_SET6 = 53;
        pub R_RISCV_SET8 = 54;
        pub R_RISCV_SET16 = 55;
        pub R_RISCV_SET32 = 56;
        pub R_RISCV_32_PCREL = 57;
        pub R_RISCV_IRELATIVE = 58;
        pub R_RISCV_PLT32 = 59;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        pub DT_RISCV_VARIANT_CC = 0x70000001;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
