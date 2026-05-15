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

pub struct Decoder {
    start_address: u32,
    endian: Endian,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            start_address: 0,
            endian: Endian::Little,
        }
    }

    pub fn start_address(mut self, start_address: u32) -> Self {
        self.start_address = start_address;
        self
    }

    pub fn endian(mut self, endian: Endian) -> Self {
        self.endian = endian;
        self
    }

    pub fn disassemble(self, bytes: &[u8]) -> Result<Vec<String>, DecodeError> {
        if !bytes.len().is_multiple_of(4) {
            return Err(DecodeError::UnalignedInput);
        }

        let mut results = Vec::new();

        for (i, chunk) in bytes.chunks_exact(4).enumerate() {
            let addr = self.start_address + (i as u32) * 4;
            let word = match self.endian {
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
    Decoder::new().disassemble(bytes)
}
