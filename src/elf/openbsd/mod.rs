use crate::stringable_consts_blocks::option_stringable_consts_block;

// Segment types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Like bss, but not immutable.
        pub PT_OPENBSD_MUTABLE = 0x65a3dbe5;
        /// Fill with random data.
        pub PT_OPENBSD_RANDOMIZE = 0x65a3dbe6;
        /// Program does W^X violations.
        pub PT_OPENBSD_WXNEEDED = 0x65a3dbe7;
        /// Section for boot arguments.
        pub PT_OPENBSD_BOOTDATA = 0x65a41be6;
    }

    const ignore: u32 {}

    pub fn pt_to_str(value: u32) -> Option<&'static str>;
}

// OpenBSD core note types.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_OPENBSD_PROCINFO = 10;
        pub NT_OPENBSD_AUXV = 11;
        pub NT_OPENBSD_REGS = 20;
        pub NT_OPENBSD_FPREGS = 21;
        pub NT_OPENBSD_XFPREGS = 22;
        pub NT_OPENBSD_WCOOKIE = 23;
    }

    const ignore: u32 {}

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}
