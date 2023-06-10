use crate::dex::AnnotationItem;

pub struct ParameterAnnotation {
    /// Index into the `method_ids` list for the identity of the method whose parameters are being annotated
    pub method_index: u32,
    /// List of annotations for the method
    pub annotations: Vec<Vec<AnnotationItem>>,
}
