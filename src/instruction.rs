use crate::opcode::Opcode;
use crate::register::IRegister;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    //
    // Instructions from RV32I
    //
    /// Load upper immediate
    LUI(IRegister, u32),
    /// Add upper immediate to PC
    AUIPC(IRegister, u32),
    /// Jump and Link
    JAL(IRegister, i32),
    /// Jump and Link Register
    JALR(IRegister, IRegister, i16),
    BEQ(IRegister, IRegister, i16),
    BNE(IRegister, IRegister, i16),
    BLT(IRegister, IRegister, i16),
    BGE(IRegister, IRegister, i16),
    BLTU(IRegister, IRegister, i16),
    BGEU(IRegister, IRegister, i16),
    /// Load Byte
    LB(IRegister, IRegister, i16),
    /// Load Halfword
    LH(IRegister, IRegister, i16),
    /// Load Word
    LW(IRegister, IRegister, i16),
    /// Load Byte Unsigned
    LBU(IRegister, IRegister, i16),
    /// Load Halfword Unsigned
    LHU(IRegister, IRegister, i16),
    /// Store Byte
    SB(IRegister, IRegister, i16),
    /// Store Halfword
    SH(IRegister, IRegister, i16),
    /// Store Word
    SW(IRegister, IRegister, i16),
    ADDI(IRegister, IRegister, i16),
    SLTI(IRegister, IRegister, i16),
    SLTIU(IRegister, IRegister, i16),
    XORI(IRegister, IRegister, i16),
    ORI(IRegister, IRegister, i16),
    ANDI(IRegister, IRegister, i16),
    /// Left Shift Immediate
    SLLI(IRegister, IRegister, u8),
    /// Logical Right Shift Immediate
    SRLI(IRegister, IRegister, u8),
    /// Arithmetic Right Shift Immediate
    SRAI(IRegister, IRegister, u8),
    ADD(IRegister, IRegister, IRegister),
    SUB(IRegister, IRegister, IRegister),
    /// Left Shift
    SLL(IRegister, IRegister, IRegister),
    /// Branch if Equal
    SLT(IRegister, IRegister, IRegister),
    SLTU(IRegister, IRegister, IRegister),
    XOR(IRegister, IRegister, IRegister),
    /// Logical Right Shift Immediate
    SRL(IRegister, IRegister, IRegister),
    /// Arithmetic Right Shift Immediate
    SRA(IRegister, IRegister, IRegister),
    OR(IRegister, IRegister, IRegister),
    AND(IRegister, IRegister, IRegister),
    FENCE(IRegister, IRegister, u8, u8),
    ECALL,
    EBREAK,
    //
    // Instructions Added In RV64I
    //
    /// Load Word Unsigned
    LWU(IRegister, IRegister, i16),
    /// Load Doubleword
    LD(IRegister, IRegister, i16),
    /// Store Doubleword
    SD(IRegister, IRegister, i16),
    /// Add Immediate (word)
    ADDIW(IRegister, IRegister, i16),
    /// Left Shift Immediate (word)
    SLLIW(IRegister, IRegister, u8),
    /// Logical Right Shift Immediate (word)
    SRLIW(IRegister, IRegister, u8),
    /// Arithmetic Right Shift Immediate (word)
    SRAIW(IRegister, IRegister, u8),
    /// Add (word)
    ADDW(IRegister, IRegister, IRegister),
    /// Subtract (word)
    SUBW(IRegister, IRegister, IRegister),
    /// Left Shift (word)
    SLLW(IRegister, IRegister, IRegister),
    /// Logical Right Shift (word)
    SRLW(IRegister, IRegister, IRegister),
    /// Arithmetic Right Shift (word)
    SRAW(IRegister, IRegister, IRegister),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::LUI(rd, imm) => write!(f, "lui {},{}", rd, imm),
            Instruction::AUIPC(rd, imm) => write!(f, "auipc {},{}", rd, imm),
            Instruction::JAL(rd, offset) => write!(f, "jal {},{}", rd, offset),
            Instruction::JALR(rd, rs1, imm) => write!(f, "jalr {},{}({})", rd, imm, rs1),
            Instruction::BEQ(rs1, rs2, imm) => write!(f, "beq {},{},{}", rs1, rs2, imm),
            Instruction::BNE(rs1, rs2, imm) => write!(f, "bne {},{},{}", rs1, rs2, imm),
            Instruction::BLT(rs1, rs2, imm) => write!(f, "blt {},{},{}", rs1, rs2, imm),
            Instruction::BGE(rs1, rs2, imm) => write!(f, "bge {rs1},{rs2},{imm}"),
            Instruction::BLTU(rs1, rs2, imm) => write!(f, "bltu {rs1},{rs2},{imm}"),
            Instruction::BGEU(rs1, rs2, imm) => write!(f, "bgeu {rs1},{rs2},{imm}"),
            Instruction::LB(rd, rs1, imm) => write!(f, "lb {rd}, {imm}({rs1})"),
            Instruction::LH(rd, rs1, imm) => write!(f, "lh {rd}, {imm}({rs1})"),
            Instruction::LW(rd, rs1, imm) => write!(f, "lw {rd}, {imm}({rs1})"),
            Instruction::LBU(rd, rs1, imm) => write!(f, "lbu {rd}, {imm}({rs1})"),
            Instruction::LHU(rd, rs1, imm) => write!(f, "lhu {rd}, {imm}({rs1})"),
            Instruction::SB(rs1, rs2, imm) => write!(f, "sb {rs2} {imm}({rs1})"),
            Instruction::SH(rs1, rs2, imm) => write!(f, "sh {rs2} {imm}({rs1})"),
            Instruction::SW(rs1, rs2, imm) => write!(f, "sw {rs2} {imm}({rs1})"),
            Instruction::ADDI(rd, rs1, imm) => write!(f, "addi {rd},{rs1},{imm}"),
            Instruction::SLTI(rd, rs1, imm) => write!(f, "slti {rd},{rs1},{imm}"),
            Instruction::SLTIU(rd, rs1, imm) => write!(f, "sltiu {rd},{rs1},{imm}"),
            Instruction::XORI(rd, rs1, imm) => write!(f, "xori {rd},{rs1},{imm}"),
            Instruction::ORI(rd, rs1, imm) => write!(f, "ori {rd},{rs1},{imm}"),
            Instruction::ANDI(rd, rs1, imm) => write!(f, "andi {rd},{rs1},{imm}"),
            Instruction::SLLI(rd, rs1, imm) => write!(f, "slli {},{},0x{:x}", rd, rs1, imm),
            Instruction::SRLI(rd, rs1, imm) => write!(f, "srli {rd},{rs1},{imm}"),
            Instruction::SRAI(rd, rs1, imm) => write!(f, "srai {rd},{rs1},{imm}"),
            Instruction::ADD(rd, rs1, rs2) => write!(f, "add {rd},{rs1},{rs2}"),
            Instruction::SUB(rd, rs1, rs2) => write!(f, "sub {rd},{rs1},{rs2}"),
            Instruction::SLL(rd, rs1, rs2) => write!(f, "sll {rd},{rs1},{rs2}"),
            Instruction::SLT(rd, rs1, rs2) => write!(f, "slt {rd},{rs1},{rs2}"),
            Instruction::SLTU(rd, rs1, rs2) => write!(f, "sltu {rd},{rs1},{rs2}"),
            Instruction::XOR(rd, rs1, rs2) => write!(f, "xor {rd},{rs1},{rs2}"),
            Instruction::SRL(rd, rs1, rs2) => write!(f, "srl {rd},{rs1},{rs2}"),
            Instruction::SRA(rd, rs1, rs2) => write!(f, "sra {rd},{rs1},{rs2}"),
            Instruction::OR(rd, rs1, rs2) => write!(f, "or {rd},{rs1},{rs2}"),
            Instruction::AND(rd, rs1, rs2) => write!(f, "and {rd},{rs1},{rs2}"),
            Instruction::FENCE(_, _, ops, fm) => write!(f, "{}", self.fmt_fence()),
            Instruction::ECALL => write!(f, "ecall"),
            Instruction::EBREAK => write!(f, "ebreak"),
            Instruction::LWU(rd, rs1, imm) => write!(f, "lwu {rd},{imm}({rs1})"),
            Instruction::LD(rd, rs1, imm) => write!(f, "ld {rd},{imm}({rs1})"),
            Instruction::SD(rs1, rs2, imm) => write!(f, "sd {rs2},{imm}({rs1})"),
            Instruction::ADDIW(rd, rs1, imm) => write!(f, "addiw {rd},{rs1},{imm}"),
            Instruction::SLLIW(rd, rs1, imm) => write!(f, "slliw {rd},{rs1},{imm}"),
            Instruction::SRLIW(rd, rs1, imm) => write!(f, "srliw {rd},{rs1},{imm}"),
            Instruction::SRAIW(rd, rs1, imm) => write!(f, "sraiw {rd},{rs1},{imm}"),
            Instruction::ADDW(rd, rs1, rs2) => write!(f, "addw {rd}{rs1}{rs2}"),
            Instruction::SUBW(rd, rs1, rs2) => write!(f, "subw {rd}{rs1}{rs2}"),
            Instruction::SLLW(rd, rs1, rs2) => write!(f, "sllw {rd}{rs1}{rs2}"),
            Instruction::SRLW(rd, rs1, rs2) => write!(f, "srlw {rd}{rs1}{rs2}"),
            Instruction::SRAW(rd, rs1, rs2) => write!(f, "sraw {rd}{rs1}{rs2}"),
        }
    }
}

impl Instruction {
    fn fmt_fence(&self) -> String {
        if let Instruction::FENCE(_, _, ops, fm) = *self {
            if fm == 0b1000 {
                return "fence.tso".to_owned();
            }
            let sw = if ops & 0b0000_0001 != 0 { "w" } else { "" };
            let sr = if ops & 0b0000_0010 != 0 { "r" } else { "" };
            let so = if ops & 0b0000_0100 != 0 { "o" } else { "" };
            let si = if ops & 0b0000_1000 != 0 { "i" } else { "" };
            let pw = if ops & 0b0001_0000 != 0 { "w" } else { "" };
            let pr = if ops & 0b0010_0000 != 0 { "r" } else { "" };
            let po = if ops & 0b0100_0000 != 0 { "o" } else { "" };
            let pi = if ops & 0b1000_0000 != 0 { "i" } else { "" };

            return format!("fence {pi}{po}{pr}{pw},{si}{so}{sr}{sw}");
        } else {
            unreachable!();
        }
    }
}

fn sign_extend_i_immediate(immediate: u16) -> i16 {
    if immediate & 0b1000_0000_0000 == 0 {
        return immediate as i16;
    } else {
        return -((0b1_0000_0000_0000 - immediate) as i16);
    }
}

fn sign_extend_s_immediate(immediate: u16) -> i16 {
    if immediate & 0b1000_0000_0000 == 0 {
        return immediate as i16;
    } else {
        return -((0b1_0000_0000_0000 - immediate) as i16);
    }
}

fn j_immediate_from_u_immediate(u: u32) -> i32 {
    let a = u >> 12 & 0b1111_1111;
    let b = u >> 20 & 0b1;
    let c = u >> 21 & 0b11_1111_1111;
    let d = u >> 31;

    let i = (c << 1) | (b << 11) | (a << 12) | (d << 20);
    ((i << 12) as i32) >> 12
}

fn parse_int(str: &str) -> Result<i64, String> {
    match str.parse::<i64>() {
        Ok(e) => Ok(e),
        Err(_) => Err("unable to parse int".to_owned()),
    }
}

fn parse_address_expression(str: &str) -> Result<(IRegister, i16), String> {
    let (offset, register): (&str, &str) = if let Some(x) = str.split_once("(") {
        x
    } else {
        panic!("no (");
    };
    match register.strip_suffix(")") {
        Some(y) => {
            let r = IRegister::from_string(y)?;
            let i = parse_int(offset)?;
            Ok((r, i.try_into().unwrap()))
        }
        None => Err("Address expression should end in a )".to_owned()),
    }
}

/// Constructs an `Instruction` from a line of assembly.
pub fn assemble_line(line: &str) -> Result<Instruction, String> {
    let (mnemonic, operands): (&str, &str) = if let Some(x) = line.split_once(" ") {
        x
    } else {
        panic!("no space");
    };

    let operands: Vec<&str> = operands.split(',').collect();

    return match mnemonic {
        "addi" => {
            if operands.len() != 3 {
                Err("addi instruction requires 3 operands".to_owned())
            } else {
                Ok(Instruction::ADDI(
                    IRegister::from_string(operands[0])?,
                    IRegister::from_string(operands[1])?,
                    parse_int(operands[2])?.try_into().unwrap(),
                ))
            }
        }
        "sd" => {
            if operands.len() != 2 {
                Err("sd instruction requires 2 operands".to_owned())
            } else {
                let (base, offset) = parse_address_expression(operands[1])?;
                Ok(Instruction::SD(
                    base,
                    IRegister::from_string(operands[0])?,
                    offset,
                ))
            }
        }
        "jalr" => {
            if operands.len() != 2 {
                Err("jalr instruction requires 2 operands".to_owned())
            } else {
                let (base, offset) = parse_address_expression(operands[1])?;
                Ok(Instruction::JALR(
                    IRegister::from_string(operands[0])?,
                    base,
                    offset,
                ))
            }
        }
        "lui" => {
            if operands.len() != 2 {
                Err("lui instruction requires 2 operands".to_owned())
            } else {
                let int: u32 = parse_int(operands[1])? as u32;
                Ok(Instruction::LUI(IRegister::from_string(operands[0])?, int))
            }
        }
        "slti" => {
            if operands.len() != 3 {
                Err("slti instruction requires 3 operands".to_owned())
            } else {
                Ok(Instruction::SLTI(
                    IRegister::from_string(operands[0])?,
                    IRegister::from_string(operands[1])?,
                    parse_int(operands[2])? as i16,
                ))
            }
        }
        "jal" => {
            if operands.len() != 2 {
                Err("jal instruction requires 2 operands".to_owned())
            } else {
                Ok(Instruction::JAL(
                    IRegister::from_string(operands[0])?,
                    parse_int(operands[1])? as i32,
                ))
            }
        }
        "fence" => {
            if operands.len() != 2 {
                Err("fence instruction requires 2 operands".to_owned())
            } else {
                let ops = parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                Ok(Instruction::FENCE(
                    // rd and rs1 are currently unused
                    IRegister::Zero,
                    IRegister::Zero,
                    ops,
                    0, //fm field, always zero for a non-tso fence
                ))
            }
        }
        _ => Err(format!("unknown mnemonic: {}", mnemonic)),
    };
}

/// Converts a string representing operations into a
pub fn parse_fence_set(s: &str) -> u8 {
    let mut x = 0;
    if (s.contains("w")) {
        x |= 0b1;
    }
    if (s.contains("r")) {
        x |= 0b10;
    }
    if (s.contains("o")) {
        x |= 0b100;
    }
    if (s.contains("i")) {
        x |= 0b1000;
    }
    return x;
}

/// Disassembles an instruction.
pub fn disassemble_instruction(instruction: &Instruction) -> String {
    return format!("{}", instruction);
}

/// Constructs an `Instruction` from it's machine code representation.
pub fn decode_instruction(instruction: u32) -> Result<Instruction, String> {
    let opcode = Opcode::from_int(instruction & 0b111_1111);

    let func3 = (instruction >> 12) & 0b111;
    let func7 = (instruction >> 25) & 0b111_1111;

    let rd = IRegister::from_int((instruction >> 7) & 0b1_1111);
    let rs1 = IRegister::from_int((instruction >> 15) & 0b1_1111);
    let rs2 = IRegister::from_int((instruction >> 20) & 0b1_1111);
    // println!("0b{:b}", instruction);

    let i_immediate: u16 = ((instruction >> 20) & 0b1111_1111_1111).try_into().unwrap();

    let s_immediate: i16 = sign_extend_s_immediate(
        <u32 as TryInto<u16>>::try_into(
            (((instruction >> 25) & 0b111_1111) << 5) | ((instruction >> 7) & 0b1_1111),
        )
        .unwrap(),
    );

    let u_immediate = instruction & (!0b1111_1111_1111);

    let b = (((instruction >> 7) & 0b1) << 11)
        | (((instruction >> 8) & 0b1111) << 1)
        | (((instruction >> 25) & 0b11_1111) << 5)
        | ((instruction >> 31) << 12);

    let b_immediate = ((b << 20 as i32) >> 20) as i16;

    let shamt: u8 = ((instruction >> 20) & 0b11_1111).try_into().unwrap();

    match opcode {
        Opcode::Load => match func3 {
            0b000 => Ok(Instruction::LB(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b001 => Ok(Instruction::LH(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b010 => Ok(Instruction::LW(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b011 => Ok(Instruction::LD(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b100 => Ok(Instruction::LBU(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b101 => Ok(Instruction::LHU(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b110 => Ok(Instruction::LWU(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b111 => Err("Invalid load func3".to_owned()),
            _ => unreachable!(),
        },
        Opcode::Auipc => Ok(Instruction::AUIPC(rd, u_immediate)),
        Opcode::Store => match func3 {
            0b000 => Ok(Instruction::SB(rs1, rs2, s_immediate)),
            0b001 => Ok(Instruction::SH(rs1, rs2, s_immediate)),
            0b010 => Ok(Instruction::SW(rs1, rs2, s_immediate)),
            0b011 => Ok(Instruction::SD(rs1, rs2, s_immediate)),
            x => Err(format!("invalid store func3: {}", x)),
        },
        Opcode::Lui => Ok(Instruction::LUI(rd, u_immediate)),
        Opcode::Op => match func3 {
            0b000 => match func7 {
                0b000_0000 => Ok(Instruction::ADD(rd, rs1, rs2)),
                0b010_0000 => Ok(Instruction::SUB(rd, rs1, rs2)),
                x => Err(format!("unknown Op(000) func 7: {}", x)),
            },
            0b001 => Ok(Instruction::SLL(rd, rs1, rs2)),
            0b010 => Ok(Instruction::SLT(rd, rs1, rs2)),
            0b011 => Ok(Instruction::SLTU(rd, rs1, rs2)),
            0b100 => Ok(Instruction::XOR(rd, rs1, rs2)),
            0b101 => match func7 {
                0b000_0000 => Ok(Instruction::SRL(rd, rs1, rs2)),
                0b010_0000 => Ok(Instruction::SRA(rd, rs1, rs2)),
                x => Err(format!("unknown Op(000) func 7: {}", x)),
            },
            0b110 => Ok(Instruction::OR(rd, rs1, rs2)),
            0b111 => Ok(Instruction::AND(rd, rs1, rs2)),
            x => unreachable!(),
        },
        Opcode::Op32 => match func3 {
            0b000 => match func7 {
                0b000_0000 => Ok(Instruction::ADDW(rd, rs1, rs2)),
                0b010_0000 => Ok(Instruction::SUBW(rd, rs1, rs2)),
                x => Err(format!("unknown Op32(000) func 7: {}", x)),
            },
            0b001 => Ok(Instruction::SLLW(rd, rs1, rs2)),
            0b101 => match func7 {
                0b000_0000 => Ok(Instruction::SRLW(rd, rs1, rs2)),
                0b010_0000 => Ok(Instruction::SRAW(rd, rs1, rs2)),
                x => Err(format!("unknown Op32(101) func 7: {}", x)),
            },
            x => Err(format!("unknown Op32 func3: {}", x)),
        },
        Opcode::OpImm => match func3 {
            0b000 => Ok(Instruction::ADDI(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b001 => Ok(Instruction::SLLI(rd, rs1, shamt)),
            0b010 => Ok(Instruction::SLTI(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b011 => Ok(Instruction::SLTIU(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b100 => Ok(Instruction::XORI(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            // the bottom bit of func7 is actually the top bit of shamt, so we need to ignore it
            0b101 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SRLI(rd, rs1, shamt)),
                0b010000_1 => Ok(Instruction::SRAI(rd, rs1, shamt)),
                x => Err(format!("unknown OpImm(101) func 7: {x}").to_owned()),
            },
            0b110 => Ok(Instruction::ORI(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b111 => Ok(Instruction::ANDI(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            x => Err(format!("unknown OpImm func3: {}", x)),
        },
        Opcode::OpImm32 => match func3 {
            0b000 => Ok(Instruction::ADDIW(
                rd,
                rs1,
                sign_extend_i_immediate(i_immediate),
            )),
            0b001 => {
                if shamt & 0b100000 != 0 {
                    Err("SLLIW with shamt[5] set".to_owned())
                } else {
                    Ok(Instruction::SLLIW(rd, rs1, shamt))
                }
            }
            0b101 => match func7 {
                0b000_0000 => {
                    if shamt & 0b100000 != 0 {
                        Err("SRLIW with shamt[5] set".to_owned())
                    } else {
                        Ok(Instruction::SRLIW(rd, rs1, shamt))
                    }
                }
                0b010_0000 => {
                    if shamt & 0b100000 != 0 {
                        Err("SRAIW with shamt[5] set".to_owned())
                    } else {
                        Ok(Instruction::SRAIW(rd, rs1, shamt))
                    }
                }
                x => Err(format!("unknown OpImm32(101) func7: {}", x).to_owned()),
            },
            x => Err(format!("unkown OpImm32 func3: {}", x).to_owned()),
        },
        Opcode::Jalr => Ok(Instruction::JALR(
            rd,
            rs1,
            sign_extend_i_immediate(i_immediate),
        )),
        Opcode::Jal => Ok(Instruction::JAL(
            rd,
            j_immediate_from_u_immediate(u_immediate),
        )),
        Opcode::Branch => match func3 {
            0b000 => Ok(Instruction::BEQ(rs1, rs2, b_immediate)),
            0b001 => Ok(Instruction::BNE(rs1, rs2, b_immediate)),
            0b100 => Ok(Instruction::BLT(rs1, rs2, b_immediate)),
            0b101 => Ok(Instruction::BGE(rs1, rs2, b_immediate)),
            0b110 => Ok(Instruction::BLTU(rs1, rs2, b_immediate)),
            0b111 => Ok(Instruction::BGEU(rs1, rs2, b_immediate)),
            x => Err(format!("invalid branch func3: {x}").to_owned()),
        },
        Opcode::MiscMem => match func3 {
            0b000 => {
                if rd != IRegister::Zero || rs1 != IRegister::Zero {
                    // technicially, we are supposed to ignore these fields
                    Err(format!("reserved register fields not set to zero").to_owned())
                } else {
                    Ok(Instruction::FENCE(
                        rd,
                        rs1,
                        ((instruction >> 20) & 0xFF) as u8,
                        ((instruction >> 28) & 0b1111) as u8,
                    ))
                }
            }
            x => Err(format!("unknown fence func3: {x}")),
        },
        Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
    }
}
