use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_check() {
        assert_eq!(::std::mem::size_of::<Header>(), 0x34);
    }
}

impl TryFrom<crate::elf::Header> for Header {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::Header) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            e_ident: value.e_ident,
            e_type: value.e_type,
            e_machine: value.e_machine,
            e_version: value.e_version,
            e_entry: u32::try_from(value.e_entry)?,
            e_phoff: u32::try_from(value.e_phoff)?,
            e_shoff: u32::try_from(value.e_shoff)?,
            e_flags: value.e_flags,
            e_ehsize: value.e_ehsize,
            e_phentsize: value.e_phentsize,
            e_phnum: value.e_phnum,
            e_shentsize: value.e_shentsize,
            e_shnum: value.e_shnum,
            e_shstrndx: value.e_shstrndx,
        })
    }
}

impl From<Header> for crate::elf::Header {
    fn from(value: Header) -> Self {
        Self {
            e_ident: value.e_ident,
            e_type: value.e_type,
            e_machine: value.e_machine,
            e_version: value.e_version,
            e_entry: u64::from(value.e_entry),
            e_phoff: u64::from(value.e_phoff),
            e_shoff: u64::from(value.e_shoff),
            e_flags: value.e_flags,
            e_ehsize: value.e_ehsize,
            e_phentsize: value.e_phentsize,
            e_phnum: value.e_phnum,
            e_shentsize: value.e_shentsize,
            e_shnum: value.e_shnum,
            e_shstrndx: value.e_shstrndx,
        }
    }
}
