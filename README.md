# exofmt

WARNING: This library is currently v0.0.1, meaning it is a WIP and will experience major changes.

exofmt is a binary format parser. A future goal is to implement binary format writing but this is not a priority at the moment.

The main use case for this library is to analyze the structures stored within a binary, load a binary for conversion to a higher level format, or modifying a binary. This library makes use of allocations and the parsers are abstracted to `Read + Seek`. If you only work directly on byte arrays and/or require a no-alloc library, there are better alternatives out there such as Goblin for ELF, MACH-O, or PE.

For ELF, most if not all of the constants should be implemented for nearly every ABI published (or at least the ones I could find). Their respective types, not so much as that takes much longer to implement and verify. I will be slowly working my way to completing everything as I get interest.

## Current Support

This library currently has support for loading the following file types:

- ELF
- Dex
- CDex (Compact Dex)
- VDex
    - Versions 006, 010, 019, 021, and 027
    - Version 021 is untested. While VDex quickening was officially removed in Android 12, it appears that it was disabled in Android 10. I haven't been able to find a VDex file of this version that contains anything in the Dex section
    - Version 027 is fully supported other than the `TypeLookupTable`. I don't need this table so if anyone else needs it, please submit a merge request with support for it. Otherwise, I may add support for it in the distant future.

## Limitations

- This library makes heavy use of `alloc`
- This library makes heavy use of `<u32 value> as usize`, as a result it will not run on 16-bit machines
