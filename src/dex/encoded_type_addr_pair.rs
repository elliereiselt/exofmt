pub struct EncodedTypeAddressPair {
    /// Index into the `type_ids` list for the type of the exception to catch
    pub type_index: u32,
    /// Bytecode address of the associated exception handler
    pub address: u32,
}
