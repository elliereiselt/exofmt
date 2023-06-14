use crate::dex::CodeItem;
use crate::dex::HiddenApiRestriction;

pub struct EncodedMethod {
    pub method_index: u32,
    pub access_flags: u32,
    pub code: Option<CodeItem>,
    pub hiddenapi_flag: Option<HiddenApiRestriction>,
}
