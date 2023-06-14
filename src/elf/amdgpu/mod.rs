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

// ABI versions
option_stringable_consts_block! {
    const stringable: u32 {
        // ABI identification was not defined in V1, hence no ABIVERSION_AMDGPU_HSA_V1
        pub ABIVERSION_AMDGPU_HSA_V2 = 0;
        pub ABIVERSION_AMDGPU_HSA_V3 = 1;
        pub ABIVERSION_AMDGPU_HSA_V4 = 2;
        pub ABIVERSION_AMDGPU_HSA_V5 = 3;
    }

    const ignore: u32 {}

    pub fn abiversion_to_str(value: u32) -> Option<&'static str>;
}

// e_flags
/// Not specified processor.
pub const EF_AMDGPU_MACH_NONE: u32 = 0x000;

/// R600-based processors.
/// Radeon HD 2000/3000 Series (R600).
pub const EF_AMDGPU_MACH_R600_R600: u32 = 0x001;
pub const EF_AMDGPU_MACH_R600_R630: u32 = 0x002;
pub const EF_AMDGPU_MACH_R600_RS880: u32 = 0x003;
pub const EF_AMDGPU_MACH_R600_RV670: u32 = 0x004;
/// Radeon HD 4000 Series (R700).
pub const EF_AMDGPU_MACH_R600_RV710: u32 = 0x005;
pub const EF_AMDGPU_MACH_R600_RV730: u32 = 0x006;
pub const EF_AMDGPU_MACH_R600_RV770: u32 = 0x007;
/// Radeon HD 5000 Series (Evergreen).
pub const EF_AMDGPU_MACH_R600_CEDAR: u32 = 0x008;
pub const EF_AMDGPU_MACH_R600_CYPRESS: u32 = 0x009;
pub const EF_AMDGPU_MACH_R600_JUNIPER: u32 = 0x00a;
pub const EF_AMDGPU_MACH_R600_REDWOOD: u32 = 0x00b;
pub const EF_AMDGPU_MACH_R600_SUMO: u32 = 0x00c;
/// Radeon HD 6000 Series (Northern Islands).
pub const EF_AMDGPU_MACH_R600_BARTS: u32 = 0x00d;
pub const EF_AMDGPU_MACH_R600_CAICOS: u32 = 0x00e;
pub const EF_AMDGPU_MACH_R600_CAYMAN: u32 = 0x00f;
pub const EF_AMDGPU_MACH_R600_TURKS: u32 = 0x010;

/// Reserved for R600-based processors.
pub const EF_AMDGPU_MACH_R600_RESERVED_FIRST: u32 = 0x011;
pub const EF_AMDGPU_MACH_R600_RESERVED_LAST: u32 = 0x01f;

/// First/last R600-based processors.
pub const EF_AMDGPU_MACH_R600_FIRST: u32 = EF_AMDGPU_MACH_R600_R600;
pub const EF_AMDGPU_MACH_R600_LAST: u32 = EF_AMDGPU_MACH_R600_TURKS;

/// AMDGCN-based processors.
pub const EF_AMDGPU_MACH_AMDGCN_GFX600: u32 = 0x020;
pub const EF_AMDGPU_MACH_AMDGCN_GFX601: u32 = 0x021;
pub const EF_AMDGPU_MACH_AMDGCN_GFX700: u32 = 0x022;
pub const EF_AMDGPU_MACH_AMDGCN_GFX701: u32 = 0x023;
pub const EF_AMDGPU_MACH_AMDGCN_GFX702: u32 = 0x024;
pub const EF_AMDGPU_MACH_AMDGCN_GFX703: u32 = 0x025;
pub const EF_AMDGPU_MACH_AMDGCN_GFX704: u32 = 0x026;
pub const EF_AMDGPU_MACH_AMDGCN_RESERVED_0X27: u32 = 0x027;
pub const EF_AMDGPU_MACH_AMDGCN_GFX801: u32 = 0x028;
pub const EF_AMDGPU_MACH_AMDGCN_GFX802: u32 = 0x029;
pub const EF_AMDGPU_MACH_AMDGCN_GFX803: u32 = 0x02a;
pub const EF_AMDGPU_MACH_AMDGCN_GFX810: u32 = 0x02b;
pub const EF_AMDGPU_MACH_AMDGCN_GFX900: u32 = 0x02c;
pub const EF_AMDGPU_MACH_AMDGCN_GFX902: u32 = 0x02d;
pub const EF_AMDGPU_MACH_AMDGCN_GFX904: u32 = 0x02e;
pub const EF_AMDGPU_MACH_AMDGCN_GFX906: u32 = 0x02f;
pub const EF_AMDGPU_MACH_AMDGCN_GFX908: u32 = 0x030;
pub const EF_AMDGPU_MACH_AMDGCN_GFX909: u32 = 0x031;
pub const EF_AMDGPU_MACH_AMDGCN_GFX90C: u32 = 0x032;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1010: u32 = 0x033;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1011: u32 = 0x034;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1012: u32 = 0x035;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1030: u32 = 0x036;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1031: u32 = 0x037;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1032: u32 = 0x038;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1033: u32 = 0x039;
pub const EF_AMDGPU_MACH_AMDGCN_GFX602: u32 = 0x03a;
pub const EF_AMDGPU_MACH_AMDGCN_GFX705: u32 = 0x03b;
pub const EF_AMDGPU_MACH_AMDGCN_GFX805: u32 = 0x03c;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1035: u32 = 0x03d;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1034: u32 = 0x03e;
pub const EF_AMDGPU_MACH_AMDGCN_GFX90A: u32 = 0x03f;
pub const EF_AMDGPU_MACH_AMDGCN_GFX940: u32 = 0x040;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1100: u32 = 0x041;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1013: u32 = 0x042;
pub const EF_AMDGPU_MACH_AMDGCN_RESERVED_0X43: u32 = 0x043;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1103: u32 = 0x044;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1036: u32 = 0x045;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1101: u32 = 0x046;
pub const EF_AMDGPU_MACH_AMDGCN_GFX1102: u32 = 0x047;
pub const EF_AMDGPU_MACH_AMDGCN_RESERVED_0X48: u32 = 0x048;
pub const EF_AMDGPU_MACH_AMDGCN_RESERVED_0X49: u32 = 0x049;
pub const EF_AMDGPU_MACH_AMDGCN_RESERVED_0X4A: u32 = 0x04a;
pub const EF_AMDGPU_MACH_AMDGCN_GFX941: u32 = 0x04b;
pub const EF_AMDGPU_MACH_AMDGCN_GFX942: u32 = 0x04c;

/// First/last AMDGCN-based processors.
pub const EF_AMDGPU_MACH_AMDGCN_FIRST: u32 = EF_AMDGPU_MACH_AMDGCN_GFX600;
pub const EF_AMDGPU_MACH_AMDGCN_LAST: u32 = EF_AMDGPU_MACH_AMDGCN_GFX942;

/// Indicates if the "xnack" target feature is enabled for all code contained
/// in the object.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V2.
pub const EF_AMDGPU_FEATURE_XNACK_V2: u32 = 0x01;
/// Indicates if the trap handler is enabled for all code contained
/// in the object.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V2.
pub const EF_AMDGPU_FEATURE_TRAP_HANDLER_V2: u32 = 0x02;

/// Indicates if the "xnack" target feature is enabled for all code contained
/// in the object.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V3.
pub const EF_AMDGPU_FEATURE_XNACK_V3: u32 = 0x100;
/// Indicates if the "sramecc" target feature is enabled for all code
/// contained in the object.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V3.
pub const EF_AMDGPU_FEATURE_SRAMECC_V3: u32 = 0x200;

/// XNACK selection mask for EF_AMDGPU_FEATURE_XNACK_* values.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V4.
pub const EF_AMDGPU_FEATURE_XNACK_V4: u32 = 0x300;
/// XNACK is not supported.
pub const EF_AMDGPU_FEATURE_XNACK_UNSUPPORTED_V4: u32 = 0x000;
/// XNACK is any/default/unspecified.
pub const EF_AMDGPU_FEATURE_XNACK_ANY_V4: u32 = 0x100;
/// XNACK is off.
pub const EF_AMDGPU_FEATURE_XNACK_OFF_V4: u32 = 0x200;
/// XNACK is on.
pub const EF_AMDGPU_FEATURE_XNACK_ON_V4: u32 = 0x300;

/// SRAMECC selection mask for EF_AMDGPU_FEATURE_SRAMECC_* values.
///
/// Only valid for ELFOSABI_AMDGPU_HSA and ELFABIVERSION_AMDGPU_HSA_V4.
pub const EF_AMDGPU_FEATURE_SRAMECC_V4: u32 = 0xc00;
/// SRAMECC is not supported.
pub const EF_AMDGPU_FEATURE_SRAMECC_UNSUPPORTED_V4: u32 = 0x000;
/// SRAMECC is any/default/unspecified.
pub const EF_AMDGPU_FEATURE_SRAMECC_ANY_V4: u32 = 0x400;
/// SRAMECC is off.
pub const EF_AMDGPU_FEATURE_SRAMECC_OFF_V4: u32 = 0x800;
/// SRAMECC is on.
pub const EF_AMDGPU_FEATURE_SRAMECC_ON_V4: u32 = 0xc00;

// OS ABI types
option_stringable_consts_block! {
    const stringable: u8 {
        /// AMD HSA runtime
        pub OSABI_AMDGPU_HSA = 64;
        /// AMD PAL runtime
        pub OSABI_AMDGPU_PAL = 65;
        /// AMD GCN GPUs (GFX6+) for MESA runtime
        pub OSABI_AMDGPU_MESA3D = 66;
    }

    const ignore: u8 {}

    pub fn osabi_to_str(value: u8) -> Option<&'static str>;
}

// AMDGPU-specific section indices.
pub const SHN_AMDGPU_LDS: u32 = 0xff00;

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        // AMD vendor specific notes. (Code Object V2)
        pub NT_AMD_HSA_CODE_OBJECT_VERSION = 1;
        pub NT_AMD_HSA_HSAIL = 2;
        pub NT_AMD_HSA_ISA_VERSION = 3;
        // Note types with values between 4 and 9 (inclusive) are reserved.
        pub NT_AMD_HSA_METADATA = 10;
        pub NT_AMD_HSA_ISA_NAME = 11;
        pub NT_AMD_PAL_METADATA = 12;
    }

    const ignore: u32 {
        // AMDGPU vendor specific notes. (Code Object V3)
        // Note types with values between 0 and 31 (inclusive) are reserved.
        pub NT_AMDGPU_METADATA = 32;
    }

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Symbol types
option_stringable_consts_block! {
    const stringable: u8 {
        /// AMDGPU symbol types
        pub STT_AMDGPU_HSA_KERNEL = 10;
    }

    const ignore: u8 {

    }

    pub fn _to_str(value: u8) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_AMDGPU_NONE = 0;
        pub R_AMDGPU_ABS32_LO = 1;
        pub R_AMDGPU_ABS32_HI = 2;
        pub R_AMDGPU_ABS64 = 3;
        pub R_AMDGPU_REL32 = 4;
        pub R_AMDGPU_REL64 = 5;
        pub R_AMDGPU_ABS32 = 6;
        pub R_AMDGPU_GOTPCREL = 7;
        pub R_AMDGPU_GOTPCREL32_LO = 8;
        pub R_AMDGPU_GOTPCREL32_HI = 9;
        pub R_AMDGPU_REL32_LO = 10;
        pub R_AMDGPU_REL32_HI = 11;
        pub R_AMDGPU_RELATIVE64 = 13;
        pub R_AMDGPU_REL16 = 14;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
