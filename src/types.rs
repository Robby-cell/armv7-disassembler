use crate::error::DecodeError;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    Sp,
    Lr,
    Pc,
}

impl Register {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),
            11 => Ok(Register::R11),
            12 => Ok(Register::R12),
            13 => Ok(Register::Sp),
            14 => Ok(Register::Lr),
            15 => Ok(Register::Pc),
            _ => Err(DecodeError::InvalidRegisterCode { code }),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::R3 => "r3",
            Register::R4 => "r4",
            Register::R5 => "r5",
            Register::R6 => "r6",
            Register::R7 => "r7",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::Sp => "sp",
            Register::Lr => "lr",
            Register::Pc => "pc",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    Eq,
    Ne,
    Cs,
    Cc,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
}

impl Condition {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0x0 => Ok(Condition::Eq),
            0x1 => Ok(Condition::Ne),
            0x2 => Ok(Condition::Cs),
            0x3 => Ok(Condition::Cc),
            0x4 => Ok(Condition::Mi),
            0x5 => Ok(Condition::Pl),
            0x6 => Ok(Condition::Vs),
            0x7 => Ok(Condition::Vc),
            0x8 => Ok(Condition::Hi),
            0x9 => Ok(Condition::Ls),
            0xA => Ok(Condition::Ge),
            0xB => Ok(Condition::Lt),
            0xC => Ok(Condition::Gt),
            0xD => Ok(Condition::Le),
            0xE => Ok(Condition::Al),
            _ => Err(DecodeError::InvalidConditionCode { code }),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Condition::Al => "",
            Condition::Eq => "eq",
            Condition::Ne => "ne",
            Condition::Cs => "cs",
            Condition::Cc => "cc",
            Condition::Mi => "mi",
            Condition::Pl => "pl",
            Condition::Vs => "vs",
            Condition::Vc => "vc",
            Condition::Hi => "hi",
            Condition::Ls => "ls",
            Condition::Ge => "ge",
            Condition::Lt => "lt",
            Condition::Gt => "gt",
            Condition::Le => "le",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftType {
    LSl,
    Lsr,
    Asr,
    Ror,
}

impl ShiftType {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0 => Ok(ShiftType::LSl),
            1 => Ok(ShiftType::Lsr),
            2 => Ok(ShiftType::Asr),
            3 => Ok(ShiftType::Ror),
            _ => Err(DecodeError::InvalidShiftType { code }),
        }
    }
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ShiftType::LSl => "lsl",
            ShiftType::Lsr => "lsr",
            ShiftType::Asr => "asr",
            ShiftType::Ror => "ror",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataOpcode {
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,
}

impl DataOpcode {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0x0 => Ok(DataOpcode::And),
            0x1 => Ok(DataOpcode::Eor),
            0x2 => Ok(DataOpcode::Sub),
            0x3 => Ok(DataOpcode::Rsb),
            0x4 => Ok(DataOpcode::Add),
            0x5 => Ok(DataOpcode::Adc),
            0x6 => Ok(DataOpcode::Sbc),
            0x7 => Ok(DataOpcode::Rsc),
            0x8 => Ok(DataOpcode::Tst),
            0x9 => Ok(DataOpcode::Teq),
            0xA => Ok(DataOpcode::Cmp),
            0xB => Ok(DataOpcode::Cmn),
            0xC => Ok(DataOpcode::Orr),
            0xD => Ok(DataOpcode::Mov),
            0xE => Ok(DataOpcode::Bic),
            0xF => Ok(DataOpcode::Mvn),
            _ => Err(DecodeError::InvalidOpcode { code }),
        }
    }
}

impl fmt::Display for DataOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataOpcode::And => "and",
            DataOpcode::Eor => "eor",
            DataOpcode::Sub => "sub",
            DataOpcode::Rsb => "rsb",
            DataOpcode::Add => "add",
            DataOpcode::Adc => "adc",
            DataOpcode::Sbc => "sbc",
            DataOpcode::Rsc => "rsc",
            DataOpcode::Tst => "tst",
            DataOpcode::Teq => "teq",
            DataOpcode::Cmp => "cmp",
            DataOpcode::Cmn => "cmn",
            DataOpcode::Orr => "orr",
            DataOpcode::Mov => "mov",
            DataOpcode::Bic => "bic",
            DataOpcode::Mvn => "mvn",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtraOp {
    Strh,
    Ldrh,
    Strd,
    Ldrd,
    Ldrsb,
    Ldrsh,
}

impl fmt::Display for ExtraOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExtraOp::Strh => "strh",
            ExtraOp::Ldrh => "ldrh",
            ExtraOp::Strd => "strd",
            ExtraOp::Ldrd => "ldrd",
            ExtraOp::Ldrsb => "ldrsb",
            ExtraOp::Ldrsh => "ldrsh",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum ShifterOperand {
    Immediate(u32),
    Register(Register),
    ImmediateShift(Register, ShiftType, u32),
    RegisterShift(Register, ShiftType, Register),
    RRX(Register),
}

fn format_imm(val: u32) -> String {
    if val <= 9 {
        format!("#{}", val)
    } else {
        format!("#0x{:x}", val)
    }
}

fn format_offset(val: i32) -> String {
    if val.abs() <= 9 {
        format!("#{}", val)
    } else if val < 0 {
        format!("#-0x{:x}", val.abs())
    } else {
        format!("#0x{:x}", val)
    }
}

impl fmt::Display for ShifterOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShifterOperand::Immediate(val) => write!(f, "{}", format_imm(*val)),
            ShifterOperand::Register(r) => write!(f, "{}", r),
            ShifterOperand::ImmediateShift(r, st, imm) => write!(f, "{}, {} #{}", r, st, imm),
            ShifterOperand::RegisterShift(r, st, rs) => write!(f, "{}, {} {}", r, st, rs),
            ShifterOperand::RRX(r) => write!(f, "{}, rrx", r),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddressingMode {
    OffsetImmediate(Register, i32),
    OffsetRegister(Register, Register, bool),
    OffsetScaled(Register, Register, ShiftType, u32, bool),
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressingMode::OffsetImmediate(rn, offset) => {
                if *offset == 0 {
                    write!(f, "[{}]", rn)
                } else {
                    write!(f, "[{}, {}]", rn, format_offset(*offset))
                }
            }
            AddressingMode::OffsetRegister(rn, rm, pos) => {
                let sign = if *pos { "" } else { "-" };
                write!(f, "[{}, {}{}]", rn, sign, rm)
            }
            AddressingMode::OffsetScaled(rn, rm, st, imm, pos) => {
                let sign = if *pos { "" } else { "-" };
                write!(f, "[{}, {}{}, {} #{}]", rn, sign, rm, st, imm)
            }
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    DataProcessing {
        cond: Condition,
        s: bool,
        opcode: DataOpcode,
        rd: Register,
        rn: Option<Register>,
        operand2: ShifterOperand,
    },
    LoadStore {
        cond: Condition,
        load: bool,
        byte: bool,
        rd: Register,
        addressing: AddressingMode,
    },
    LoadStoreExtra {
        cond: Condition,
        op: ExtraOp,
        rd: Register,
        addressing: AddressingMode,
    },
    Push {
        cond: Condition,
        reg_list: Vec<Register>,
    },
    Pop {
        cond: Condition,
        reg_list: Vec<Register>,
    },
    Multiply {
        cond: Condition,
        s: bool,
        rd: Register,
        rn: Register,
        rm: Register,
    },
    MultiplyAccumulate {
        cond: Condition,
        s: bool,
        rd: Register,
        rn: Register,
        rm: Register,
        ra: Register,
    },
    MultiplySubtract {
        cond: Condition,
        rd: Register,
        rn: Register,
        rm: Register,
        ra: Register,
    },
    Divide {
        cond: Condition,
        signed: bool,
        rd: Register,
        rn: Register,
        rm: Register,
    },
    Branch {
        cond: Condition,
        link: bool,
        target_addr: u32,
    },
    BranchExchange {
        cond: Condition,
        rm: Register,
    },
    Svc {
        cond: Condition,
        imm: u32,
    },
    Nop {
        cond: Condition,
    },
    Yield {
        cond: Condition,
    },
    Wfe {
        cond: Condition,
    },
    Wfi {
        cond: Condition,
    },
    Sev {
        cond: Condition,
    },
    Bkpt {
        imm: u16,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Nop { cond } => write!(f, "nop{}", cond),
            Instruction::Yield { cond } => write!(f, "yield{}", cond),
            Instruction::Wfe { cond } => write!(f, "wfe{}", cond),
            Instruction::Wfi { cond } => write!(f, "wfi{}", cond),
            Instruction::Sev { cond } => write!(f, "sev{}", cond),
            Instruction::Bkpt { imm } => write!(f, "bkpt {}", format_imm(*imm as u32)),
            Instruction::Svc { cond, imm } => write!(f, "svc{} {}", cond, format_imm(*imm)),
            Instruction::BranchExchange { cond, rm } => write!(f, "bx{} {}", cond, rm),
            Instruction::Branch {
                cond,
                link,
                target_addr,
            } => {
                let mnem = if *link { "bl" } else { "b" };
                write!(f, "{}{} 0x{:08x}", mnem, cond, target_addr)
            }
            Instruction::Multiply {
                cond,
                s,
                rd,
                rn,
                rm,
            } => {
                let s_str = if *s { "s" } else { "" };
                write!(f, "mul{}{} {}, {}, {}", cond, s_str, rd, rn, rm)
            }
            Instruction::MultiplyAccumulate {
                cond,
                s,
                rd,
                rn,
                rm,
                ra,
            } => {
                let s_str = if *s { "s" } else { "" };
                write!(f, "mla{}{} {}, {}, {}, {}", cond, s_str, rd, rn, rm, ra)
            }
            Instruction::MultiplySubtract {
                cond,
                rd,
                rn,
                rm,
                ra,
            } => {
                write!(f, "mls{} {}, {}, {}, {}", cond, rd, rn, rm, ra)
            }
            Instruction::Divide {
                cond,
                signed,
                rd,
                rn,
                rm,
            } => {
                let mnem = if *signed { "sdiv" } else { "udiv" };
                write!(f, "{}{} {}, {}, {}", mnem, cond, rd, rn, rm)
            }
            Instruction::Push { cond, reg_list } => {
                let regs: Vec<String> = reg_list.iter().map(|r| r.to_string()).collect();
                write!(f, "push{} {{{}}}", cond, regs.join(", "))
            }
            Instruction::Pop { cond, reg_list } => {
                let regs: Vec<String> = reg_list.iter().map(|r| r.to_string()).collect();
                write!(f, "pop{} {{{}}}", cond, regs.join(", "))
            }
            Instruction::LoadStore {
                cond,
                load,
                byte,
                rd,
                addressing,
            } => {
                let mnem = match (*load, *byte) {
                    (true, false) => "ldr",
                    (false, false) => "str",
                    (true, true) => "ldrb",
                    (false, true) => "strb",
                };
                write!(f, "{}{} {}, {}", mnem, cond, rd, addressing)
            }
            Instruction::LoadStoreExtra {
                cond,
                op,
                rd,
                addressing,
            } => {
                write!(f, "{}{} {}, {}", op, cond, rd, addressing)
            }
            Instruction::DataProcessing {
                cond,
                s,
                opcode,
                rd,
                rn,
                operand2,
            } => {
                let s_str = if *s
                    && !matches!(
                        opcode,
                        DataOpcode::Cmp | DataOpcode::Cmn | DataOpcode::Tst | DataOpcode::Teq
                    ) {
                    "s"
                } else {
                    ""
                };
                match opcode {
                    DataOpcode::Mov | DataOpcode::Mvn => {
                        write!(f, "{}{}{} {}, {}", opcode, cond, s_str, rd, operand2)
                    }
                    DataOpcode::Cmp | DataOpcode::Cmn | DataOpcode::Tst | DataOpcode::Teq => {
                        write!(f, "{}{} {}, {}", opcode, cond, rn.unwrap(), operand2)
                    }
                    _ => write!(
                        f,
                        "{}{}{} {}, {}, {}",
                        opcode,
                        cond,
                        s_str,
                        rd,
                        rn.unwrap(),
                        operand2
                    ),
                }
            }
        }
    }
}
