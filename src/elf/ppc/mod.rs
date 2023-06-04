use crate::stringable_consts_blocks::option_stringable_consts_block;

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_PPC_VMX = 0x100;
        pub NT_PPC_VSX = 0x102;
        pub NT_PPC_TAR = 0x103;
        pub NT_PPC_PPR = 0x104;
        pub NT_PPC_DSCR = 0x105;
        pub NT_PPC_EBB = 0x106;
        pub NT_PPC_PMU = 0x107;
        pub NT_PPC_TM_CGPR = 0x108;
        pub NT_PPC_TM_CFPR = 0x109;
        pub NT_PPC_TM_CVMX = 0x10a;
        pub NT_PPC_TM_CVSX = 0x10b;
        pub NT_PPC_TM_SPR = 0x10c;
        pub NT_PPC_TM_CTAR = 0x10d;
        pub NT_PPC_TM_CPPR = 0x10e;
        pub NT_PPC_TM_CDSCR = 0x10f;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_PPC_NONE = 0;
        pub R_PPC_ADDR32 = 1;
        pub R_PPC_ADDR24 = 2;
        pub R_PPC_ADDR16 = 3;
        pub R_PPC_ADDR16_LO = 4;
        pub R_PPC_ADDR16_HI = 5;
        pub R_PPC_ADDR16_HA = 6;
        pub R_PPC_ADDR14 = 7;
        pub R_PPC_ADDR14_BRTAKEN = 8;
        pub R_PPC_ADDR14_BRNTAKEN = 9;
        pub R_PPC_REL24 = 10;
        pub R_PPC_REL14 = 11;
        pub R_PPC_REL14_BRTAKEN = 12;
        pub R_PPC_REL14_BRNTAKEN = 13;
        pub R_PPC_GOT16 = 14;
        pub R_PPC_GOT16_LO = 15;
        pub R_PPC_GOT16_HI = 16;
        pub R_PPC_GOT16_HA = 17;
        pub R_PPC_PLTREL24 = 18;
        pub R_PPC_COPY = 19;
        pub R_PPC_GLOB_DAT = 20;
        pub R_PPC_JMP_SLOT = 21;
        pub R_PPC_RELATIVE = 22;
        pub R_PPC_LOCAL24PC = 23;
        pub R_PPC_UADDR32 = 24;
        pub R_PPC_UADDR16 = 25;
        pub R_PPC_REL32 = 26;
        pub R_PPC_PLT32 = 27;
        pub R_PPC_PLTREL32 = 28;
        pub R_PPC_PLT16_LO = 29;
        pub R_PPC_PLT16_HI = 30;
        pub R_PPC_PLT16_HA = 31;
        pub R_PPC_SDAREL16 = 32;
        pub R_PPC_SECTOFF = 33;
        pub R_PPC_SECTOFF_LO = 34;
        pub R_PPC_SECTOFF_HI = 35;
        pub R_PPC_SECTOFF_HA = 36;
        pub R_PPC_ADDR30 = 37;
        pub R_PPC_TLS = 67;
        pub R_PPC_DTPMOD32 = 68;
        pub R_PPC_TPREL16 = 69;
        pub R_PPC_TPREL16_LO = 70;
        pub R_PPC_TPREL16_HI = 71;
        pub R_PPC_TPREL16_HA = 72;
        pub R_PPC_TPREL32 = 73;
        pub R_PPC_DTPREL16 = 74;
        pub R_PPC_DTPREL16_LO = 75;
        pub R_PPC_DTPREL16_HI = 76;
        pub R_PPC_DTPREL16_HA = 77;
        pub R_PPC_DTPREL32 = 78;
        pub R_PPC_GOT_TLSGD16 = 79;
        pub R_PPC_GOT_TLSGD16_LO = 80;
        pub R_PPC_GOT_TLSGD16_HI = 81;
        pub R_PPC_GOT_TLSGD16_HA = 82;
        pub R_PPC_GOT_TLSLD16 = 83;
        pub R_PPC_GOT_TLSLD16_LO = 84;
        pub R_PPC_GOT_TLSLD16_HI = 85;
        pub R_PPC_GOT_TLSLD16_HA = 86;
        pub R_PPC_GOT_TPREL16 = 87;
        pub R_PPC_GOT_TPREL16_LO = 88;
        pub R_PPC_GOT_TPREL16_HI = 89;
        pub R_PPC_GOT_TPREL16_HA = 90;
        pub R_PPC_GOT_DTPREL16 = 91;
        pub R_PPC_GOT_DTPREL16_LO = 92;
        pub R_PPC_GOT_DTPREL16_HI = 93;
        pub R_PPC_GOT_DTPREL16_HA = 94;
        pub R_PPC_TLSGD = 95;
        pub R_PPC_TLSLD = 96;
        pub R_PPC_IRELATIVE = 248;
        pub R_PPC_REL16 = 249;
        pub R_PPC_REL16_LO = 250;
        pub R_PPC_REL16_HI = 251;
        pub R_PPC_REL16_HA = 252;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        /// Uses Secure PLT ABI.
        pub DT_PPC_GOT = 0x70000000;
        /// Has TLS optimization.
        pub DT_PPC_OPT = 0x70000001;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}
