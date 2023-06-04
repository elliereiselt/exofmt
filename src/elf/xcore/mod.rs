// Section flags
/// All sections with the "d" flag are grouped together by the linker to form
/// the data section and the dp register is set to the start of the section by
/// the boot code.
pub const SHF_XCORE_DP_SECTION: u32 = 0x10000000;
/// All sections with the "c" flag are grouped together by the linker to form
/// the constant pool and the cp register is set to the start of the constant
/// pool by the boot code.
pub const SHF_XCORE_CP_SECTION: u32 = 0x20000000;
