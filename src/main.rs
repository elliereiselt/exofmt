// Start future `lib.rs`
pub mod error;

pub mod elf;

pub(crate) mod stringable_consts_blocks;

#[cfg(test)]
mod tests {
    #[test]
    fn size_check() {
        assert!(::std::mem::size_of::<usize>() >= 4);
    }
}

// Start temporary `main.rs`
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    let path = Path::new("test_files/libcamera_client.so");
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("test file not found");
    let elf_ident = match elf::get_elf_ident(&mut file) {
        Ok(elf_ident) => elf_ident,
        Err(error) => panic!("{}", error),
    };

    println!("ELF class is: {:?}", elf_ident.class);

    if elf_ident.class == elf::ElfClass::Elf64 {
        let mut reader = elf::elf64::IoReader::new(&mut file, elf_ident.endianness).unwrap();

        match reader.parse() {
            Ok(elf) => {
                println!("Header:");
                println!("\tEI_CLASS: {}", elf.header.e_ident[elf::EI_CLASS]);
                println!("\tEI_DATA: {}", elf.header.e_ident[elf::EI_DATA]);
                println!("\tEI_VERSION: {}", elf.header.e_ident[elf::EI_VERSION]);
                println!(
                    "\tEI_OSABI: {}",
                    elf::osabi_to_str(elf.header.e_ident[elf::EI_OSABI])
                );
                println!(
                    "\tEI_ABIVERSION: {}",
                    elf.header.e_ident[elf::EI_ABIVERSION]
                );
                println!("\tType: {}", elf::et_to_str(elf.header.e_type));
                println!("\tMachine: {}", elf::em_to_str(elf.header.e_machine));
                println!("\tVersion: {}", elf.header.e_version);
                println!("\tEntry: {:x}", elf.header.e_entry);
                println!("\tProgram headers offset: {:x}", elf.header.e_phoff);
                println!("\tSection headers offset: {:x}", elf.header.e_shoff);
                println!("\tFlags: {}", elf.header.e_flags);
                println!("\tHeader size: {}", elf.header.e_ehsize);
                println!("\tProgram header size: {}", elf.header.e_phentsize);
                println!("\tProgram header count: {}", elf.header.e_phnum);
                println!("\tSection header size: {}", elf.header.e_shentsize);
                println!("\tSection header count: {}", elf.header.e_shnum);
                println!("\tSection header strtab index: {}", elf.header.e_shstrndx);

                println!();

                println!("Section Headers:");

                for section_header in &elf.section_headers {
                    println!("\tSection Header:");
                    println!(
                        "\t\tName: {:?}",
                        elf.shstrtab.get_at_offset(section_header.sh_name).unwrap()
                    );
                    println!("\t\tType: {:x}", section_header.sh_type);
                    println!(
                        "\t\tType str: {}",
                        crate::elf::sht_to_str(section_header.sh_type)
                    );
                    println!("\t\tFlags: {:?}", section_header.sh_flags);
                    println!("\t\tAddr: {:x}", section_header.sh_addr);
                    println!("\t\tOffset: {:x}", section_header.sh_offset);
                }
            }
            Err(error) => eprintln!("{}", error),
        };
    }
}
