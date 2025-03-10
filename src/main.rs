#![allow(dead_code)]

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
    JAL(IRegister, u32),
    /// Jump and Link Register
    JALR(IRegister, IRegister, u16),
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
    SLTI(IRegister, IRegister, u16),
    XORI(IRegister, IRegister, u16),
    ORI(IRegister, IRegister, u16),
    ANDI(IRegister, IRegister, u16),
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
    ADDIW(IRegister, IRegister, u16),
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
            Instruction::LUI(iregister, _) => todo!(),
            Instruction::AUIPC(iregister, _) => todo!(),
            Instruction::JAL(iregister, _) => todo!(),
            Instruction::JALR(rd, rs1, imm) => write!(f, "jal {}, {}({})", rd, imm, rs1),
            Instruction::BEQ(iregister, iregister1, _) => todo!(),
            Instruction::BNE(iregister, iregister1, _) => todo!(),
            Instruction::BLT(iregister, iregister1, _) => todo!(),
            Instruction::BGE(iregister, iregister1, _) => todo!(),
            Instruction::BLTU(iregister, iregister1, _) => todo!(),
            Instruction::BGEU(iregister, iregister1, _) => todo!(),
            Instruction::LB(iregister, iregister1, _) => todo!(),
            Instruction::LH(iregister, iregister1, _) => todo!(),
            Instruction::LW(iregister, iregister1, _) => todo!(),
            Instruction::LBU(iregister, iregister1, _) => todo!(),
            Instruction::LHU(iregister, iregister1, _) => todo!(),
            Instruction::SB(iregister, iregister2, _) => todo!(),
            Instruction::SH(iregister, iregister2, _) => todo!(),
            Instruction::SW(iregister, iregister2, _) => todo!(),
            Instruction::ADDI(rd, rs1, imm) => write!(f, "addi {},{},{}", rd, rs1, imm),
            Instruction::SLTI(iregister, iregister1, _) => todo!(),
            Instruction::XORI(iregister, iregister1, _) => todo!(),
            Instruction::ORI(iregister, iregister1, _) => todo!(),
            Instruction::ANDI(iregister, iregister1, _) => todo!(),
            Instruction::SLLI(rd, rs1, imm) => write!(f, "slli {},{},0x{:x}", rd, rs1, imm),
            Instruction::SRLI(iregister, iregister1, _) => todo!(),
            Instruction::SRAI(iregister, iregister1, _) => todo!(),
            Instruction::ADD(iregister, iregister1, iregister2) => todo!(),
            Instruction::SUB(iregister, iregister1, iregister2) => todo!(),
            Instruction::SLL(iregister, iregister1, iregister2) => todo!(),
            Instruction::SLT(iregister, iregister1, iregister2) => todo!(),
            Instruction::SLTU(iregister, iregister1, iregister2) => todo!(),
            Instruction::XOR(iregister, iregister1, iregister2) => todo!(),
            Instruction::SRL(iregister, iregister1, iregister2) => todo!(),
            Instruction::SRA(iregister, iregister1, iregister2) => todo!(),
            Instruction::OR(iregister, iregister1, iregister2) => todo!(),
            Instruction::AND(iregister, iregister1, iregister2) => todo!(),
            Instruction::FENCE(iregister, iregister1, _, _, _) => todo!(),
            Instruction::ECALL => todo!(),
            Instruction::EBREAK => todo!(),
            Instruction::LWU(iregister, iregister1, _) => todo!(),
            Instruction::LD(rd, rs1, imm) => write!(f, "ld {},{}({})", rd, imm, rs1),
            Instruction::SD(rs1, rs2, imm) => write!(f, "sd {},{}({})", rs2, imm, rs1),
            Instruction::ADDIW(iregister, iregister1, _) => todo!(),
            Instruction::SLLIW(iregister, iregister1, _) => todo!(),
            Instruction::SRLIW(iregister, iregister1, _) => todo!(),
            Instruction::SRAIW(iregister, iregister1, _) => todo!(),
            Instruction::ADDW(iregister, iregister1, iregister2) => todo!(),
            Instruction::SUBW(iregister, iregister1, iregister2) => todo!(),
            Instruction::SLLW(iregister, iregister1, iregister2) => todo!(),
            Instruction::SRLW(iregister, iregister1, iregister2) => todo!(),
            Instruction::SRAW(iregister, iregister1, iregister2) => todo!(),
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
}

impl Opcode {
    fn from_int(int: u32) -> Self {
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
            x => panic!("attempted to decode invalid opcode: {}", x),
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

fn decode_instruction(instruction: u32) -> Instruction {
    let opcode = Opcode::from_int(instruction & 0b111_1111);

    let func3 = (instruction >> 12) & 0b111;

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

    let shamt: u8 = ((instruction >> 20) & 0b1_1111).try_into().unwrap();

    match opcode {
        Opcode::Load => match func3 {
            0b011 => Instruction::LD(rd, rs1, sign_extend_i_immediate(i_immediate)),
            x => panic!("unexpected load func3: {}", x),
        },
        Opcode::Auipc => todo!(),
        Opcode::Store => match func3 {
            0b011 => Instruction::SD(rs1, rs2, s_immediate),
            0b010 => Instruction::SW(rs1, rs2, s_immediate),
            x => panic!("unexpected store func3: {}", x),
        },
        Opcode::Lui => todo!(),
        Opcode::Op => todo!(),
        Opcode::Op32 => todo!(),
        Opcode::OpImm => match func3 {
            0b000 => Instruction::ADDI(rd, rs1, sign_extend_i_immediate(i_immediate)),
            0b001 => Instruction::SLLI(rd, rs1, shamt),
            _ => unreachable!(),
        },
        Opcode::OpImm32 => todo!(),
        Opcode::Jalr => Instruction::JALR(rd, rs1, i_immediate),
        Opcode::Jal => todo!(),
    }
}

const input: &str = include_str!("../input");

fn main() {
    for line in input.lines() {
        println!(
            "{} {}",
            line,
            decode_instruction(u32::from_str_radix(line, 16).unwrap())
        );
    }
}
