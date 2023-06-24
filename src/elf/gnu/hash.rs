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

use scroll::ctx::{FromCtx, SizeWith};
use scroll::{Endian, IOread};

use crate::elf::gnu::common::HashHeader;
use crate::Error;
use std::io::Seek;

type Result<T> = std::result::Result<T, Error>;

// Documentation modified from: https://sourceware.org/legacy-ml/binutils/2006-10/msg00377.html
//                         and: http://www.linker-aliens.org/blogs/ali/entry/gnu_hash_elf_sections/
// Who knows how accurate anything is since it's been 17 years and no formal documentation has been released by the GNU team. Yes I'm angry. They first implemented this when I was 10 years old and 17 YEARS LATER they still have yet to write a formal specification for this.
pub struct HashTable<TMaskword = u64> {
    /// Start of the first symbol index within `.dynsym` that can be looked up with `.gnu.hash`
    ///
    /// I.e. `.dynsym[symndx..dynsymcount - 1]` symbols are sorted by the `gnu_hash` function using `gnu_hash(&.dynstr[s.st_name]) % nbuckets` values
    pub symndx: u32,
    // /// Number of mask words
    // ///
    // /// I.e. `maskwords == bloom_filters.len()`
    // pub maskwords: u32,
    /// Shift count used in the bloom_filter
    pub shift2: u32,
    /// Maskwords used for the bloom filter on `hash_values`
    ///
    /// Size: `<HashHeader.maskwords>`
    pub bloom_filters: Vec<TMaskword>,
    /// Contains indexes into the `.dynsym` section, accessible through the operation:
    ///  - Calculate N using buckets[gnu_hash(symname) % nbuckets]
    ///  - Given `N != 0`, obtain symbol using `&.dynsym[N]`
    ///
    /// Size: `<HashHeader.nbuckets>`
    pub buckets: Vec<u32>,
    /// Symbol hash values for the sorted section of `.dynsym` starting at `symndx`
    ///
    /// Size: `<.dynsym>.len() - symndx`
    pub hash_values: Vec<u32>,
}

// TODO: I don't like this... but oh well.
//       I kind of wish I could do a `HashTable::parse_32` and `HashTable::parse_64` but that would require testing to see if `TMaskword` can be replaced
//       entirely with a `u64`... which I fucking hope it can.
// TODO: I'm thinking more and more that parsing for OS and proc specific things should just be added to `Reader` and give the programmer the option to shoot themselves in the foot.
impl<'a, TMaskword> HashTable<TMaskword>
where
    TMaskword: SizeWith<Endian>,
    TMaskword: FromCtx<Endian, [u8]>,
    u64: From<TMaskword>,
{
    pub fn parse<TRead: IOread<Endian> + Seek>(
        reader: &'a mut TRead,
        endianness: Endian,
        dynsym_len: usize,
    ) -> Result<HashTable<u64>> {
        let hash_header = reader.ioread_with::<HashHeader>(endianness)?;

        if dynsym_len >= (hash_header.symndx as usize) {
            let bloom_filters_len = hash_header.maskwords as usize;
            let mut bloom_filters: Vec<u64> = Vec::with_capacity(bloom_filters_len);
            let buckets_len = hash_header.nbuckets as usize;
            let mut buckets: Vec<u32> = Vec::with_capacity(buckets_len);
            let hash_values_len = dynsym_len - (hash_header.symndx as usize);
            let mut hash_values: Vec<u32> = Vec::with_capacity(hash_values_len);

            for _ in 0..bloom_filters_len {
                bloom_filters.push(u64::from(reader.ioread_with::<TMaskword>(endianness)?));
            }

            for _ in 0..buckets_len {
                buckets.push(reader.ioread_with::<u32>(endianness)?);
            }

            for _ in 0..hash_values_len {
                hash_values.push(reader.ioread_with::<u32>(endianness)?);
            }

            Ok(HashTable::<u64> {
                symndx: hash_header.symndx,
                shift2: hash_header.shift2,
                bloom_filters,
                buckets,
                hash_values,
            })
        } else {
            Err(Error::Malformed(format!(
                "GNU hash header provided `symndx` value ({}) larger than the number of dynsym entries ({})!",
                hash_header.symndx,
                dynsym_len,
            )))
        }
    }
}
