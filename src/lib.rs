// Start future `lib.rs`
pub mod error;
pub mod leb128;
pub mod mutf8;

pub mod dex;
pub mod elf;

pub(crate) mod stringable_consts_blocks;

#[cfg(test)]
mod tests {
    #[test]
    fn size_check() {
        assert!(::std::mem::size_of::<usize>() >= 4);
    }
}
