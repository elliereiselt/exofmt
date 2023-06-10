use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AnnotationSetRefItem {
    /// Offset from the start of the file to the referenced annotation set or `0` if there are no annotations for this element.
    /// The offset, if non-zero, should be to a location in the data section.
    pub annotations_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annotation_set_ref_item_size_check() {
        assert_eq!(::std::mem::size_of::<AnnotationSetRefItem>(), 0x4);
    }
}
