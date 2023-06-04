use crate::stringable_consts_blocks::option_stringable_consts_block;

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// LLVM ODR table.
        pub SHT_LLVM_ODRTAB = 0x6fff4c00;
        /// LLVM Linker Options.
        pub SHT_LLVM_LINKER_OPTIONS = 0x6fff4c01;
        /// List of address-significant symbols for safe ICF.
        pub SHT_LLVM_ADDRSIG = 0x6fff4c03;
        /// LLVM Dependent Library Specifiers.
        pub SHT_LLVM_DEPENDENT_LIBRARIES = 0x6fff4c04;
        /// Symbol partition specification.
        pub SHT_LLVM_SYMPART = 0x6fff4c05;
        /// ELF header for loadable partition.
        pub SHT_LLVM_PART_EHDR = 0x6fff4c06;
        /// Phdrs for loadable partition.
        pub SHT_LLVM_PART_PHDR = 0x6fff4c07;
        /// LLVM Basic Block Address Map (old version kept for backward-compatibility).
        pub SHT_LLVM_BB_ADDR_MAP_V0 = 0x6fff4c08;
        /// LLVM Call Graph Profile.
        pub SHT_LLVM_CALL_GRAPH_PROFILE = 0x6fff4c09;
        /// LLVM Basic Block Address Map.
        pub SHT_LLVM_BB_ADDR_MAP = 0x6fff4c0a;
        /// LLVM device offloading data.
        pub SHT_LLVM_OFFLOADING = 0x6fff4c0b;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// LLVMOMPOFFLOAD specific notes.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_LLVM_OPENMP_OFFLOAD_VERSION = 1;
        pub NT_LLVM_OPENMP_OFFLOAD_PRODUCER = 2;
        pub NT_LLVM_OPENMP_OFFLOAD_PRODUCER_VERSION = 3;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}
