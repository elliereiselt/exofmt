use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct MethodAnnotation {
    pub method_index: u32,
    /// Offset from the start of the file to the list of annotations for the method.
    /// The offset should be to a location in the data section.
    pub annotations_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_annotation_size_check() {
        assert_eq!(::std::mem::size_of::<MethodAnnotation>(), 0x8);
    }
}
