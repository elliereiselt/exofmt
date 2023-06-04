use crate::stringable_consts_blocks::option_stringable_consts_block;

// e_flags
// Object processor version flags, bits[11:0]
/// Hexagon V2
pub const EF_HEXAGON_MACH_V2: u32 = 0x00000001;
/// Hexagon V3
pub const EF_HEXAGON_MACH_V3: u32 = 0x00000002;
/// Hexagon V4
pub const EF_HEXAGON_MACH_V4: u32 = 0x00000003;
/// Hexagon V5
pub const EF_HEXAGON_MACH_V5: u32 = 0x00000004;
/// Hexagon V55
pub const EF_HEXAGON_MACH_V55: u32 = 0x00000005;
/// Hexagon V60
pub const EF_HEXAGON_MACH_V60: u32 = 0x00000060;
/// Hexagon V62
pub const EF_HEXAGON_MACH_V62: u32 = 0x00000062;
/// Hexagon V65
pub const EF_HEXAGON_MACH_V65: u32 = 0x00000065;
/// Hexagon V66
pub const EF_HEXAGON_MACH_V66: u32 = 0x00000066;
/// Hexagon V67
pub const EF_HEXAGON_MACH_V67: u32 = 0x00000067;
/// Hexagon V67T
pub const EF_HEXAGON_MACH_V67T: u32 = 0x00008067;
/// Hexagon V68
pub const EF_HEXAGON_MACH_V68: u32 = 0x00000068;
/// Hexagon V69
pub const EF_HEXAGON_MACH_V69: u32 = 0x00000069;
/// Hexagon V71
pub const EF_HEXAGON_MACH_V71: u32 = 0x00000071;
/// Hexagon V71T
pub const EF_HEXAGON_MACH_V71T: u32 = 0x00008071;
/// Hexagon V73
pub const EF_HEXAGON_MACH_V73: u32 = 0x00000073;
/// Hexagon V..
pub const EF_HEXAGON_MACH: u32 = 0x000003ff;

// Highest ISA version flags
/// Same as specified in bits[11:0] of e_flags
pub const EF_HEXAGON_ISA_MACH: u32 = 0x00000000;
/// Hexagon V2 ISA
pub const EF_HEXAGON_ISA_V2: u32 = 0x00000010;
/// Hexagon V3 ISA
pub const EF_HEXAGON_ISA_V3: u32 = 0x00000020;
/// Hexagon V4 ISA
pub const EF_HEXAGON_ISA_V4: u32 = 0x00000030;
/// Hexagon V5 ISA
pub const EF_HEXAGON_ISA_V5: u32 = 0x00000040;
/// Hexagon V55 ISA
pub const EF_HEXAGON_ISA_V55: u32 = 0x00000050;
/// Hexagon V60 ISA
pub const EF_HEXAGON_ISA_V60: u32 = 0x00000060;
/// Hexagon V62 ISA
pub const EF_HEXAGON_ISA_V62: u32 = 0x00000062;
/// Hexagon V65 ISA
pub const EF_HEXAGON_ISA_V65: u32 = 0x00000065;
/// Hexagon V66 ISA
pub const EF_HEXAGON_ISA_V66: u32 = 0x00000066;
/// Hexagon V67 ISA
pub const EF_HEXAGON_ISA_V67: u32 = 0x00000067;
/// Hexagon V68 ISA
pub const EF_HEXAGON_ISA_V68: u32 = 0x00000068;
/// Hexagon V69 ISA
pub const EF_HEXAGON_ISA_V69: u32 = 0x00000069;
/// Hexagon V71 ISA
pub const EF_HEXAGON_ISA_V71: u32 = 0x00000071;
/// Hexagon V73 ISA
pub const EF_HEXAGON_ISA_V73: u32 = 0x00000073;
/// Hexagon V75 ISA
pub const EF_HEXAGON_ISA_V75: u32 = 0x00000075;
/// Hexagon V.. ISA
pub const EF_HEXAGON_ISA: u32 = 0x000003ff;

// Section indexes for common small data
/// Other access sizes
pub const SHN_HEXAGON_SCOMMON: u32 = 0xff00;
/// Byte-sized access
pub const SHN_HEXAGON_SCOMMON_1: u32 = 0xff01;
/// Half-word-sized access
pub const SHN_HEXAGON_SCOMMON_2: u32 = 0xff02;
/// Word-sized access
pub const SHN_HEXAGON_SCOMMON_4: u32 = 0xff03;
/// Double-word-size access
pub const SHN_HEXAGON_SCOMMON_8: u32 = 0xff04;

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Link editor is to sort the entries in this section based on their sizes
        pub SHT_HEXAGON_ORDERED = 0x70000000;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Section flags
/// All sections with the GPREL flag are grouped into a global data area
/// for faster accesses
pub const SHF_HEXAGON_GPREL: u32 = 0x10000000;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_HEXAGON_NONE = 0;
        pub R_HEXAGON_B22_PCREL = 1;
        pub R_HEXAGON_B15_PCREL = 2;
        pub R_HEXAGON_B7_PCREL = 3;
        pub R_HEXAGON_LO16 = 4;
        pub R_HEXAGON_HI16 = 5;
        pub R_HEXAGON_32 = 6;
        pub R_HEXAGON_16 = 7;
        pub R_HEXAGON_8 = 8;
        pub R_HEXAGON_GPREL16_0 = 9;
        pub R_HEXAGON_GPREL16_1 = 10;
        pub R_HEXAGON_GPREL16_2 = 11;
        pub R_HEXAGON_GPREL16_3 = 12;
        pub R_HEXAGON_HL16 = 13;
        pub R_HEXAGON_B13_PCREL = 14;
        pub R_HEXAGON_B9_PCREL = 15;
        pub R_HEXAGON_B32_PCREL_X = 16;
        pub R_HEXAGON_32_6_X = 17;
        pub R_HEXAGON_B22_PCREL_X = 18;
        pub R_HEXAGON_B15_PCREL_X = 19;
        pub R_HEXAGON_B13_PCREL_X = 20;
        pub R_HEXAGON_B9_PCREL_X = 21;
        pub R_HEXAGON_B7_PCREL_X = 22;
        pub R_HEXAGON_16_X = 23;
        pub R_HEXAGON_12_X = 24;
        pub R_HEXAGON_11_X = 25;
        pub R_HEXAGON_10_X = 26;
        pub R_HEXAGON_9_X = 27;
        pub R_HEXAGON_8_X = 28;
        pub R_HEXAGON_7_X = 29;
        pub R_HEXAGON_6_X = 30;
        pub R_HEXAGON_32_PCREL = 31;
        pub R_HEXAGON_COPY = 32;
        pub R_HEXAGON_GLOB_DAT = 33;
        pub R_HEXAGON_JMP_SLOT = 34;
        pub R_HEXAGON_RELATIVE = 35;
        pub R_HEXAGON_PLT_B22_PCREL = 36;
        pub R_HEXAGON_GOTREL_LO16 = 37;
        pub R_HEXAGON_GOTREL_HI16 = 38;
        pub R_HEXAGON_GOTREL_32 = 39;
        pub R_HEXAGON_GOT_LO16 = 40;
        pub R_HEXAGON_GOT_HI16 = 41;
        pub R_HEXAGON_GOT_32 = 42;
        pub R_HEXAGON_GOT_16 = 43;
        pub R_HEXAGON_DTPMOD_32 = 44;
        pub R_HEXAGON_DTPREL_LO16 = 45;
        pub R_HEXAGON_DTPREL_HI16 = 46;
        pub R_HEXAGON_DTPREL_32 = 47;
        pub R_HEXAGON_DTPREL_16 = 48;
        pub R_HEXAGON_GD_PLT_B22_PCREL = 49;
        pub R_HEXAGON_GD_GOT_LO16 = 50;
        pub R_HEXAGON_GD_GOT_HI16 = 51;
        pub R_HEXAGON_GD_GOT_32 = 52;
        pub R_HEXAGON_GD_GOT_16 = 53;
        pub R_HEXAGON_IE_LO16 = 54;
        pub R_HEXAGON_IE_HI16 = 55;
        pub R_HEXAGON_IE_32 = 56;
        pub R_HEXAGON_IE_GOT_LO16 = 57;
        pub R_HEXAGON_IE_GOT_HI16 = 58;
        pub R_HEXAGON_IE_GOT_32 = 59;
        pub R_HEXAGON_IE_GOT_16 = 60;
        pub R_HEXAGON_TPREL_LO16 = 61;
        pub R_HEXAGON_TPREL_HI16 = 62;
        pub R_HEXAGON_TPREL_32 = 63;
        pub R_HEXAGON_TPREL_16 = 64;
        pub R_HEXAGON_6_PCREL_X = 65;
        pub R_HEXAGON_GOTREL_32_6_X = 66;
        pub R_HEXAGON_GOTREL_16_X = 67;
        pub R_HEXAGON_GOTREL_11_X = 68;
        pub R_HEXAGON_GOT_32_6_X = 69;
        pub R_HEXAGON_GOT_16_X = 70;
        pub R_HEXAGON_GOT_11_X = 71;
        pub R_HEXAGON_DTPREL_32_6_X = 72;
        pub R_HEXAGON_DTPREL_16_X = 73;
        pub R_HEXAGON_DTPREL_11_X = 74;
        pub R_HEXAGON_GD_GOT_32_6_X = 75;
        pub R_HEXAGON_GD_GOT_16_X = 76;
        pub R_HEXAGON_GD_GOT_11_X = 77;
        pub R_HEXAGON_IE_32_6_X = 78;
        pub R_HEXAGON_IE_16_X = 79;
        pub R_HEXAGON_IE_GOT_32_6_X = 80;
        pub R_HEXAGON_IE_GOT_16_X = 81;
        pub R_HEXAGON_IE_GOT_11_X = 82;
        pub R_HEXAGON_TPREL_32_6_X = 83;
        pub R_HEXAGON_TPREL_16_X = 84;
        pub R_HEXAGON_TPREL_11_X = 85;
        pub R_HEXAGON_LD_PLT_B22_PCREL = 86;
        pub R_HEXAGON_LD_GOT_LO16 = 87;
        pub R_HEXAGON_LD_GOT_HI16 = 88;
        pub R_HEXAGON_LD_GOT_32 = 89;
        pub R_HEXAGON_LD_GOT_16 = 90;
        pub R_HEXAGON_LD_GOT_32_6_X = 91;
        pub R_HEXAGON_LD_GOT_16_X = 92;
        pub R_HEXAGON_LD_GOT_11_X = 93;
        pub R_HEXAGON_23_REG = 94;
        pub R_HEXAGON_GD_PLT_B22_PCREL_X = 95;
        pub R_HEXAGON_GD_PLT_B32_PCREL_X = 96;
        pub R_HEXAGON_LD_PLT_B22_PCREL_X = 97;
        pub R_HEXAGON_LD_PLT_B32_PCREL_X = 98;
        pub R_HEXAGON_27_REG = 99;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        pub DT_HEXAGON_SYMSZ = 0x70000000;
        pub DT_HEXAGON_VER = 0x70000001;
        pub DT_HEXAGON_PLT = 0x70000002;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
