use crate::stringable_consts_blocks::option_stringable_consts_block;

// OS ABI types
option_stringable_consts_block! {
    const stringable: u8 {
        /// Bare-metal TMS320C6000
        pub OSABI_C6000_ELFABI = 64;
        /// Linux TMS320C6000
        pub OSABI_C6000_LINUX = 65;
    }

    const ignore: u8 {}

    pub fn osabi_to_str(value: u8) -> Option<&'static str>;
}
