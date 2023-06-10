use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct TypeItem {
    /// Index into the `type_ids` list
    pub type_index: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_item_size_check() {
        assert_eq!(::std::mem::size_of::<TypeItem>(), 0x2);
    }
}
