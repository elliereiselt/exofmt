use crate::dex::TypeItem;

pub struct ProtoIdItem {
    /// Index into the `string_ids` list for the short-form descriptor string of this prototype.
    /// The string must conform to the syntax for [ShortyDescriptor](https://source.android.com/docs/core/runtime/dex-format#shortydescriptor)
    /// and must correspond to the return type and parameters of this item
    pub shorty_index: u32,
    /// Index into the `type_ids` list for the return type of this prototype
    pub return_type_index: u32,
    /// List of `TypeItem` values for the parameter types of this prototype
    pub parameters: Vec<TypeItem>,
}
