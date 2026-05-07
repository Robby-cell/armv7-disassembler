use crate::error::DecodeError;
use crate::types::*;

pub fn decode_word(word: u32, addr: u32) -> Result<Instruction, DecodeError> {
    let cond = Condition::from_code(word >> 28)?;

    // NOP
    if (word & 0x0FFFFFFF) == 0x0320F000 {
        return Ok(Instruction::Nop { cond });
    }
    // BX
    if (word & 0x0FFFFFF0) == 0x012FFF10 {
        let rm = Register::from_code(word & 0xF)?;
        return Ok(Instruction::BranchExchange { cond, rm });
    }
    // SVC
    if (word & 0x0F000000) == 0x0F000000 {
        let imm = word & 0x00FFFFFF;
        return Ok(Instruction::Svc { cond, imm });
    }
    // PUSH Single (Optimized equivalent to STR Rd, [SP, #-4]!)
    if (word & 0x0FFF0FFF) == 0x052D0004 {
        let rd = Register::from_code((word >> 12) & 0xF)?;
        return Ok(Instruction::Push {
            cond,
            reg_list: vec![rd],
        });
    }
    // POP Single (Optimized equivalent to LDR Rd, [SP], #4)
    if (word & 0x0FFF0FFF) == 0x049D0004 {
        let rd = Register::from_code((word >> 12) & 0xF)?;
        return Ok(Instruction::Pop {
            cond,
            reg_list: vec![rd],
        });
    }
    // PUSH Multiple (STMDB SP!)
    if (word & 0x0FFF0000) == 0x092D0000 {
        let mask = word & 0xFFFF;
        return Ok(Instruction::Push {
            cond,
            reg_list: decode_reg_list(mask),
        });
    }
    // POP Multiple (LDMIA SP!)
    if (word & 0x0FFF0000) == 0x08BD0000 {
        let mask = word & 0xFFFF;
        return Ok(Instruction::Pop {
            cond,
            reg_list: decode_reg_list(mask),
        });
    }
    // MUL
    if (word & 0x0FC000F0) == 0x00000090 {
        let s = (word & (1 << 20)) != 0;
        let rd = Register::from_code((word >> 16) & 0xF)?;
        let rm = Register::from_code((word >> 8) & 0xF)?;
        let rn = Register::from_code(word & 0xF)?;
        return Ok(Instruction::Multiply {
            cond,
            s,
            rd,
            rn,
            rm,
        });
    }
    // B / BL
    if (word & 0x0E000000) == 0x0A000000 {
        let link = (word & (1 << 24)) != 0;
        let mut imm24 = word & 0x00FFFFFF;
        if (imm24 & 0x00800000) != 0 {
            imm24 |= 0xFF000000; // Sign extend
        }
        let offset = imm24 as i32;
        let target_addr = (addr.wrapping_add(8)).wrapping_add_signed(offset << 2);
        return Ok(Instruction::Branch {
            cond,
            link,
            target_addr,
        });
    }
    // LDR / STR / LDRB / STRB
    if (word & 0x0E000000) == 0x04000000 {
        let load = (word & (1 << 20)) != 0;
        let byte = (word & (1 << 22)) != 0;
        let i = (word & (1 << 25)) != 0;
        let u = (word & (1 << 23)) != 0;
        let rn = Register::from_code((word >> 16) & 0xF)?;
        let rd = Register::from_code((word >> 12) & 0xF)?;

        let addressing = if !i {
            let imm = word & 0xFFF;
            let offset = if u { imm as i32 } else { -(imm as i32) };
            AddressingMode::OffsetImmediate(rn, offset)
        } else {
            let rm = Register::from_code(word & 0xF)?;
            let shift_code = (word >> 5) & 0b11;
            let shift_imm = (word >> 7) & 0x1F;
            if shift_imm == 0 && shift_code == 0 {
                AddressingMode::OffsetRegister(rn, rm)
            } else {
                AddressingMode::OffsetScaled(rn, rm, ShiftType::from_code(shift_code)?, shift_imm)
            }
        };
        return Ok(Instruction::LoadStore {
            cond,
            load,
            byte,
            rd,
            addressing,
        });
    }
    // Data Processing
    if (word & 0x0C000000) == 0x00000000 {
        let i = (word & (1 << 25)) != 0;
        let opcode = DataOpcode::from_code((word >> 21) & 0xF)?;
        let s = (word & (1 << 20)) != 0;
        let rn = Register::from_code((word >> 16) & 0xF)?;
        let rd = Register::from_code((word >> 12) & 0xF)?;

        let operand2 = if i {
            let rot = (word >> 8) & 0xF;
            let imm8 = word & 0xFF;
            let val = imm8.rotate_right(rot * 2);
            ShifterOperand::Immediate(val)
        } else {
            let rm = Register::from_code(word & 0xF)?;
            let bit4 = (word & (1 << 4)) != 0;
            if !bit4 {
                let shift_code = (word >> 5) & 0b11;
                let shift_imm = (word >> 7) & 0x1F;
                if shift_imm == 0 {
                    if shift_code == 0 {
                        ShifterOperand::Register(rm)
                    } else if shift_code == 3 {
                        ShifterOperand::RRX(rm)
                    } else {
                        ShifterOperand::ImmediateShift(rm, ShiftType::from_code(shift_code)?, 0)
                    }
                } else {
                    ShifterOperand::ImmediateShift(rm, ShiftType::from_code(shift_code)?, shift_imm)
                }
            } else {
                let shift_code = (word >> 5) & 0b11;
                let rs = Register::from_code((word >> 8) & 0xF)?;
                ShifterOperand::RegisterShift(rm, ShiftType::from_code(shift_code)?, rs)
            }
        };

        let rn_opt = match opcode {
            DataOpcode::MOV | DataOpcode::MVN => None,
            _ => Some(rn),
        };

        return Ok(Instruction::DataProcessing {
            cond,
            s,
            opcode,
            rd,
            rn: rn_opt,
            operand2,
        });
    }

    Err(DecodeError::UnknownInstruction { word })
}

fn decode_reg_list(mask: u32) -> Vec<Register> {
    let mut regs = Vec::new();
    for i in 0..16 {
        if (mask & (1 << i)) != 0 {
            regs.push(Register::from_code(i).unwrap());
        }
    }
    regs
}
