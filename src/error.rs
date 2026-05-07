use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    UnalignedInput,
    InvalidRegisterCode { code: u32 },
    InvalidConditionCode { code: u32 },
    InvalidOpcode { code: u32 },
    InvalidShiftType { code: u32 },
    UnknownInstruction { word: u32 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::UnalignedInput => write!(f, "Input byte slice is not aligned to 4 bytes"),
            DecodeError::InvalidRegisterCode { code } => {
                write!(f, "Invalid register code: {}", code)
            }
            DecodeError::InvalidConditionCode { code } => {
                write!(f, "Invalid condition code: {}", code)
            }
            DecodeError::InvalidOpcode { code } => write!(f, "Invalid data opcode: {}", code),
            DecodeError::InvalidShiftType { code } => write!(f, "Invalid shift type: {}", code),
            DecodeError::UnknownInstruction { word } => {
                write!(f, "Unknown instruction: 0x{:08x}", word)
            }
        }
    }
}

impl core::error::Error for DecodeError {}
