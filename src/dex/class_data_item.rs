use crate::dex::EncodedField;
use crate::dex::EncodedMethod;

pub struct ClassDataItem {
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}
