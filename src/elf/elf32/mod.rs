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
    crate::elf::gnu::HashTable<u32>,
);
