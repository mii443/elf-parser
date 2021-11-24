mod elf_struct;
mod elf_parser;
use std::io::Read;

use crate::elf_parser::ElfParser;

fn main() {
    println!("Hello, world!");
    let mut file = std::fs::File::open("./test.elf").expect("Failed to open a file.");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read a file.");
    let mut elf_parser = ElfParser { file: buf };

    match ElfParser::parse_header(&elf_parser.file) {
        Ok(input) => {
            println!("{:?}", input.0[0]);
            println!("{:?}", input.0[1]);
            println!("{:?}", input.0[2]);
            println!("{:?}", input.0[3]);
            println!("{:?}", input.1);
        },
        Err(_) => {
            println!("Error.");
        }
    }
}
// 282584257676671