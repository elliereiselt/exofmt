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
pub const EF_MSP430_MACH_MSP430X11: u32 = 11;
pub const EF_MSP430_MACH_MSP430X11X1: u32 = 110;
pub const EF_MSP430_MACH_MSP430X12: u32 = 12;
pub const EF_MSP430_MACH_MSP430X13: u32 = 13;
pub const EF_MSP430_MACH_MSP430X14: u32 = 14;
pub const EF_MSP430_MACH_MSP430X15: u32 = 15;
pub const EF_MSP430_MACH_MSP430X16: u32 = 16;
pub const EF_MSP430_MACH_MSP430X20: u32 = 20;
pub const EF_MSP430_MACH_MSP430X22: u32 = 22;
pub const EF_MSP430_MACH_MSP430X23: u32 = 23;
pub const EF_MSP430_MACH_MSP430X24: u32 = 24;
pub const EF_MSP430_MACH_MSP430X26: u32 = 26;
pub const EF_MSP430_MACH_MSP430X31: u32 = 31;
pub const EF_MSP430_MACH_MSP430X32: u32 = 32;
pub const EF_MSP430_MACH_MSP430X33: u32 = 33;
pub const EF_MSP430_MACH_MSP430X41: u32 = 41;
pub const EF_MSP430_MACH_MSP430X42: u32 = 42;
pub const EF_MSP430_MACH_MSP430X43: u32 = 43;
pub const EF_MSP430_MACH_MSP430X44: u32 = 44;
pub const EF_MSP430_MACH_MSP430X: u32 = 45;
pub const EF_MSP430_MACH_MSP430X46: u32 = 46;
pub const EF_MSP430_MACH_MSP430X47: u32 = 47;
pub const EF_MSP430_MACH_MSP430X54: u32 = 54;

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        pub SHT_MSP430_ATTRIBUTES = 0x70000003;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_MSP430_NONE = 0;
        pub R_MSP430_32 = 1;
        pub R_MSP430_10_PCREL = 2;
        pub R_MSP430_16 = 3;
        pub R_MSP430_16_PCREL = 4;
        pub R_MSP430_16_BYTE = 5;
        pub R_MSP430_16_PCREL_BYTE = 6;
        pub R_MSP430_2X_PCREL = 7;
        pub R_MSP430_RL_PCREL = 8;
        pub R_MSP430_8 = 9;
        pub R_MSP430_SYM_DIFF = 10;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
