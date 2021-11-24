#[derive(Debug)]
pub struct ElfHeader {
    pub magic_number: u64,
    pub elf_type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub elf_header_size: u16,
    pub program_header_entry_size: u16,
    pub program_header_num: u16,
    pub section_header_entry_size: u16,
    pub section_header_num: u16,
    pub section_header_name_index: u16
}