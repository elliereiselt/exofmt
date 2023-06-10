use crate::dex::AnnotationItem;

pub struct FieldAnnotation {
    /// Index into the `field_ids` list for the identity of the field being annotated
    pub field_index: u32,
    /// List of annotations for the field
    pub annotations: Vec<AnnotationItem>,
}
