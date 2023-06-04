use crate::stringable_consts_blocks::option_stringable_consts_block;

// e_flags
/// Don't reorder instructions
pub const EF_MIPS_NOREORDER: u32 = 0x00000001;
/// Position independent code
pub const EF_MIPS_PIC: u32 = 0x00000002;
/// Call object with Position independent code
pub const EF_MIPS_CPIC: u32 = 0x00000004;
/// File uses N32 ABI
pub const EF_MIPS_ABI2: u32 = 0x00000020;
/// Code compiled for a 64-bit machi in 32-bit mode
pub const EF_MIPS_32BITMODE: u32 = 0x00000100;
/// Code compiled for a 32-bit machine but uses 64-bit FP registers
pub const EF_MIPS_FP64: u32 = 0x00000200;
/// Uses IEE 754-2008 NaN encoding
pub const EF_MIPS_NAN2008: u32 = 0x00000400;

// st_other
option_stringable_consts_block! {
    const stringable: u8 {
        /// Symbol whose definition is optional
        pub STO_MIPS_OPTIONAL = 0x04;
        /// PLT entry related dynamic table record
        pub STO_MIPS_PLT = 0x08;
        /// PIC func in an object mixes PIC/non-PIC
        pub STO_MIPS_PIC = 0x20;
        /// MIPS Specific ISA for MicroMips
        pub STO_MIPS_MICROMIPS = 0x80;
        /// MIPS Specific ISA for Mips16
        pub STO_MIPS_MIPS16 = 0xf0;
    }

    const ignore: u8 {}

    pub fn sto_to_str(value: u8) -> Option<&'static str>;
}

// Values for `kind' field in options
option_stringable_consts_block! {
    const stringable: u8 {
        /// Undefined
        pub ODK_NULL = 0;
        /// Register usage information
        pub ODK_REGINFO = 1;
        /// Exception processing options
        pub ODK_EXCEPTIONS = 2;
        /// Section padding options
        pub ODK_PAD = 3;
        /// Hardware patches applied
        pub ODK_HWPATCH = 4;
        /// Linker fill value
        pub ODK_FILL = 5;
        /// Space for tool identification
        pub ODK_TAGS = 6;
        /// Hardware AND patches applied
        pub ODK_HWAND = 7;
        /// Hardware OR patches applied
        pub ODK_HWOR = 8;
        /// GP group to use for text/data sections
        pub ODK_GP_GROUP = 9;
        /// ID information
        pub ODK_IDENT = 10;
        /// Page size information
        pub ODK_PAGESIZE = 11;
    }

    const ignore: u8 {}

    pub fn odk_to_str(value: u8) -> Option<&'static str>;
}

// Values for `info' in options for ODK_EXCEPTIONS entries
option_stringable_consts_block! {
    const stringable: u32 {
        /// FPE's which MUST be enabled
        pub OEX_FPU_MIN = 0x1f;
        pub OEX_FPU_INVAL = 0x10;
        pub OEX_FPU_DIV0 = 0x08;
        pub OEX_FPU_OFLO = 0x04;
        pub OEX_FPU_UFLO = 0x02;
        pub OEX_FPU_INEX = 0x01;
        /// FPE's which MAY be enabled
        pub OEX_FPU_MAX = 0x1f00;
        /// page zero must be mapped
        pub OEX_PAGE0 = 0x10000;
        /// Force sequential memory mode?  */
        pub OEX_SMM = 0x20000;
        /// Force floating point debug mode?  */
        pub OEX_FPDBUG = 0x40000;
        /// Dismiss invalid address faults?  */
        pub OEX_DISMISS = 0x80000;
    }

    const ignore: u32 {
        pub OEX_PRECISEFP = OEX_FPDBUG;
    }

    pub fn oex_to_str(value: u32) -> Option<&'static str>;
}

// Masks for `info' in options for an ODK_HWPATCH entry
/// R4000 end-of-page patch
pub const OHW_R4KEOP: u32 = 0x1;
/// may need R8000 prefetch patch
pub const OHW_R8KPFETCH: u32 = 0x2;
/// R5000 end-of-page patch
pub const OHW_R5KEOP: u32 = 0x4;
/// R5000 cvt.[ds].l bug.  clean=1
pub const OHW_R5KCVTL: u32 = 0x8;

pub const OPAD_PREFIX: u32 = 0x1;
pub const OPAD_POSTFIX: u32 = 0x2;
pub const OPAD_SYMBOL: u32 = 0x4;

// ABI flags
/// This file follows the first MIPS 32 bit ABI
pub const EF_MIPS_ABI_O32: u32 = 0x00001000;
/// O32 ABI extended for 64-bit architecture.
pub const EF_MIPS_ABI_O64: u32 = 0x00002000;
/// EABI in 32 bit mode.
pub const EF_MIPS_ABI_EABI32: u32 = 0x00003000;
/// EABI in 64 bit mode.
pub const EF_MIPS_ABI_EABI64: u32 = 0x00004000;
/// Mask for selecting EF_MIPS_ABI_ variant.
pub const EF_MIPS_ABI: u32 = 0x0000f000;

// MIPS machine variant
/// A standard MIPS implementation.
pub const EF_MIPS_MACH_NONE: u32 = 0x00000000;
/// Toshiba R3900
pub const EF_MIPS_MACH_3900: u32 = 0x00810000;
/// LSI R4010
pub const EF_MIPS_MACH_4010: u32 = 0x00820000;
/// NEC VR4100
pub const EF_MIPS_MACH_4100: u32 = 0x00830000;
/// MIPS R4650
pub const EF_MIPS_MACH_4650: u32 = 0x00850000;
/// NEC VR4120
pub const EF_MIPS_MACH_4120: u32 = 0x00870000;
/// NEC VR4111/VR4181
pub const EF_MIPS_MACH_4111: u32 = 0x00880000;
/// Broadcom SB-1
pub const EF_MIPS_MACH_SB1: u32 = 0x008a0000;
/// Cavium Networks Octeon
pub const EF_MIPS_MACH_OCTEON: u32 = 0x008b0000;
/// RMI Xlr
pub const EF_MIPS_MACH_XLR: u32 = 0x008c0000;
/// Cavium Networks Octeon2
pub const EF_MIPS_MACH_OCTEON2: u32 = 0x008d0000;
/// Cavium Networks Octeon3
pub const EF_MIPS_MACH_OCTEON3: u32 = 0x008e0000;
/// NEC VR5400
pub const EF_MIPS_MACH_5400: u32 = 0x00910000;
/// MIPS R5900
pub const EF_MIPS_MACH_5900: u32 = 0x00920000;
/// NEC VR5500
pub const EF_MIPS_MACH_5500: u32 = 0x00980000;
/// Unknown
pub const EF_MIPS_MACH_9000: u32 = 0x00990000;
/// ST Microelectronics Loongson 2E
pub const EF_MIPS_MACH_LS2E: u32 = 0x00a00000;
/// ST Microelectronics Loongson 2F
pub const EF_MIPS_MACH_LS2F: u32 = 0x00a10000;
/// Loongson 3A
pub const EF_MIPS_MACH_LS3A: u32 = 0x00a20000;
/// EF_MIPS_MACH_xxx selection mask
pub const EF_MIPS_MACH: u32 = 0x00ff0000;

// ARCH_ASE
/// microMIPS
pub const EF_MIPS_MICROMIPS: u32 = 0x02000000;
/// Has Mips-16 ISA extensions
pub const EF_MIPS_ARCH_ASE_M16: u32 = 0x04000000;
/// Has MDMX multimedia extensions
pub const EF_MIPS_ARCH_ASE_MDMX: u32 = 0x08000000;
/// Mask for EF_MIPS_ARCH_ASE_xxx flags
pub const EF_MIPS_ARCH_ASE: u32 = 0x0f000000;

// ARCH
/// MIPS1 instruction set
pub const EF_MIPS_ARCH_1: u32 = 0x00000000;
/// MIPS2 instruction set
pub const EF_MIPS_ARCH_2: u32 = 0x10000000;
/// MIPS3 instruction set
pub const EF_MIPS_ARCH_3: u32 = 0x20000000;
/// MIPS4 instruction set
pub const EF_MIPS_ARCH_4: u32 = 0x30000000;
/// MIPS5 instruction set
pub const EF_MIPS_ARCH_5: u32 = 0x40000000;
/// MIPS32 instruction set per linux not elf.h
pub const EF_MIPS_ARCH_32: u32 = 0x50000000;
/// MIPS64 instruction set per linux not elf.h
pub const EF_MIPS_ARCH_64: u32 = 0x60000000;
/// mips32r2, mips32r3, mips32r5
pub const EF_MIPS_ARCH_32R2: u32 = 0x70000000;
/// mips64r2, mips64r3, mips64r5
pub const EF_MIPS_ARCH_64R2: u32 = 0x80000000;
/// mips32r6
pub const EF_MIPS_ARCH_32R6: u32 = 0x90000000;
/// mips64r6
pub const EF_MIPS_ARCH_64R6: u32 = 0xa0000000;
/// Mask for applying EF_MIPS_ARCH_ variant
pub const EF_MIPS_ARCH: u32 = 0xf0000000;

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Register usage information.
        pub PT_MIPS_REGINFO = 0x70000000;
        /// Runtime procedure table.
        pub PT_MIPS_RTPROC = 0x70000001;
        /// Options segment.
        pub PT_MIPS_OPTIONS = 0x70000002;
        /// Abiflags segment.
        pub PT_MIPS_ABIFLAGS = 0x70000003;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Register usage information
        pub SHT_MIPS_REGINFO = 0x70000006;
        /// General options
        pub SHT_MIPS_OPTIONS = 0x7000000d;
        /// DWARF debugging section.
        pub SHT_MIPS_DWARF = 0x7000001e;
        /// ABI information.
        pub SHT_MIPS_ABIFLAGS = 0x7000002a;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Section flags
/// Section contains text/data which may be replicated in other sections.
/// Linker must retain only one copy.
pub const SHF_MIPS_NODUPES: u32 = 0x01000000;
/// Linker must generate implicit hidden weak names.
pub const SHF_MIPS_NAMES: u32 = 0x02000000;
/// Section data local to process.
pub const SHF_MIPS_LOCAL: u32 = 0x04000000;
/// Do not strip this section.
pub const SHF_MIPS_NOSTRIP: u32 = 0x08000000;
/// Section must be part of global data area.
pub const SHF_MIPS_GPREL: u32 = 0x10000000;
/// This section should be merged.
pub const SHF_MIPS_MERGE: u32 = 0x20000000;
/// Address size to be inferred from section entry size.
pub const SHF_MIPS_ADDR: u32 = 0x40000000;
/// Section data is string data by default.
pub const SHF_MIPS_STRING: u32 = 0x80000000;

// Special section indices
/// Common symbols which are defined and allocated
pub const SHN_MIPS_ACOMMON: u32 = 0xff00;
/// Not ABI compliant
pub const SHN_MIPS_TEXT: u32 = 0xff01;
/// Not ABI compliant
pub const SHN_MIPS_DATA: u32 = 0xff02;
/// Common symbols for global data area
pub const SHN_MIPS_SCOMMON: u32 = 0xff03;
/// Undefined symbols for global data area
pub const SHN_MIPS_SUNDEFINED: u32 = 0xff04;

// Special relocation symbols used in the MIPS64 ELF relocation entries
option_stringable_consts_block! {
    const stringable: u32 {
        /// None
        pub RSS_UNDEF = 0;
        /// Value of gp
        pub RSS_GP = 1;
        /// Value of gp used to create object being relocated
        pub RSS_GP0 = 2;
        /// Address of location being relocated
        pub RSS_LOC = 3;
    }

    const ignore: u32 {}

    pub fn rss_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_MIPS_NONE = 0;
        pub R_MIPS_16 = 1;
        pub R_MIPS_32 = 2;
        pub R_MIPS_REL32 = 3;
        pub R_MIPS_26 = 4;
        pub R_MIPS_HI16 = 5;
        pub R_MIPS_LO16 = 6;
        pub R_MIPS_GPREL16 = 7;
        pub R_MIPS_LITERAL = 8;
        pub R_MIPS_GOT16 = 9;
        pub R_MIPS_PC16 = 10;
        pub R_MIPS_CALL16 = 11;
        pub R_MIPS_GPREL32 = 12;
        pub R_MIPS_UNUSED1 = 13;
        pub R_MIPS_UNUSED2 = 14;
        pub R_MIPS_UNUSED3 = 15;
        pub R_MIPS_SHIFT5 = 16;
        pub R_MIPS_SHIFT6 = 17;
        pub R_MIPS_64 = 18;
        pub R_MIPS_GOT_DISP = 19;
        pub R_MIPS_GOT_PAGE = 20;
        pub R_MIPS_GOT_OFST = 21;
        pub R_MIPS_GOT_HI16 = 22;
        pub R_MIPS_GOT_LO16 = 23;
        pub R_MIPS_SUB = 24;
        pub R_MIPS_INSERT_A = 25;
        pub R_MIPS_INSERT_B = 26;
        pub R_MIPS_DELETE = 27;
        pub R_MIPS_HIGHER = 28;
        pub R_MIPS_HIGHEST = 29;
        pub R_MIPS_CALL_HI16 = 30;
        pub R_MIPS_CALL_LO16 = 31;
        pub R_MIPS_SCN_DISP = 32;
        pub R_MIPS_REL16 = 33;
        pub R_MIPS_ADD_IMMEDIATE = 34;
        pub R_MIPS_PJUMP = 35;
        pub R_MIPS_RELGOT = 36;
        pub R_MIPS_JALR = 37;
        pub R_MIPS_TLS_DTPMOD32 = 38;
        pub R_MIPS_TLS_DTPREL32 = 39;
        pub R_MIPS_TLS_DTPMOD64 = 40;
        pub R_MIPS_TLS_DTPREL64 = 41;
        pub R_MIPS_TLS_GD = 42;
        pub R_MIPS_TLS_LDM = 43;
        pub R_MIPS_TLS_DTPREL_HI16 = 44;
        pub R_MIPS_TLS_DTPREL_LO16 = 45;
        pub R_MIPS_TLS_GOTTPREL = 46;
        pub R_MIPS_TLS_TPREL32 = 47;
        pub R_MIPS_TLS_TPREL64 = 48;
        pub R_MIPS_TLS_TPREL_HI16 = 49;
        pub R_MIPS_TLS_TPREL_LO16 = 50;
        pub R_MIPS_GLOB_DAT = 51;
        pub R_MIPS_PC21_S2 = 60;
        pub R_MIPS_PC26_S2 = 61;
        pub R_MIPS_PC18_S3 = 62;
        pub R_MIPS_PC19_S2 = 63;
        pub R_MIPS_PCHI16 = 64;
        pub R_MIPS_PCLO16 = 65;
        pub R_MIPS16_26 = 100;
        pub R_MIPS16_GPREL = 101;
        pub R_MIPS16_GOT16 = 102;
        pub R_MIPS16_CALL16 = 103;
        pub R_MIPS16_HI16 = 104;
        pub R_MIPS16_LO16 = 105;
        pub R_MIPS16_TLS_GD = 106;
        pub R_MIPS16_TLS_LDM = 107;
        pub R_MIPS16_TLS_DTPREL_HI16 = 108;
        pub R_MIPS16_TLS_DTPREL_LO16 = 109;
        pub R_MIPS16_TLS_GOTTPREL = 110;
        pub R_MIPS16_TLS_TPREL_HI16 = 111;
        pub R_MIPS16_TLS_TPREL_LO16 = 112;
        pub R_MIPS_COPY = 126;
        pub R_MIPS_JUMP_SLOT = 127;
        pub R_MICROMIPS_26_S1 = 133;
        pub R_MICROMIPS_HI16 = 134;
        pub R_MICROMIPS_LO16 = 135;
        pub R_MICROMIPS_GPREL16 = 136;
        pub R_MICROMIPS_LITERAL = 137;
        pub R_MICROMIPS_GOT16 = 138;
        pub R_MICROMIPS_PC7_S1 = 139;
        pub R_MICROMIPS_PC10_S1 = 140;
        pub R_MICROMIPS_PC16_S1 = 141;
        pub R_MICROMIPS_CALL16 = 142;
        pub R_MICROMIPS_GOT_DISP = 145;
        pub R_MICROMIPS_GOT_PAGE = 146;
        pub R_MICROMIPS_GOT_OFST = 147;
        pub R_MICROMIPS_GOT_HI16 = 148;
        pub R_MICROMIPS_GOT_LO16 = 149;
        pub R_MICROMIPS_SUB = 150;
        pub R_MICROMIPS_HIGHER = 151;
        pub R_MICROMIPS_HIGHEST = 152;
        pub R_MICROMIPS_CALL_HI16 = 153;
        pub R_MICROMIPS_CALL_LO16 = 154;
        pub R_MICROMIPS_SCN_DISP = 155;
        pub R_MICROMIPS_JALR = 156;
        pub R_MICROMIPS_HI0_LO16 = 157;
        pub R_MICROMIPS_TLS_GD = 162;
        pub R_MICROMIPS_TLS_LDM = 163;
        pub R_MICROMIPS_TLS_DTPREL_HI16 = 164;
        pub R_MICROMIPS_TLS_DTPREL_LO16 = 165;
        pub R_MICROMIPS_TLS_GOTTPREL = 166;
        pub R_MICROMIPS_TLS_TPREL_HI16 = 169;
        pub R_MICROMIPS_TLS_TPREL_LO16 = 170;
        pub R_MICROMIPS_GPREL7_S2 = 172;
        pub R_MICROMIPS_PC23_S2 = 173;
        pub R_MICROMIPS_PC21_S1 = 174;
        pub R_MICROMIPS_PC26_S1 = 175;
        pub R_MICROMIPS_PC18_S3 = 176;
        pub R_MICROMIPS_PC19_S2 = 177;
        pub R_MIPS_NUM = 218;
        pub R_MIPS_PC32 = 248;
        pub R_MIPS_EH = 249;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        /// 32 bit version number for runtime linker interface.
        pub DT_MIPS_RLD_VERSION = 0x70000001;
        /// Time stamp.
        pub DT_MIPS_TIME_STAMP = 0x70000002;
        /// Checksum of external strings and common sizes.
        pub DT_MIPS_ICHECKSUM = 0x70000003;
        /// Index of version string in string table.
        pub DT_MIPS_IVERSION = 0x70000004;
        /// 32 bits of flags.
        pub DT_MIPS_FLAGS = 0x70000005;
        /// Base address of the segment.
        pub DT_MIPS_BASE_ADDRESS = 0x70000006;
        /// Address of .msym section.
        pub DT_MIPS_MSYM = 0x70000007;
        /// Address of .conflict section.
        pub DT_MIPS_CONFLICT = 0x70000008;
        /// Address of .liblist section.
        pub DT_MIPS_LIBLIST = 0x70000009;
        /// Number of local global offset table entries.
        pub DT_MIPS_LOCAL_GOTNO = 0x7000000a;
        /// Number of entries in the .conflict section.
        pub DT_MIPS_CONFLICTNO = 0x7000000b;
        /// Number of entries in the .liblist section.
        pub DT_MIPS_LIBLISTNO = 0x70000010;
        /// Number of entries in the .dynsym section.
        pub DT_MIPS_SYMTABNO = 0x70000011;
        /// Index of first external dynamic symbol not referenced locally.
        pub DT_MIPS_UNREFEXTNO = 0x70000012;
        /// Index of first dynamic symbol in global offset table.
        pub DT_MIPS_GOTSYM = 0x70000013;
        /// Number of page table entries in global offset table.
        pub DT_MIPS_HIPAGENO = 0x70000014;
        /// Address of run time loader map used for debugging.
        pub DT_MIPS_RLD_MAP = 0x70000016;
        /// Delta C++ class definition.
        pub DT_MIPS_DELTA_CLASS = 0x70000017;
        /// Number of entries in DT_MIPS_DELTA_CLASS.
        pub DT_MIPS_DELTA_CLASS_NO = 0x70000018;
        /// Delta C++ class instances.
        pub DT_MIPS_DELTA_INSTANCE = 0x70000019;
        /// Number of entries in DT_MIPS_DELTA_INSTANCE.
        pub DT_MIPS_DELTA_INSTANCE_NO = 0x7000001A;
        /// Delta relocations.
        pub DT_MIPS_DELTA_RELOC = 0x7000001B;
        /// Number of entries in DT_MIPS_DELTA_RELOC.
        pub DT_MIPS_DELTA_RELOC_NO = 0x7000001C;
        /// Delta symbols that Delta relocations refer to.
        pub DT_MIPS_DELTA_SYM = 0x7000001D;
        /// Number of entries in DT_MIPS_DELTA_SYM.
        pub DT_MIPS_DELTA_SYM_NO = 0x7000001E;
        /// Delta symbols that hold class declarations.
        pub DT_MIPS_DELTA_CLASSSYM = 0x70000020;
        /// Number of entries in DT_MIPS_DELTA_CLASSSYM.
        pub DT_MIPS_DELTA_CLASSSYM_NO = 0x70000021;
        /// Flags indicating information about C++ flavor.
        pub DT_MIPS_CXX_FLAGS = 0x70000022;
        /// Pixie information.
        pub DT_MIPS_PIXIE_INIT = 0x70000023;
        /// Address of .MIPS.symlib
        pub DT_MIPS_SYMBOL_LIB = 0x70000024;
        /// The GOT index of the first PTE for a segment
        pub DT_MIPS_LOCALPAGE_GOTIDX = 0x70000025;
        /// The GOT index of the first PTE for a local symbol
        pub DT_MIPS_LOCAL_GOTIDX = 0x70000026;
        /// The GOT index of the first PTE for a hidden symbol
        pub DT_MIPS_HIDDEN_GOTIDX = 0x70000027;
        /// The GOT index of the first PTE for a protected symbol
        pub DT_MIPS_PROTECTED_GOTIDX = 0x70000028;
        /// Address of `.MIPS.options'.
        pub DT_MIPS_OPTIONS = 0x70000029;
        /// Address of `.interface'.
        pub DT_MIPS_INTERFACE = 0x7000002A;
        /// Unknown.
        pub DT_MIPS_DYNSTR_ALIGN = 0x7000002B;
        /// Size of the .interface section.
        pub DT_MIPS_INTERFACE_SIZE = 0x7000002C;
        /// Size of rld_text_resolve function stored in the GOT.
        pub DT_MIPS_RLD_TEXT_RESOLVE_ADDR = 0x7000002D;
        /// Default suffix of DSO to be added by rld on dlopen() calls.
        pub DT_MIPS_PERF_SUFFIX = 0x7000002E;
        /// Size of compact relocation section (O32).
        pub DT_MIPS_COMPACT_SIZE = 0x7000002F;
        /// GP value for auxiliary GOTs.
        pub DT_MIPS_GP_VALUE = 0x70000030;
        /// Address of auxiliary .dynamic.
        pub DT_MIPS_AUX_DYNAMIC = 0x70000031;
        /// Address of the base of the PLTGOT.
        pub DT_MIPS_PLTGOT = 0x70000032;
        /// Points to the base of a writable PLT.
        pub DT_MIPS_RWPLT = 0x70000034;
        /// Relative offset of run time loader map, used for debugging.
        pub DT_MIPS_RLD_MAP_REL = 0x70000035;
        /// GNU-style hash table with xlat.
        pub DT_MIPS_XHASH = 0x70000036;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}

// DT_MIPS_FLAGS
/// No flags.
pub const RHF_NONE: i64 = 0x00000000;
/// Uses shortcut pointers.
pub const RHF_QUICKSTART: i64 = 0x00000001;
/// Hash size is not a power of two.
pub const RHF_NOTPOT: i64 = 0x00000002;
/// Ignore LD_LIBRARY_PATH.
pub const RHS_NO_LIBRARY_REPLACEMENT: i64 = 0x00000004;
/// DSO address may not be relocated.
pub const RHF_NO_MOVE: i64 = 0x00000008;
/// SGI specific features.
pub const RHF_SGI_ONLY: i64 = 0x00000010;
/// Guarantee that .init will finish executing before any non-init code in DSO is called.
pub const RHF_GUARANTEE_INIT: i64 = 0x00000020;
/// Contains Delta C++ code.
pub const RHF_DELTA_C_PLUS_PLUS: i64 = 0x00000040;
/// Guarantee that .init will start executing before any non-init code in DSO is called.
pub const RHF_GUARANTEE_START_INIT: i64 = 0x00000080;
/// Generated by pixie.
pub const RHF_PIXIE: i64 = 0x00000100;
/// Delay-load DSO by default.
pub const RHF_DEFAULT_DELAY_LOAD: i64 = 0x00000200;
/// Object may be requickstarted
pub const RHF_REQUICKSTART: i64 = 0x00000400;
/// Object has been requickstarted
pub const RHF_REQUICKSTARTED: i64 = 0x00000800;
/// Generated by cord.
pub const RHF_CORD: i64 = 0x00001000;
/// Object contains no unresolved undef symbols.
pub const RHF_NO_UNRES_UNDEF: i64 = 0x00002000;
/// Symbol table is in a safe order.
pub const RHF_RLD_ORDER_SAFE: i64 = 0x00004000;
