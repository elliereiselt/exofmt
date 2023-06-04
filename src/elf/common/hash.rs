use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct HashHeader {
    pub n_buckets: u32,
    pub n_chains: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_header_size_check() {
        assert_eq!(::std::mem::size_of::<HashHeader>(), 0x8);
    }
}
