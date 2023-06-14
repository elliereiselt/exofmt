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
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

// This is the piece that broke me and made me start hating GNU's developers for not fucking documenting their additions for a widely used section of the ELF file format.
// As far as I can tell, it was finalzied in 2006. I was ten years old when they added this.
// At the time of writing this, I am now 27 and it has been 17 YEARS that they could have, at any point in time, written formal documentation for this section.
//
// Sources I looked to to determine how to implment this:
//  - https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/Object/ELFTypes.h
//  - http://www.linker-aliens.org/blogs/ali/entry/gnu_hash_elf_sections/
//  - https://sourceware.org/git/?p=elfutils.git;a=blob;f=libdwfl/dwfl_module_getdwarf.c;h=9ba499bbfed34bae5fe12309a6563e753c759bce;hb=HEAD
//
// Ultimately, it was the more sanely written, much better documented code of LLVM that helped me the most with this.
// If you ever want to know how GNU implements something my suggestion is to not look at GNU's code and look at a competitor that implements the exact same thing. They most likely have both implemented it better or actually documented it.
#[repr(C)]
#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct HashHeader {
    pub nbuckets: u32,
    pub symndx: u32,
    pub maskwords: u32,
    pub shift2: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_header_size_check() {
        assert_eq!(::std::mem::size_of::<HashHeader>(), 0x10);
    }
}
