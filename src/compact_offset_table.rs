/*
 * Copyright (C) 2018 The Android Open Source Project
 * Copyright (C) 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// This is a port from: https://cs.android.com/android/platform/superproject/+/master:art/libdexfile/dex/compact_offset_table.cc

use crate::leb128;
use crate::Error;
use scroll::Endian;
use scroll::IOread;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

type Result<T> = std::result::Result<T, Error>;

pub struct CompactOffsetTableReader<'a, TRead: IOread<Endian> + Seek> {
    reader: &'a mut BufReader<TRead>,
    endianness: Endian,
    data_begin: u64,
    table_offset: u64,
    minimum_offset: u32,
}

/// This value is coupled with the leb chunk bitmask. That logic must also be adjusted when the
/// integer is modified.
const ELEMENTS_PER_INDEX: usize = 16;

// Leb block format:
// [uint16_t] 16 bit mask for what indexes actually have a non zero offset for the chunk.
// [lebs] Up to 16 lebs encoded using leb128, one leb bit. The leb specifies how the offset
// changes compared to the previous index.

impl<'a, TRead: IOread<Endian> + Seek> CompactOffsetTableReader<'a, TRead> {
    // NOTE: `end_offset` _should_ be `owned_data_end` for CDex... I think.
    pub fn new(
        reader: &'a mut BufReader<TRead>,
        endianness: Endian,
        data_begin: u64,
        minimum_offset: u32,
        table_offset: u64,
    ) -> Result<Self> {
        Ok(Self {
            reader,
            endianness,
            data_begin,
            minimum_offset,
            table_offset,
        })
    }

    fn get_table_entry(&mut self, index: usize) -> Result<u32> {
        let table_offset: u64 =
            self.data_begin + self.table_offset + (index * std::mem::size_of::<u32>()) as u64;

        self.reader.seek(SeekFrom::Start(table_offset))?;

        Ok(self.reader.ioread_with::<u32>(self.endianness)?)
    }

    fn get_data_value(&mut self, offset: usize) -> Result<u8> {
        let value_offset = self.data_begin + offset as u64;

        self.reader.seek(SeekFrom::Start(value_offset))?;

        Ok(self.reader.ioread_with::<u8>(self.endianness)?)
    }

    pub fn get_offset(&mut self, index: u32) -> Result<u32> {
        // NOTE: 16 as the offsets are `u16`...
        let offset: usize = self.get_table_entry(index as usize / ELEMENTS_PER_INDEX)? as usize;
        let bit_index: usize = index as usize % ELEMENTS_PER_INDEX;

        let mut block_index: usize = 0;
        let mut bit_mask: u16 = u16::from(self.get_data_value(offset + block_index)?);
        block_index += 1;
        bit_mask = (bit_mask << 8) | u16::from(self.get_data_value(offset + block_index)?);
        block_index += 1;

        if (bit_mask & (1 << bit_index)) == 0 {
            // Bit is not set means the offset is 0
            return Ok(0);
        }

        // Trim off the bits above the index we want and count how many bits are set. This is how many
        // lebs we need to decode.
        // TODO: Hm... I've made this `u64` because almost all Android phones are 64 bit nowadayss but I haven't
        //       confirmed whether this code works on 32 bit machines... Which now that I'm thinking of it,
        //       while 64 bit is most common today, 32 bit was still used for emulators during VDex v019.
        //       So if this is working on v019 then it will work everywhere (and I don't see why it wouldn't work?
        //       Unless they're relying on bits going past 32 bits to equal zero?)
        let mut count = (u64::from(bit_mask) << ((std::mem::size_of::<u64>() * 8) - 1 - bit_index))
            .count_ones();

        if count == 0 {
            return Err(Error::InvalidArguments(format!(
                "CompactOffsetTable was provided an invalid index which resulted in a count of `0`"
            )));
        }

        let mut current_offset: u32 = self.minimum_offset;

        self.reader.seek(SeekFrom::Start(block_index as u64))?;

        loop {
            current_offset +=
                match leb128::decode_uleb128::<u32, BufReader<TRead>>(&mut self.reader) {
                    Ok(value) => value,
                    Err(error) => {
                        panic!("{}", error);
                    }
                };
            count -= 1;

            if count == 0 {
                break;
            }
        }

        Ok(current_offset)
    }
}
