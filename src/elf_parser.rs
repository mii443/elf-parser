use nom::{combinator::not, IResult, bytes::complete::{tag, take, take_while}, character::{is_alphabetic, is_alphanumeric}, multi::count, number::complete::{le_u16, le_u32, le_u64}};

use crate::elf_struct::{Elf, ElfHeader, ProgramHeader, SectionHeader, Section};

pub struct ElfParser {
    pub file: Vec<u8>
}

impl ElfParser {
    
    pub fn parse(&mut self) -> IResult<&[u8], Elf> {
        let (_, elf_header) = ElfParser::parse_header(&self.file)?;

        let (_, program_headers) = ElfParser::parse_program_headers(
            elf_header.program_header_num as usize
        )(
            &self.file[elf_header.program_header_offset as usize..]
        )?;

        let (input, section_headers) = ElfParser::parse_section_headers(
            elf_header.section_header_num as usize
        )(
            &self.file[elf_header.section_header_offset as usize..]
        )?;

        let section_names_offset = section_headers[elf_header.section_header_name_index as usize].offset;
        let section_names_bytes = &self.file[section_names_offset as usize..];
        let sections = section_headers.into_iter().map(|header| Section {
            name: String::from_utf8(ElfParser::take_alphabetic(&section_names_bytes[header.name as usize..]).unwrap().1.to_vec()).unwrap(),
            header
        }).collect::<Vec<Section>>();

        Ok((input, Elf {
            header: elf_header,
            program_headers,
            sections
        }))
    }

    fn take_alphabetic(input: &[u8]) -> IResult<&[u8], &[u8]> {
        Ok(take_while(ElfParser::is_not_end)(input)?)
    }
//  17
    fn is_not_end(input: u8) -> bool {
        input != b'\x00'
    }

    pub fn parse_section_headers(num: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<SectionHeader>> {
        move |raw: &[u8]| count(ElfParser::parse_section_header, num)(raw)
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

    pub fn parse_program_headers(num: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<ProgramHeader>> {
        move |raw: &[u8]| count(ElfParser::parse_program_header, num)(raw)
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