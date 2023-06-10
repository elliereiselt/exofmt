use crate::dex::EncodedTypeAddressPair;

pub struct EncodedCatchHandler {
    pub handlers: Vec<EncodedTypeAddressPair>,
    pub catch_all_address: Option<u32>,
}
