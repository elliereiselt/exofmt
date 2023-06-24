/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::dex::Dex;
use crate::dex::Header as DexHeader;
use crate::leb128;
use crate::Error;
use scroll::Endian;
use scroll::IOread;
use std::borrow::Cow;
use std::ffi::CString;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

mod header;
pub use header::*;
mod verifier_deps;
pub use verifier_deps::*;
mod quickening_info;
pub use quickening_info::*;

type Result<T> = std::result::Result<T, Error>;

// Android versions: 8.0.0
// File format:
//     - header: Header
//     - dex_sections_checksums: [u32; header.number_of_dex_files]
//     - dex_sections: [[u8; N]; header.number_of_dex_files]
//         - v006 did not have any concept of section headers and forced you to parse Dex headers to determine index
//         - Dex data may have been quickened
//     - verifier_deps: [DexFileDeps; header.verifier_deps_size]
//     - quickening_info: [QuickeningInfo; header.number_of_dex_files]
//         - Quickening info per `class_defs` list in each Dex
//         - Quickening info is for direct methods or virtual methods

pub const MAGIC: [u8; 4] = [b'v', b'd', b'e', b'x'];
pub const VERSION: [u8; 4] = [b'0', b'0', b'6', b'\0'];

pub struct IoReader<'a, TRead: IOread<Endian> + Seek> {
    pub reader: &'a mut BufReader<TRead>,
    pub endianness: Endian,
    pub stream_len: u64,
}

impl<'a, TRead: IOread<Endian> + Seek> IoReader<'a, TRead> {
    pub fn new(reader: &'a mut BufReader<TRead>) -> Result<Self> {
        let stream_len = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Current(0))?;
        Ok(Self {
            reader,
            endianness: Endian::Little,
            stream_len,
        })
    }

    pub fn read_header(&mut self) -> Result<Header> {
        self.reader.seek(SeekFrom::Start(0))?;

        let result = self.reader.ioread_with::<Header>(self.endianness)?;

        if result.magic != MAGIC {
            Err(Error::InvalidMagicNumber(result.magic.to_vec()))
        } else if result.version != VERSION {
            Err(Error::InvalidVersionNumber(format!("{:x?}", VERSION)))
        } else {
            Ok(result)
        }
    }

    pub fn read_dex_sections_checksums(&mut self, header: &Header) -> Result<Vec<u32>> {
        if header.number_of_dex_files > 0 {
            let size = header.number_of_dex_files as usize;
            let offset = std::mem::size_of::<Header>() as u64;
            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<u32> = Vec::with_capacity(size);

            for _ in 0..size {
                result.push(self.reader.ioread_with::<u32>(self.endianness)?);
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_dex_sections(&mut self, header: &Header) -> Result<Vec<Cow<'static, [u8]>>> {
        if header.number_of_dex_files > 0 {
            let size = header.number_of_dex_files as usize;
            let header_size = std::mem::size_of::<Header>();
            let checksums_size = std::mem::size_of::<u32>() * size;
            let offset = (header_size + checksums_size) as u64;

            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<Cow<'static, [u8]>> = Vec::with_capacity(size);

            for _ in 0..size {
                let dex_index_offset = self.reader.seek(SeekFrom::Current(0))?;
                let dex_header = DexHeader::read_from_buf_reader(self.reader)?;

                // Jump back to before the header so we can get the bytes for the whole section...
                self.reader.seek(SeekFrom::Start(dex_index_offset))?;

                let mut bytes: Vec<u8> = Vec::with_capacity(dex_header.file_size as usize);
                bytes.resize(dex_header.file_size as usize, 0);

                self.reader.read_exact(&mut bytes)?;

                result.push(Cow::Owned(bytes));
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_verifier_deps(&mut self, header: &Header) -> Result<Vec<DexFileDeps>> {
        if header.verifier_deps_size > 0 {
            let header_size = std::mem::size_of::<Header>();
            let checksums_size = std::mem::size_of::<u32>() * header.number_of_dex_files as usize;
            let dex_files_size = header.dex_section_size as usize;
            let offset = (header_size + checksums_size + dex_files_size) as u64;

            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<DexFileDeps> =
                Vec::with_capacity(header.number_of_dex_files as usize);

            for _ in 0..header.number_of_dex_files {
                let strings = self.read_verifier_deps_strings()?;
                let assignable_types = self.read_verifier_deps_type_asignability_set()?;
                let unassignable_types = self.read_verifier_deps_type_asignability_set()?;
                let classes = self.read_verifier_deps_class_resolution_set()?;
                let fields = self.read_verifier_deps_field_resolution_set()?;
                let direct_methods = self.read_verifier_deps_method_resolution_set()?;
                let virtual_methods = self.read_verifier_deps_method_resolution_set()?;
                let interface_methods = self.read_verifier_deps_method_resolution_set()?;
                let unverified_classes = self.read_verifier_deps_unverified_classes()?;

                result.push(DexFileDeps {
                    strings,
                    assignable_types,
                    unassignable_types,
                    classes,
                    fields,
                    direct_methods,
                    virtual_methods,
                    interface_methods,
                    unverified_classes,
                });
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    fn read_verifier_deps_strings(&mut self) -> Result<Vec<CString>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<CString> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let mut raw_string: Vec<u8> = Vec::new();

            self.reader.read_until(0, &mut raw_string)?;

            // NOTE: Unlike in Dex, VDex version 006 stores strings as nul-terminated C strings... this shouldn't fail.
            let string = match CString::from_vec_with_nul(raw_string) {
                Ok(string) => string,
                Err(error) => return Err(Error::Malformed(format!(
                    "VDex file (version 006) contained a malformed string in its `verifier_deps` section, raw error: {}", 
                    error
                ))),
            };

            result.push(string);
        }

        Ok(result)
    }

    fn read_verifier_deps_type_asignability_set(&mut self) -> Result<Vec<TypeAssignability>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<TypeAssignability> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let destination_index: u32 = leb128::decode_uleb128(self.reader)?;
            let source_index: u32 = leb128::decode_uleb128(self.reader)?;
            result.push(TypeAssignability {
                destination_index,
                source_index,
            });
        }

        Ok(result)
    }

    fn read_verifier_deps_class_resolution_set(&mut self) -> Result<Vec<ClassResolution>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<ClassResolution> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let type_index: u16 =
                leb128::decode_uleb128::<u32, BufReader<TRead>>(self.reader)? as u16;
            let access_flags: u16 =
                leb128::decode_uleb128::<u32, BufReader<TRead>>(self.reader)? as u16;
            result.push(ClassResolution {
                type_index,
                access_flags,
            });
        }

        Ok(result)
    }

    fn read_verifier_deps_field_resolution_set(&mut self) -> Result<Vec<FieldResolution>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<FieldResolution> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let field_index: u32 = leb128::decode_uleb128(self.reader)?;
            let access_flags: u16 =
                leb128::decode_uleb128::<u32, BufReader<TRead>>(self.reader)? as u16;
            let declaring_class_index: u32 = leb128::decode_uleb128(self.reader)?;
            result.push(FieldResolution {
                field_index,
                access_flags,
                declaring_class_index,
            });
        }

        Ok(result)
    }

    fn read_verifier_deps_method_resolution_set(&mut self) -> Result<Vec<MethodResolution>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<MethodResolution> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let method_index: u32 = leb128::decode_uleb128(self.reader)?;
            let access_flags: u16 =
                leb128::decode_uleb128::<u32, BufReader<TRead>>(self.reader)? as u16;
            let declaring_class_index: u32 = leb128::decode_uleb128(self.reader)?;
            result.push(MethodResolution {
                method_index,
                access_flags,
                declaring_class_index,
            });
        }

        Ok(result)
    }

    fn read_verifier_deps_unverified_classes(&mut self) -> Result<Vec<u16>> {
        let count: u32 = leb128::decode_uleb128(self.reader)?;
        let mut result: Vec<u16> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            // NOTE: You _probably_ could do `decode_uleb128::<u16>` but I'm too nervous to do that.
            //       Android creates leb128 values that differ from what I think most resources say
            //       for writing leb128...
            let value: u32 = leb128::decode_uleb128(self.reader)?;
            let value: u16 = match u16::try_from(value) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::Malformed(format!(
                        "VDex verifier deps had invalid unverified `class_index`"
                    )))
                }
            };
            result.push(value);
        }

        Ok(result)
    }

    pub fn read_quickening_info(
        &mut self,
        header: &Header,
        dex_files: &Vec<Dex>,
    ) -> Result<Vec<QuickeningInfo<'static>>> {
        if header.quickening_info_size > 0 {
            let header_size = std::mem::size_of::<Header>() as u64;
            let checksums_size =
                (std::mem::size_of::<u32>() * header.number_of_dex_files as usize) as u64;
            let dex_files_size = u64::from(header.dex_section_size);
            let verifier_deps_size = u64::from(header.verifier_deps_size);
            let offset = header_size + checksums_size + dex_files_size + verifier_deps_size;

            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<QuickeningInfo> = Vec::with_capacity(dex_files.len());

            for dex_file in dex_files {
                let mut class_defs_quickening: Vec<ClassQuickeningInfo> =
                    Vec::with_capacity(dex_file.class_defs.len());

                for class_def in &dex_file.class_defs {
                    if let Some(class_data) = &class_def.class_data {
                        let mut direct_methods: Vec<Option<CodeItemQuickening<'static>>> =
                            Vec::with_capacity(class_data.direct_methods.len());
                        let mut virtual_methods: Vec<Option<CodeItemQuickening<'static>>> =
                            Vec::with_capacity(class_data.virtual_methods.len());

                        for method in &class_data.direct_methods {
                            if let Some(_) = &method.code {
                                let quickening_size =
                                    self.reader.ioread_with::<u32>(self.endianness)?;
                                let mut quickening_info: Vec<u8> =
                                    Vec::with_capacity(quickening_size as usize);
                                quickening_info.resize(quickening_size as usize, 0);
                                self.reader.read_exact(&mut quickening_info)?;

                                direct_methods.push(Some(CodeItemQuickening {
                                    info: Cow::Owned(quickening_info),
                                }));
                            } else {
                                direct_methods.push(None);
                            }
                        }

                        for method in &class_data.virtual_methods {
                            if let Some(_) = &method.code {
                                let quickening_size =
                                    self.reader.ioread_with::<u32>(self.endianness)?;
                                let mut quickening_info: Vec<u8> =
                                    Vec::with_capacity(quickening_size as usize);
                                quickening_info.resize(quickening_size as usize, 0);
                                self.reader.read_exact(&mut quickening_info)?;

                                virtual_methods.push(Some(CodeItemQuickening {
                                    info: Cow::Owned(quickening_info),
                                }));
                            } else {
                                virtual_methods.push(None);
                            }
                        }

                        class_defs_quickening.push(ClassQuickeningInfo {
                            class_index: class_def.class_index,
                            direct_methods,
                            virtual_methods,
                        });
                    }
                }

                result.push(QuickeningInfo {
                    class_defs: class_defs_quickening,
                });
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }
}
