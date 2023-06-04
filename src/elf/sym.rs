use crate::stringable_consts_blocks::stringable_consts_block;

#[derive(Clone)]
pub struct Sym {
    /// Symbol name (index into string table)
    pub st_name: u32,
    /// Symbol's type and binding attributes
    pub st_info: u8,
    /// Symbol's visibility attribute
    pub st_other: u8,
    /// Which section (header tbl index) it's defined in
    pub st_shndx: u16,
    /// Value or address associated with the symbol
    pub st_value: u64,
    /// Size of the symbol
    pub st_size: u64,
}

impl Sym {
    pub fn st_bind(&self) -> u8 {
        self.st_info >> 4
    }

    pub fn st_type(&self) -> u8 {
        self.st_info & 0xf
    }

    pub fn st_visibility(&self) -> u8 {
        self.st_other & 0x3
    }
}

// Symbol bindings
stringable_consts_block! {
    const stringable: u8 {
        /// Local symbol, not visible outside object file containing definition
        pub STB_LOCAL = 0;
        /// Global symbol, visible to all object files being combined
        pub STB_GLOBAL = 1;
        /// Weak symbol, like global but lower-precedence
        pub STB_WEAK = 2;
    }

    const ignore: u8 {
        /// Lowest operating system-specific binding type
        pub STB_LOOS = 10;
        /// Highest operating system-specific binding type
        pub STB_HIOS = 12;
        /// Lowest processor-specific binding type
        pub STB_LOPROC = 13;
        /// Highest processor-specific binding type
        pub STB_HIPROC = 15;
    }

    pub fn stb_to_str(value: u8) -> &'static str {
        match value {
            unknown => {
                if unknown >= STB_LOOS && unknown <= STB_HIOS {
                    "STB_UNKNOWN_OS_RANGE"
                } else if unknown >= STB_LOPROC && unknown <= STB_HIPROC {
                    "STB_UNKNOWN_PROC_RANGE"
                } else {
                    "STB_UNKNOWN"
                }
            }
        }
    }
}

// Symbol types
stringable_consts_block! {
    const stringable: u8 {
        /// Symbol's type is not specified
        pub STT_NOTYPE = 0;
        /// Symbol is a data object (variable, array, etc.)
        pub STT_OBJECT = 1;
        /// Symbol is executable code (function, etc.)
        pub STT_FUNC = 2;
        /// Symbol refers to a section
        pub STT_SECTION = 3;
        /// Local, absolute symbol that refers to a file
        pub STT_FILE = 4;
        /// An uninitialized common block
        pub STT_COMMON = 5;
        /// Thread local data object
        pub STT_TLS = 6;
    }

    const ignore: u8 {
        /// Lowest operating system-specific symbol type
        pub STT_LOOS = 10;
        /// Highest operating system-specific symbol type
        pub STT_HIOS = 12;
        /// Lowest processor-specific symbol type
        pub STT_LOPROC = 13;
        /// Highest processor-specific symbol type
        pub STT_HIPROC = 15;
    }

    pub fn stt_to_str(value: u8) -> &'static str {
        match value {
            unknown => {
                if unknown >= STT_LOOS && unknown <= STT_HIOS {
                    "STT_UNKNOWN_OS_RANGE"
                } else if unknown >= STT_LOPROC && unknown <= STT_HIPROC {
                    "STT_UNKNOWN_PROC_RANGE"
                } else {
                    "STT_UNKNOWN"
                }
            }
        }
    }
}

// Symbol number
pub const STN_UNDEF: usize = 0;
