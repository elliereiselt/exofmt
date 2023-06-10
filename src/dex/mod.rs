// NOTE: While the Dex spec says many of the sections below should exist, I've found that there are valid Dex files that exist that do not have some of these "required" sections.
//       As such, all sections which are "required" will still check if the offset is `0` and return an empty list on `0` rather than return `Error::Malformed`
pub mod raw;
pub mod uleb128p1;

mod header;
pub use header::*;
mod map_item;
pub use map_item::*;
mod string_id_item;
use num::Integer;
pub use string_id_item::*;
mod type_id_item;
pub use type_id_item::*;
mod proto_id_item;
pub use proto_id_item::*;
mod type_item;
pub use type_item::*;
mod string_data_item;
pub use string_data_item::*;
mod field_id_item;
pub use field_id_item::*;
mod method_id_item;
pub use method_id_item::*;
mod class_def_item;
pub use class_def_item::*;
mod annotations_directory_item;
pub use annotations_directory_item::*;
mod encoded_annotation;
pub use encoded_annotation::*;
mod annotation_element;
pub use annotation_element::*;
mod encoded_value;
pub use encoded_value::*;
mod annotation_item;
pub use annotation_item::*;
mod field_annotation;
pub use field_annotation::*;
mod method_annotation;
pub use method_annotation::*;
mod parameter_annotation;
pub use parameter_annotation::*;
mod class_data_item;
pub use class_data_item::*;
mod encoded_field;
pub use encoded_field::*;
mod code_item;
pub use code_item::*;
mod try_item;
pub use try_item::*;
mod encoded_catch_handler;
pub use encoded_catch_handler::*;
mod encoded_type_addr_pair;
pub use encoded_type_addr_pair::*;
mod debug_info_item;
pub use debug_info_item::*;
mod encoded_method;
pub use encoded_method::*;

use crate::leb128;
use crate::mutf8::MUTF8;
use crate::Error;
use scroll::{Endian, IOread};
use std::borrow::Cow;
use std::io::{Seek, SeekFrom};

type Result<T> = std::result::Result<T, Error>;

// TODO: What I'm thinking here is:
//        - `trait Reader<'a>` which has all of the required parsing functions
//        - `struct IoReader` which implements `Reader<'static>`
//        - `struct ByteReader<'a>` which implements `Reader<'a>`
pub struct Dex<'a> {
    pub header: Header,
    pub link_section: Cow<'a, [u8]>,
    pub map_list: Vec<MapItem>,
    pub string_ids: Vec<StringIdItem>,
    pub type_ids: Vec<TypeIdItem>,
    pub proto_ids: Vec<ProtoIdItem>,
    pub field_ids: Vec<FieldIdItem>,
    pub method_ids: Vec<MethodIdItem>,
    pub class_defs: Vec<ClassDefItem>,
}

/// Used to indicate that an index value is absent
pub const NO_INDEX: u32 = 0xffffffff;

/// Visibile anywhere
pub const ACC_PUBLIC: u32 = 0x1;
/// Only visible to defining class
pub const ACC_PRIVATE: u32 = 0x2;
/// Visible to package and subclasses
pub const ACC_PROTECTED: u32 = 0x4;
/// For Classes and InnerClass annotations: Not constructed with an outer `this` reference
///
/// For Fields: Global to defining class
///
/// For Methods: Does not take a `this` argument
pub const ACC_STATIC: u32 = 0x8;
/// For Classes and InnerClass annotations: Not subclassable
///
/// For Fields: Immutable after construction
///
/// For Methods: Not overridable
pub const ACC_FINAL: u32 = 0x10;
/// For Methods: Associated lock automatically acquired around call to this method
///
/// NOTE: This is only valid to set when `ACC_NATIVE` is also set
pub const ACC_SYNCHRONIZED: u32 = 0x20;
/// For Fields: Special access rules to help with thread safety
pub const ACC_VOLATILE: u32 = 0x40;
/// For Methods: Bridge method, added automatically by compiler as a type-safe bridge
pub const ACC_BRIDGE: u32 = 0x40;
/// For Fields: Not to be saved by default serialization
pub const ACC_TRANSIENT: u32 = 0x80;
/// For Methods: Last argument should be treated as a "rest" argument by compiler
pub const ACC_VARARGS: u32 = 0x80;
/// For Methods: Implemented in native code
pub const ACC_NATIVE: u32 = 0x100;
/// For Classes and InnerClass annotations: Multiply-implementable abstract class
pub const ACC_INTERFACE: u32 = 0x200;
/// For Classes and InnerClass annotations: Not directly instantiable
///
/// For Methods: Unimplemented by this class
pub const ACC_ABSTRACT: u32 = 0x400;
/// For Methods: Strict rules for floating-point arithmetic
pub const ACC_STRICT: u32 = 0x800;
/// Not directly defined in source code
pub const ACC_SYNTHETIC: u32 = 0x1000;
/// For Classes and InnerClass annotations: Declared as an annotation class
pub const ACC_ANNOTATION: u32 = 0x2000;
/// For Classes and InnerClass annotations: Declared as an enumerated type
///
/// For Fields: Declared as an enumerated value
pub const ACC_ENUM: u32 = 0x4000;
/// For Methods: Constructor method (class or instance initializer)
pub const ACC_CONSTRUCTOR: u32 = 0x10000;
/// For Methods: Declared synchronized
///
/// NOTE: This has no effect on execution (other than in reflection of this flag, per se).
pub const ACC_DECLARED_SYNCHRONIZED: u32 = 0x20000;

pub trait Reader<'a> {
    fn read_header(&mut self) -> Result<Header>;
    fn read_link_section(&mut self, link_size: u32, link_offset: u32) -> Result<Cow<'a, [u8]>>;
    fn read_map_list(&mut self, map_offset: u32) -> Result<Vec<MapItem>>;
    fn read_string_ids(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>>;
    fn read_type_ids(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>>;
    fn read_proto_ids(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>>;
    fn read_type_list(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>>;
    fn read_string_data_item(&mut self, string_data_offset: u32) -> Result<String>;
    fn read_field_ids(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>>;
    fn read_method_ids(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>>;

    fn read_encoded_value(&mut self) -> Result<EncodedValue>;
}

pub struct IoReader<'a, TRead: IOread<Endian> + Seek> {
    pub reader: &'a mut TRead,
    pub endianness: Endian,
    pub stream_len: u64,
}

macro_rules! io_read_section_as_array {
    ($reader:expr, $endianness:expr, $section_size:expr, $section_offset:expr, $index_type:ty) => {{
        if $section_size > 0 {
            let section_size = $section_size as usize;
            let section_offset = u64::from($section_offset);

            $reader.seek(SeekFrom::Start(section_offset))?;

            let mut section_items: Vec<$index_type> = Vec::with_capacity(section_size);

            for _ in 0..section_size {
                section_items.push($reader.ioread_with::<$index_type>($endianness)?);
            }

            Ok(section_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }};
}

macro_rules! io_read_section_as_list {
    ($reader:expr, $endianness:expr, $section_offset:expr, $index_type:ty) => {{
        if $section_offset > 0 {
            let section_offset = u64::from($section_offset);

            $reader.seek(SeekFrom::Start(section_offset))?;

            let size = $reader.ioread_with::<u32>($endianness)? as usize;
            let mut result_list: Vec<$index_type> = Vec::with_capacity(size);

            for _ in 0..size {
                result_list.push($reader.ioread_with::<$index_type>($endianness)?);
            }

            Ok(result_list)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }};
}

impl<'a, TRead: IOread<Endian> + Seek> IoReader<'a, TRead> {
    pub fn new(reader: &'a mut TRead) -> Result<Self> {
        let stream_len = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(0))?;
        Ok(Self {
            reader,
            // Start with `Little` at first, `read_header` will correct this
            endianness: Endian::Little,
            stream_len,
        })
    }

    pub fn read_dex(&mut self) -> Result<Dex> {
        // TODO: This should be reorganized to read in the most common ordering for Dex files
        let header = self.read_header()?;
        let link_section = self.read_link_section(header.link_size, header.link_offset)?;
        let map_list = self.read_map_list(header.map_offset)?;
        let string_ids = self.read_string_ids(header.string_ids_size, header.string_ids_offset)?;
        let type_ids = self.read_type_ids(header.type_ids_size, header.type_ids_offset)?;
        let proto_ids = self.read_proto_ids(header.proto_ids_size, header.proto_ids_offset)?;
        let field_ids = self.read_field_ids(header.field_ids_size, header.field_ids_offset)?;
        let method_ids = self.read_method_ids(header.method_ids_size, header.method_ids_offset)?;
        let class_defs = self.read_class_defs(header.class_defs_size, header.class_defs_offset)?;

        Ok(Dex {
            header,
            link_section,
            map_list,
            string_ids,
            type_ids,
            proto_ids,
            field_ids,
            method_ids,
            class_defs,
        })
    }

    pub fn read_header(&mut self) -> Result<Header> {
        // Start by reading everything as byte arrays until we know the "file endianness"
        let mut magic = [0u8; 8];
        let mut checksum = [0u8; 4];
        let mut signature = [0u8; 20];
        let mut file_size = [0u8; 4];
        let mut header_size = [0u8; 4];
        let mut endian_tag = [0u8; 4];

        self.reader.read_exact(&mut magic)?;

        // I don't bother to validate the Dex version number but maybe we should... TBD
        if magic[0..4] != [0x64, 0x65, 0x78, 0x0a] || magic[7] != 0 {
            return Err(Error::InvalidMagicNumber(u64::from_le_bytes(magic)));
        }

        self.reader.read_exact(&mut checksum)?;
        self.reader.read_exact(&mut signature)?;
        self.reader.read_exact(&mut file_size)?;
        self.reader.read_exact(&mut header_size)?;
        self.reader.read_exact(&mut endian_tag)?;

        self.endianness = if endian_tag == REVERSE_ENDIAN_CONSTANT_BYTES {
            Endian::Little
        } else if endian_tag == ENDIAN_CONSTANT_BYTES {
            Endian::Big
        } else {
            return Err(Error::Malformed(format!(
                "Dex file `endian_tag` field malformed - found `{:02x?}` but expected `{:02x?}` or `{:02x?}`",
                endian_tag, ENDIAN_CONSTANT_BYTES, REVERSE_ENDIAN_CONSTANT_BYTES,
            )));
        };

        let checksum = if self.endianness.is_little() {
            u32::from_le_bytes(checksum)
        } else {
            u32::from_be_bytes(checksum)
        };
        let file_size = if self.endianness.is_little() {
            u32::from_le_bytes(file_size)
        } else {
            u32::from_be_bytes(file_size)
        };
        let header_size = if self.endianness.is_little() {
            u32::from_le_bytes(header_size)
        } else {
            u32::from_be_bytes(header_size)
        };
        let endian_tag = if self.endianness.is_little() {
            u32::from_le_bytes(endian_tag)
        } else {
            u32::from_be_bytes(endian_tag)
        };
        let link_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let link_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let map_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let string_ids_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let string_ids_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let type_ids_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let type_ids_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let proto_ids_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let proto_ids_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let field_ids_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let field_ids_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let method_ids_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let method_ids_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let class_defs_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let class_defs_offset = self.reader.ioread_with::<u32>(self.endianness)?;
        let data_size = self.reader.ioread_with::<u32>(self.endianness)?;
        let data_offset = self.reader.ioread_with::<u32>(self.endianness)?;

        Ok(Header {
            magic,
            checksum,
            signature,
            file_size,
            header_size,
            endian_tag,
            link_size,
            link_offset,
            map_offset,
            string_ids_size,
            string_ids_offset,
            type_ids_size,
            type_ids_offset,
            proto_ids_size,
            proto_ids_offset,
            field_ids_size,
            field_ids_offset,
            method_ids_size,
            method_ids_offset,
            class_defs_size,
            class_defs_offset,
            data_size,
            data_offset,
        })
    }

    pub fn read_link_section(
        &mut self,
        link_size: u32,
        link_offset: u32,
    ) -> Result<Cow<'static, [u8]>> {
        if link_size > 0 {
            let link_size = link_size as usize;
            let link_offset = u64::from(link_offset);

            let mut result: Vec<u8> = Vec::with_capacity(link_size);
            result.resize(link_size, 0);

            self.reader.seek(SeekFrom::Start(link_offset))?;

            self.reader.read_exact(&mut result)?;

            Ok(Cow::Owned(result))
        } else {
            Ok(Cow::Owned(Vec::with_capacity(0)))
        }
    }

    pub fn read_map_list(&mut self, map_offset: u32) -> Result<Vec<MapItem>> {
        io_read_section_as_list!(self.reader, self.endianness, map_offset, MapItem)
    }

    pub fn read_string_ids(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>> {
        if string_ids_size > 0 {
            let string_ids_size = string_ids_size as usize;
            let string_ids_offset = u64::from(string_ids_offset);

            self.reader.seek(SeekFrom::Start(string_ids_offset))?;

            let mut string_id_items: Vec<StringIdItem> = Vec::with_capacity(string_ids_size);

            for _ in 0..string_ids_size {
                let raw_string_id_item = self
                    .reader
                    .ioread_with::<raw::StringIdItem>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let string_data_item =
                    self.read_string_data_item(raw_string_id_item.string_data_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                string_id_items.push(StringIdItem {
                    string_data: string_data_item,
                });
            }

            Ok(string_id_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_type_ids(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>> {
        io_read_section_as_array!(
            self.reader,
            self.endianness,
            type_ids_size,
            type_ids_offset,
            TypeIdItem
        )
    }

    pub fn read_proto_ids(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>> {
        if proto_ids_size > 0 {
            let proto_ids_size = proto_ids_size as usize;
            let proto_ids_offset = u64::from(proto_ids_offset);

            self.reader.seek(SeekFrom::Start(proto_ids_offset))?;

            let mut proto_id_items: Vec<ProtoIdItem> = Vec::with_capacity(proto_ids_size);

            for _ in 0..proto_ids_size {
                let raw_proto_id_item = self
                    .reader
                    .ioread_with::<raw::ProtoIdItem>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let type_list = self.read_type_list(raw_proto_id_item.parameters_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                proto_id_items.push(ProtoIdItem {
                    shorty_index: raw_proto_id_item.shorty_index,
                    return_type_index: raw_proto_id_item.return_type_index,
                    parameters: type_list,
                });
            }

            Ok(proto_id_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_type_list(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>> {
        io_read_section_as_list!(self.reader, self.endianness, type_list_offset, TypeItem)
    }

    fn read_string_data(&mut self) -> Result<Vec<u8>> {
        let mut input_buffer = [0u8; 1];
        let mut output_buffer: Vec<u8> = Vec::new();

        loop {
            self.reader.read_exact(&mut input_buffer)?;

            // I'm not keeping the nul byte, it isn't needed.
            if input_buffer[0] == 0 {
                break;
            } else {
                output_buffer.push(input_buffer[0]);
            }
        }

        Ok(output_buffer)
    }

    pub fn read_string_data_item(&mut self, string_data_offset: u32) -> Result<String> {
        self.reader
            .seek(SeekFrom::Start(u64::from(string_data_offset)))?;

        // We don't care about what appears to just be an optimization to the loader.
        let _ = leb128::decode_uleb128::<u32, TRead>(self.reader)?;
        let string_data = self.read_string_data()?;

        Ok(String::from_mutf8(&string_data)?)
    }

    pub fn read_field_ids(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>> {
        io_read_section_as_array!(
            self.reader,
            self.endianness,
            field_ids_size,
            field_ids_offset,
            FieldIdItem
        )
    }

    pub fn read_method_ids(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>> {
        io_read_section_as_array!(
            self.reader,
            self.endianness,
            method_ids_size,
            method_ids_offset,
            MethodIdItem
        )
    }

    pub fn read_encoded_value(&mut self) -> Result<EncodedValue> {
        let raw_value = self.reader.ioread_with::<u8>(self.endianness)?;
        let value_type = raw_value & 0x1f;
        let value_arg = raw_value >> 5;

        // TODO: I'm currently making the assumption that `endian_tag` applies here.
        //       Afaik, the only way to confirm this would be to check an `odex` file
        //       on a phone using big endian... which I think is only mips...
        macro_rules! read_integer {
            ($type:ty, $type_size:literal, $fill:literal, $value_type_name:literal) => {
                if value_arg > ($type_size - 1) {
                    Err(Error::Malformed(format!(
                        "Malformed encoded value with type `{}` has invalid size `{}`",
                        $value_type_name, value_arg,
                    )))
                } else {
                    let mut buffer: [u8; $type_size] = [$fill; $type_size];

                    let slice_start = ($type_size - 1) - value_arg as usize;
                    self.reader
                        .read_exact(&mut buffer[slice_start..$type_size])?;

                    match self.endianness {
                        Endian::Little => {
                            buffer.rotate_left(slice_start);
                            Ok(<$type>::from_le_bytes(buffer))
                        }
                        Endian::Big => Ok(<$type>::from_be_bytes(buffer)),
                    }
                }
            };
        }

        macro_rules! read_float {
            ($type:ty, $type_size:literal, $fill:literal, $value_type_name:literal) => {
                if value_arg > ($type_size - 1) {
                    Err(Error::Malformed(format!(
                        "Malformed encoded value with type `{}` has invalid size `{}`",
                        $value_type_name, value_arg,
                    )))
                } else {
                    let mut buffer: [u8; $type_size] = [$fill; $type_size];

                    let slice_end = value_arg as usize + 1;
                    self.reader.read_exact(&mut buffer[0..slice_end])?;

                    match self.endianness {
                        // TODO: Does this need rotate as well?
                        Endian::Little => Ok(<$type>::from_le_bytes(buffer)),
                        Endian::Big => Ok(<$type>::from_be_bytes(buffer)),
                    }
                }
            };
        }

        match value_type {
            raw::VALUE_BYTE => {
                // TODO: Should we confirm `value_arg` is `0`?
                Ok(EncodedValue::Byte(
                    self.reader.ioread_with::<i8>(self.endianness)?,
                ))
            }
            raw::VALUE_SHORT => Ok(EncodedValue::Short(read_integer!(
                i16,
                2,
                0xff,
                "VALUE_SHORT"
            )?)),
            raw::VALUE_CHAR => Ok(EncodedValue::Char(read_integer!(
                u16,
                2,
                0x00,
                "VALUE_CHAR"
            )?)),
            raw::VALUE_INT => Ok(EncodedValue::Int(read_integer!(i32, 4, 0xff, "VALUE_INT")?)),
            raw::VALUE_LONG => Ok(EncodedValue::Long(read_integer!(
                i64,
                8,
                0xff,
                "VALUE_LONG"
            )?)),
            raw::VALUE_FLOAT => Ok(EncodedValue::Float(read_float!(
                f32,
                4,
                0x00,
                "VALUE_FLOAT"
            )?)),
            raw::VALUE_DOUBLE => Ok(EncodedValue::Double(read_float!(
                f64,
                8,
                0x00,
                "VALUE_DOUBLE"
            )?)),
            raw::VALUE_METHOD_TYPE => Ok(EncodedValue::MethodType {
                proto_id_index: read_integer!(u32, 4, 0x00, "VALUE_METHOD_TYPE")?,
            }),
            raw::VALUE_METHOD_HANDLE => Ok(EncodedValue::MethodHandle {
                method_handle_index: read_integer!(u32, 4, 0x00, "VALUE_METHOD_HANDLE")?,
            }),
            raw::VALUE_STRING => Ok(EncodedValue::String {
                string_id_index: read_integer!(u32, 4, 0x00, "VALUE_STRING")?,
            }),
            raw::VALUE_TYPE => Ok(EncodedValue::Type {
                type_id_index: read_integer!(u32, 4, 0x00, "VALUE_TYPE")?,
            }),
            raw::VALUE_FIELD => Ok(EncodedValue::Field {
                field_id_index: read_integer!(u32, 4, 0x00, "VALUE_FIELD")?,
            }),
            raw::VALUE_METHOD => Ok(EncodedValue::Method {
                method_id_index: read_integer!(u32, 4, 0x00, "VALUE_METHOD")?,
            }),
            raw::VALUE_ENUM => Ok(EncodedValue::Enum {
                field_id_inex: read_integer!(u32, 4, 0x00, "VALUE_ENUM")?,
            }),
            // TODO: Should we confirm `value_arg` is `0`?
            raw::VALUE_ARRAY => Ok(EncodedValue::Array(self.read_encoded_array()?)),
            // TODO: Should we confirm `value_arg` is `0`?
            raw::VALUE_ANNOTATION => Ok(EncodedValue::Annotation(self.read_encoded_annotation()?)),
            // TODO: Should we confirm `value_arg` is `0`?
            raw::VALUE_NULL => Ok(EncodedValue::Null),
            // TODO: Should we confirm `value_arg` is either `0` or `1`? Right now we ignore incorrect values
            raw::VALUE_BOOLEAN => Ok(EncodedValue::Boolean(value_arg != 0)),
            unknown => Err(Error::Malformed(format!(
                "Unknown `value_type` value `{}` found in `read_encoded_value`",
                unknown,
            ))),
        }
    }

    pub fn read_encoded_array(&mut self) -> Result<Vec<EncodedValue>> {
        let size: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<EncodedValue> = Vec::with_capacity(size as usize);

        for _ in 0..size {
            result.push(self.read_encoded_value()?);
        }

        Ok(result)
    }

    pub fn read_encoded_annotation(&mut self) -> Result<EncodedAnnotation> {
        let type_index: u32 = leb128::decode_uleb128(self.reader)?;
        let size: u32 = leb128::decode_uleb128(self.reader)?;
        let mut elements: Vec<AnnotationElement> = Vec::with_capacity(size as usize);

        for _ in 0..size {
            elements.push(self.read_annotation_element()?);
        }

        Ok(EncodedAnnotation {
            type_index,
            elements,
        })
    }

    pub fn read_annotation_element(&mut self) -> Result<AnnotationElement> {
        let name_index: u32 = leb128::decode_uleb128(self.reader)?;
        let value = self.read_encoded_value()?;

        Ok(AnnotationElement { name_index, value })
    }

    pub fn read_annotation_set_ref_list(
        &mut self,
        annotations_offset: u32,
    ) -> Result<Vec<Vec<AnnotationItem>>> {
        if annotations_offset > 0 {
            self.reader
                .seek(SeekFrom::Start(u64::from(annotations_offset)))?;

            let size = self.reader.ioread_with::<u32>(self.endianness)?;
            let mut result: Vec<Vec<AnnotationItem>> = Vec::with_capacity(size as usize);

            for _ in 0..size {
                let annotation_set_ref_item = self
                    .reader
                    .ioread_with::<raw::AnnotationSetRefItem>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let annotation_set_item =
                    self.read_annotation_set_item(annotation_set_ref_item.annotations_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                result.push(annotation_set_item);
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_annotation_set_item(
        &mut self,
        annotations_offset: u32,
    ) -> Result<Vec<AnnotationItem>> {
        if annotations_offset > 0 {
            self.reader
                .seek(SeekFrom::Start(u64::from(annotations_offset)))?;

            let size = self.reader.ioread_with::<u32>(self.endianness)?;
            let mut result: Vec<AnnotationItem> = Vec::with_capacity(size as usize);

            for _ in 0..size {
                let annotation_off_item = self
                    .reader
                    .ioread_with::<raw::AnnotationOffsetItem>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let annotation_item =
                    self.read_annotation_item(annotation_off_item.annotation_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                result.push(annotation_item);
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_annotation_item(&mut self, annotation_offset: u32) -> Result<AnnotationItem> {
        self.reader
            .seek(SeekFrom::Start(u64::from(annotation_offset)))?;

        let visibility = self.reader.ioread_with::<u8>(self.endianness)?;
        let annotation = self.read_encoded_annotation()?;

        Ok(AnnotationItem {
            visibility,
            annotation,
        })
    }

    pub fn read_annotations_directory_item(
        &mut self,
        annotations_offset: u32,
    ) -> Result<AnnotationsDirectoryItem> {
        if annotations_offset > 0 {
            self.reader
                .seek(SeekFrom::Start(u64::from(annotations_offset)))?;

            let header = self
                .reader
                .ioread_with::<raw::AnnotationsDirectoryItemHeader>(self.endianness)?;
            // We will come back to `class_annotations_offset`, I just don't want to deal with the offset resetting stuff...
            let mut field_annotations: Vec<FieldAnnotation> =
                Vec::with_capacity(header.fields_size as usize);
            let mut method_annotations: Vec<MethodAnnotation> =
                Vec::with_capacity(header.annotated_methods_size as usize);
            let mut parameter_annotations: Vec<ParameterAnnotation> =
                Vec::with_capacity(header.annotated_parameters_size as usize);

            for _ in 0..header.fields_size {
                let raw_field_annotation = self
                    .reader
                    .ioread_with::<raw::FieldAnnotation>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let annotation_set_item =
                    self.read_annotation_set_item(raw_field_annotation.annotations_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                field_annotations.push(FieldAnnotation {
                    field_index: raw_field_annotation.field_index,
                    annotations: annotation_set_item,
                });
            }

            for _ in 0..header.annotated_methods_size {
                let raw_method_annotation = self
                    .reader
                    .ioread_with::<raw::MethodAnnotation>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let annotation_set_item =
                    self.read_annotation_set_item(raw_method_annotation.annotations_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                method_annotations.push(MethodAnnotation {
                    method_index: raw_method_annotation.method_index,
                    annotations: annotation_set_item,
                });
            }

            for _ in 0..header.annotated_parameters_size {
                let raw_parameter_annotation = self
                    .reader
                    .ioread_with::<raw::ParameterAnnotation>(self.endianness)?;

                let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                let annotation_set_ref_list =
                    self.read_annotation_set_ref_list(raw_parameter_annotation.annotations_offset)?;

                self.reader.seek(SeekFrom::Start(current_offset))?;

                parameter_annotations.push(ParameterAnnotation {
                    method_index: raw_parameter_annotation.method_index,
                    annotations: annotation_set_ref_list,
                });
            }

            let class_annotations =
                self.read_annotation_set_item(header.class_annotations_offset)?;

            Ok(AnnotationsDirectoryItem {
                class_annotations,
                field_annotations,
                method_annotations,
                parameter_annotations,
            })
        } else {
            Ok(AnnotationsDirectoryItem {
                class_annotations: Vec::with_capacity(0),
                field_annotations: Vec::with_capacity(0),
                method_annotations: Vec::with_capacity(0),
                parameter_annotations: Vec::with_capacity(0),
            })
        }
    }

    pub fn read_code_item(&mut self, code_item_offset: u32) -> Result<CodeItem> {
        assert!(code_item_offset != 0);

        self.reader
            .seek(SeekFrom::Start(u64::from(code_item_offset)))?;

        let code_item_header = self
            .reader
            .ioread_with::<raw::CodeItemHeader>(self.endianness)?;
        let mut instructions: Vec<u16> = Vec::with_capacity(code_item_header.insns_size as usize);

        for _ in 0..code_item_header.insns_size {
            instructions.push(self.reader.ioread_with::<u16>(self.endianness)?);
        }

        // If the instructions list is not zero or is odd then we have to parse a padding u16
        // to align the `tries` list properly
        if code_item_header.insns_size != 0 && code_item_header.insns_size.is_odd() {
            let _ = self.reader.ioread_with::<u16>(self.endianness)?;
        }

        let mut tries: Vec<TryItem> = Vec::with_capacity(usize::from(code_item_header.tries_size));

        for _ in 0..code_item_header.tries_size {
            // We insert with the raw `offset` still in use, we'll replace them while parsing the handlers later.
            tries.push(self.reader.ioread_with::<TryItem>(self.endianness)?);
        }

        let handlers_offset_start = self.reader.seek(SeekFrom::Current(0))?;
        let handlers_size = leb128::decode_uleb128::<u32, TRead>(self.reader)?;
        let mut handlers: Vec<EncodedCatchHandler> = Vec::with_capacity(handlers_size as usize);
        let mut handlers_offset_translations: Vec<(u64, u16)> =
            Vec::with_capacity(handlers_size as usize);

        for handlers_index in 0..handlers_size {
            let handlers_offset = self.reader.seek(SeekFrom::Current(0))? - handlers_offset_start;

            handlers_offset_translations.push((handlers_offset, handlers_index as u16));

            handlers.push(self.read_encoded_catch_handler()?);
        }

        for trie in &mut tries {
            for trans in &handlers_offset_translations {
                if trans.0 == trie.handler_index as u64 {
                    trie.handler_index = trans.1;
                    break;
                }
            }
        }

        let debug_info_item = if code_item_header.debug_info_offset > 0 {
            Some(self.read_debug_info_item(code_item_header.debug_info_offset)?)
        } else {
            None
        };

        Ok(CodeItem {
            registers_size: code_item_header.registers_size,
            debug_info: debug_info_item,
            instructions,
            tries,
            handlers,
        })
    }

    pub fn read_encoded_catch_handler(&mut self) -> Result<EncodedCatchHandler> {
        let size = leb128::decode_sleb128::<i32, TRead>(self.reader)?;
        let abs_size = size.abs() as usize;
        let mut handlers: Vec<EncodedTypeAddressPair> = Vec::with_capacity(abs_size);

        for _ in 0..abs_size {
            handlers.push(self.read_encoded_type_addr_pair()?);
        }

        let catch_all_address = if abs_size <= 0 {
            Some(leb128::decode_uleb128::<u32, TRead>(self.reader)?)
        } else {
            None
        };

        Ok(EncodedCatchHandler {
            handlers,
            catch_all_address,
        })
    }

    pub fn read_encoded_type_addr_pair(&mut self) -> Result<EncodedTypeAddressPair> {
        let type_index: u32 = leb128::decode_uleb128(self.reader)?;
        let address: u32 = leb128::decode_uleb128(self.reader)?;

        Ok(EncodedTypeAddressPair {
            type_index,
            address,
        })
    }

    pub fn read_debug_info_item(&mut self, debug_info_offset: u32) -> Result<DebugInfoItem> {
        assert!(debug_info_offset != 0);

        self.reader
            .seek(SeekFrom::Start(u64::from(debug_info_offset)))?;

        let line_start: u32 = leb128::decode_uleb128(self.reader)?;
        let parameters_size: u32 = leb128::decode_uleb128(self.reader)?;
        let mut parameters: Vec<uleb128p1::uleb128p1> =
            Vec::with_capacity(parameters_size as usize);

        for _ in 0..parameters_size {
            parameters.push(uleb128p1::uleb128p1::decode(self.reader)?)
        }

        let mut bytecode: Vec<u8> = Vec::new();
        let mut read_buffer = [0u8; 1];

        loop {
            self.reader.read_exact(&mut read_buffer)?;
            bytecode.push(read_buffer[0]);

            if read_buffer[0] == DBG_END_SEQUENCE {
                break;
            }
        }

        Ok(DebugInfoItem {
            line_start,
            parameters,
            bytecode,
        })
    }

    pub fn read_class_data_item(&mut self, class_data_offset: u32) -> Result<ClassDataItem> {
        assert!(class_data_offset != 0);

        self.reader
            .seek(SeekFrom::Start(u64::from(class_data_offset)))?;

        let static_fields_size: u32 = leb128::decode_uleb128(self.reader)?;
        let instance_fields_size: u32 = leb128::decode_uleb128(self.reader)?;
        let direct_methods_size: u32 = leb128::decode_uleb128(self.reader)?;
        let virtual_methods_size: u32 = leb128::decode_uleb128(self.reader)?;

        let mut static_fields: Vec<EncodedField> = Vec::with_capacity(static_fields_size as usize);
        let mut instance_fields: Vec<EncodedField> =
            Vec::with_capacity(instance_fields_size as usize);
        let mut direct_methods: Vec<EncodedMethod> =
            Vec::with_capacity(direct_methods_size as usize);
        let mut virtual_methods: Vec<EncodedMethod> =
            Vec::with_capacity(virtual_methods_size as usize);

        macro_rules! read_encoded_field {
            ($result_list:ident, $last_field_index:ident) => {
                let field_index_diff: u32 = leb128::decode_uleb128(self.reader)?;
                let field_index = $last_field_index + field_index_diff;
                let access_flags: u32 = leb128::decode_uleb128(self.reader)?;

                $result_list.push(EncodedField {
                    field_index,
                    access_flags,
                });

                $last_field_index = field_index;
            };
        }

        macro_rules! read_encoded_method {
            ($result_list:ident, $last_method_index:ident) => {
                let method_index_diff: u32 = leb128::decode_uleb128(self.reader)?;
                let method_index = $last_method_index + method_index_diff;
                let access_flags: u32 = leb128::decode_uleb128(self.reader)?;
                let code_offset: u32 = leb128::decode_uleb128(self.reader)?;

                let code_item = if code_offset > 0 {
                    let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                    let code_item = self.read_code_item(code_offset)?;

                    self.reader.seek(SeekFrom::Start(current_offset))?;

                    Some(code_item)
                } else {
                    None
                };

                $result_list.push(EncodedMethod {
                    method_index,
                    access_flags,
                    code: code_item,
                });

                $last_method_index = method_index;
            };
        }

        let mut last_static_field_index: u32 = 0;

        for _ in 0..static_fields_size {
            read_encoded_field!(static_fields, last_static_field_index);
        }

        let mut last_instance_field_index: u32 = 0;

        for _ in 0..instance_fields_size {
            read_encoded_field!(instance_fields, last_instance_field_index);
        }

        let mut last_direct_method_index: u32 = 0;

        for _ in 0..direct_methods_size {
            read_encoded_method!(direct_methods, last_direct_method_index);
        }

        let mut last_virtual_method_index: u32 = 0;

        for _ in 0..virtual_methods_size {
            read_encoded_method!(virtual_methods, last_virtual_method_index);
        }

        Ok(ClassDataItem {
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }

    pub fn read_class_defs(
        &mut self,
        class_defs_size: u32,
        class_defs_offset: u32,
    ) -> Result<Vec<ClassDefItem>> {
        if class_defs_offset > 0 {
            self.reader
                .seek(SeekFrom::Start(u64::from(class_defs_offset)))?;

            let mut result: Vec<ClassDefItem> = Vec::with_capacity(class_defs_size as usize);

            for _ in 0..class_defs_size {
                let raw_class_def_item = self
                    .reader
                    .ioread_with::<raw::ClassDefItem>(self.endianness)?;

                macro_rules! run_offset_call {
                    ($func_call:expr) => {{
                        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                        let result = $func_call;

                        self.reader.seek(SeekFrom::Start(current_offset))?;

                        result
                    }};
                }

                let interfaces =
                    run_offset_call!(self.read_type_list(raw_class_def_item.interfaces_offset))?;
                let annotations = if raw_class_def_item.annotations_offset > 0 {
                    Some(run_offset_call!(self.read_annotations_directory_item(
                        raw_class_def_item.annotations_offset
                    ))?)
                } else {
                    None
                };
                let class_data = if raw_class_def_item.class_data_offset > 0 {
                    Some(run_offset_call!(
                        self.read_class_data_item(raw_class_def_item.class_data_offset)
                    )?)
                } else {
                    None
                };
                let static_values = run_offset_call!(
                    self.read_encoded_array_item(raw_class_def_item.static_values_offset)
                )?;

                result.push(ClassDefItem {
                    class_index: raw_class_def_item.class_index,
                    access_flags: raw_class_def_item.access_flags,
                    superclass_index: raw_class_def_item.superclass_index,
                    interfaces,
                    source_file_index: raw_class_def_item.source_file_index,
                    annotations,
                    class_data,
                    static_values,
                })
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_encoded_array_item(&mut self, item_offset: u32) -> Result<Vec<EncodedValue>> {
        if item_offset > 0 {
            self.reader.seek(SeekFrom::Start(u64::from(item_offset)))?;

            Ok(self.read_encoded_array()?)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }
}

impl<'a, TRead: IOread<Endian> + Seek> Reader<'static> for IoReader<'a, TRead> {
    fn read_header(&mut self) -> Result<Header> {
        self.read_header()
    }

    fn read_link_section(
        &mut self,
        link_size: u32,
        link_offset: u32,
    ) -> Result<Cow<'static, [u8]>> {
        self.read_link_section(link_size, link_offset)
    }

    fn read_map_list(&mut self, map_offset: u32) -> Result<Vec<MapItem>> {
        self.read_map_list(map_offset)
    }

    fn read_string_ids(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>> {
        self.read_string_ids(string_ids_size, string_ids_offset)
    }

    fn read_type_ids(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>> {
        self.read_type_ids(type_ids_size, type_ids_offset)
    }

    fn read_proto_ids(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>> {
        self.read_proto_ids(proto_ids_size, proto_ids_offset)
    }

    fn read_type_list(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>> {
        self.read_type_list(type_list_offset)
    }

    fn read_string_data_item(&mut self, string_data_offset: u32) -> Result<String> {
        self.read_string_data_item(string_data_offset)
    }

    fn read_field_ids(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>> {
        self.read_field_ids(field_ids_size, field_ids_offset)
    }

    fn read_method_ids(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>> {
        self.read_method_ids(method_ids_size, method_ids_offset)
    }

    fn read_encoded_value(&mut self) -> Result<EncodedValue> {
        self.read_encoded_value()
    }
}
