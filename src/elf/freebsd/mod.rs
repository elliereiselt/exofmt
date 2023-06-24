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

use crate::stringable_consts_blocks::option_stringable_consts_block;

// Note types
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_FREEBSD_ABI_TAG = 1;
        pub NT_FREEBSD_NOINIT_TAG = 2;
        pub NT_FREEBSD_ARCH_TAG = 3;
        pub NT_FREEBSD_FEATURE_CTL = 4;
    }

    const ignore: u32 {
        // NT_FREEBSD_FEATURE_CTL values (see FreeBSD's sys/sys/elf_common.h).
        // These are flags.
        pub NT_FREEBSD_FCTL_ASLR_DISABLE = 0x00000001;
        pub NT_FREEBSD_FCTL_PROTMAX_DISABLE = 0x00000002;
        pub NT_FREEBSD_FCTL_STKGAP_DISABLE = 0x00000004;
        pub NT_FREEBSD_FCTL_WXNEEDED = 0x00000008;
        pub NT_FREEBSD_FCTL_LA48 = 0x00000010;
        pub NT_FREEBSD_FCTL_ASG_DISABLE = 0x00000020;
    }

    /// Convert a note type value to a FreeBSD note type constant string name or `None`
    ///
    /// NOTE: This is split into `nt_to_str` and `core_nt_to_str` similar to others but FreeBSD
    ///       appears to keep the values separate between non-core and core constants. As such
    ///       you may have to also chain `core_nt_to_str` even on non-core objects.
    pub fn nt_to_str(value: u32) -> Option<&'static str>;
}

// FreeBSD core note types.
option_stringable_consts_block! {
    const stringable: u32 {
        pub NT_FREEBSD_THRMISC = 7;
        pub NT_FREEBSD_PROCSTAT_PROC = 8;
        pub NT_FREEBSD_PROCSTAT_FILES = 9;
        pub NT_FREEBSD_PROCSTAT_VMMAP = 10;
        pub NT_FREEBSD_PROCSTAT_GROUPS = 11;
        pub NT_FREEBSD_PROCSTAT_UMASK = 12;
        pub NT_FREEBSD_PROCSTAT_RLIMIT = 13;
        pub NT_FREEBSD_PROCSTAT_OSREL = 14;
        pub NT_FREEBSD_PROCSTAT_PSSTRINGS = 15;
        pub NT_FREEBSD_PROCSTAT_AUXV = 16;
    }

    const ignore: u32 {}

    pub fn core_nt_to_str(value: u32) -> Option<&'static str>;
}
