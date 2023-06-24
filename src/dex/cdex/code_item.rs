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
// NOTE: This entire file is based on the Compact Dex version 001

/// Max size of the pre-header in u16s
pub const CODE_ITEM_MAX_PREHEADER_SIZE: usize = 6;

/// Bit shift for the `registers_size` section of the compressed u16 value for [registers_size, ins_size, outs_size, tries_size]
pub const CODE_ITEM_REGISTERS_SIZE_SHIFT: usize = 12;
/// Bit shift for the `ins_size` section of the compressed u16 value for [registers_size, ins_size, outs_size, tries_size]
pub const CODE_ITEM_INS_SIZE_SHIFT: usize = 8;
/// Bit shift for the `ins_size` section of the compressed u16 value for [registers_size, ins_size, outs_size, tries_size]
pub const CODE_ITEM_OUTS_SIZE_SHIFT: usize = 4;
/// Bit shift for the `ins_size` section of the compressed u16 value for [registers_size, ins_size, outs_size, tries_size]
pub const CODE_ITEM_TRIES_SIZE_SHIFT: usize = 0;

/// Flag value to signify the need to extract `registers_size` from the preheader
pub const CODE_ITEM_FLAG_PREHEADER_REGISTER_SIZE: u16 = 0x1 << 0;
/// Flag value to signify the need to extract `ins_size` from the preheader
pub const CODE_ITEM_FLAG_PREHEADER_INS_SIZE: u16 = 0x1 << 1;
/// Flag value to signify the need to extract `outs_size` from the preheader
pub const CODE_ITEM_FLAG_PREHEADER_OUTS_SIZE: u16 = 0x1 << 2;
/// Flag value to signify the need to extract `tries_size` from the preheader
pub const CODE_ITEM_FLAG_PREHEADER_TRIES_SIZE: u16 = 0x1 << 3;
/// Flag value to signify the need to extract `insns_size` from the preheader
pub const CODE_ITEM_FLAG_PREHEADER_INSNS_SIZE: u16 = 0x01 << 4;

/// Bit shift for the `insns_size` section of the compressed u16 value for [flags, insns_count]
pub const CODE_ITEM_INSNS_SIZE_SHIFT: usize = 5;
/// Bit mask for the `insns_size` section of the compressed u16 value for [flags, insns_count]
pub const CODE_ITEM_INSNS_SIZE_BITS: u16 = 16 - CODE_ITEM_INSNS_SIZE_SHIFT as u16;
