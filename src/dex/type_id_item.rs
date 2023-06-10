use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct TypeIdItem {
    /// Index into the `string_ids` list for the descriptor string of this type. The string must conform to the syntax for [TypeDescriptor](https://source.android.com/docs/core/runtime/dex-format#typedescriptor)
    pub descriptor_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<TypeIdItem>(), 0x4);
    }
}
