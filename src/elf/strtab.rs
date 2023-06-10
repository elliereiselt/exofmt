use crate::Error;
use std::borrow::Cow;
use std::collections::hash_map;
use std::collections::HashMap;
use std::str;

type Result<T> = std::result::Result<T, Error>;

pub struct StrTab<'a> {
    bytes: Cow<'a, [u8]>,
    delimeter: u8,
    strings: HashMap<u32, Cow<'a, str>>,
}

impl<'a> StrTab<'a> {
    pub fn parse(bytes: Cow<'a, [u8]>, delimeter: u8) -> Result<StrTab<'a>> {
        let mut result = Self {
            bytes,
            delimeter,
            strings: HashMap::new(),
        };

        let mut current_offset: u32 = 0;
        let mut current_length: u32 = 0;

        while current_offset + current_length < result.bytes.len() as u32 {
            if result.bytes[(current_offset + current_length) as usize] == delimeter {
                let str_bytes = &result.bytes
                    [(current_offset as usize)..(current_offset + current_length) as usize];

                if str_bytes.len() != 0 {
                    match str::from_utf8(str_bytes) {
                        Ok(str_result) => result
                            .strings
                            .insert(current_offset, Cow::Owned(str_result.to_owned())),
                        Err(utf8_error) => {
                            return Err(Error::Malformed(format!(
                                "Invalid string found in strtab at offset {}, {}",
                                current_offset, utf8_error
                            )))
                        }
                    };
                }

                current_offset = current_offset + current_length + 1;
                current_length = 0;
            } else {
                current_length += 1;
            }
        }

        Ok(result)
    }

    fn parse_at_offset<'b>(&'b self, offset: u32) -> Result<Cow<'b, str>>
    where
        'a: 'b,
    {
        if offset as usize >= self.bytes.len() {
            return Err(Error::Malformed(format!(
                "Offset `{}` is out of range for strtab length `{}`",
                offset,
                self.bytes.len()
            )));
        }

        let current_offset: usize = offset as usize;
        let mut current_length: usize = 0;

        while current_offset + current_length < self.bytes.len() {
            if self.bytes[current_offset + current_length] == self.delimeter {
                break;
            } else {
                current_length += 1;
            }
        }

        let str_bytes = &self.bytes[current_offset..current_offset + current_length];

        // NOTE: Zero length strings are allowed. The `Null` header has an empty string for its name.
        match str::from_utf8(str_bytes) {
            Ok(result) => Ok(Cow::Borrowed(result)),
            Err(utf8_error) => Err(Error::Malformed(format!(
                "Invalid string found in strtab at offset {}, {}",
                current_offset, utf8_error
            ))),
        }
    }

    pub fn get_at_offset<'b>(&'b self, offset: u32) -> Result<Cow<'b, str>>
    where
        'a: 'b,
    {
        if let Some(cached_result) = self.strings.get(&offset) {
            Ok(cached_result.clone())
        } else {
            self.parse_at_offset(offset)
        }
    }

    /// Gets the value at the offset and caches it if it isn't already cached
    pub fn mut_get_at_offset(&mut self, offset: u32) -> Result<&Cow<'a, str>> {
        if self.strings.contains_key(&offset) {
            Ok(self.strings.get(&offset).unwrap())
        } else {
            let parsed_result = Cow::Owned(self.parse_at_offset(offset)?.into_owned());
            self.strings.insert(offset, parsed_result);
            Ok(self.strings.get(&offset).unwrap())
        }
    }
}

// Iterator stuff
impl<'a> StrTab<'a> {
    pub fn iter(&self) -> hash_map::Iter<u32, Cow<'a, str>> {
        self.strings.iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<u32, Cow<'a, str>> {
        self.strings.iter_mut()
    }
}

impl<'a> IntoIterator for StrTab<'a> {
    type Item = (u32, Cow<'a, str>);
    type IntoIter = hash_map::IntoIter<u32, Cow<'a, str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.strings.into_iter()
    }
}

impl<'a> IntoIterator for &'a StrTab<'a> {
    type Item = (&'a u32, &'a Cow<'a, str>);
    type IntoIter = hash_map::Iter<'a, u32, Cow<'a, str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut StrTab<'a> {
    type Item = (&'a u32, &'a mut Cow<'a, str>);
    type IntoIter = hash_map::IterMut<'a, u32, Cow<'a, str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
