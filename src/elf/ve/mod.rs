use crate::stringable_consts_blocks::option_stringable_consts_block;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_VE_NONE = 0;
        pub R_VE_REFLONG = 1;
        pub R_VE_REFQUAD = 2;
        pub R_VE_SREL32 = 3;
        pub R_VE_HI32 = 4;
        pub R_VE_LO32 = 5;
        pub R_VE_PC_HI32 = 6;
        pub R_VE_PC_LO32 = 7;
        pub R_VE_GOT32 = 8;
        pub R_VE_GOT_HI32 = 9;
        pub R_VE_GOT_LO32 = 10;
        pub R_VE_GOTOFF32 = 11;
        pub R_VE_GOTOFF_HI32 = 12;
        pub R_VE_GOTOFF_LO32 = 13;
        pub R_VE_PLT32 = 14;
        pub R_VE_PLT_HI32 = 15;
        pub R_VE_PLT_LO32 = 16;
        pub R_VE_RELATIVE = 17;
        pub R_VE_GLOB_DAT = 18;
        pub R_VE_JUMP_SLOT = 19;
        pub R_VE_COPY = 20;
        pub R_VE_DTPMOD64 = 22;
        pub R_VE_DTPOFF64 = 23;
        pub R_VE_TPOFF64 = 24;
        pub R_VE_TLS_GD_HI32 = 25;
        pub R_VE_TLS_GD_LO32 = 26;
        pub R_VE_TLS_LD_HI32 = 27;
        pub R_VE_TLS_LD_LO32 = 28;
        pub R_VE_DTPOFF32 = 29;
        pub R_VE_TLS_IE_HI32 = 30;
        pub R_VE_TLS_IE_LO32 = 31;
        pub R_VE_TPOFF_HI32 = 32;
        pub R_VE_TPOFF_LO32 = 33;
        pub R_VE_TPOFF32 = 34;
        pub R_VE_CALL_HI32 = 35;
        pub R_VE_CALL_LO32 = 36;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
