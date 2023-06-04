use crate::stringable_consts_blocks::option_stringable_consts_block;

// Relocations
option_stringable_consts_block! {
    const stringable: u32 {
        pub R_BPF_NONE = 0;
        pub R_BPF_64_64 = 1;
        pub R_BPF_64_ABS64 = 2;
        pub R_BPF_64_ABS32 = 3;
        pub R_BPF_64_NODYLD32 = 4;
        pub R_BPF_64_32 = 10;
    }

    const ignore: u32 {}

    pub fn r_to_str(value: u32) -> Option<&'static str>;
}
