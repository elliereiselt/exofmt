use crate::dex::EncodedAnnotation;

pub enum EncodedValue {
    Byte(i8),
    Short(i16),                                // TODO: Sign extended
    Char(u16),                                 // TODO: Zero extended
    Int(i32),                                  // TODO: Sign extended
    Long(i64),                                 // TODO: Sign extended
    Float(f32),                                // TODO: Zero-extended to the right
    Double(f64),                               // TODO: Zero-extended to the right
    MethodType { proto_id_index: u32 },        // TODO: Zero-extended
    MethodHandle { method_handle_index: u32 }, // TODO: Zero-extended
    String { string_id_index: u32 },           // TODO: Zero extended
    Type { type_id_index: u32 },               // TODO: Zero extended
    Field { field_id_index: u32 },             // TODO: Zero extended
    Method { method_id_index: u32 },           // TODO: Zero extended
    Enum { field_id_inex: u32 },               // TODO: Zero extended
    Array(Vec<EncodedValue>),
    Annotation(EncodedAnnotation),
    Null,
    Boolean(bool),
}
