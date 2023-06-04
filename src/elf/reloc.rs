#[derive(Clone)]
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

#[derive(Clone)]
pub struct RelA {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

#[derive(Clone)]
pub struct RelR {
    pub r_offset: u64,
}

impl Rel {
    pub fn r_sym(&self) -> u32 {
        crate::elf::elf64::elf64_r_sym(self.r_info)
    }

    pub fn r_type(&self) -> u32 {
        crate::elf::elf64::elf64_r_type(self.r_info)
    }

    pub fn set_r_sym_and_r_type(&mut self, r_sym: u32, r_type: u32) {
        self.r_info = crate::elf::elf64::elf64_r_info(r_sym, r_type);
    }
}

impl RelA {
    pub fn r_sym(&self) -> u32 {
        crate::elf::elf64::elf64_r_sym(self.r_info)
    }

    pub fn r_type(&self) -> u32 {
        crate::elf::elf64::elf64_r_type(self.r_info)
    }

    pub fn set_r_sym_and_r_type(&mut self, r_sym: u32, r_type: u32) {
        self.r_info = crate::elf::elf64::elf64_r_info(r_sym, r_type);
    }
}
