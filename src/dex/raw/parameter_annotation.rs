use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct ParameterAnnotation {
    pub method_index: u32,
    /// Offset from the start of the file to the list of annotations for the method parameters.
    /// The offset should be to a location in the data section.
    pub annotations_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_annotation_size_check() {
        assert_eq!(::std::mem::size_of::<ParameterAnnotation>(), 0x8);
    }
}
