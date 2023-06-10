use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AnnotationOffsetItem {
    /// Offset from the start of the file to an annotation. The offset should be to a
    /// location in the data section, and the format of the data at that location is
    /// specified by "annotation_item"
    pub annotation_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annotation_offset_item_size_check() {
        assert_eq!(::std::mem::size_of::<AnnotationOffsetItem>(), 0x4);
    }
}
