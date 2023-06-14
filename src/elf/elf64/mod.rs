// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
    RelR,
    crate::elf::gnu::HashTable<u64>,
);
