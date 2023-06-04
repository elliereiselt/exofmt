mod header;
pub use header::*;
mod program_header;
pub use program_header::*;
mod section_header;
pub use section_header::*;
mod sym;
pub use sym::*;
mod reloc;
pub use reloc::*;
mod dynamic;
pub use dynamic::*;
mod compressed;
pub use compressed::*;

crate::elf::elf_io_reader_impl!(
    Header,
    ProgramHeader,
    SectionHeader,
    Sym,
    CompressionHeader,
    Dyn,
    Rel,
    RelA,
    RelR
);
