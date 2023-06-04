# exofmt

WARNING: This library is currently v0.0.1, meaning it is a WIP and will experience major changes. The goal is to create a `devel` branch once v0.1.0 is released and the ELF API is finalized. This also isn't yet a "library" as I'm testing using a `main.rs` that will also be removed once v0.1.0 is reached.

exofmt is a binary format parser. A future goal is to implement binary format writing but this is not a priority at the moment.

The main use case for this library is to analyze the structures stored within a binary, load a binary for conversion to a higher level format, or modifying a binary. This library makes use of allocations and the parsers are abstracted to `Read + Seek`. If you only work directly on byte arrays and/or require a no-alloc library, there are better alternatives out there such as Goblin for ELF, MACH-O, or PE.

For ELF, most if not all of the constants should be implemented for nearly every ABI published (or at least the ones I could find). Their respective types, not so much as that takes much longer to implement and verify. I will be slowly working my way to completing everything as I get interest.

## Goals

- Priority support for ELF and Dex
    - Next priority will then be ART, OAT, and VDEX
    - Future plans for MACH-O and PE, though much less of a priority for my personal uses for the library currently.
- Priority support for Android
    - Existing libraries I looked at did not have any attempt at support for Android-specific ELF files. Android is my main purpose for loading ELF files, as such I'll prioritize Android before all other platforms.
- Future plans for writing these formats
    - Writing will most likely perform no validation
    - Maybe future plans for validators? I'm unsure and getting ahead of myself here.

## Limitations

- This library makes heavy use of `alloc`
- This library makes heavy use of `<u32 value> as usize`, as a result it will not run on 16-bit machines
