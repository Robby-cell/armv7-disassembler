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
    SP,
    LR,
    PC,
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
            13 => Ok(Register::SP),
            14 => Ok(Register::LR),
            15 => Ok(Register::PC),
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
            Register::SP => "sp",
            Register::LR => "lr",
            Register::PC => "pc",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
}

impl Condition {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0x0 => Ok(Condition::EQ),
            0x1 => Ok(Condition::NE),
            0x2 => Ok(Condition::CS),
            0x3 => Ok(Condition::CC),
            0x4 => Ok(Condition::MI),
            0x5 => Ok(Condition::PL),
            0x6 => Ok(Condition::VS),
            0x7 => Ok(Condition::VC),
            0x8 => Ok(Condition::HI),
            0x9 => Ok(Condition::LS),
            0xA => Ok(Condition::GE),
            0xB => Ok(Condition::LT),
            0xC => Ok(Condition::GT),
            0xD => Ok(Condition::LE),
            0xE => Ok(Condition::AL),
            _ => Err(DecodeError::InvalidConditionCode { code }),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Condition::AL => "",
            Condition::EQ => "eq",
            Condition::NE => "ne",
            Condition::CS => "cs",
            Condition::CC => "cc",
            Condition::MI => "mi",
            Condition::PL => "pl",
            Condition::VS => "vs",
            Condition::VC => "vc",
            Condition::HI => "hi",
            Condition::LS => "ls",
            Condition::GE => "ge",
            Condition::LT => "lt",
            Condition::GT => "gt",
            Condition::LE => "le",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
}

impl ShiftType {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0 => Ok(ShiftType::LSL),
            1 => Ok(ShiftType::LSR),
            2 => Ok(ShiftType::ASR),
            3 => Ok(ShiftType::ROR),
            _ => Err(DecodeError::InvalidShiftType { code }),
        }
    }
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ShiftType::LSL => "lsl",
            ShiftType::LSR => "lsr",
            ShiftType::ASR => "asr",
            ShiftType::ROR => "ror",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataOpcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

impl DataOpcode {
    pub fn from_code(code: u32) -> Result<Self, DecodeError> {
        match code {
            0x0 => Ok(DataOpcode::AND),
            0x1 => Ok(DataOpcode::EOR),
            0x2 => Ok(DataOpcode::SUB),
            0x3 => Ok(DataOpcode::RSB),
            0x4 => Ok(DataOpcode::ADD),
            0x5 => Ok(DataOpcode::ADC),
            0x6 => Ok(DataOpcode::SBC),
            0x7 => Ok(DataOpcode::RSC),
            0x8 => Ok(DataOpcode::TST),
            0x9 => Ok(DataOpcode::TEQ),
            0xA => Ok(DataOpcode::CMP),
            0xB => Ok(DataOpcode::CMN),
            0xC => Ok(DataOpcode::ORR),
            0xD => Ok(DataOpcode::MOV),
            0xE => Ok(DataOpcode::BIC),
            0xF => Ok(DataOpcode::MVN),
            _ => Err(DecodeError::InvalidOpcode { code }),
        }
    }
}

impl fmt::Display for DataOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataOpcode::AND => "and",
            DataOpcode::EOR => "eor",
            DataOpcode::SUB => "sub",
            DataOpcode::RSB => "rsb",
            DataOpcode::ADD => "add",
            DataOpcode::ADC => "adc",
            DataOpcode::SBC => "sbc",
            DataOpcode::RSC => "rsc",
            DataOpcode::TST => "tst",
            DataOpcode::TEQ => "teq",
            DataOpcode::CMP => "cmp",
            DataOpcode::CMN => "cmn",
            DataOpcode::ORR => "orr",
            DataOpcode::MOV => "mov",
            DataOpcode::BIC => "bic",
            DataOpcode::MVN => "mvn",
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
    OffsetRegister(Register, Register),
    OffsetScaled(Register, Register, ShiftType, u32),
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
            AddressingMode::OffsetRegister(rn, rm) => write!(f, "[{}, {}]", rn, rm),
            AddressingMode::OffsetScaled(rn, rm, st, imm) => {
                write!(f, "[{}, {}, {} #{}]", rn, rm, st, imm)
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
                        DataOpcode::CMP | DataOpcode::CMN | DataOpcode::TST | DataOpcode::TEQ
                    ) {
                    "s"
                } else {
                    ""
                };
                match opcode {
                    DataOpcode::MOV | DataOpcode::MVN => {
                        write!(f, "{}{}{} {}, {}", opcode, cond, s_str, rd, operand2)
                    }
                    DataOpcode::CMP | DataOpcode::CMN | DataOpcode::TST | DataOpcode::TEQ => {
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
