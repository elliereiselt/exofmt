use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerSym {
    pub vs_val: u16,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerDef {
    pub vd_version: u16,
    pub vd_flags: u16,
    pub vd_ndx: u16,
    pub vd_cnt: u16,
    pub vd_hash: u32,
    pub vd_aux: u32,
    pub vd_next: u32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerDefAux {
    pub vda_name: u32,
    pub vda_next: u32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerNeed {
    pub vn_version: u16,
    pub vn_cnt: u16,
    pub vn_file: u32,
    pub vn_aux: u32,
    pub vn_next: u32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct VerNeedAux {
    pub vna_hash: u32,
    pub vna_flags: u16,
    pub vna_other: u16,
    pub vna_name: u32,
    pub vna_next: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ver_sym_size_check() {
        assert_eq!(::std::mem::size_of::<VerSym>(), 0x2);
    }

    #[test]
    fn ver_def_size_check() {
        assert_eq!(::std::mem::size_of::<VerDef>(), 0x14);
    }

    #[test]
    fn ver_def_aux_size_check() {
        assert_eq!(::std::mem::size_of::<VerDefAux>(), 0x8);
    }

    #[test]
    fn ver_need_size_check() {
        assert_eq!(::std::mem::size_of::<VerNeed>(), 0x10);
    }

    #[test]
    fn ver_need_aux_size_check() {
        assert_eq!(::std::mem::size_of::<VerNeedAux>(), 0x10);
    }
}

impl From<crate::elf::gnu::VerDef> for VerDef {
    fn from(value: crate::elf::gnu::VerDef) -> Self {
        Self {
            vd_version: value.vd_version,
            vd_flags: value.vd_flags,
            vd_ndx: value.vd_ndx,
            vd_cnt: value.vd_cnt,
            vd_hash: value.vd_hash,
            vd_aux: value.vd_aux,
            vd_next: value.vd_next,
        }
    }
}

impl From<VerDef> for crate::elf::gnu::VerDef {
    fn from(value: VerDef) -> Self {
        Self {
            vd_version: value.vd_version,
            vd_flags: value.vd_flags,
            vd_ndx: value.vd_ndx,
            vd_cnt: value.vd_cnt,
            vd_hash: value.vd_hash,
            vd_aux: value.vd_aux,
            vd_next: value.vd_next,
        }
    }
}

impl From<crate::elf::gnu::VerDefAux> for VerDefAux {
    fn from(value: crate::elf::gnu::VerDefAux) -> Self {
        Self {
            vda_name: value.vda_name,
            vda_next: value.vda_next,
        }
    }
}

impl From<VerDefAux> for crate::elf::gnu::VerDefAux {
    fn from(value: VerDefAux) -> Self {
        Self {
            vda_name: value.vda_name,
            vda_next: value.vda_next,
        }
    }
}

impl From<crate::elf::gnu::VerNeed> for VerNeed {
    fn from(value: crate::elf::gnu::VerNeed) -> Self {
        Self {
            vn_version: value.vn_version,
            vn_cnt: value.vn_cnt,
            vn_file: value.vn_file,
            vn_aux: value.vn_aux,
            vn_next: value.vn_next,
        }
    }
}

impl From<VerNeed> for crate::elf::gnu::VerNeed {
    fn from(value: VerNeed) -> Self {
        Self {
            vn_version: value.vn_version,
            vn_cnt: value.vn_cnt,
            vn_file: value.vn_file,
            vn_aux: value.vn_aux,
            vn_next: value.vn_next,
        }
    }
}

impl From<crate::elf::gnu::VerNeedAux> for VerNeedAux {
    fn from(value: crate::elf::gnu::VerNeedAux) -> Self {
        Self {
            vna_hash: value.vna_hash,
            vna_flags: value.vna_flags,
            vna_other: value.vna_other,
            vna_name: value.vna_name,
            vna_next: value.vna_next,
        }
    }
}

impl From<VerNeedAux> for crate::elf::gnu::VerNeedAux {
    fn from(value: VerNeedAux) -> Self {
        Self {
            vna_hash: value.vna_hash,
            vna_flags: value.vna_flags,
            vna_other: value.vna_other,
            vna_name: value.vna_name,
            vna_next: value.vna_next,
        }
    }
}
