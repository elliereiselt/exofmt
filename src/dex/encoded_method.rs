use crate::dex::CodeItem;

pub struct EncodedMethod {
    pub method_index: u32,
    pub access_flags: u32,
    pub code: Option<CodeItem>,
}
