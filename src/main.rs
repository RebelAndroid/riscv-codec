use rand::{Rng, SeedableRng};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum IRegister {
    Zero = 0,
    ReturnAddress = 1,
    StackPointer = 2,
    GlobalPointer = 3,
    ThreadPointer = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    /// Also called s0
    FramePointer = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl Display for IRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                IRegister::Zero => "zero",
                IRegister::ReturnAddress => "ra",
                IRegister::StackPointer => "sp",
                IRegister::GlobalPointer => "gp",
                IRegister::ThreadPointer => "tp",
                IRegister::T0 => "t0",
                IRegister::T1 => "t1",
                IRegister::T2 => "t2",
                IRegister::FramePointer => "s0",
                IRegister::S1 => "s1",
                IRegister::A0 => "a0",
                IRegister::A1 => "a1",
                IRegister::A2 => "a2",
                IRegister::A3 => "a3",
                IRegister::A4 => "a4",
                IRegister::A5 => "a5",
                IRegister::A6 => "a6",
                IRegister::A7 => "a7",
                IRegister::S2 => "s2",
                IRegister::S3 => "s3",
                IRegister::S4 => "s4",
                IRegister::S5 => "s5",
                IRegister::S6 => "s6",
                IRegister::S7 => "s7",
                IRegister::S8 => "s8",
                IRegister::S9 => "s9",
                IRegister::S10 => "s10",
                IRegister::S11 => "s11",
                IRegister::T3 => "t3",
                IRegister::T4 => "t4",
                IRegister::T5 => "t5",
                IRegister::T6 => "t6",
            }
        )
    }
}

impl IRegister {
    fn from_int(int: u32) -> Self {
        return match int {
            0 => Self::Zero,
            1 => Self::ReturnAddress,
            2 => Self::StackPointer,
            3 => Self::GlobalPointer,
            4 => Self::ThreadPointer,
            5 => Self::T0,
            6 => Self::T1,
            7 => Self::T2,
            8 => Self::FramePointer,
            9 => Self::S1,
            10 => Self::A0,
            11 => Self::A1,
            12 => Self::A2,
            13 => Self::A3,
            14 => Self::A4,
            15 => Self::A5,
            16 => Self::A6,
            17 => Self::A7,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::S8,
            25 => Self::S9,
            26 => Self::S10,
            27 => Self::S11,
            28 => Self::T3,
            29 => Self::T4,
            30 => Self::T5,
            31 => Self::T6,
            x => panic!("converted invalid to register {}", x),
        };
    }

    fn from_string(str: &str) -> Result<Self, String> {
        return match str {
            "zero" => Ok(Self::Zero),
            "ra" => Ok(Self::ReturnAddress),
            "sp" => Ok(Self::StackPointer),
            "gp" => Ok(Self::GlobalPointer),
            "tp" => Ok(Self::ThreadPointer),
            "t0" => Ok(Self::T0),
            "t1" => Ok(Self::T1),
            "t2" => Ok(Self::T2),
            "s0" => Ok(Self::FramePointer),
            "s1" => Ok(Self::S1),
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "a7" => Ok(Self::A7),
            "s2" => Ok(Self::S2),
            "s3" => Ok(Self::S3),
            "s4" => Ok(Self::S4),
            "s5" => Ok(Self::S5),
            "s6" => Ok(Self::S6),
            "s7" => Ok(Self::S7),
            "s8" => Ok(Self::S8),
            "s9" => Ok(Self::S9),
            "s10" => Ok(Self::S10),
            "s11" => Ok(Self::S11),
            "t3" => Ok(Self::T3),
            "t4" => Ok(Self::T4),
            "t5" => Ok(Self::T5),
            "t6" => Ok(Self::T6),
            x => Err(format!("converted invalid str to register {}", x)),
        };
    }
}

#[derive(Debug)]
enum Instruction {
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
    BEQ(IRegister, IRegister, u16),
    BNE(IRegister, IRegister, u16),
    BLT(IRegister, IRegister, u16),
    BGE(IRegister, IRegister, u16),
    BLTU(IRegister, IRegister, u16),
    BGEU(IRegister, IRegister, u16),
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
    FENCE(IRegister, IRegister, u8, u8, u8),
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
            Instruction::FENCE(iregister, iregister1, _, _, _) => todo!(),
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

// Table 70, page 553 of the Unprivileged ISA Manual
#[derive(Debug)]
enum Opcode {
    Load = 0b00_000_11,
    Auipc = 0b00_101_11,
    Store = 0b01_000_11,
    Lui = 0b01_101_11,
    Op = 0b01_100_11,
    Op32 = 0b01_110_11,
    OpImm = 0b00_100_11,
    OpImm32 = 0b00_110_11,
    Jalr = 0b11_001_11,
    Jal = 0b11_011_11,
    Reserved = 0,
}

impl Opcode {
    fn from_int(int: u32) -> Self {
        if int > 0b11_111_11 {
            panic!("attempted to convert too large int to opcode")
        }
        return match int {
            0b00_000_11 => Self::Load,
            0b00_101_11 => Self::Auipc,
            0b01_000_11 => Self::Store,
            0b01_101_11 => Self::Lui,
            0b01_100_11 => Self::Op,
            0b01_110_11 => Self::Op32,
            0b00_100_11 => Self::OpImm,
            0b00_110_11 => Self::OpImm32,
            0b11_001_11 => Self::Jalr,
            0b11_011_11 => Self::Jal,
            _ => Self::Reserved,
        };
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

fn parse_line(line: &str) -> Result<Instruction, String> {
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
        _ => Err(format!("unknown mnemonic: {}", mnemonic)),
    };
}

fn assemble_instruction(instruction: u32) -> u32 {
    todo!();
}

fn decode_instruction(instruction: u32) -> Result<Instruction, String> {
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
        Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
    }
}

const INPUT: &str = include_str!("../input");

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut i = 0;
    loop {
        let x: u32 = rng.random();
        let d = decode_instruction(x);
        if let Ok(instr) = d {
            println!("{}: {:x} {}", i, x, instr);
        }

        i += 1;
    }
}
