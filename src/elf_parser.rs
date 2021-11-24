use nom::{IResult, bytes::complete::tag, number::complete::{le_u16, le_u32, le_u64}};

use crate::elf_struct::ElfHeader;

pub struct ElfParser {
    pub file: Vec<u8>
}

impl ElfParser {
    pub fn parse_header(input: &[u8]) -> IResult<&[u8], u64> {
        let (input, ident) = le_u64(input)?;
        let (input, elf_type) = le_u16(input)?;
        let (input, version) = le_u32(input)?;
        Ok((input, ident))
    }
}