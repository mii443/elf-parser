#[derive(Debug)]
pub struct Elf {
    pub header: ElfHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub sections: Vec<Section>
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub header: SectionHeader
}

#[derive(Debug)]
pub struct SectionHeader {
    pub name: u32,
    pub section_header_type: u32,
    pub flags: u64,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub align: u64,
    pub entry_size: u64
}

#[derive(Debug, Clone, Copy)]
pub struct ElfHeader {
    pub ident: (u64, u64),
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
/*
+-----------------------------------------------+
|e_ident[EI_NIDENT]                             |
+-----------------------------------------------+
|7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00|
+-----+-----+-----------+-----------------------+
|type |mach |ver        |e_entry                |
+-----+-----+-----------+-----------------------+
|02 00|3e 00|01 00 00 00|78 00 40 00 00 00 00 00|
+-----+-----+-----------+-----------------------+
|e_phoff                |e_shoff                |
+-----------------------+-----------------------+
|40 00 00 00 00 00 00 00|00 00 00 00 00 00 00 00|
+-----------+-----+-----+-----+-----+-----+-----+
|e_flags    |ehsiz|phesi|phnum|shesi|shnum|shstr|
+-----------+-----+-----+-----+-----+-----+-----+
|00 00 00 00|40 00|38 00|01 00|40 00|00 00|00 00|
+-----------+-----+-----+-----+-----+-----+-----+
*/

#[derive(Debug)]
pub struct ProgramHeader {
    pub program_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64
}
/*
+-----------+-----------+-----------------------+
|p_type     |p_flags    |p_offset               |
+-----------+-----------+-----------------------+
|01 00 00 00|07 00 00 00|00 00 00 00 00 00 00 00|
+-----------+-----------+-----------------------+
|p_vaddr                |p_paddr                |
+-----------------------+-----------------------+
|00 00 40 00 00 00 00 00|00 00 40 00 00 00 00 00|
+-----------------------+-----------------------+
|p_filesz               |p_memsz                |
+-----------------------+-----------------------+
|90 00 00 00 00 00 00 00|90 00 00 00 00 00 00 00|
+-----------------------+-----------------------+
|p_align                |
+-----------------------+
|00 00 20 00 00 00 00 00|
+-----------------------+
*/

