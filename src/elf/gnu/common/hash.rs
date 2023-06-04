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
pub struct HashHeader {
    pub nbuckets: u32,
    pub symndx: u32,
    pub maskwords: u32,
    pub shift2: u32,
}
