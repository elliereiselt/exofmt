use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct StringIdItem {
    /// Offset from the start of the file to the string data for this item. The offset should be to a location in the `data` section
    pub string_data_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<StringIdItem>(), 0x4);
    }
}
