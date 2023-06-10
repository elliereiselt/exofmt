use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct CodeItemHeader {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_offset: u32,
    pub insns_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_item_header_size_check() {
        assert_eq!(::std::mem::size_of::<CodeItemHeader>(), 0x10);
    }
}
