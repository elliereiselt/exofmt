use std::borrow::Cow;

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
