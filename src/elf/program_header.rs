/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::stringable_consts_blocks::stringable_consts_block;
use bitflags::bitflags;

pub struct ProgramHeader {
    /// Identifies the type of the segment
    pub p_type: u32,
    /// Segment-dependent flags
    pub p_flags: PFlags,
    /// Offset of the segment in the file image
    pub p_offset: u64,
    /// Virtual address of the segment in memory
    pub p_vaddr: u64,
    /// On systems where physical address is relevant, reserved for segment's physical address
    pub p_paddr: u64,
    /// Size in bytes of the segment in the file image, may be 0
    pub p_filesz: u64,
    /// Size in bytes of the segment in memory, may be 0
    pub p_memsz: u64,
    /// 0 and 1 specify no alignment; otherwise should be a positive, integral power of 2, with p_vaddr equating p_offset modulus p_align
    pub p_align: u64,
}

// Segment types
stringable_consts_block! {
    const stringable: u32 {
        /// Unused segment.
        pub PT_NULL = 0;
        /// Loadable segment.
        pub PT_LOAD = 1;
        /// Dynamic linking information.
        pub PT_DYNAMIC = 2;
        /// Interpreter pathname.
        pub PT_INTERP = 3;
        /// Auxiliary information.
        pub PT_NOTE = 4;
        /// Reserved.
        pub PT_SHLIB = 5;
        /// The program header table itself.
        pub PT_PHDR = 6;
        /// The thread-local storage template.
        pub PT_TLS = 7;
    }

    const ignore: u32 {
        /// Lowest operating system-specific pt entry type.
        pub PT_LOOS = 0x60000000;
        /// Highest operating system-specific pt entry type.
        pub PT_HIOS = 0x6fffffff;
        /// Lowest processor-specific program hdr entry type.
        pub PT_LOPROC = 0x70000000;
        /// Highest processor-specific program hdr entry type.
        pub PT_HIPROC = 0x7fffffff;
    }

    pub fn pt_to_str(value: u32) -> &'static str {
        match value {
            unknown => {
                if unknown >= PT_LOOS && unknown <= PT_HIOS {
                    "PT_UNKNOWN_IN_OS_RANGE"
                } else if unknown >= PT_LOPROC && unknown <= PT_HIPROC {
                    "PT_UNKNOWN_IN_PROC_RANGE"
                } else {
                    "unknown"
                }
            }
        }
    }
}

// Segment flags
/// Execute
pub const PF_X: u32 = 1;
/// Write
pub const PF_W: u32 = 2;
/// Read
pub const PF_R: u32 = 4;
/// Bits for operating system-specific semantics.
pub const PF_MASKOS: u32 = 0x0ff00000;
/// Bits for processor-specific semantics.
pub const PF_MASKPROC: u32 = 0xf0000000;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PFlags: u32 {
        const EXECUTE = PF_X;
        const WRITE = PF_W;
        const READ = PF_R;
    }
}

impl PFlags {
    pub fn has_os_flags(&self) -> bool {
        self.bits() & PF_MASKOS > 0
    }

    pub fn has_proc_flags(&self) -> bool {
        self.bits() & PF_MASKPROC > 0
    }
}
