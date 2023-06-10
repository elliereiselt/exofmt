use crate::dex::AnnotationElement;

pub struct EncodedAnnotation {
    /// Type of the annotation. This must be a class (not array or primitive) type
    pub type_index: u32,
    /// Elements of the annotation, represented directly in-line (not as offsets).
    /// Elements must be sorted in increasing order by `name_index`
    pub elements: Vec<AnnotationElement>,
}
