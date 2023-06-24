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
/// Legacy pre EABI_VER5
pub const EF_ARM_SOFT_FLOAT: u32 = 0x00000200;
/// EABI_VER5
pub const EF_ARM_ABI_FLOAT_SOFT: u32 = 0x00000200;
/// Legacy pre EABI_VER5
pub const EF_ARM_VFP_FLOAT: u32 = 0x00000400;
/// EABI_VER5
pub const EF_ARM_ABI_FLOAT_HARD: u32 = 0x00000400;
pub const EF_ARM_BE8: u32 = 0x00800000;
pub const EF_ARM_EABI_UNKNOWN: u32 = 0x00000000;
pub const EF_ARM_EABI_VER1: u32 = 0x01000000;
pub const EF_ARM_EABI_VER2: u32 = 0x02000000;
pub const EF_ARM_EABI_VER3: u32 = 0x03000000;
pub const EF_ARM_EABI_VER4: u32 = 0x04000000;
pub const EF_ARM_EABI_VER5: u32 = 0x05000000;
pub const EF_ARM_EABIMASK: u32 = 0xFF000000;

// OS ABI types
option_stringable_consts_block! {
    const stringable: u8 {
        /// ARM
        pub OSABI_ARM = 97;
    }

    const ignore: u8 {}

    pub fn osabi_to_str(value: u8) -> Option<&'static str>;
}

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Contains stack unwind tables
        pub PT_ARM_ARCHEXT = 0x70000000;
        /// Contains stack unwind tables
        pub PT_ARM_UNWIND = 0x70000001;
    }

    const ignore: u32 {
        /// Contains stack unwind tables
        pub PT_ARM_EXIDX = 0x70000001;
    }

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Exception Index table
        pub SHT_ARM_EXIDX = 0x70000001;
        /// BPABI DLL dynamic linking pre-emption map
        pub SHT_ARM_PREEMPTMAP = 0x70000002;
        /// Object file compatibility attribute
        pub SHT_ARM_ATTRIBUTES = 0x70000003;
        /// Object file compatibility attribute
        pub SHT_ARM_DEBUGOVERLAY = 0x70000004;
        /// Object file compatibility attribute
        pub SHT_ARM_OVERLAYSECTION = 0x70000005;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Section flags
/// Make code section unreadable when in execute-only mode
pub const SHF_ARM_PURECODE: u32 = 0x20000000;

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_ARM_VFP = 0x400;
        pub NT_ARM_TLS = 0x401;
        pub NT_ARM_HW_BREAK = 0x402;
        pub NT_ARM_HW_WATCH = 0x403;
        pub NT_ARM_SVE = 0x405;
        pub NT_ARM_PAC_MASK = 0x406;
        pub NT_ARM_SSVE = 0x40b;
        pub NT_ARM_ZA = 0x40c;
        pub NT_ARM_ZT = 0x40d;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Symbol types
option_stringable_consts_block! {
    const stringable: u8 {
        /// A THUMB function.
        /// This is not defined in the ARM ELF Specification but is used by the GNU toolchain
        pub STT_ARM_TFUNC = 13; // TODO: Actually... should this be moved to GNU?
    }

    const ignore: u8 {}

    pub fn stt_to_str(value: u8) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_ARM_NONE = 0x00;
        pub R_ARM_PC24 = 0x01;
        pub R_ARM_ABS32 = 0x02;
        pub R_ARM_REL32 = 0x03;
        pub R_ARM_LDR_PC_G0 = 0x04;
        pub R_ARM_ABS16 = 0x05;
        pub R_ARM_ABS12 = 0x06;
        pub R_ARM_THM_ABS5 = 0x07;
        pub R_ARM_ABS8 = 0x08;
        pub R_ARM_SBREL32 = 0x09;
        pub R_ARM_THM_CALL = 0x0a;
        pub R_ARM_THM_PC8 = 0x0b;
        pub R_ARM_BREL_ADJ = 0x0c;
        pub R_ARM_TLS_DESC = 0x0d;
        pub R_ARM_THM_SWI8 = 0x0e;
        pub R_ARM_XPC25 = 0x0f;
        pub R_ARM_THM_XPC22 = 0x10;
        pub R_ARM_TLS_DTPMOD32 = 0x11;
        pub R_ARM_TLS_DTPOFF32 = 0x12;
        pub R_ARM_TLS_TPOFF32 = 0x13;
        pub R_ARM_COPY = 0x14;
        pub R_ARM_GLOB_DAT = 0x15;
        pub R_ARM_JUMP_SLOT = 0x16;
        pub R_ARM_RELATIVE = 0x17;
        pub R_ARM_GOTOFF32 = 0x18;
        pub R_ARM_BASE_PREL = 0x19;
        pub R_ARM_GOT_BREL = 0x1a;
        pub R_ARM_PLT32 = 0x1b;
        pub R_ARM_CALL = 0x1c;
        pub R_ARM_JUMP24 = 0x1d;
        pub R_ARM_THM_JUMP24 = 0x1e;
        pub R_ARM_BASE_ABS = 0x1f;
        pub R_ARM_ALU_PCREL_7_0 = 0x20;
        pub R_ARM_ALU_PCREL_15_8 = 0x21;
        pub R_ARM_ALU_PCREL_23_15 = 0x22;
        pub R_ARM_LDR_SBREL_11_0_NC = 0x23;
        pub R_ARM_ALU_SBREL_19_12_NC = 0x24;
        pub R_ARM_ALU_SBREL_27_20_CK = 0x25;
        pub R_ARM_TARGET1 = 0x26;
        pub R_ARM_SBREL31 = 0x27;
        pub R_ARM_V4BX = 0x28;
        pub R_ARM_TARGET2 = 0x29;
        pub R_ARM_PREL31 = 0x2a;
        pub R_ARM_MOVW_ABS_NC = 0x2b;
        pub R_ARM_MOVT_ABS = 0x2c;
        pub R_ARM_MOVW_PREL_NC = 0x2d;
        pub R_ARM_MOVT_PREL = 0x2e;
        pub R_ARM_THM_MOVW_ABS_NC = 0x2f;
        pub R_ARM_THM_MOVT_ABS = 0x30;
        pub R_ARM_THM_MOVW_PREL_NC = 0x31;
        pub R_ARM_THM_MOVT_PREL = 0x32;
        pub R_ARM_THM_JUMP19 = 0x33;
        pub R_ARM_THM_JUMP6 = 0x34;
        pub R_ARM_THM_ALU_PREL_11_0 = 0x35;
        pub R_ARM_THM_PC12 = 0x36;
        pub R_ARM_ABS32_NOI = 0x37;
        pub R_ARM_REL32_NOI = 0x38;
        pub R_ARM_ALU_PC_G0_NC = 0x39;
        pub R_ARM_ALU_PC_G0 = 0x3a;
        pub R_ARM_ALU_PC_G1_NC = 0x3b;
        pub R_ARM_ALU_PC_G1 = 0x3c;
        pub R_ARM_ALU_PC_G2 = 0x3d;
        pub R_ARM_LDR_PC_G1 = 0x3e;
        pub R_ARM_LDR_PC_G2 = 0x3f;
        pub R_ARM_LDRS_PC_G0 = 0x40;
        pub R_ARM_LDRS_PC_G1 = 0x41;
        pub R_ARM_LDRS_PC_G2 = 0x42;
        pub R_ARM_LDC_PC_G0 = 0x43;
        pub R_ARM_LDC_PC_G1 = 0x44;
        pub R_ARM_LDC_PC_G2 = 0x45;
        pub R_ARM_ALU_SB_G0_NC = 0x46;
        pub R_ARM_ALU_SB_G0 = 0x47;
        pub R_ARM_ALU_SB_G1_NC = 0x48;
        pub R_ARM_ALU_SB_G1 = 0x49;
        pub R_ARM_ALU_SB_G2 = 0x4a;
        pub R_ARM_LDR_SB_G0 = 0x4b;
        pub R_ARM_LDR_SB_G1 = 0x4c;
        pub R_ARM_LDR_SB_G2 = 0x4d;
        pub R_ARM_LDRS_SB_G0 = 0x4e;
        pub R_ARM_LDRS_SB_G1 = 0x4f;
        pub R_ARM_LDRS_SB_G2 = 0x50;
        pub R_ARM_LDC_SB_G0 = 0x51;
        pub R_ARM_LDC_SB_G1 = 0x52;
        pub R_ARM_LDC_SB_G2 = 0x53;
        pub R_ARM_MOVW_BREL_NC = 0x54;
        pub R_ARM_MOVT_BREL = 0x55;
        pub R_ARM_MOVW_BREL = 0x56;
        pub R_ARM_THM_MOVW_BREL_NC = 0x57;
        pub R_ARM_THM_MOVT_BREL = 0x58;
        pub R_ARM_THM_MOVW_BREL = 0x59;
        pub R_ARM_TLS_GOTDESC = 0x5a;
        pub R_ARM_TLS_CALL = 0x5b;
        pub R_ARM_TLS_DESCSEQ = 0x5c;
        pub R_ARM_THM_TLS_CALL = 0x5d;
        pub R_ARM_PLT32_ABS = 0x5e;
        pub R_ARM_GOT_ABS = 0x5f;
        pub R_ARM_GOT_PREL = 0x60;
        pub R_ARM_GOT_BREL12 = 0x61;
        pub R_ARM_GOTOFF12 = 0x62;
        pub R_ARM_GOTRELAX = 0x63;
        pub R_ARM_GNU_VTENTRY = 0x64;
        pub R_ARM_GNU_VTINHERIT = 0x65;
        pub R_ARM_THM_JUMP11 = 0x66;
        pub R_ARM_THM_JUMP8 = 0x67;
        pub R_ARM_TLS_GD32 = 0x68;
        pub R_ARM_TLS_LDM32 = 0x69;
        pub R_ARM_TLS_LDO32 = 0x6a;
        pub R_ARM_TLS_IE32 = 0x6b;
        pub R_ARM_TLS_LE32 = 0x6c;
        pub R_ARM_TLS_LDO12 = 0x6d;
        pub R_ARM_TLS_LE12 = 0x6e;
        pub R_ARM_TLS_IE12GP = 0x6f;
        pub R_ARM_PRIVATE_0 = 0x70;
        pub R_ARM_PRIVATE_1 = 0x71;
        pub R_ARM_PRIVATE_2 = 0x72;
        pub R_ARM_PRIVATE_3 = 0x73;
        pub R_ARM_PRIVATE_4 = 0x74;
        pub R_ARM_PRIVATE_5 = 0x75;
        pub R_ARM_PRIVATE_6 = 0x76;
        pub R_ARM_PRIVATE_7 = 0x77;
        pub R_ARM_PRIVATE_8 = 0x78;
        pub R_ARM_PRIVATE_9 = 0x79;
        pub R_ARM_PRIVATE_10 = 0x7a;
        pub R_ARM_PRIVATE_11 = 0x7b;
        pub R_ARM_PRIVATE_12 = 0x7c;
        pub R_ARM_PRIVATE_13 = 0x7d;
        pub R_ARM_PRIVATE_14 = 0x7e;
        pub R_ARM_PRIVATE_15 = 0x7f;
        pub R_ARM_ME_TOO = 0x80;
        pub R_ARM_THM_TLS_DESCSEQ16 = 0x81;
        pub R_ARM_THM_TLS_DESCSEQ32 = 0x82;
        pub R_ARM_THM_BF16 = 0x88;
        pub R_ARM_THM_BF12 = 0x89;
        pub R_ARM_THM_BF18 = 0x8a;
        pub R_ARM_IRELATIVE = 0xa0;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
