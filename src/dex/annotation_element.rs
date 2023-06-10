use crate::dex::EncodedValue;

pub struct AnnotationElement {
    /// Element name, represented as an index into the `string_ids` section. The string must conform to the syntax for [MemberName](https://source.android.com/docs/core/runtime/dex-format#membername)
    pub name_index: u32,
    /// Element value
    pub value: EncodedValue,
}
