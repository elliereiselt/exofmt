use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct NoteHeader {
    pub n_namesz: u32,
    pub n_descsz: u32,
    pub n_type: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_header_size_check() {
        assert_eq!(::std::mem::size_of::<NoteHeader>(), 0xc);
    }
}
