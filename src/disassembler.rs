use crate::decoder::decode_word;
use crate::error::DecodeError;

/// Endianness selection with distinct discriminants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// Big‑endian mode – numeric value.
    Big = 1 << 30,

    /// Little‑endian mode – numeric value 0.
    Little = 0,
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

pub struct Decoder {
    pub options: DisassemblerOptions,
}

impl Decoder {
    pub fn new(options: DisassemblerOptions) -> Self {
        Self { options }
    }

    pub fn disassemble(&self, bytes: &[u8]) -> Result<Vec<String>, DecodeError> {
        if !bytes.len().is_multiple_of(4) {
            return Err(DecodeError::UnalignedInput);
        }

        let mut results = Vec::new();

        for (i, chunk) in bytes.chunks_exact(4).enumerate() {
            let addr = self.options.start_address + (i as u32) * 4;
            let word = match self.options.endian {
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
}

/// Standalone helper: Disassemble bytes using default options.
pub fn disassemble(bytes: &[u8]) -> Result<Vec<String>, DecodeError> {
    Decoder::new(DisassemblerOptions::default()).disassemble(bytes)
}

/// Standalone helper: Disassemble bytes using explicit options.
pub fn disassemble_with_options(
    bytes: &[u8],
    options: DisassemblerOptions,
) -> Result<Vec<String>, DecodeError> {
    Decoder::new(options).disassemble(bytes)
}
