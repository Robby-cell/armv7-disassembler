use crate::decoder::decode_word;
use crate::error::DecodeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    Little,
}

pub struct DisassemblerOptions {
    pub start_address: u32,
    pub endian: Endian,
}

impl Default for DisassemblerOptions {
    fn default() -> Self {
        DisassemblerOptions {
            start_address: 0,
            endian: Endian::Little,
        }
    }
}

pub fn disassemble(bytes: &[u8]) -> Result<Vec<String>, DecodeError> {
    disassemble_with_options(bytes, DisassemblerOptions::default())
}

pub fn disassemble_with_options(
    bytes: &[u8],
    options: DisassemblerOptions,
) -> Result<Vec<String>, DecodeError> {
    if bytes.len() % 4 != 0 {
        return Err(DecodeError::UnalignedInput);
    }

    let mut results = Vec::new();

    for (i, chunk) in bytes.chunks_exact(4).enumerate() {
        let addr = options.start_address + (i as u32) * 4;
        let word = match options.endian {
            Endian::Little => u32::from_le_bytes(chunk.try_into().unwrap()),
            Endian::Big => u32::from_be_bytes(chunk.try_into().unwrap()),
        };

        match decode_word(word, addr) {
            Ok(instr) => results.push(instr.to_string()),
            Err(_) => results.push(format!(".word 0x{:08x}", word)),
        }
    }

    Ok(results)
}
