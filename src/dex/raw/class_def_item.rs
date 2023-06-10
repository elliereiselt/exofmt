use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct ClassDefItem {
    pub class_index: u32,
    pub access_flags: u32,
    pub superclass_index: u32,
    /// Offset from the start of the file to the list of interfaces, or `0` if there are none.
    /// This offset should be in the data section, and the data there should be in the format specified by "type_list".
    /// Each of the elements of the list must be a class type (not an array or primitive type), and there must not be any duplicates
    pub interfaces_offset: u32,
    pub source_file_index: u32,
    /// Offset from the start of the file to the annotations structure for this class, or `0` if there are no annotations on this class.
    /// This offset, if non-zero, should be in the data section, and the data there should be in the format specified by "annotations_directory_item",
    /// with all items referring to this class as the definer
    pub annotations_offset: u32,
    /// Offset from the start of the file to the associated class data for this item, or `0` if there is no class data for this class.
    /// (This may be the case, for example, if this class is a marker interface.)
    /// The offset, if non-zero, should be in the data section, and the data there should be in the format specified by "class_data_item",
    /// with all items referring to this class as the definer
    pub class_data_offset: u32,
    /// Offset from the start of the file to the list of initial values for static fields, or `0` if there are none
    /// (and all static fields are to be initialized with 0 or null).
    /// This offset should be in the data section, and the data there should be in the format specified by "encoded_array_item".
    /// The size of the array must be no larger than the number of `static` fields declared by this class,
    /// and the elements correspond to the `static` fields in the same order as declared in the corresponding `field_list`.
    /// The type of each array element must match the declared type of its corresponding field.
    /// If there are fewer elements in the array than there are static fields, then the leftover fields are initialized with a type-appropriate `0` or `null`
    pub static_values_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_def_item_size_check() {
        assert_eq!(::std::mem::size_of::<ClassDefItem>(), 0x20);
    }
}
