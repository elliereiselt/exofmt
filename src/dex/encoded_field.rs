use crate::dex::HiddenApiRestriction;

pub struct EncodedField {
    /// Index into the field_ids list for the identity of this field (includes the name and descriptor)
    pub field_index: u32,
    /// Access flags for the field (`public`, `final`, etc.).
    pub access_flags: u32,
    pub hiddenapi_flag: Option<HiddenApiRestriction>,
}
