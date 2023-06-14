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
mod call_site_id_item;
pub use call_site_id_item::*;
mod method_handle_item;
pub use method_handle_item::*;
mod hiddenapi_flag;
pub use hiddenapi_flag::*;

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
    pub call_site_ids: Vec<CallSiteIdItem>,
    pub method_handles: Vec<MethodHandleItem>,
    pub data_section: Cow<'a, [u8]>,
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
    fn read_link_section_at(&mut self, link_size: u32, link_offset: u32) -> Result<Cow<'a, [u8]>>;
    fn read_map_list_at(&mut self, map_offset: u32) -> Result<Vec<MapItem>>;
    fn read_string_ids_at(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>>;
    fn read_type_ids_at(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>>;
    fn read_proto_ids_at(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>>;
    fn read_type_list_at(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>>;
    fn read_string_data_item_at(&mut self, string_data_offset: u32) -> Result<String>;
    fn read_field_ids_at(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>>;
    fn read_method_ids_at(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>>;
}

pub struct IoReader<'a, TRead: IOread<Endian> + Seek> {
    pub reader: &'a mut TRead,
    pub endianness: Endian,
    pub stream_len: u64,
}

macro_rules! io_read_section_as_array_at {
    ($reader:expr, $endianness:expr, $section_size:expr, $section_offset:expr, $index_type:ty) => {{
        if $section_size > 0 {
            let section_size = $section_size as usize;
            let section_offset = u64::from($section_offset);

            let current_offset = $reader.seek(SeekFrom::Current(0))?;

            $reader.seek(SeekFrom::Start(section_offset))?;

            let mut section_items: Vec<$index_type> = Vec::with_capacity(section_size);

            for _ in 0..section_size {
                section_items.push($reader.ioread_with::<$index_type>($endianness)?);
            }

            $reader.seek(SeekFrom::Start(current_offset))?;

            Ok(section_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }};
}

macro_rules! io_read_section_as_list_at {
    ($reader:expr, $endianness:expr, $section_offset:expr, $index_type:ty) => {{
        if $section_offset > 0 {
            let section_offset = u64::from($section_offset);

            let current_offset = $reader.seek(SeekFrom::Current(0))?;

            $reader.seek(SeekFrom::Start(section_offset))?;

            let size = $reader.ioread_with::<u32>($endianness)? as usize;
            let mut result_list: Vec<$index_type> = Vec::with_capacity(size);

            for _ in 0..size {
                result_list.push($reader.ioread_with::<$index_type>($endianness)?);
            }

            $reader.seek(SeekFrom::Start(current_offset))?;

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
        self.reader.seek(SeekFrom::Start(0))?;

        let header = self.read_header()?;
        let string_ids =
            self.read_string_ids_at(header.string_ids_size, header.string_ids_offset)?;
        let type_ids = self.read_type_ids_at(header.type_ids_size, header.type_ids_offset)?;
        let proto_ids = self.read_proto_ids_at(header.proto_ids_size, header.proto_ids_offset)?;
        let field_ids = self.read_field_ids_at(header.field_ids_size, header.field_ids_offset)?;
        let method_ids =
            self.read_method_ids_at(header.method_ids_size, header.method_ids_offset)?;
        let mut class_defs =
            self.read_class_defs_at(header.class_defs_size, header.class_defs_offset)?;
        let link_section = self.read_link_section_at(header.link_size, header.link_offset)?;
        let data_section = self.read_data_section_at(header.data_size, header.data_offset)?;
        let map_list = self.read_map_list_at(header.map_offset)?;

        let mut call_site_ids: Option<Vec<CallSiteIdItem>> = None;
        let mut method_handles: Option<Vec<MethodHandleItem>> = None;

        for map_item in &map_list {
            if map_item.type_code == TYPE_CALL_SITE_ID_ITEM {
                call_site_ids = Some(self.read_call_site_ids_at(map_item.size, map_item.offset)?);
            } else if map_item.type_code == TYPE_METHOD_HANDLE_ITEM {
                method_handles = Some(self.read_method_handles_at(map_item.size, map_item.offset)?);
            } else if map_item.type_code == TYPE_HIDDENAPI_CLASS_DATA_ITEM {
                self.read_hiddenapi_data_at(map_item.size, map_item.offset, &mut class_defs)?;
            }
        }

        let call_site_ids = if let Some(call_site_ids) = call_site_ids {
            call_site_ids
        } else {
            Vec::with_capacity(0)
        };
        let method_handles = if let Some(method_handles) = method_handles {
            method_handles
        } else {
            Vec::with_capacity(0)
        };

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
            call_site_ids,
            method_handles,
            data_section,
        })
    }

    pub fn read_header(&mut self) -> Result<Header> {
        // Start by reading everything as byte arrays until we know the "file endianness"
        self.reader.seek(SeekFrom::Start(0))?;

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

    pub fn read_link_section_at(
        &mut self,
        link_size: u32,
        link_offset: u32,
    ) -> Result<Cow<'static, [u8]>> {
        if link_size > 0 {
            let link_size = link_size as usize;
            let link_offset = u64::from(link_offset);

            let mut result: Vec<u8> = Vec::with_capacity(link_size);
            result.resize(link_size, 0);

            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader.seek(SeekFrom::Start(link_offset))?;

            self.reader.read_exact(&mut result)?;

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(Cow::Owned(result))
        } else {
            Ok(Cow::Owned(Vec::with_capacity(0)))
        }
    }

    pub fn read_data_section_at(
        &mut self,
        data_size: u32,
        data_offset: u32,
    ) -> Result<Cow<'static, [u8]>> {
        if data_size > 0 {
            let data_size = data_size as usize;
            let data_offset = u64::from(data_offset);

            let mut result: Vec<u8> = Vec::with_capacity(data_size);
            result.resize(data_size, 0);

            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader.seek(SeekFrom::Start(data_offset))?;

            self.reader.read_exact(&mut result)?;

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(Cow::Owned(result))
        } else {
            Ok(Cow::Owned(Vec::with_capacity(0)))
        }
    }

    pub fn read_map_list_at(&mut self, map_offset: u32) -> Result<Vec<MapItem>> {
        io_read_section_as_list_at!(self.reader, self.endianness, map_offset, MapItem)
    }

    pub fn read_string_ids_at(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>> {
        if string_ids_size > 0 {
            let string_ids_size = string_ids_size as usize;
            let string_ids_offset = u64::from(string_ids_offset);

            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader.seek(SeekFrom::Start(string_ids_offset))?;

            let mut string_id_items: Vec<StringIdItem> = Vec::with_capacity(string_ids_size);

            for _ in 0..string_ids_size {
                let raw_string_id_item = self
                    .reader
                    .ioread_with::<raw::StringIdItem>(self.endianness)?;

                let string_data_item =
                    self.read_string_data_item_at(raw_string_id_item.string_data_offset)?;

                string_id_items.push(StringIdItem {
                    string_data: string_data_item,
                });
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(string_id_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_type_ids_at(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>> {
        io_read_section_as_array_at!(
            self.reader,
            self.endianness,
            type_ids_size,
            type_ids_offset,
            TypeIdItem
        )
    }

    pub fn read_proto_ids_at(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>> {
        if proto_ids_size > 0 {
            let proto_ids_size = proto_ids_size as usize;
            let proto_ids_offset = u64::from(proto_ids_offset);

            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader.seek(SeekFrom::Start(proto_ids_offset))?;

            let mut proto_id_items: Vec<ProtoIdItem> = Vec::with_capacity(proto_ids_size);

            for _ in 0..proto_ids_size {
                let raw_proto_id_item = self
                    .reader
                    .ioread_with::<raw::ProtoIdItem>(self.endianness)?;

                let type_list = self.read_type_list_at(raw_proto_id_item.parameters_offset)?;

                proto_id_items.push(ProtoIdItem {
                    shorty_index: raw_proto_id_item.shorty_index,
                    return_type_index: raw_proto_id_item.return_type_index,
                    parameters: type_list,
                });
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(proto_id_items)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_type_list_at(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>> {
        io_read_section_as_list_at!(self.reader, self.endianness, type_list_offset, TypeItem)
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

    pub fn read_string_data_item_at(&mut self, string_data_offset: u32) -> Result<String> {
        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

        self.reader
            .seek(SeekFrom::Start(u64::from(string_data_offset)))?;

        // We don't care about what appears to just be an optimization to the loader.
        let _ = leb128::decode_uleb128::<u32, TRead>(self.reader)?;
        let string_data = self.read_string_data()?;

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(String::from_mutf8(&string_data)?)
    }

    pub fn read_field_ids_at(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>> {
        io_read_section_as_array_at!(
            self.reader,
            self.endianness,
            field_ids_size,
            field_ids_offset,
            FieldIdItem
        )
    }

    pub fn read_method_ids_at(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>> {
        io_read_section_as_array_at!(
            self.reader,
            self.endianness,
            method_ids_size,
            method_ids_offset,
            MethodIdItem
        )
    }

    fn read_encoded_value(&mut self) -> Result<EncodedValue> {
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

    fn read_encoded_array(&mut self) -> Result<Vec<EncodedValue>> {
        let size: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<EncodedValue> = Vec::with_capacity(size as usize);

        for _ in 0..size {
            result.push(self.read_encoded_value()?);
        }

        Ok(result)
    }

    fn read_encoded_annotation(&mut self) -> Result<EncodedAnnotation> {
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

    fn read_annotation_element(&mut self) -> Result<AnnotationElement> {
        let name_index: u32 = leb128::decode_uleb128(self.reader)?;
        let value = self.read_encoded_value()?;

        Ok(AnnotationElement { name_index, value })
    }

    pub fn read_annotation_set_ref_list_at(
        &mut self,
        annotations_offset: u32,
    ) -> Result<Vec<Vec<AnnotationItem>>> {
        if annotations_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader
                .seek(SeekFrom::Start(u64::from(annotations_offset)))?;

            let size = self.reader.ioread_with::<u32>(self.endianness)?;
            let mut result: Vec<Vec<AnnotationItem>> = Vec::with_capacity(size as usize);

            for _ in 0..size {
                let annotation_set_ref_item = self
                    .reader
                    .ioread_with::<raw::AnnotationSetRefItem>(self.endianness)?;

                let annotation_set_item =
                    self.read_annotation_set_item_at(annotation_set_ref_item.annotations_offset)?;

                result.push(annotation_set_item);
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_annotation_set_item_at(
        &mut self,
        annotations_offset: u32,
    ) -> Result<Vec<AnnotationItem>> {
        if annotations_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader
                .seek(SeekFrom::Start(u64::from(annotations_offset)))?;

            let size = self.reader.ioread_with::<u32>(self.endianness)?;
            let mut result: Vec<AnnotationItem> = Vec::with_capacity(size as usize);

            for _ in 0..size {
                let annotation_off_item = self
                    .reader
                    .ioread_with::<raw::AnnotationOffsetItem>(self.endianness)?;

                let annotation_item =
                    self.read_annotation_item_at(annotation_off_item.annotation_offset)?;

                result.push(annotation_item);
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_annotation_item_at(&mut self, annotation_offset: u32) -> Result<AnnotationItem> {
        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

        self.reader
            .seek(SeekFrom::Start(u64::from(annotation_offset)))?;

        let visibility = self.reader.ioread_with::<u8>(self.endianness)?;
        let annotation = self.read_encoded_annotation()?;

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(AnnotationItem {
            visibility,
            annotation,
        })
    }

    pub fn read_annotations_directory_item_at(
        &mut self,
        annotations_offset: u32,
    ) -> Result<AnnotationsDirectoryItem> {
        if annotations_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

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

                let annotation_set_item =
                    self.read_annotation_set_item_at(raw_field_annotation.annotations_offset)?;

                field_annotations.push(FieldAnnotation {
                    field_index: raw_field_annotation.field_index,
                    annotations: annotation_set_item,
                });
            }

            for _ in 0..header.annotated_methods_size {
                let raw_method_annotation = self
                    .reader
                    .ioread_with::<raw::MethodAnnotation>(self.endianness)?;

                let annotation_set_item =
                    self.read_annotation_set_item_at(raw_method_annotation.annotations_offset)?;

                method_annotations.push(MethodAnnotation {
                    method_index: raw_method_annotation.method_index,
                    annotations: annotation_set_item,
                });
            }

            for _ in 0..header.annotated_parameters_size {
                let raw_parameter_annotation = self
                    .reader
                    .ioread_with::<raw::ParameterAnnotation>(self.endianness)?;

                let annotation_set_ref_list = self
                    .read_annotation_set_ref_list_at(raw_parameter_annotation.annotations_offset)?;

                parameter_annotations.push(ParameterAnnotation {
                    method_index: raw_parameter_annotation.method_index,
                    annotations: annotation_set_ref_list,
                });
            }

            let class_annotations =
                self.read_annotation_set_item_at(header.class_annotations_offset)?;

            self.reader.seek(SeekFrom::Start(current_offset))?;

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

    pub fn read_code_item_at(&mut self, code_item_offset: u32) -> Result<CodeItem> {
        assert!(code_item_offset != 0);

        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

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
            Some(self.read_debug_info_item_at(code_item_header.debug_info_offset)?)
        } else {
            None
        };

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(CodeItem {
            registers_size: code_item_header.registers_size,
            debug_info: debug_info_item,
            instructions,
            tries,
            handlers,
        })
    }

    fn read_encoded_catch_handler(&mut self) -> Result<EncodedCatchHandler> {
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

    fn read_encoded_type_addr_pair(&mut self) -> Result<EncodedTypeAddressPair> {
        let type_index: u32 = leb128::decode_uleb128(self.reader)?;
        let address: u32 = leb128::decode_uleb128(self.reader)?;

        Ok(EncodedTypeAddressPair {
            type_index,
            address,
        })
    }

    pub fn read_debug_info_item_at(&mut self, debug_info_offset: u32) -> Result<DebugInfoItem> {
        assert!(debug_info_offset != 0);

        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

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

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(DebugInfoItem {
            line_start,
            parameters,
            bytecode,
        })
    }

    pub fn read_class_data_item_at(&mut self, class_data_offset: u32) -> Result<ClassDataItem> {
        assert!(class_data_offset != 0);

        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

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
                    hiddenapi_flag: None,
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
                    let code_item = self.read_code_item_at(code_offset)?;

                    Some(code_item)
                } else {
                    None
                };

                $result_list.push(EncodedMethod {
                    method_index,
                    access_flags,
                    code: code_item,
                    hiddenapi_flag: None,
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

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(ClassDataItem {
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }

    pub fn read_class_defs_at(
        &mut self,
        class_defs_size: u32,
        class_defs_offset: u32,
    ) -> Result<Vec<ClassDefItem>> {
        if class_defs_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader
                .seek(SeekFrom::Start(u64::from(class_defs_offset)))?;

            let mut result: Vec<ClassDefItem> = Vec::with_capacity(class_defs_size as usize);

            for _ in 0..class_defs_size {
                let raw_class_def_item = self
                    .reader
                    .ioread_with::<raw::ClassDefItem>(self.endianness)?;

                let interfaces = self.read_type_list_at(raw_class_def_item.interfaces_offset)?;
                let annotations = if raw_class_def_item.annotations_offset > 0 {
                    Some(self.read_annotations_directory_item_at(
                        raw_class_def_item.annotations_offset,
                    )?)
                } else {
                    None
                };
                let class_data = if raw_class_def_item.class_data_offset > 0 {
                    Some(self.read_class_data_item_at(raw_class_def_item.class_data_offset)?)
                } else {
                    None
                };
                let static_values =
                    self.read_encoded_array_item_at(raw_class_def_item.static_values_offset)?;

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

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_encoded_array_item_at(&mut self, item_offset: u32) -> Result<Vec<EncodedValue>> {
        if item_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader.seek(SeekFrom::Start(u64::from(item_offset)))?;

            let result = self.read_encoded_array()?;

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_call_site_ids_at(
        &mut self,
        call_site_ids_size: u32,
        call_site_ids_offset: u32,
    ) -> Result<Vec<CallSiteIdItem>> {
        if call_site_ids_offset > 0 {
            let current_offset = self.reader.seek(SeekFrom::Current(0))?;

            self.reader
                .seek(SeekFrom::Start(u64::from(call_site_ids_offset)))?;

            let mut result: Vec<CallSiteIdItem> = Vec::with_capacity(call_site_ids_size as usize);

            for _ in 0..call_site_ids_size {
                result.push(self.read_call_site_id_item()?);
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    fn read_call_site_id_item(&mut self) -> Result<CallSiteIdItem> {
        let call_site_offset = self.reader.ioread_with::<u32>(self.endianness)?;

        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

        let result = self.read_call_site_item_at(call_site_offset)?;

        self.reader.seek(SeekFrom::Start(current_offset))?;

        Ok(CallSiteIdItem { values: result })
    }

    pub fn read_call_site_item_at(&mut self, call_site_offset: u32) -> Result<Vec<EncodedValue>> {
        self.read_encoded_array_item_at(call_site_offset)
    }

    pub fn read_method_handles_at(
        &mut self,
        method_handles_size: u32,
        method_handles_offset: u32,
    ) -> Result<Vec<MethodHandleItem>> {
        io_read_section_as_array_at!(
            self.reader,
            self.endianness,
            method_handles_size,
            method_handles_offset,
            MethodHandleItem
        )
    }

    pub fn read_hiddenapi_data_at(
        &mut self,
        hiddenapi_size: u32,
        hiddenapi_offset: u32,
        class_defs: &mut Vec<ClassDefItem>,
    ) -> Result<()> {
        if hiddenapi_offset > 0 {
            if hiddenapi_size != 1 {
                return Err(Error::Malformed(format!(
                    "Map list items of `TYPE_HIDDENAPI_CLASS_DATA_ITEM` should only have a size value of `1`, found `{}`",
                    hiddenapi_size
                )));
            }

            let current_offset = self.reader.seek(SeekFrom::Current(0))?;
            // Afaik `hiddenapi_offset` is the offset we are to use for the `offsets` list...
            // I'm unsure, though, as the documentation for this is shit.
            let hiddenapi_offset = u64::from(hiddenapi_offset);

            self.reader
                .seek(SeekFrom::Start(u64::from(hiddenapi_offset)))?;

            // TODO: Maybe double check we don't overflow past this?
            let _section_size = self.reader.ioread_with::<u32>(self.endianness)?;
            // This is used for yet another `seek` operation when indexing `offsets` array.
            // TODO: It might be more performant to request the bytes for this section and just handle it that way
            //       rather than constant seeking. Only if it becomes a performance issue, of course.
            let offsets_start_offset = u64::from(self.reader.ioread_with::<u32>(self.endianness)?);

            for class_def in class_defs {
                // Jump to the offset for this `class_index`...
                self.reader.seek(SeekFrom::Start(
                    offsets_start_offset
                        + (u64::from(class_def.class_index) * (std::mem::size_of::<u32>() as u64)),
                ))?;

                let flags_offset = u64::from(self.reader.ioread_with::<u32>(self.endianness)?);

                if flags_offset != 0 {
                    if let Some(class_data) = &mut class_def.class_data {
                        let current_offset = self.reader.seek(SeekFrom::Current(0))?;

                        self.reader
                            .seek(SeekFrom::Start(hiddenapi_offset + flags_offset))?;

                        for static_field in &mut class_data.static_fields {
                            let flag: HiddenApiRestriction = leb128::decode_uleb128(self.reader)?;
                            static_field.hiddenapi_flag = Some(flag);
                        }

                        for instance_field in &mut class_data.instance_fields {
                            let flag: HiddenApiRestriction = leb128::decode_uleb128(self.reader)?;
                            instance_field.hiddenapi_flag = Some(flag);
                        }

                        for direct_method in &mut class_data.direct_methods {
                            let flag: HiddenApiRestriction = leb128::decode_uleb128(self.reader)?;
                            direct_method.hiddenapi_flag = Some(flag);
                        }

                        for virtual_method in &mut class_data.virtual_methods {
                            let flag: HiddenApiRestriction = leb128::decode_uleb128(self.reader)?;
                            virtual_method.hiddenapi_flag = Some(flag);
                        }

                        self.reader.seek(SeekFrom::Start(current_offset))?;
                    }
                }
            }

            self.reader.seek(SeekFrom::Start(current_offset))?;

            Ok(())
        } else {
            Ok(())
        }
    }
}

impl<'a, TRead: IOread<Endian> + Seek> Reader<'static> for IoReader<'a, TRead> {
    fn read_header(&mut self) -> Result<Header> {
        self.read_header()
    }

    fn read_link_section_at(
        &mut self,
        link_size: u32,
        link_offset: u32,
    ) -> Result<Cow<'static, [u8]>> {
        self.read_link_section_at(link_size, link_offset)
    }

    fn read_map_list_at(&mut self, map_offset: u32) -> Result<Vec<MapItem>> {
        self.read_map_list_at(map_offset)
    }

    fn read_string_ids_at(
        &mut self,
        string_ids_size: u32,
        string_ids_offset: u32,
    ) -> Result<Vec<StringIdItem>> {
        self.read_string_ids_at(string_ids_size, string_ids_offset)
    }

    fn read_type_ids_at(
        &mut self,
        type_ids_size: u32,
        type_ids_offset: u32,
    ) -> Result<Vec<TypeIdItem>> {
        self.read_type_ids_at(type_ids_size, type_ids_offset)
    }

    fn read_proto_ids_at(
        &mut self,
        proto_ids_size: u32,
        proto_ids_offset: u32,
    ) -> Result<Vec<ProtoIdItem>> {
        self.read_proto_ids_at(proto_ids_size, proto_ids_offset)
    }

    fn read_type_list_at(&mut self, type_list_offset: u32) -> Result<Vec<TypeItem>> {
        self.read_type_list_at(type_list_offset)
    }

    fn read_string_data_item_at(&mut self, string_data_offset: u32) -> Result<String> {
        self.read_string_data_item_at(string_data_offset)
    }

    fn read_field_ids_at(
        &mut self,
        field_ids_size: u32,
        field_ids_offset: u32,
    ) -> Result<Vec<FieldIdItem>> {
        self.read_field_ids_at(field_ids_size, field_ids_offset)
    }

    fn read_method_ids_at(
        &mut self,
        method_ids_size: u32,
        method_ids_offset: u32,
    ) -> Result<Vec<MethodIdItem>> {
        self.read_method_ids_at(method_ids_size, method_ids_offset)
    }
}
