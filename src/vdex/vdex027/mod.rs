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

mod vdex_file_header;
pub use vdex_file_header::*;
mod vdex_section_header;
pub use vdex_section_header::*;
mod verifier_deps;
pub use verifier_deps::*;

type Result<T> = std::result::Result<T, Error>;

// Android version: 12.0.0-Current (as of writing this, Android 14 beta still uses this version)
//
// NOTE: ART expects the `VDexSectionHeader`s to be in ascending order of `section_kind`
//
// File format:
//
//     - TODO

pub const MAGIC: [u8; 4] = [b'v', b'd', b'e', b'x'];
pub const VERIFIER_DEPS_VERSION: [u8; 4] = [b'0', b'2', b'7', b'\0'];
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

    pub fn read_vdex_file_header(&mut self) -> Result<VDexFileHeader> {
        self.reader.seek(SeekFrom::Start(0))?;

        let result = self.reader.ioread_with::<VDexFileHeader>(self.endianness)?;

        if result.magic != MAGIC {
            Err(Error::InvalidMagicNumber(result.magic.to_vec()))
        } else {
            Ok(result)
        }
    }

    pub fn read_vdex_section_header(
        &mut self,
        header: &VDexFileHeader,
        index: u32,
    ) -> Result<VDexSectionHeader> {
        if index < header.number_of_sections {
            let offset = header.get_vdex_section_header_offset(index);

            self.reader.seek(SeekFrom::Start(offset))?;

            Ok(self
                .reader
                .ioread_with::<VDexSectionHeader>(self.endianness)?)
        } else {
            Err(Error::InvalidArguments(format!(
                "Requested VDex section index `{}` was greater than the number of VDex sections `{}`", 
                index,
                header.number_of_sections
            )))
        }
    }

    pub fn read_checksum_section(&mut self, section_header: &VDexSectionHeader) -> Result<Vec<u8>> {
        if section_header.section_kind != VDEX_SECTION_CHECKSUM {
            Err(Error::InvalidArguments(format!(
                "`read_checksum_section` was passed an invalid section header with kind `{}`",
                vdex_section_to_str(section_header.section_kind)
            )))
        } else if section_header.section_offset == 0 || section_header.section_size == 0 {
            Err(Error::InvalidArguments(format!(
                "Attempted to read an empty checksum section"
            )))
        } else {
            self.reader
                .seek(SeekFrom::Start(u64::from(section_header.section_offset)))?;

            let mut result: Vec<u8> = Vec::with_capacity(section_header.section_size as usize);
            result.resize(section_header.section_size as usize, 0);

            self.reader.read_exact(&mut result)?;

            Ok(result)
        }
    }

    // NOTE: I'm making the assumption that the dex section is still CDex as it was from 019-021 but I'm not positive
    //       I haven't actually found a VDex since 019 that has data in its Dex section... It seems that since
    //       Andorid 10, the VDex only contains verifier information and other misc data.
    //       In Android 12, quickening is no longer a thing that is saved in a way we can recover.
    //       I don't think there will ever actually be Dex data in the Dex section anymore, either.
    pub fn read_dex_files_section(
        &mut self,
        section_header: &VDexSectionHeader,
    ) -> Result<Vec<CDex>> {
        if section_header.section_kind != VDEX_SECTION_DEX_FILE {
            Err(Error::InvalidArguments(format!(
                "`read_dex_files_section` was passed an invalid section header with kind `{}`",
                vdex_section_to_str(section_header.section_kind)
            )))
        } else if section_header.section_offset == 0 || section_header.section_size == 0 {
            Err(Error::InvalidArguments(format!(
                "Attempted to read an empty Dex section"
            )))
        } else {
            self.reader
                .seek(SeekFrom::Start(u64::from(section_header.section_offset)))?;

            // NOTE: We don't know how many Dex files there are anymore. Because of this, I'm assuming
            //       there isn't any quickening info anymore either. I'm unsure.
            let mut result: Vec<CDex> = Vec::new();
            let mut magic_check: [u8; 4] = [0; 4];

            loop {
                self.reader.read_exact(&mut magic_check)?;
                // Reset to before the magic check
                let dex_index_offset = self.reader.seek(SeekFrom::Current(-4))?;

                if magic_check == crate::dex::MAGIC || magic_check == crate::dex::cdex::MAGIC {
                    // TODO: Just read as CDex for now but in the future we should support
                    //       detecting Dex vs CDex and parsing either into Dex. That's how
                    //       VDex works (or used to work? I haven't gone as deep into 027)
                    let mut dex_reader = DexReader::new(self.reader, dex_index_offset)?;

                    let cdex = dex_reader.read_cdex()?;
                    let file_size = cdex.header.header.file_size;

                    result.push(cdex);

                    // The parser isn't guaranteed to end at the true end of the file...
                    self.reader
                        .seek(SeekFrom::Start(dex_index_offset + u64::from(file_size)))?;
                } else {
                    break;
                }
            }

            Ok(result)
        }
    }

    pub fn read_verifier_deps(
        &mut self,
        section_header: &VDexSectionHeader,
    ) -> Result<Vec<DexFileDeps>> {
        if section_header.section_kind != VDEX_SECTION_VERIFIER_DEPS {
            Err(Error::InvalidArguments(format!(
                "`read_verifier_deps` was passed an invalid section header with kind `{}`",
                vdex_section_to_str(section_header.section_kind)
            )))
        } else if section_header.section_offset == 0 || section_header.section_size == 0 {
            Err(Error::InvalidArguments(format!(
                "Attempted to read an empty verifier deps section"
            )))
        } else {
            let section_start_offset = self
                .reader
                .seek(SeekFrom::Start(u64::from(section_header.section_offset)))?;

            // This is a bit of a hack... But verifier deps are written in the format of:
            //     - dex_offsets: [u32; num_dex_files]
            //     - verifier_deps: [DexFileDeps; num_of_dex_files]
            // So to detect the number of dex files, I just take the first offset and divide by `sizeof(u32)`
            // to get the number of Dex files since the first Dex file's `DexFileDeps` comes immediately
            // after the offsets. Now, there is a 4 byte alignment requirement BUT this shouldn't cause any
            // issue since the offset table is made up of 4 byte values.
            let num_dex_files = (self.reader.ioread_with::<u32>(self.endianness)? as usize)
                / std::mem::size_of::<u32>();

            // Seek backwards so we can get the offsets again...
            self.reader.seek(SeekFrom::Current(-4))?;

            let mut dex_offsets: Vec<u32> = Vec::with_capacity(num_dex_files);

            for _ in 0..num_dex_files {
                dex_offsets.push(self.reader.ioread_with::<u32>(self.endianness)?);
            }

            let mut result: Vec<DexFileDeps> = Vec::with_capacity(num_dex_files);

            for dex_index in 0..num_dex_files {
                let dex_offset = dex_offsets[dex_index];
                let offset_table_start = self.reader.seek(SeekFrom::Start(
                    section_start_offset + u64::from(dex_offset),
                ))?;

                println!("read_verifier_deps_set_vector");
                let (assignable_types, verified_classes) = self.read_verifier_deps_set_vector(
                    section_header,
                    dex_index,
                    u64::from(dex_offset),
                    offset_table_start,
                )?;

                // Alignment...
                let check_position = self.reader.seek(SeekFrom::Current(0))?;

                if check_position % 4 != 0 {
                    self.reader
                        .seek(SeekFrom::Start(check_position + (4 - (check_position % 4))))?;
                }

                println!("read_verifier_deps_string_vector");
                let strings = self.read_verifier_deps_string_vector()?;

                result.push(DexFileDeps {
                    strings,
                    assignable_types,
                    verified_classes,
                });
            }

            Ok(result)
        }
    }

    fn read_verifier_deps_set_vector(
        &mut self,
        section_header: &VDexSectionHeader,
        dex_index: usize,
        dex_offset: u64,
        offset_table_start: u64,
    ) -> Result<(Vec<Vec<TypeAssignability>>, Vec<bool>)> {
        // This becomes fun again... The format for this is:
        //     - offsets: [u32; num_class_defs + 1]
        //     - sparse_vector: [(u32, u32); num_class_defs]
        // I then make the dangerous decision that AT LEAST one of the `class_def_index`
        // values is `verified`... meaning we just have to:
        //     - Search for the first parsed `u32` that is NOT `u32::MAX`
        //     - Check if that `u32` is equal to the `current_position - offset_table_start`
        //         - If it is equal, then this Dex didn't have any verified classes...
        //     - If it isn't `u32::MAX` and it isn't `current_position - offset_table_start`,
        //       then it is `num_class_defs + 1`
        let num_class_defs: u64;
        // NOTE: This will contain both the offsets AND the final size of the set.
        //       The reason I do this is to make the logic for calculating the size of any sets easier
        //       I.e. you can get the size of each set by doing `for i in 0..num_class_defs { offsets[i + 1] - offsets[i] }`
        let mut offsets: Vec<u32> = Vec::new();
        let mut current_offset_index: u64 = 0;

        loop {
            let check_offset: u32 = self.reader.ioread_with::<u32>(self.endianness)?;
            let current_position = self.reader.seek(SeekFrom::Current(0))?;

            offsets.push(check_offset);
            current_offset_index += 1;

            // NOTE: We need to do `- dex_offset` as the offsets here are NOT relative to itself, they're relative
            //       to the ENTIRE verifier deps section.
            if check_offset != u32::MAX {
                if u64::from(check_offset) - dex_offset != (current_position - offset_table_start) {
                    num_class_defs = ((u64::from(check_offset) - dex_offset) / 4) - 1;
                    current_offset_index += 1;
                } else {
                    // If we reach this point, it means that `check_offset - dex_offset == current_position - offset_table_start`
                    // In otherwords, we reached the index that is equal to the size of the entire offset table
                    // which is the last index of the offset table... we have to break and parse 0 more items.
                    // The next `u32` that will be parsed is the length of the strings table that comes next.
                    num_class_defs = 0;
                }
                break;
            }

            if current_position
                >= u64::from(section_header.section_offset) + u64::from(section_header.section_size)
            {
                num_class_defs = 0;
                break;
            }
        }

        if num_class_defs > 0 {
            // Plus one to include the final index which is the total size...
            offsets.reserve((num_class_defs as usize + 1) - offsets.len());
        } else {
            // Return early. Not really needed...
            return Ok((Vec::with_capacity(0), Vec::with_capacity(0)));
        }

        // Now we parse the rest...
        for _ in current_offset_index..num_class_defs + 2 {
            offsets.push(self.reader.ioread_with::<u32>(self.endianness)?);
        }

        let mut assignable_types: Vec<Vec<TypeAssignability>> =
            Vec::with_capacity(num_class_defs as usize);
        let mut verified_classes: Vec<bool> = Vec::with_capacity(num_class_defs as usize);

        // Now it just comes down to two things:
        //  - If the `offset` is not `u32::MAX`, read from `offset` to next valid offset
        //  - If the `offset` is `u32::MAX`, set that in the bit vector and continue on
        for i in 0..offsets.len() - 1 {
            let current_offset: u32 = offsets[i as usize];

            if current_offset == u32::MAX {
                verified_classes.push(false);
                assignable_types.push(Vec::with_capacity(0));
            } else {
                verified_classes.push(true);

                let next_valid_offset: u32 = {
                    let mut result: u32 = 0;

                    // NOTE: The `j + 1` here isn't a bug, the value at `offsets[num_class_defs]`
                    //       is the total size of this vector...
                    for j in i..offsets.len() {
                        if offsets[(j + 1) as usize] != u32::MAX {
                            result = offsets[(j + 1) as usize];
                            break;
                        }
                    }

                    result
                };

                if next_valid_offset == 0 {
                    return Err(Error::Malformed(format!(
                        "VDex file contained invalid verifier deps offset for Dex file number `{}`",
                        dex_index
                    )));
                }

                let check_offset = self.reader.seek(SeekFrom::Current(0))?;

                if check_offset != (u64::from(current_offset) - dex_offset) + offset_table_start {
                    return Err(Error::Malformed(format!(
                        "VDex file contained invalid verifier deps offset for Dex file number `{}`, expected offset `{}` but found offset `{}`",
                        dex_index,
                        check_offset,
                        current_offset
                    )));
                }

                let mut type_assignabilities: Vec<TypeAssignability> = Vec::new();
                let mut current_offset: u64 = check_offset;
                let end_offset = (u64::from(next_valid_offset) - dex_offset) + offset_table_start;

                while current_offset != end_offset {
                    let destination_index: u32 = leb128::decode_uleb128(self.reader)?;
                    let source_index: u32 = leb128::decode_uleb128(self.reader)?;

                    type_assignabilities.push(TypeAssignability {
                        destination_index,
                        source_index,
                    });

                    current_offset = self.reader.seek(SeekFrom::Current(0))?;

                    if current_offset > end_offset {
                        return Err(Error::Malformed(format!(
                            "VDex file contained incorrect type assignabilities list for Dex file {}",
                            dex_index
                        )));
                    }
                }

                assignable_types.push(type_assignabilities);
            }
        }

        Ok((assignable_types, verified_classes))
    }

    fn read_verifier_deps_string_vector(&mut self) -> Result<Vec<CString>> {
        // Format:
        //     - string_count: u32
        //     - offsets: [u32; string_count]
        //     - strings: [c_str; string_count]
        let string_count: u32 = self.reader.ioread_with::<u32>(self.endianness)?;

        // I ignore the string offsets, they're not useful to us (we just use `read_until`...)
        for _ in 0..string_count {
            let _ = self.reader.ioread_with::<u32>(self.endianness)?;
        }

        let mut result: Vec<CString> = Vec::with_capacity(string_count as usize);

        for _ in 0..string_count {
            let mut string_bytes: Vec<u8> = Vec::new();
            self.reader.read_until(0, &mut string_bytes)?;
            // Unwrap because `CString::from_vec_with_nul` only returns Err if there isn't
            // exactly 1 nul byte at the end of the vec... which should be impossible here.
            result.push(CString::from_vec_with_nul(string_bytes).unwrap());
        }

        Ok(result)
    }

    // pub fn read_type_lookup_table(&mut self, section_header: &VDexSectionHeader) -> Result<()> {
    //     todo!("We need to support the `TypeLookupTable` or similar... I just don't have the energy or need for it right now")
    // }
}
