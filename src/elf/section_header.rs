use crate::stringable_consts_blocks::stringable_consts_block;
use bitflags::bitflags;

pub struct SectionHeader {
    /// Section name (index into string table)
    pub sh_name: u32,
    /// Section type (SHT_*)
    pub sh_type: u32,
    /// Section flags (SHF_*)
    pub sh_flags: SHFlags,
    /// Address where section is to be loaded
    pub sh_addr: u64,
    /// File offset of section data, in bytes
    pub sh_offset: u64,
    /// Size of section, in bytes
    pub sh_size: u64,
    /// Section type-specific header table index link
    pub sh_link: u32,
    /// Section type-specific extra information
    pub sh_info: u32,
    /// Section address alignment
    pub sh_addralign: u64,
    /// Size of records contained within the section
    pub sh_entsize: u64,
}

// Special section indices
stringable_consts_block! {
    const stringable: u32 {
        /// Undefined, missing, irrelevant, or meaningless
        pub SHN_UNDEF = 0;
        /// Symbol has absolute value; does not need relocation
        pub SHN_ABS = 0xfff1;
        /// FORTRAN COMMON or C external global variables
        pub SHN_COMMON = 0xfff2;
        /// Mark that the index is >= SHN_LORESERVE
        pub SHN_XINDEX = 0xffff;
    }

    const ignore: u32 {
        /// Lowest reserved index
        pub SHN_LORESERVE = 0xff00;
        /// Lowest processor-specific index
        pub SHN_LOPROC = 0xff00;
        /// Highest processor-specific index
        pub SHN_HIPROC = 0xff1f;
        /// Lowest operating system-specific index
        pub SHN_LOOS = 0xff20;
        /// Highest operating system-specific index
        pub SHN_HIOS = 0xff3f;
        /// Highest reserved index
        pub SHN_HIRESERVE = 0xffff;
    }

    pub fn shn_to_str(value: u32) -> &'static str {
        match value {
            unknown => {
                if unknown >= SHN_LOPROC && unknown <= SHN_HIPROC {
                    "SHN_UNKNOWN_PROC_RANGE"
                } else if unknown >= SHN_LOOS && unknown <= SHN_HIOS {
                    "SHN_UNKNOWN_OS_RANGE"
                } else if unknown >= SHN_LORESERVE && unknown <= SHN_HIRESERVE {
                    "SHN_UNKNOWN_RESERVE_RANGE"
                } else {
                    "SHN_UNKNOWN"
                }
            }
        }
    }
}

// Section types
stringable_consts_block! {
    const stringable: u32 {
        /// No associated section (inactive entry).
        pub SHT_NULL = 0;
        /// Program-defined contents.
        pub SHT_PROGBITS = 1;
        /// Symbol table.
        pub SHT_SYMTAB = 2;
        /// String table.
        pub SHT_STRTAB = 3;
        /// Relocation entries; explicit addends.
        pub SHT_RELA = 4;
        /// Symbol hash table.
        pub SHT_HASH = 5;
        /// Information for dynamic linking.
        pub SHT_DYNAMIC = 6;
        /// Information about the file.
        pub SHT_NOTE = 7;
        /// Data occupies no space in the file.
        pub SHT_NOBITS = 8;
        /// Relocation entries; no explicit addends.
        pub SHT_REL = 9;
        /// Reserved.
        pub SHT_SHLIB = 10;
        /// Symbol table.
        pub SHT_DYNSYM = 11;
        /// Pointers to initialization functions.
        pub SHT_INIT_ARRAY = 14;
        /// Pointers to termination functions.
        pub SHT_FINI_ARRAY = 15;
        /// Pointers to pre-init functions.
        pub SHT_PREINIT_ARRAY = 16;
        /// Section group.
        pub SHT_GROUP = 17;
        /// Indices for SHN_XINDEX entries.
        pub SHT_SYMTAB_SHNDX = 18;
        /// Relocation entries, only offsets.
        pub SHT_RELR = 19;
    }

    const ignore: u32 {
        /// Lowest operating system-specific type.
        pub SHT_LOOS = 0x60000000;
        /// Highest operating system-specific type.
        pub SHT_HIOS = 0x6fffffff;
        /// Lowest processor arch-specific type.
        pub SHT_LOPROC = 0x70000000;
        /// Highest processor arch-specific type.
        pub SHT_HIPROC = 0x7fffffff;
        /// Lowest type reserved for applications.
        pub SHT_LOUSER = 0x80000000;
        /// Highest type reserved for applications.
        pub SHT_HIUSER = 0xffffffff;
    }

    pub fn sht_to_str(value: u32) -> &'static str {
        match value {
            unknown => {
                if unknown >= SHT_LOOS && unknown <= SHT_HIOS {
                    "SHT_UNKNOWN_OS_RANGE"
                } else if unknown >= SHT_LOPROC && unknown <= SHT_HIPROC {
                    "SHT_UNKNOWN_PROC_RANGE"
                } else if unknown >= SHT_LOUSER && unknown <= SHT_HIUSER {
                    "SHT_UNKNOWN_USER_RANGE"
                } else {
                    "SHT_UNKNOWN"
                }
            }
        }
    }
}

// Section flags
/// Section data should be writable during execution.
pub const SHF_WRITE: u32 = 0x1;
/// Section occupies memory during program execution.
pub const SHF_ALLOC: u32 = 0x2;
/// Section contains executable machine instructions.
pub const SHF_EXECINSTR: u32 = 0x4;
/// The data in this section may be merged.
pub const SHF_MERGE: u32 = 0x10;
/// The data in this section is null-terminated strings.
pub const SHF_STRINGS: u32 = 0x20;
/// A field in this section holds a section header table index.
pub const SHF_INFO_LINK: u32 = 0x40;
/// Adds special ordering requirements for link editors.
pub const SHF_LINK_ORDER: u32 = 0x80;
/// This section requires special OS-specific processing to avoid incorrect behavior.
pub const SHF_OS_NONCONFORMING: u32 = 0x100;
/// This section is a member of a section group.
pub const SHF_GROUP: u32 = 0x200;
/// This section holds Thread-Local Storage.
pub const SHF_TLS: u32 = 0x400;
/// Identifies a section containing compressed data.
pub const SHF_COMPRESSED: u32 = 0x800;
/// This section should not be garbage collected by the linker.
pub const SHF_GNU_RETAIN: u32 = 0x200000;
/// This section is excluded from the final executable or shared library.
pub const SHF_EXCLUDE: u32 = 0x80000000;
/// Start of target-specific flags.
pub const SHF_MASKOS: u32 = 0x0ff00000;
/// Bits indicating processor-specific flags.
pub const SHF_MASKPROC: u32 = 0xf0000000;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SHFlags: u64 {
        const WRITE = SHF_WRITE as u64;
        const ALLOC = SHF_ALLOC as u64;
        const EXECINSTR = SHF_EXECINSTR as u64;
        const MERGE = SHF_MERGE as u64;
        const STRINGS = SHF_STRINGS as u64;
        const INFO_LINK = SHF_INFO_LINK as u64;
        const OS_NONCONFORMING = SHF_OS_NONCONFORMING as u64;
        const GROUP = SHF_GROUP as u64;
        const TLS = SHF_TLS as u64;
        const COMPRESSED = SHF_COMPRESSED as u64;
        const GNU_RETAIN = SHF_GNU_RETAIN as u64;
        const EXCLUDE = SHF_EXCLUDE as u64;
    }
}

impl SHFlags {
    pub fn has_os_flags(&self) -> bool {
        self.bits() & u64::from(SHF_MASKOS) > 0
    }

    pub fn has_proc_flags(&self) -> bool {
        self.bits() & u64::from(SHF_MASKPROC) > 0
    }
}

// Section group flags
pub const GRP_COMDAT: u32 = 0x1;
pub const GRP_MASKOS: u32 = 0x0ff00000;
pub const GRP_MASKPROC: u32 = 0xf0000000;
