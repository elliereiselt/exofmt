use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct FieldIdItem {
    /// Index into the `type_ids` list for the definer of this field. This must be a class type, and not an array or primitive type
    pub class_index: u16,
    /// Index into the `type_ids` list for the type of this field
    pub type_index: u16,
    /// Index into the `string_ids` list for the name of this field. The string must conform to the syntax for [MemberName](https://source.android.com/docs/core/runtime/dex-format#membername)
    pub name_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<FieldIdItem>(), 0x8);
    }
}
