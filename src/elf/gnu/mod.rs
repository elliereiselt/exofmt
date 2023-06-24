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

pub mod common;

mod hash;
pub use hash::*;
mod version;
pub use version::*;

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Contains stack unwind tables
        pub PT_GNU_EH_FRAME = 0x6474e550;
        /// Indicates stack executability.
        pub PT_GNU_STACK = 0x6474e551;
        /// Read-only after relocation.
        pub PT_GNU_RELRO = 0x6474e552;
        /// .note.gnu.property notes sections.
        pub PT_GNU_PROPERTY = 0x6474e553;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Object attributes.
        pub SHT_GNU_ATTRIBUTES = 0x6ffffff5;
        /// GNU-style hash table.
        pub SHT_GNU_HASH = 0x6ffffff6;
        /// GNU version definitions.
        pub SHT_GNU_VERDEF = 0x6ffffffd;
        /// GNU version references.
        pub SHT_GNU_VERNEED = 0x6ffffffe;
        /// GNU symbol versions table.
        pub SHT_GNU_VERSYM = 0x6fffffff;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        /// The minimum ABI level. This is used by the dynamic linker to
        /// describe the minimal kernel version on which a shared library may
        /// be used. The value should be four words. Word 0 is an OS
        /// descriptor (see below). Word 1 is the major version of the ABI.
        /// Word 2 is the minor version. Word 3 is the subminor version.
        pub NT_GNU_ABI_TAG = 1;
        /// Hardware capabilities information. Word 0 is the number of
        /// entries. Word 1 is a bitmask of enabled entries. The rest of
        /// the descriptor is a series of entries, where each entry is a
        /// single byte followed by a nul terminated string. The byte gives
        /// the bit number to test if enabled in the bitmask.
        pub NT_GNU_HWCAP = 2;
        /// The build ID as set by the linker's --build-id option. The
        /// format of the descriptor depends on the build ID style.
        pub NT_GNU_BUILD_ID = 3;
        /// The version of gold used to link. The descriptor is just a
        /// string
        pub NT_GNU_GOLD_VERSION = 4;
        // Program property note, as described in "Linux Extensions to the gABI"
        pub NT_GNU_PROPERTY_TYPE_0 = 5;
        pub NT_GNU_BUILD_ATTRIBUTE_OPEN = 0x100;
        pub NT_GNU_BUILD_ATTRIBUTE_FUNC = 0x101;
        /// FDO .note.package notes as defined on https://systemd.io/ELF_PACKAGE_METADATA/
        pub FDO_PACKAGING_METADATA = 0xcafe1a7e;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Property types used in GNU_PROPERTY_TYPE_0 notes
pub const GNU_PROPERTY_STACK_SIZE: u32 = 1;
pub const GNU_PROPERTY_NO_COPY_ON_PROTECTED: u32 = 2;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 0xc0000000;
pub const GNU_PROPERTY_X86_FEATURE_1_AND: u32 = 0xc0000002;

pub const GNU_PROPERTY_X86_UINT32_OR_LO: u32 = 0xc0008000;
pub const GNU_PROPERTY_X86_FEATURE_2_NEEDED: u32 = GNU_PROPERTY_X86_UINT32_OR_LO + 1;
pub const GNU_PROPERTY_X86_ISA_1_NEEDED: u32 = GNU_PROPERTY_X86_UINT32_OR_LO + 2;

pub const GNU_PROPERTY_X86_UINT32_OR_AND_LO: u32 = 0xc0010000;
pub const GNU_PROPERTY_X86_FEATURE_2_USED: u32 = GNU_PROPERTY_X86_UINT32_OR_AND_LO + 1;
pub const GNU_PROPERTY_X86_ISA_1_USED: u32 = GNU_PROPERTY_X86_UINT32_OR_AND_LO + 2;

// aarch64 processor feature bits.
pub const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1 << 0;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_PAC: u32 = 1 << 1;

// x86 processor feature bits.
pub const GNU_PROPERTY_X86_FEATURE_1_IBT: u32 = 1 << 0;
pub const GNU_PROPERTY_X86_FEATURE_1_SHSTK: u32 = 1 << 1;

pub const GNU_PROPERTY_X86_FEATURE_2_X86: u32 = 1 << 0;
pub const GNU_PROPERTY_X86_FEATURE_2_X87: u32 = 1 << 1;
pub const GNU_PROPERTY_X86_FEATURE_2_MMX: u32 = 1 << 2;
pub const GNU_PROPERTY_X86_FEATURE_2_XMM: u32 = 1 << 3;
pub const GNU_PROPERTY_X86_FEATURE_2_YMM: u32 = 1 << 4;
pub const GNU_PROPERTY_X86_FEATURE_2_ZMM: u32 = 1 << 5;
pub const GNU_PROPERTY_X86_FEATURE_2_FXSR: u32 = 1 << 6;
pub const GNU_PROPERTY_X86_FEATURE_2_XSAVE: u32 = 1 << 7;
pub const GNU_PROPERTY_X86_FEATURE_2_XSAVEOPT: u32 = 1 << 8;
pub const GNU_PROPERTY_X86_FEATURE_2_XSAVEC: u32 = 1 << 9;

pub const GNU_PROPERTY_X86_ISA_1_BASELINE: u32 = 1 << 0;
pub const GNU_PROPERTY_X86_ISA_1_V2: u32 = 1 << 1;
pub const GNU_PROPERTY_X86_ISA_1_V3: u32 = 1 << 2;
pub const GNU_PROPERTY_X86_ISA_1_V4: u32 = 1 << 3;

// The ABI values which may appear in word 0 of a NT_GNU_ABI_TAG note.
option_stringable_consts_block! {
    const stringable: u32 {
        pub GNU_ABI_TAG_LINUX = 0;
        pub GNU_ABI_TAG_HURD = 1;
        pub GNU_ABI_TAG_SOLARIS = 2;
        pub GNU_ABI_TAG_FREEBSD = 3;
        pub GNU_ABI_TAG_NETBSD = 4;
        pub GNU_ABI_TAG_SYLLABLE = 5;
        pub GNU_ABI_TAG_NACL = 6;
    }

    const ignore: u32 {}

    pub fn gnu_abi_tag_to_str(value: u32) -> Option<&'static str>;
}

// Symbol bindings
option_stringable_consts_block! {
    const stringable: u8 {
        /// GNU-specific "unique" symbols. This is a very annoying and imo smelly flag that is a wart on the ELF format due to ELF expecting everything to be in the same "namespace", unlike MACH-O or PE. You'd be better off reading the documentation about this.
        pub STB_GNU_UNIQUE = 10;
    }

    const ignore: u8 {}

    pub fn stb_to_str(value: u8) -> Option<&'static str>;
}

// Symbol types
option_stringable_consts_block! {
    const stringable: u8 {
        /// GNU indirect function
        pub STT_GNU_IFUNC = 10;
    }

    const ignore: u8 {}

    pub fn stt_to_str(value: u8) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        /// Reference to the GNU hash table.
        pub DT_GNU_HASH = 0x6FFFFEF5;
        /// Location of PLT entry for TLS descriptor resolver calls.
        pub DT_TLSDESC_PLT = 0x6FFFFEF6;
        /// Location of GOT entry used by TLS descriptor resolver PLT entry.
        pub DT_TLSDESC_GOT = 0x6FFFFEF7;
        /// ELF32_Rela count.
        pub DT_RELACOUNT = 0x6FFFFFF9;
        /// ELF32_Rel count.
        pub DT_RELCOUNT = 0x6FFFFFFA;
        /// Flags_1.
        pub DT_FLAGS_1 = 0x6FFFFFFB;
        /// The address of .gnu.version section.
        pub DT_VERSYM = 0x6FFFFFF0;
        /// The address of the version definition table.
        pub DT_VERDEF = 0x6FFFFFFC;
        /// The number of entries in DT_VERDEF.
        pub DT_VERDEFNUM = 0x6FFFFFFD;
        /// The address of the version dependency table.
        pub DT_VERNEED = 0x6FFFFFFE;
        /// The number of entries in DT_VERNEED.
        pub DT_VERNEEDNUM = 0x6FFFFFFF;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
