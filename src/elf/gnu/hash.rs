use scroll::ctx::TryFromCtx;
use scroll::{Endian, Pread};

use crate::elf::gnu::common::HashHeader;
use crate::Error;
use std::borrow::Cow;

type Result<T> = std::result::Result<T, Error>;

// Documentation modified from: https://sourceware.org/legacy-ml/binutils/2006-10/msg00377.html
//                         and: http://www.linker-aliens.org/blogs/ali/entry/gnu_hash_elf_sections/
// Who knows how accurate anything is since it's been 17 years and no formal documentation has been released by the GNU team. Yes I'm angry. They first implemented this when I was 10 years old and 17 YEARS LATER they still have yet to write a formal specification for this.
pub struct HashTable<'a, TMaskword>
where
    [TMaskword]: ToOwned,
{
    // /// Number of hash buckets
    // ///
    // /// I.e. `nbuckets == buckets.len()`
    // pub nbuckets: u32,
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
    pub bloom_filters: Cow<'a, [TMaskword]>,
    /// Contains indexes into the `.dynsym` section, accessible through the operation:
    ///  - Calculate N using buckets[gnu_hash(symname) % nbuckets]
    ///  - Given `N != 0`, obtain symbol using `&.dynsym[N]`
    ///
    /// Size: `<HashHeader.nbuckets>`
    pub buckets: Cow<'a, [u32]>,
    /// Symbol hash values for the sorted section of `.dynsym` starting at `symndx`
    ///
    /// Size: `<.dynsym>.len() - symndx`
    pub hash_values: Cow<'a, [u32]>,
}

// TODO: I don't like this... but oh well.
//       I kind of wish I could do a `HashTable::parse_32` and `HashTable::parse_64` but that would require testing to see if `TMaskword` can be replaced
//       entirely with a `u64`... which I fucking hope it can.
impl<'a, 'b, TMaskword> HashTable<'a, TMaskword>
where
    [TMaskword]: ToOwned,
    TMaskword: TryFromCtx<'b, Endian, [u8], Error = scroll::Error>,
{
    pub fn parse(bytes: &'b Cow<'a, [u8]>, endianness: Endian, dynsym_len: usize) -> Result<Self> {
        // TODO: Pread?
        let mut offset: usize = 0;
        let hash_header = bytes.pread_with::<HashHeader>(offset, endianness)?;
        offset += std::mem::size_of::<HashHeader>();

        if dynsym_len >= (hash_header.symndx as usize) {
            let bloom_filters_len = hash_header.maskwords as usize;
            let mut bloom_filters: Vec<TMaskword> = Vec::with_capacity(bloom_filters_len);
            let buckets_len = hash_header.nbuckets as usize;
            let mut buckets: Vec<u32> = Vec::with_capacity(buckets_len);
            let hash_values_len = dynsym_len - (hash_header.symndx as usize);
            let mut hash_values: Vec<u32> = Vec::with_capacity(hash_values_len);

            for _ in 0..bloom_filters_len {
                bloom_filters.push(bytes.pread_with::<TMaskword>(offset, endianness)?);
                offset += std::mem::size_of::<TMaskword>();
            }

            for _ in 0..buckets_len {
                buckets.push(bytes.pread_with::<u32>(offset, endianness)?);
                offset += std::mem::size_of::<u32>();
            }

            for _ in 0..hash_values_len {
                hash_values.push(bytes.pread_with::<u32>(offset, endianness)?);
                offset += std::mem::size_of::<u32>();
            }

            Ok(Self {
                symndx: hash_header.symndx,
                shift2: hash_header.shift2,
                bloom_filters: Cow::Owned(bloom_filters.to_owned()),
                buckets: Cow::Owned(buckets),
                hash_values: Cow::Owned(hash_values),
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
