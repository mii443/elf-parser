mod elf_struct;
mod elf_parser;
use std::io::Read;

use crate::elf_parser::ElfParser;

fn main() {
    let mut file = std::fs::File::open("./test.elf").expect("Failed to open a file.");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read a file.");
    let mut elf_parser = ElfParser { file: buf };

    match elf_parser.parse() {
        Ok((_, elf)) => {
            println!("{:?}", elf.header);
            for program_header in elf.program_headers {
                println!("{:?}", program_header);
            }

            for section_header in elf.section_headers {
                println!("{:?}", section_header);
            }
        },
        Err(_) => {}
    }
}
// 1236