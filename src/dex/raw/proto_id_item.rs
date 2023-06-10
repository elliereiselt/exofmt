use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct ProtoIdItem {
    pub shorty_index: u32,
    pub return_type_index: u32,
    /// Offset from the start of the file to the list of parameter types for this prototype, or 0 if this prototype has no parameters
    pub parameters_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proto_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<ProtoIdItem>(), 0xc);
    }
}
