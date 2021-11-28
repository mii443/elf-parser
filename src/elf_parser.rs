use nom::{IResult, bytes::complete::{tag, take}, number::complete::{le_u16, le_u32, le_u64}};

use crate::elf_struct::{Elf, ElfHeader, ProgramHeader, SectionHeader};

pub struct ElfParser {
    pub file: Vec<u8>
}

impl ElfParser {
    
    pub fn parse(&mut self) -> IResult<&[u8], Elf> {
        let (input_headerless, elf_header) = ElfParser::parse_header(&self.file)?;
        let (mut input, _) = take(elf_header.program_header_offset - 64)(input_headerless)?;

        let mut program_header_counter = 0;
        let mut program_headers: Vec<ProgramHeader> = vec![];
        while program_header_counter < elf_header.program_header_num {
            let (input_tmp, program_header) = ElfParser::parse_program_header(input)?;
            program_headers.push(program_header);
            input = input_tmp;
            program_header_counter += 1;
        }

        let (mut input, _) = take(elf_header.section_header_offset - 64)(input_headerless)?;

        let mut section_header_counter = 0;
        let mut section_headers: Vec<SectionHeader> = vec![];
        while section_header_counter < elf_header.section_header_num {
            let (input_tmp, section_header) = ElfParser::parse_section_header(input)?;
            section_headers.push(section_header);
            input = input_tmp;
            section_header_counter += 1;
        }        

        Ok((input, Elf {
            header: elf_header,
            program_headers,
            section_headers
        }))
    }

    pub fn parse_section_header(input: &[u8]) -> IResult<&[u8], SectionHeader> {
        let (input, name) = le_u32(input)?;
        let (input, section_header_type) = le_u32(input)?;
        let (input, flags) = le_u64(input)?;
        let (input, address) = le_u64(input)?;
        let (input, offset) = le_u64(input)?;
        let (input, size) = le_u64(input)?;
        let (input, link) = le_u32(input)?;
        let (input, info) = le_u32(input)?;
        let (input, align) = le_u64(input)?;
        let (input, entry_size) = le_u64(input)?;

        Ok((input, SectionHeader {
            name,
            section_header_type,
            flags,
            address,
            offset,
            size,
            link,
            info,
            align,
            entry_size
        }))
    }

    pub fn parse_program_header(input: &[u8]) -> IResult<&[u8], ProgramHeader> {
        let (input, program_type) = le_u32(input)?;
        let (input, flags) = le_u32(input)?;
        let (input, offset) = le_u64(input)?;
        let (input, virtual_address) = le_u64(input)?;
        let (input, physical_address) = le_u64(input)?;
        let (input, file_size) = le_u64(input)?;
        let (input, memory_size) = le_u64(input)?;
        let (input, align) = le_u64(input)?;

        Ok((input, ProgramHeader {
            program_type,
            flags,
            offset,
            virtual_address,
            physical_address,
            file_size,
            memory_size,
            align
        }))
    }

    pub fn parse_header(input: &[u8]) -> IResult<&[u8], ElfHeader> {
        let (input, ident_1) = le_u64(input)?;
        let (input, ident_2) = le_u64(input)?;
        let (input, elf_type) = le_u16(input)?;
        let (input, machine) = le_u16(input)?;
        let (input, version) = le_u32(input)?;
        let (input, entry) = le_u64(input)?;
        let (input, program_header_offset) = le_u64(input)?;
        let (input, section_header_offset) = le_u64(input)?;
        let (input, flags) = le_u32(input)?;
        let (input, elf_header_size) = le_u16(input)?;
        let (input, program_header_entry_size) = le_u16(input)?;
        let (input, program_header_num) = le_u16(input)?;
        let (input, section_header_entry_size) = le_u16(input)?;
        let (input, section_header_num) = le_u16(input)?;
        let (input, section_header_name_index) = le_u16(input)?;

        Ok((input, ElfHeader {
            ident: (ident_1, ident_2),
            elf_type,
            machine,
            version,
            entry,
            program_header_offset,
            section_header_offset,
            flags,
            elf_header_size,
            program_header_entry_size,
            program_header_num,
            section_header_entry_size,
            section_header_num,
            section_header_name_index
        }))
    }
}

// 192