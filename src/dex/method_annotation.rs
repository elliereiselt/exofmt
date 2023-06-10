use crate::dex::AnnotationItem;

pub struct MethodAnnotation {
    /// Index into the `method_ids` list for the identity of the method being annotated
    pub method_index: u32,
    /// List of annotations for the method
    pub annotations: Vec<AnnotationItem>,
}
