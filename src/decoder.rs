use crate::error::DecodeError;
use crate::types::*;

pub fn decode_word(word: u32, addr: u32) -> Result<Instruction, DecodeError> {
    let cond = Condition::from_code(word >> 28)?;

    // Hints (NOP, YIELD, WFE, WFI, SEV)
    if (word & 0x0FFFFFF0) == 0x0320F000 {
        match word & 0xF {
            0 => return Ok(Instruction::Nop { cond }),
            1 => return Ok(Instruction::Yield { cond }),
            2 => return Ok(Instruction::Wfe { cond }),
            3 => return Ok(Instruction::Wfi { cond }),
            4 => return Ok(Instruction::Sev { cond }),
            _ => {}
        }
    }
    // BKPT
    if (word & 0xFFF000F0) == 0xE1200070 {
        let imm12 = (word >> 8) & 0xFFF;
        let imm4 = word & 0xF;
        let imm = (imm12 << 4) | imm4;
        return Ok(Instruction::Bkpt { imm: imm as u16 });
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
    // PUSH Single
    if (word & 0x0FFF0FFF) == 0x052D0004 {
        let rd = Register::from_code((word >> 12) & 0xF)?;
        return Ok(Instruction::Push {
            cond,
            reg_list: vec![rd],
        });
    }
    // POP Single
    if (word & 0x0FFF0FFF) == 0x049D0004 {
        let rd = Register::from_code((word >> 12) & 0xF)?;
        return Ok(Instruction::Pop {
            cond,
            reg_list: vec![rd],
        });
    }
    // PUSH Multiple
    if (word & 0x0FFF0000) == 0x092D0000 {
        let mask = word & 0xFFFF;
        return Ok(Instruction::Push {
            cond,
            reg_list: decode_reg_list(mask),
        });
    }
    // POP Multiple
    if (word & 0x0FFF0000) == 0x08BD0000 {
        let mask = word & 0xFFFF;
        return Ok(Instruction::Pop {
            cond,
            reg_list: decode_reg_list(mask),
        });
    }
    // Divide (SDIV / UDIV)
    if (word & 0x0FF0F0F0) == 0x0710F010 || (word & 0x0FF0F0F0) == 0x0730F010 {
        let signed = (word & 0x00200000) == 0; // 0x071 is SDIV, 0x073 is UDIV
        let rd = Register::from_code((word >> 16) & 0xF)?;
        let rm = Register::from_code((word >> 8) & 0xF)?;
        let rn = Register::from_code(word & 0xF)?;
        return Ok(Instruction::Divide {
            cond,
            signed,
            rd,
            rn,
            rm,
        });
    }
    // MUL / MLA
    if (word & 0x0FC000F0) == 0x00000090 {
        let s = (word & (1 << 20)) != 0;
        let rd = Register::from_code((word >> 16) & 0xF)?;
        let rm = Register::from_code((word >> 8) & 0xF)?;
        let rn = Register::from_code(word & 0xF)?;

        let a = (word & (1 << 21)) != 0;
        if a {
            let ra = Register::from_code((word >> 12) & 0xF)?;
            return Ok(Instruction::MultiplyAccumulate {
                cond,
                s,
                rd,
                rn,
                rm,
                ra,
            });
        } else {
            return Ok(Instruction::Multiply {
                cond,
                s,
                rd,
                rn,
                rm,
            });
        }
    }
    // MLS
    if (word & 0x0FF000F0) == 0x00600090 {
        let rd = Register::from_code((word >> 16) & 0xF)?;
        let ra = Register::from_code((word >> 12) & 0xF)?;
        let rm = Register::from_code((word >> 8) & 0xF)?;
        let rn = Register::from_code(word & 0xF)?;
        return Ok(Instruction::MultiplySubtract {
            cond,
            rd,
            rn,
            rm,
            ra,
        });
    }
    // B / BL
    if (word & 0x0E000000) == 0x0A000000 {
        let link = (word & (1 << 24)) != 0;
        let mut imm24 = word & 0x00FFFFFF;
        if (imm24 & 0x00800000) != 0 {
            imm24 |= 0xFF000000; // sign extend
        }
        let offset = imm24 as i32;
        let target_addr = (addr.wrapping_add(8)).wrapping_add_signed(offset << 2);
        return Ok(Instruction::Branch {
            cond,
            link,
            target_addr,
        });
    }
    // Load/Store Extra (LDRH, STRH, LDRD, STRD, LDRSB, LDRSH)
    if (word & 0x0E000090) == 0x00000090 && (word & 0x00000060) != 0 {
        let l = (word & (1 << 20)) != 0;
        let i = (word & (1 << 22)) != 0; // 1 = Immediate, 0 = Register
        let u = (word & (1 << 23)) != 0;
        let s = (word & (1 << 6)) != 0;
        let h = (word & (1 << 5)) != 0;

        let rn = Register::from_code((word >> 16) & 0xF)?;
        let rd = Register::from_code((word >> 12) & 0xF)?;

        let op = match (l, s, h) {
            (false, false, true) => ExtraOp::Strh,
            (true, false, true) => ExtraOp::Ldrh,
            (false, true, false) => ExtraOp::Ldrd,
            (true, true, false) => ExtraOp::Ldrsb,
            (false, true, true) => ExtraOp::Strd,
            (true, true, true) => ExtraOp::Ldrsh,
            _ => return Err(DecodeError::UnknownInstruction { word }),
        };

        let addressing = if i {
            let imm8 = (((word >> 8) & 0xF) << 4) | (word & 0xF);
            let offset = if u { imm8 as i32 } else { -(imm8 as i32) };
            AddressingMode::OffsetImmediate(rn, offset)
        } else {
            let rm = Register::from_code(word & 0xF)?;
            AddressingMode::OffsetRegister(rn, rm, u)
        };

        return Ok(Instruction::LoadStoreExtra {
            cond,
            op,
            rd,
            addressing,
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
                AddressingMode::OffsetRegister(rn, rm, u)
            } else {
                AddressingMode::OffsetScaled(
                    rn,
                    rm,
                    ShiftType::from_code(shift_code)?,
                    shift_imm,
                    u,
                )
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
            DataOpcode::Mov | DataOpcode::Mvn => None,
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
