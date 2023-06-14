// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use crate::stringable_consts_blocks::stringable_consts_block;

#[derive(Clone)]
pub struct Dyn {
    /// Type of dynamic table entry
    pub d_tag: u64, // This was originally `u64`, I've replaced with `u64` as it's just easier to work with
    /// Integer or pointer value of entry
    pub d_val: u64,
}

// Dynamic tags
stringable_consts_block! {
    const stringable: u64 {
        /// Marks end of dynamic array.
        pub DT_NULL = 0;
        /// String table offset of needed library.
        pub DT_NEEDED = 1;
        /// Size of relocation entries in PLT.
        pub DT_PLTRELSZ = 2;
        /// Address associated with linkage table.
        pub DT_PLTGOT = 3;
        /// Address of symbolic hash table.
        pub DT_HASH = 4;
        /// Address of dynamic string table.
        pub DT_STRTAB = 5;
        /// Address of dynamic symbol table.
        pub DT_SYMTAB = 6;
        /// Address of relocation table (Rela entries).
        pub DT_RELA = 7;
        /// Size of Rela relocation table.
        pub DT_RELASZ = 8;
        /// Size of a Rela relocation entry.
        pub DT_RELAENT = 9;
        /// Total size of the string table.
        pub DT_STRSZ = 10;
        /// Size of a symbol table entry.
        pub DT_SYMENT = 11;
        /// Address of initialization function.
        pub DT_INIT = 12;
        /// Address of termination function.
        pub DT_FINI = 13;
        /// String table offset of a shared objects name.
        pub DT_SONAME = 14;
        /// String table offset of library search path.
        pub DT_RPATH = 15;
        /// Changes symbol resolution algorithm.
        pub DT_SYMBOLIC = 16;
        /// Address of relocation table (Rel entries).
        pub DT_REL = 17;
        /// Size of Rel relocation table.
        pub DT_RELSZ = 18;
        /// Size of a Rel relocation entry.
        pub DT_RELENT = 19;
        /// Type of relocation entry used for linking.
        pub DT_PLTREL = 20;
        /// Reserved for debugger.
        pub DT_DEBUG = 21;
        /// Relocations exist for non-writable segments.
        pub DT_TEXTREL = 22;
        /// Address of relocations associated with PLT.
        pub DT_JMPREL = 23;
        /// Process all relocations before execution.
        pub DT_BIND_NOW = 24;
        /// Pointer to array of initialization functions.
        pub DT_INIT_ARRAY = 25;
        /// Pointer to array of termination functions.
        pub DT_FINI_ARRAY = 26;
        /// Size of DT_INIT_ARRAY.
        pub DT_INIT_ARRAYSZ = 27;
        /// Size of DT_FINI_ARRAY.
        pub DT_FINI_ARRAYSZ = 28;
        /// String table offset of lib search path.
        pub DT_RUNPATH = 29;
        /// Flags.
        pub DT_FLAGS = 30;
        /// Pointer to array of preinit functions.
        pub DT_PREINIT_ARRAY = 32;
        /// Size of the DT_PREINIT_ARRAY array.
        pub DT_PREINIT_ARRAYSZ = 33;
        /// Address of the SHT_SYMTAB_SHNDX section.
        pub DT_SYMTAB_SHNDX = 34;
        /// Size of Relr relocation table.
        pub DT_RELRSZ = 35;
        /// Address of relocation table (Relr entries).
        pub DT_RELR = 36;
        /// Size of a Relr relocation entry.
        pub DT_RELRENT = 37;
    }

    const ignore: u64 {
        /// Values from here to DT_LOOS follow the rules for the interpretation of the d_un union.
        pub DT_ENCODING = 32;
        /// Start of environment specific tags.
        pub DT_LOOS = 0x60000000;
        /// End of environment specific tags.
        pub DT_HIOS = 0x6FFFFFFF;
        /// Start of processor specific tags.
        pub DT_LOPROC = 0x70000000;
        /// End of processor specific tags.
        pub DT_HIPROC = 0x7FFFFFFF;
    }

    pub fn dt_to_str(value: u64) -> &'static str {
        match value {
            unknown => {
                if unknown >= DT_LOOS && unknown <= DT_HIOS {
                    "DT_UNKNOWN_OS_RANGE"
                } else if unknown >= DT_LOPROC && unknown <= DT_HIPROC {
                    "DT_UNKNOWN_PROC_RANGE"
                } else {
                    "DT_UNKNOWN"
                }
            }
        }
    }
}

// DT_FLAGS values
/// The object may reference $ORIGIN.
pub const DF_ORIGIN: u64 = 0x01;
/// Search the shared lib before searching the exe.
pub const DF_SYMBOLIC: u64 = 0x02;
/// Relocations may modify a non-writable segment.
pub const DF_TEXTREL: u64 = 0x04;
/// Process all relocations on load.
pub const DF_BIND_NOW: u64 = 0x08;
/// Reject attempts to load dynamically.
pub const DF_STATIC_TLS: u64 = 0x10;

// State flags selectable in the `d_un.d_val' element of the DT_FLAGS_1 entry
/// Set RTLD_NOW for this object.
pub const DF_1_NOW: u64 = 0x00000001;
/// Set RTLD_GLOBAL for this object.
pub const DF_1_GLOBAL: u64 = 0x00000002;
/// Set RTLD_GROUP for this object.
pub const DF_1_GROUP: u64 = 0x00000004;
/// Set RTLD_NODELETE for this object.
pub const DF_1_NODELETE: u64 = 0x00000008;
/// Trigger filtee loading at runtime.
pub const DF_1_LOADFLTR: u64 = 0x00000010;
/// Set RTLD_INITFIRST for this object.
pub const DF_1_INITFIRST: u64 = 0x00000020;
/// Set RTLD_NOOPEN for this object.
pub const DF_1_NOOPEN: u64 = 0x00000040;
/// $ORIGIN must be handled.
pub const DF_1_ORIGIN: u64 = 0x00000080;
/// Direct binding enabled.
pub const DF_1_DIRECT: u64 = 0x00000100;
pub const DF_1_TRANS: u64 = 0x00000200;
/// Object is used to interpose.
pub const DF_1_INTERPOSE: u64 = 0x00000400;
/// Ignore default lib search path.
pub const DF_1_NODEFLIB: u64 = 0x00000800;
/// Object can't be dldump'ed.
pub const DF_1_NODUMP: u64 = 0x00001000;
/// Configuration alternative created.
pub const DF_1_CONFALT: u64 = 0x00002000;
/// Filtee terminates filters search.
pub const DF_1_ENDFILTEE: u64 = 0x00004000;
/// Disp reloc applied at build time.
pub const DF_1_DISPRELDNE: u64 = 0x00008000;
/// Disp reloc applied at run-time.
pub const DF_1_DISPRELPND: u64 = 0x00010000;
/// Object has no-direct binding.
pub const DF_1_NODIRECT: u64 = 0x00020000;
pub const DF_1_IGNMULDEF: u64 = 0x00040000;
pub const DF_1_NOKSYMS: u64 = 0x00080000;
pub const DF_1_NOHDR: u64 = 0x00100000;
/// Object is modified after built.
pub const DF_1_EDITED: u64 = 0x00200000;
pub const DF_1_NORELOC: u64 = 0x00400000;
/// Object has individual interposers.
pub const DF_1_SYMINTPOSE: u64 = 0x00800000;
/// Global auditing required.
pub const DF_1_GLOBAUDIT: u64 = 0x01000000;
/// Singleton symbols are used.
pub const DF_1_SINGLETON: u64 = 0x02000000;
/// Object is a position-independent executable.
pub const DF_1_PIE: u64 = 0x08000000;
