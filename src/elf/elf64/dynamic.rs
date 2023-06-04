use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Dyn {
    // This was originally `i64`, I've replaced it with `u64` as it's just easier to work with
    pub d_tag: u64,
    pub d_val: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_size_check() {
        assert_eq!(::std::mem::size_of::<Dyn>(), 0x10);
    }
}

impl From<crate::elf::Dyn> for Dyn {
    fn from(value: crate::elf::Dyn) -> Self {
        Self {
            d_tag: value.d_tag,
            d_val: value.d_val,
        }
    }
}

impl From<Dyn> for crate::elf::Dyn {
    fn from(value: Dyn) -> Self {
        Self {
            d_tag: value.d_tag,
            d_val: value.d_val,
        }
    }
}
