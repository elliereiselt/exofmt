use crate::stringable_consts_blocks::option_stringable_consts_block;

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_386_TLS = 0x200;
        pub NT_386_IOPERM = 0x201;
        pub NT_X86_XSTATE = 0x202;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_386_NONE = 0;
        pub R_386_32 = 1;
        pub R_386_PC32 = 2;
        pub R_386_GOT32 = 3;
        pub R_386_PLT32 = 4;
        pub R_386_COPY = 5;
        pub R_386_GLOB_DAT = 6;
        pub R_386_JUMP_SLOT = 7;
        pub R_386_RELATIVE = 8;
        pub R_386_GOTOFF = 9;
        pub R_386_GOTPC = 10;
        pub R_386_32PLT = 11;
        pub R_386_TLS_TPOFF = 14;
        pub R_386_TLS_IE = 15;
        pub R_386_TLS_GOTIE = 16;
        pub R_386_TLS_LE = 17;
        pub R_386_TLS_GD = 18;
        pub R_386_TLS_LDM = 19;
        pub R_386_16 = 20;
        pub R_386_PC16 = 21;
        pub R_386_8 = 22;
        pub R_386_PC8 = 23;
        pub R_386_TLS_GD_32 = 24;
        pub R_386_TLS_GD_PUSH = 25;
        pub R_386_TLS_GD_CALL = 26;
        pub R_386_TLS_GD_POP = 27;
        pub R_386_TLS_LDM_32 = 28;
        pub R_386_TLS_LDM_PUSH = 29;
        pub R_386_TLS_LDM_CALL = 30;
        pub R_386_TLS_LDM_POP = 31;
        pub R_386_TLS_LDO_32 = 32;
        pub R_386_TLS_IE_32 = 33;
        pub R_386_TLS_LE_32 = 34;
        pub R_386_TLS_DTPMOD32 = 35;
        pub R_386_TLS_DTPOFF32 = 36;
        pub R_386_TLS_TPOFF32 = 37;
        pub R_386_TLS_GOTDESC = 39;
        pub R_386_TLS_DESC_CALL = 40;
        pub R_386_TLS_DESC = 41;
        pub R_386_IRELATIVE = 42;
        pub R_386_GOT32X = 43;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
