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

// TODO: I think `VerDef` and `VerNeed` should contain a `Vec<VerDefAux>` and `Vec<VerNeedAux>` respectively...
//       OR create `VerDefTab` and `VerNeedTab`? Example: `VerDefTab` would contain `ver_def: VerDef` and `aux_values: Vec<VerDefAux>`
pub struct VerSym {
    pub vs_val: u16,
}

pub struct VerDef {
    /// Version revision
    pub vd_version: u16,
    /// Version information
    pub vd_flags: u16,
    /// Version Index
    pub vd_ndx: u16,
    /// Number of associated aux entries
    pub vd_cnt: u16,
    /// Version name hash value
    pub vd_hash: u32,
    /// Offset in bytes to verdaux array
    pub vd_aux: u32,
    /// Offset in bytes to next VerDef entry
    pub vd_next: u32,
}

pub struct VerDefAux {
    /// Version or dependency names
    pub vda_name: u32,
    /// Offset in bytes to next VerDefAux entry
    pub vda_next: u32,
}

pub struct VerNeed {
    /// Version of structure
    pub vn_version: u16,
    /// Number of associated aux entries
    pub vn_cnt: u16,
    /// Offset of filename for this dependency
    pub vn_file: u32,
    /// Offset in bytes to vernaux array
    pub vn_aux: u32,
    /// Offset in bytes to next VerNeed entry
    pub vn_next: u32,
}

pub struct VerNeedAux {
    /// Hash value of dependency name
    pub vna_hash: u32,
    /// Dependency specific information
    pub vna_flags: u16,
    /// Unused?
    pub vna_other: u16,
    /// Dependency name string offset
    pub vna_name: u32,
    /// Offset in bytes to next VerNeedAux entry
    pub vna_next: u32,
}

// VerDef versions
/// No version
pub const VER_DEF_NONE: u16 = 0;
/// Current version
pub const VER_DEF_CURRENT: u16 = 1;
/// Given version number
pub const VER_DEF_NUM: u16 = 2;

// VerDef flags
/// Version definition of file itself
pub const VER_FLG_BASE: u16 = 0x1;
/// Weak version identifier
///
/// NOTE: Also valid for VerNeedAux vna_flags
pub const VER_FLG_WEAK: u16 = 0x2;
pub const VER_FLG_INFO: u16 = 0x4;

// VerSym symbol index values
/// Symbol is local and unversioned
pub const VER_NDX_LOCAL: u16 = 0;
/// Symbol is global and unversioned
pub const VER_NDX_GLOBAL: u16 = 1;
/// Beginning of reserved entries
pub const VER_NDX_LORESERVE: u16 = 0xff00;
/// Symbol is to be eliminated
pub const VER_NDX_ELIMINATE: u16 = 0xff01;

// VerSym symbol index masks
/// Version Index mask
pub const VERSYM_VERSION: u16 = 0x7fff;
/// Hidden bit (non-default version)
pub const VERSYM_HIDDEN: u16 = 0x8000;

// VerNeed vn_version values
/// No version
pub const VER_NEED_NONE: u16 = 0;
/// Current version
pub const VER_NEED_CURRENT: u16 = 1;
/// Given version number
pub const VER_NEED_NUM: u16 = 2;
