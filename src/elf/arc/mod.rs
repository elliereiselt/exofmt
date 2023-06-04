use crate::stringable_consts_blocks::option_stringable_consts_block;

// e_flags
pub const EF_ARC_MACH_MSK: u32 = 0x000000ff;
pub const EF_ARC_OSABI_MSK: u32 = 0x00000f00;
pub const EF_ARC_MACH_ARC600: u32 = 0x00000002;
pub const EF_ARC_MACH_ARC601: u32 = 0x00000004;
pub const EF_ARC_MACH_ARC700: u32 = 0x00000003;
pub const EF_ARC_CPU_ARCV2EM: u32 = 0x00000005;
pub const EF_ARC_CPU_ARCV2HS: u32 = 0x00000006;
pub const EF_ARC_OSABI_ORIG: u32 = 0x00000000;
pub const EF_ARC_OSABI_V2: u32 = 0x00000200;
pub const EF_ARC_OSABI_V3: u32 = 0x00000300;
pub const EF_ARC_OSABI_V4: u32 = 0x00000400;
pub const EF_ARC_PIC: u32 = 0x00000100;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_ARC_NONE = 0;
        pub R_ARC_8 = 1;
        pub R_ARC_16 = 2;
        pub R_ARC_24 = 3;
        pub R_ARC_32 = 4;
        pub R_ARC_N8 = 8;
        pub R_ARC_N16 = 9;
        pub R_ARC_N24 = 10;
        pub R_ARC_N32 = 11;
        pub R_ARC_SDA = 12;
        pub R_ARC_SECTOFF = 13;
        pub R_ARC_S21H_PCREL = 14;
        pub R_ARC_S21W_PCREL = 15;
        pub R_ARC_S25H_PCREL = 16;
        pub R_ARC_S25W_PCREL = 17;
        pub R_ARC_SDA32 = 18;
        pub R_ARC_SDA_LDST = 19;
        pub R_ARC_SDA_LDST1 = 20;
        pub R_ARC_SDA_LDST2 = 21;
        pub R_ARC_SDA16_LD = 22;
        pub R_ARC_SDA16_LD1 = 23;
        pub R_ARC_SDA16_LD2 = 24;
        pub R_ARC_S13_PCREL = 25;
        pub R_ARC_W = 26;
        pub R_ARC_32_ME = 27;
        pub R_ARC_32_ME_S = 105;
        pub R_ARC_N32_ME = 28;
        pub R_ARC_SECTOFF_ME = 29;
        pub R_ARC_SDA32_ME = 30;
        pub R_ARC_W_ME = 31;
        pub R_AC_SECTOFF_U8 = 35;
        pub R_AC_SECTOFF_U8_1 = 36;
        pub R_AC_SECTOFF_U8_2 = 37;
        pub R_AC_SECTOFF_S9 = 38;
        pub R_AC_SECTOFF_S9_1 = 39;
        pub R_AC_SECTOFF_S9_2 = 40;
        pub R_ARC_SECTOFF_ME_1 = 41;
        pub R_ARC_SECTOFF_ME_2 = 42;
        pub R_ARC_SECTOFF_1 = 43;
        pub R_ARC_SECTOFF_2 = 44;
        pub R_ARC_SDA_12 = 45;
        pub R_ARC_SDA16_ST2 = 48;
        pub R_ARC_32_PCREL = 49;
        pub R_ARC_PC32 = 50;
        pub R_ARC_GOT32 = 59;
        pub R_ARC_GOTPC32 = 51;
        pub R_ARC_PLT32 = 52;
        pub R_ARC_COPY = 53;
        pub R_ARC_GLOB_DAT = 54;
        pub R_ARC_JMP_SLOT = 55;
        pub R_ARC_RELATIVE = 56;
        pub R_ARC_GOTOFF = 57;
        pub R_ARC_GOTPC = 58;
        pub R_ARC_S21W_PCREL_PLT = 60;
        pub R_ARC_S25H_PCREL_PLT = 61;
        pub R_ARC_JLI_SECTOFF = 63;
        pub R_ARC_TLS_DTPMOD = 66;
        pub R_ARC_TLS_TPOFF = 68;
        pub R_ARC_TLS_GD_GOT = 69;
        pub R_ARC_TLS_GD_LD = 70;
        pub R_ARC_TLS_GD_CALL = 71;
        pub R_ARC_TLS_IE_GOT = 72;
        pub R_ARC_TLS_DTPOFF = 67;
        pub R_ARC_TLS_DTPOFF_S9 = 73;
        pub R_ARC_TLS_LE_S9 = 74;
        pub R_ARC_TLS_LE_32 = 75;
        pub R_ARC_S25W_PCREL_PLT = 76;
        pub R_ARC_S21H_PCREL_PLT = 77;
        pub R_ARC_NPS_CMEM16 = 78;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
