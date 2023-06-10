use crate::dex::AnnotationItem;
use crate::dex::FieldAnnotation;
use crate::dex::MethodAnnotation;
use crate::dex::ParameterAnnotation;

pub struct AnnotationsDirectoryItem {
    /// Annotations made directly on the class
    pub class_annotations: Vec<AnnotationItem>,
    /// List of associated field annotations
    pub field_annotations: Vec<FieldAnnotation>,
    /// List of associated method annotations
    pub method_annotations: Vec<MethodAnnotation>,
    /// List of associated method parameter annotations
    pub parameter_annotations: Vec<ParameterAnnotation>,
}
