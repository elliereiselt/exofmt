use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Dyn {
    // This was originally `i32`, I've replaced it with `u32` as it's just easier to work with
    pub d_tag: u32,
    pub d_val: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_size_check() {
        assert_eq!(::std::mem::size_of::<Dyn>(), 0x8);
    }
}

impl TryFrom<crate::elf::Dyn> for Dyn {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: crate::elf::Dyn) -> Result<Self, Self::Error> {
        Ok(Self {
            d_tag: u32::try_from(value.d_tag)?,
            d_val: u32::try_from(value.d_val)?,
        })
    }
}

impl From<Dyn> for crate::elf::Dyn {
    fn from(value: Dyn) -> Self {
        Self {
            d_tag: u64::from(value.d_tag),
            d_val: u64::from(value.d_val),
        }
    }
}
