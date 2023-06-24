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

use crate::compact_offset_table::CompactOffsetTableReader;
use crate::dex::CDex;
use crate::dex::IoReader as DexReader;
use crate::leb128;
use crate::Error;
use scroll::Endian;
use scroll::IOread;
use std::ffi::CString;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

mod verifier_deps_header;
pub use verifier_deps_header::*;
mod dex_section_header;
pub use dex_section_header::*;
mod verifier_deps;
pub use verifier_deps::*;
mod quickening_info;
pub use quickening_info::*;

type Result<T> = std::result::Result<T, Error>;

// Android version: 9.0.0
//
// File format:
//     - verifier_deps_header: VerifierDepsHeader
//     - dex_sections_checksums: [u32; verifier_deps_header.number_of_dex_files]
//     - dex_section_header: DexSectionHeader
//         - This section is optional and can be missing if `verifier_deps_header.dex_section_version == DEX_SECTION_VERSION_EMPTY`
//     - dex_section: [(quickening_offset, Dex); verifier_deps_header.number_of_dex_files]
//     - dex_shared_data: [u8; header.dex_shared_data_size]
//         - This area of the VDex is the result of all contained Dex data sections being concatenated together and deduplicated
//     - verifier_deps: [DexFileDeps; verifier_deps_header.number_of_dex_files]
//     - quickening_info: ???

pub const MAGIC: [u8; 4] = [b'v', b'd', b'e', b'x'];
pub const VERIFIER_DEPS_VERSION: [u8; 4] = [b'0', b'1', b'9', b'\0'];
pub const DEX_SECTION_VERSION: [u8; 4] = [b'0', b'0', b'2', b'\0'];
pub const DEX_SECTION_VERSION_EMPTY: [u8; 4] = [b'0', b'0', b'0', b'\0'];

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

    pub fn read_verifier_deps_header(&mut self) -> Result<VerifierDepsHeader> {
        self.reader.seek(SeekFrom::Start(0))?;

        let result = self
            .reader
            .ioread_with::<VerifierDepsHeader>(self.endianness)?;

        if result.magic != MAGIC {
            Err(Error::InvalidMagicNumber(result.magic.to_vec()))
        } else if result.verifier_deps_version != VERIFIER_DEPS_VERSION {
            Err(Error::InvalidVersionNumber(format!(
                "{:x?}",
                VERIFIER_DEPS_VERSION
            )))
        } else if result.dex_section_version != DEX_SECTION_VERSION
            && result.dex_section_version != DEX_SECTION_VERSION_EMPTY
        {
            Err(Error::InvalidVersionNumber(format!(
                "{:x?}",
                DEX_SECTION_VERSION
            )))
        } else {
            Ok(result)
        }
    }

    pub fn read_dex_sections_checksums(
        &mut self,
        verifier_deps_header: &VerifierDepsHeader,
    ) -> Result<Vec<u32>> {
        if verifier_deps_header.number_of_dex_files > 0 {
            let size = verifier_deps_header.number_of_dex_files as usize;
            let offset = std::mem::size_of::<VerifierDepsHeader>() as u64;

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

    pub fn read_dex_section_header(
        &mut self,
        verifier_deps_header: &VerifierDepsHeader,
    ) -> Result<DexSectionHeader> {
        let size = verifier_deps_header.number_of_dex_files as usize;
        let verifier_deps_header_size = std::mem::size_of::<VerifierDepsHeader>();
        let checksums_size = std::mem::size_of::<u32>() * size;
        let offset = (verifier_deps_header_size + checksums_size) as u64;

        self.reader.seek(SeekFrom::Start(offset))?;

        Ok(self
            .reader
            .ioread_with::<DexSectionHeader>(self.endianness)?)
    }

    pub fn read_dex_files_with_quickening_offsets(
        &mut self,
        verifier_deps_header: &VerifierDepsHeader,
    ) -> Result<Vec<(u32, CDex<'static>)>> {
        if verifier_deps_header.number_of_dex_files > 0 {
            let size = verifier_deps_header.number_of_dex_files as usize;
            let verifier_deps_header_size = std::mem::size_of::<VerifierDepsHeader>();
            let checksums_size = std::mem::size_of::<u32>() * size;
            let dex_section_header_size = std::mem::size_of::<DexSectionHeader>();
            let offset =
                (verifier_deps_header_size + checksums_size + dex_section_header_size) as u64;

            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<(u32, CDex<'static>)> = Vec::with_capacity(size);

            for _ in 0..verifier_deps_header.number_of_dex_files {
                let quickening_offset = self.reader.ioread_with::<u32>(self.endianness)?;
                // TODO: We need to make sure `dex_index_offset` is aligned to 4 bytes and correct if not
                let dex_index_offset = self.reader.seek(SeekFrom::Current(0))?;

                // TODO: In the future, we need to replace this with generic code that can parse both Dex AND CDex then return Dex...
                let mut dex_reader = DexReader::new(self.reader, dex_index_offset)?;

                let cdex = dex_reader.read_cdex()?;
                let file_size = cdex.header.header.file_size;

                result.push((quickening_offset, cdex));

                // The parser isn't guaranteed to end at the true end of the file...
                self.reader
                    .seek(SeekFrom::Start(dex_index_offset + u64::from(file_size)))?;
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn read_verifier_deps(
        &mut self,
        verifier_deps_header: &VerifierDepsHeader,
        dex_section_header: Option<&DexSectionHeader>,
    ) -> Result<Vec<DexFileDeps>> {
        if verifier_deps_header.verifier_deps_size > 0 {
            let verifier_deps_header_size = std::mem::size_of::<VerifierDepsHeader>();
            let checksums_size =
                std::mem::size_of::<u32>() * verifier_deps_header.number_of_dex_files as usize;
            let dex_section_header_size = if verifier_deps_header.has_dex_section() {
                std::mem::size_of::<DexSectionHeader>()
            } else {
                0
            };
            let dex_files_size = if let Some(dex_section_header) = dex_section_header {
                dex_section_header.dex_section_size as usize
                    + dex_section_header.dex_shared_data_size as usize
            } else {
                0
            };
            let offset = (verifier_deps_header_size
                + checksums_size
                + dex_section_header_size
                + dex_files_size) as u64;

            self.reader.seek(SeekFrom::Start(offset))?;

            let mut result: Vec<DexFileDeps> =
                Vec::with_capacity(verifier_deps_header.number_of_dex_files as usize);

            for _ in 0..verifier_deps_header.number_of_dex_files {
                let strings = self.read_verifier_deps_strings()?;
                let assignable_types = self.read_verifier_deps_type_asignability_set()?;
                let unassignable_types = self.read_verifier_deps_type_asignability_set()?;
                let classes = self.read_verifier_deps_class_resolution_set()?;
                let fields = self.read_verifier_deps_field_resolution_set()?;
                let methods = self.read_verifier_deps_method_resolution_set()?;
                let unverified_classes = self.read_verifier_deps_unverified_classes()?;

                result.push(DexFileDeps {
                    strings,
                    assignable_types,
                    unassignable_types,
                    classes,
                    fields,
                    methods,
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
        verifier_deps_header: &VerifierDepsHeader,
        dex_section_header: &DexSectionHeader,
        dex_and_offsets: &Vec<(u32, CDex)>,
    ) -> Result<Vec<QuickeningInfo>> {
        if dex_section_header.quickening_info_size > 0 {
            let verifier_deps_header_size = std::mem::size_of::<VerifierDepsHeader>() as u64;
            let checksums_size = (std::mem::size_of::<u32>()
                * verifier_deps_header.number_of_dex_files as usize)
                as u64;
            let dex_section_header_size = std::mem::size_of::<DexSectionHeader>() as u64;
            let dex_files_size = dex_section_header.dex_section_size as u64
                + dex_section_header.dex_shared_data_size as u64;
            let verifier_deps_size = u64::from(verifier_deps_header.verifier_deps_size);
            let section_start_offset = verifier_deps_header_size
                + checksums_size
                + dex_section_header_size
                + dex_files_size
                + verifier_deps_size;
            // let section_end_offset =
            //     section_start_offset + u64::from(dex_section_header.quickening_info_size);

            let mut result: Vec<QuickeningInfo> = Vec::with_capacity(dex_and_offsets.len());

            for (quickening_info_table_offset, dex_file) in dex_and_offsets {
                self.reader.seek(SeekFrom::Start(
                    section_start_offset + u64::from(*quickening_info_table_offset),
                ))?;

                // The first two values are the `minimum_offset` and `table_offset` respectively
                // I would just move this to `compact_offset_table`, since it's used by most VDex versions,
                // but I'm too lazy to work around the single mutable ref to `reader` and have decided to
                // treat `CompactOffsetTableReader` as a thin layer over `reader` instead... I.e. we create
                // the reader for every single index we look up the offset of. Same as the CDex reader
                let minimum_offset: u32 = self.reader.ioread_with::<u32>(self.endianness)?;
                let table_offset = u64::from(self.reader.ioread_with::<u32>(self.endianness)?);
                let data_begin = self.reader.seek(SeekFrom::Current(0))?;

                let mut class_def_quickenings: Vec<ClassQuickeningInfo> = Vec::new();

                for class_def in &dex_file.class_defs {
                    if let Some(class_data) = &class_def.class_data {
                        let mut direct_methods: Vec<Option<MethodQuickening>> =
                            Vec::with_capacity(class_data.direct_methods.len() as usize);
                        let mut virtual_methods: Vec<Option<MethodQuickening>> =
                            Vec::with_capacity(class_data.virtual_methods.len() as usize);

                        for method in &class_data.direct_methods {
                            if method.code.is_some() {
                                let offset = self.get_quickening_info_offset(
                                    data_begin,
                                    minimum_offset,
                                    table_offset,
                                    method.method_index,
                                )?;

                                let quickening_info = if offset > 0 {
                                    self.get_quickening_info(offset)?
                                } else {
                                    Vec::with_capacity(0)
                                };

                                direct_methods.push(Some(MethodQuickening {
                                    method_index: method.method_index,
                                    info: quickening_info,
                                }));
                            } else {
                                direct_methods.push(None);
                            }
                        }

                        for method in &class_data.virtual_methods {
                            if method.code.is_some() {
                                let offset = self.get_quickening_info_offset(
                                    data_begin,
                                    minimum_offset,
                                    table_offset,
                                    method.method_index,
                                )?;

                                let quickening_info = if offset > 0 {
                                    self.get_quickening_info(offset)?
                                } else {
                                    Vec::with_capacity(0)
                                };

                                virtual_methods.push(Some(MethodQuickening {
                                    method_index: method.method_index,
                                    info: quickening_info,
                                }));
                            } else {
                                virtual_methods.push(None);
                            }
                        }

                        class_def_quickenings.push(ClassQuickeningInfo {
                            class_index: class_def.class_index,
                            direct_methods,
                            virtual_methods,
                        })
                    }
                }

                result.push(QuickeningInfo {
                    class_defs: class_def_quickenings,
                });
            }

            Ok(result)
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    fn get_quickening_info_offset(
        &mut self,
        data_begin: u64,
        minimum_offset: u32,
        table_offset: u64,
        index: u32,
    ) -> Result<u32> {
        let mut compact_offset_table_reader = CompactOffsetTableReader::new(
            self.reader,
            self.endianness,
            data_begin,
            minimum_offset,
            table_offset,
        )?;

        Ok(compact_offset_table_reader.get_offset(index)?)
    }

    fn get_quickening_info(&mut self, offset: u32) -> Result<Vec<u8>> {
        self.reader.seek(SeekFrom::Start(u64::from(offset - 1)))?;
        let size: u32 = leb128::decode_uleb128(self.reader)?;
        let mut quickening_info: Vec<u8> = Vec::with_capacity(size as usize);
        quickening_info.resize(size as usize, 0);
        self.reader.read_exact(&mut quickening_info)?;
        Ok(quickening_info)
    }
}
