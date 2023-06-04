use crate::stringable_consts_blocks::option_stringable_consts_block;

// Section types
option_stringable_consts_block! {
    const stringable: u32 {
        /// Android packed relocation section types
        pub SHT_ANDROID_REL = 0x60000001;
        pub SHT_ANDROID_RELA = 0x60000002;
        /// Android's support for SHT_RELR sections
        /// Relocation entries, only offsets.
        pub SHT_ANDROID_RELR = 0x6fffff00;
    }

    const ignore: u32 {}

    pub fn sht_to_str(value: u32) -> Option<&'static str>;
}

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_ANDROID_TYPE_IDENT = 1;
        pub NT_ANDROID_TYPE_KUSER = 3;
        pub NT_ANDROID_TYPE_MEMTAG = 4;
    }

    const ignore: u32 {
        // Memory tagging values used in NT_ANDROID_TYPE_MEMTAG notes
        // TODO: I'm 90% sure these were supposed to be flags but I can't remember... double check to see...
        //       Just off a guess though, the `_mask` mixed with none of the values besides `_mask` using more than one bit, I think yes these are flags...
        pub NT_MEMTAG_LEVEL_NONE = 0;
        /// Use the kernels auto-upgrade feature to allow for MTE Asynchronous, Asymmetric,
        /// or Synchronous mode. This allows silicon vendors to specify, on a per-cpu basis
        /// what 'ASYNC' should mean. Generally, the expectation is "pick the most precise
        /// mode that's very fast".
        pub NT_MEMTAG_LEVEL_ASYNC = 1;
        /// Indicates running all threads in MTE Synchronous mode
        pub NT_MEMTAG_LEVEL_SYNC = 2;
        pub NT_MEMTAG_LEVEL_MASK = 3;
        /// Indicates to the loader that it should prepare for MTE to be enabled on the heap
        pub NT_MEMTAG_HEAP = 4;
        /// Indicates to the loader that it should prepare for MTE to be enabled on the stack
        pub NT_MEMTAG_STACK = 8;
    }

    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// Dynamic tags
option_stringable_consts_block! {
    const stringable: u64 {
        // Android packed relocation section tags
        pub DT_ANDROID_REL = 0x6000000F;
        pub DT_ANDROID_RELSZ = 0x60000010;
        pub DT_ANDROID_RELA = 0x60000011;
        pub DT_ANDROID_RELASZ = 0x60000012;

        // Android's support for SHT_RELR sections
        /// Address of relocation table (RelR entries).
        pub DT_ANDROID_RELR = 0x6FFFE000;
        /// Size of Relr relocation table.
        pub DT_ANDROID_RELRSZ = 0x6FFFE001;
        /// Size of a Relr relocation entry.
        pub DT_ANDROID_RELRENT = 0x6FFFE003;
    }

    const ignore: u64 {}

    pub fn dt_to_str(value: u64) -> Option<&'static str>;
}

// Packed relocation group flags
pub const RELOCATION_GROUPED_BY_INFO_FLAG: u32 = 1;
pub const RELOCATION_GROUPED_BY_OFFSET_DELTA_FLAG: u32 = 2;
pub const RELOCATION_GROUPED_BY_ADDEND_FLAG: u32 = 4;
pub const RELOCATION_GROUP_HAS_ADDEND_FLAG: u32 = 8;
