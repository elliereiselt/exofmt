use crate::stringable_consts_blocks::option_stringable_consts_block;

// NetBSD core note types.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_NETBSDCORE_PROCINFO = 1;
        pub NT_NETBSDCORE_AUXV = 2;
        pub NT_NETBSDCORE_LWPSTATUS = 24;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}
