use crate::immediates::{SImmediate, Shamt, ShamtW};
use crate::register::IRegister;
use crate::{immediates::IImmediate, opcode::Opcode};
use std::fmt::{Display, Formatter};

use proc_macros::{
    b_assemble, i_assemble, l_assemble, r_assemble, s_assemble, sh_assemble, shw_assemble,
};

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
    JALR(IRegister, IRegister, IImmediate),
    BEQ(IRegister, IRegister, i16),
    BNE(IRegister, IRegister, i16),
    BLT(IRegister, IRegister, i16),
    BGE(IRegister, IRegister, i16),
    BLTU(IRegister, IRegister, i16),
    BGEU(IRegister, IRegister, i16),
    /// Load Byte
    LB(IRegister, IRegister, IImmediate),
    /// Load Halfword
    LH(IRegister, IRegister, IImmediate),
    /// Load Word
    LW(IRegister, IRegister, IImmediate),
    /// Load Byte Unsigned
    LBU(IRegister, IRegister, IImmediate),
    /// Load Halfword Unsigned
    LHU(IRegister, IRegister, IImmediate),
    /// Store Byte
    SB(IRegister, IRegister, SImmediate),
    /// Store Halfword
    SH(IRegister, IRegister, SImmediate),
    /// Store Word
    SW(IRegister, IRegister, SImmediate),
    ADDI(IRegister, IRegister, IImmediate),
    SLTI(IRegister, IRegister, IImmediate),
    SLTIU(IRegister, IRegister, IImmediate),
    XORI(IRegister, IRegister, IImmediate),
    ORI(IRegister, IRegister, IImmediate),
    ANDI(IRegister, IRegister, IImmediate),
    /// Left Shift Immediate
    SLLI(IRegister, IRegister, Shamt),
    /// Logical Right Shift Immediate
    SRLI(IRegister, IRegister, Shamt),
    /// Arithmetic Right Shift Immediate
    SRAI(IRegister, IRegister, Shamt),
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
    LWU(IRegister, IRegister, IImmediate),
    /// Load Doubleword
    LD(IRegister, IRegister, IImmediate),
    /// Store Doubleword
    SD(IRegister, IRegister, SImmediate),
    /// Add Immediate (word)
    ADDIW(IRegister, IRegister, IImmediate),
    /// Left Shift Immediate (word)
    SLLIW(IRegister, IRegister, ShamtW),
    /// Logical Right Shift Immediate (word)
    SRLIW(IRegister, IRegister, ShamtW),
    /// Arithmetic Right Shift Immediate (word)
    SRAIW(IRegister, IRegister, ShamtW),
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
    //
    // Instructions In M Extension
    //
    /// Multiply
    MUL(IRegister, IRegister, IRegister),
    /// Multiply (High bits)
    MULH(IRegister, IRegister, IRegister),
    /// Multiply Signed-Unsigned (High bits)
    MULHSU(IRegister, IRegister, IRegister),
    /// Multiply Unsigned (High)
    MULHU(IRegister, IRegister, IRegister),
    /// Divide
    DIV(IRegister, IRegister, IRegister),
    /// Divide (Unsigned)
    DIVU(IRegister, IRegister, IRegister),
    /// Remainder
    REM(IRegister, IRegister, IRegister),
    /// Remainder (Unsigned)
    REMU(IRegister, IRegister, IRegister),
    /// Multiply Word
    MULW(IRegister, IRegister, IRegister),
    /// Divide Word
    DIVW(IRegister, IRegister, IRegister),
    /// Divide Unsigned Word
    DIVUW(IRegister, IRegister, IRegister),
    /// Remainder Word
    REMW(IRegister, IRegister, IRegister),
    /// Remainder Unsigned Word
    REMUW(IRegister, IRegister, IRegister),
    //
    // Instructions In A Extension
    //
    /// Load Reserved Word
    // rd, rs1, ac, rl
    LRW(IRegister, IRegister, bool, bool),
    SCW(IRegister, IRegister, IRegister, bool, bool),
    AMOSWAPW(IRegister, IRegister, IRegister, bool, bool),
    AMOADDW(IRegister, IRegister, IRegister, bool, bool),
    AMOXORW(IRegister, IRegister, IRegister, bool, bool),
    AMOANDW(IRegister, IRegister, IRegister, bool, bool),
    AMOORW(IRegister, IRegister, IRegister, bool, bool),
    AMOMINW(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXW(IRegister, IRegister, IRegister, bool, bool),
    AMOMINUW(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXUW(IRegister, IRegister, IRegister, bool, bool),
    //
    LRD(IRegister, IRegister, bool, bool),
    SCD(IRegister, IRegister, IRegister, bool, bool),
    AMOSWAPD(IRegister, IRegister, IRegister, bool, bool),
    AMOADDD(IRegister, IRegister, IRegister, bool, bool),
    AMOXORD(IRegister, IRegister, IRegister, bool, bool),
    AMOANDD(IRegister, IRegister, IRegister, bool, bool),
    AMOORD(IRegister, IRegister, IRegister, bool, bool),
    AMOMIND(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXD(IRegister, IRegister, IRegister, bool, bool),
    AMOMINUD(IRegister, IRegister, IRegister, bool, bool),
    AMOMAXUD(IRegister, IRegister, IRegister, bool, bool),
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
            Instruction::LB(rd, rs1, imm) => write!(f, "lb {rd},{imm}({rs1})"),
            Instruction::LH(rd, rs1, imm) => write!(f, "lh {rd},{imm}({rs1})"),
            Instruction::LW(rd, rs1, imm) => write!(f, "lw {rd},{imm}({rs1})"),
            Instruction::LBU(rd, rs1, imm) => write!(f, "lbu {rd},{imm}({rs1})"),
            Instruction::LHU(rd, rs1, imm) => write!(f, "lhu {rd},{imm}({rs1})"),
            Instruction::SB(rs1, rs2, imm) => write!(f, "sb {rs2},{imm}({rs1})"),
            Instruction::SH(rs1, rs2, imm) => write!(f, "sh {rs2},{imm}({rs1})"),
            Instruction::SW(rs1, rs2, imm) => write!(f, "sw {rs2},{imm}({rs1})"),
            Instruction::ADDI(rd, rs1, imm) => write!(f, "addi {rd},{rs1},{imm}"),
            Instruction::SLTI(rd, rs1, imm) => write!(f, "slti {rd},{rs1},{imm}"),
            Instruction::SLTIU(rd, rs1, imm) => write!(f, "sltiu {rd},{rs1},{imm}"),
            Instruction::XORI(rd, rs1, imm) => write!(f, "xori {rd},{rs1},{imm}"),
            Instruction::ORI(rd, rs1, imm) => write!(f, "ori {rd},{rs1},{imm}"),
            Instruction::ANDI(rd, rs1, imm) => write!(f, "andi {rd},{rs1},{imm}"),
            Instruction::SLLI(rd, rs1, imm) => write!(f, "slli {rd},{rs1},{imm}"),
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
            Instruction::FENCE(_, _, _, _) => write!(f, "{}", self.fmt_fence()),
            Instruction::ECALL => write!(f, "ecall"),
            Instruction::EBREAK => write!(f, "ebreak"),
            Instruction::LWU(rd, rs1, imm) => write!(f, "lwu {rd},{imm}({rs1})"),
            Instruction::LD(rd, rs1, imm) => write!(f, "ld {rd},{imm}({rs1})"),
            Instruction::SD(rs1, rs2, imm) => write!(f, "sd {rs2},{imm}({rs1})"),
            Instruction::ADDIW(rd, rs1, imm) => write!(f, "addiw {rd},{rs1},{imm}"),
            Instruction::SLLIW(rd, rs1, imm) => write!(f, "slliw {rd},{rs1},{imm}"),
            Instruction::SRLIW(rd, rs1, imm) => write!(f, "srliw {rd},{rs1},{imm}"),
            Instruction::SRAIW(rd, rs1, imm) => write!(f, "sraiw {rd},{rs1},{imm}"),
            Instruction::ADDW(rd, rs1, rs2) => write!(f, "addw {rd},{rs1},{rs2}"),
            Instruction::SUBW(rd, rs1, rs2) => write!(f, "subw {rd},{rs1},{rs2}"),
            Instruction::SLLW(rd, rs1, rs2) => write!(f, "sllw {rd},{rs1},{rs2}"),
            Instruction::SRLW(rd, rs1, rs2) => write!(f, "srlw {rd},{rs1},{rs2}"),
            Instruction::SRAW(rd, rs1, rs2) => write!(f, "sraw {rd},{rs1},{rs2}"),
            Instruction::MUL(rd, rs1, rs2) => write!(f, "mul {rd},{rs1},{rs2}"),
            Instruction::MULH(rd, rs1, rs2) => write!(f, "mulh {rd},{rs1},{rs2}"),
            Instruction::MULHSU(rd, rs1, rs2) => write!(f, "mulhsu {rd},{rs1},{rs2}"),
            Instruction::MULHU(rd, rs1, rs2) => write!(f, "mulhu {rd},{rs1},{rs2}"),
            Instruction::DIV(rd, rs1, rs2) => write!(f, "div {rd},{rs1},{rs2}"),
            Instruction::DIVU(rd, rs1, rs2) => write!(f, "divu {rd},{rs1},{rs2}"),
            Instruction::REM(rd, rs1, rs2) => write!(f, "rem {rd},{rs1},{rs2}"),
            Instruction::REMU(rd, rs1, rs2) => write!(f, "remu {rd},{rs1},{rs2}"),
            Instruction::MULW(rd, rs1, rs2) => write!(f, "mulw {rd},{rs1},{rs2}"),
            Instruction::DIVW(rd, rs1, rs2) => write!(f, "divw {rd},{rs1},{rs2}"),
            Instruction::DIVUW(rd, rs1, rs2) => write!(f, "divuw {rd},{rs1},{rs2}"),
            Instruction::REMW(rd, rs1, rs2) => write!(f, "remw {rd},{rs1},{rs2}"),
            Instruction::REMUW(rd, rs1, rs2) => write!(f, "remuw {rd},{rs1},{rs2}"),
            // TODO: fix this aqrl syntax
            Instruction::LRW(rd, rs1, aq, rl) => write!(f, "lr.w.{aq}{rl} {rd},{rs1}"),
            Instruction::SCW(rd, rs1, rs2, aq, rl) => write!(f, "sc.w.{aq}{rl} {rd},{rs1},{rs2}"),
            Instruction::AMOSWAPW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoswap.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOADDW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoadd.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOXORW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoxor.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOANDW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoand.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOORW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoor.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMINW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomin.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMAXW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomax.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMINUW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amominu.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMAXUW(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomaxu.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::LRD(rd, rs1, aq, rl) => write!(f, "lr.d.{aq}{rl} {rd},{rs1}"),
            Instruction::SCD(rd, rs1, rs2, aq, rl) => write!(f, "sc.d.{aq}{rl} {rd},{rs1},{rs2}"),
            Instruction::AMOSWAPD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoswap.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOADDD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoswap.w.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOXORD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoxor.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOANDD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoand.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOORD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amoor.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMIND(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomin.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMAXD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomax.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMINUD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amominu.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
            Instruction::AMOMAXUD(rd, rs1, rs2, aq, rl) => {
                write!(f, "amomaxu.d.{aq}{rl} {rd},{rs1},{rs2}")
            }
        }
    }
}

impl Instruction {
    fn fmt_fence(&self) -> String {
        if let Instruction::FENCE(_, _, ops, fm) = *self {
            let sw = if ops & 0b0000_0001 != 0 { "w" } else { "" };
            let sr = if ops & 0b0000_0010 != 0 { "r" } else { "" };
            let so = if ops & 0b0000_0100 != 0 { "o" } else { "" };
            let si = if ops & 0b0000_1000 != 0 { "i" } else { "" };
            let pw = if ops & 0b0001_0000 != 0 { "w" } else { "" };
            let pr = if ops & 0b0010_0000 != 0 { "r" } else { "" };
            let po = if ops & 0b0100_0000 != 0 { "o" } else { "" };
            let pi = if ops & 0b1000_0000 != 0 { "i" } else { "" };
            if fm == 0b1000 {
                format!("fence.tso {pi}{po}{pr}{pw},{si}{so}{sr}{sw}")
            } else {
                format!("fence {pi}{po}{pr}{pw},{si}{so}{sr}{sw}")
            }
        } else {
            unreachable!();
        }
    }
}

fn j_immediate_from_u_immediate(u: u32) -> i32 {
    let a = (u >> 12) & 0b1111_1111;
    let b = (u >> 20) & 0b1;
    let c = (u >> 21) & 0b11_1111_1111;
    let d = u >> 31;

    let i = (c << 1) | (b << 11) | (a << 12) | (d << 20);
    ((i << 12) as i32) >> 12
}

fn parse_int(str: &str) -> Result<i64, String> {
    match str.parse::<i64>() {
        Ok(e) => Ok(e),
        Err(_) => Err(format!("unable to parse int:{str}").to_owned()),
    }
}

fn parse_address_expression(str: &str) -> Result<(IRegister, i64), String> {
    let (offset, register): (&str, &str) = if let Some(x) = str.split_once("(") {
        x
    } else {
        panic!("no (");
    };
    match register.strip_suffix(")") {
        Some(y) => {
            let r = IRegister::from_string(y)?;
            let i = parse_int(offset)?;
            Ok((r, i))
        }
        _ => Err("Address expression should end in a )".to_owned()),
    }
}

/// Constructs an `Instruction` from a line of assembly.
pub fn assemble_line(line: &str) -> Result<Instruction, String> {
    let (mnemonic, operands): (&str, &str) = if let Some(x) = line.split_once(" ") {
        x
    } else {
        (line, "")
    };

    let operands: Vec<&str> = if operands.is_empty() {
        vec![]
    } else {
        operands.split(',').collect()
    };

    match mnemonic {
        // register-immediate instructions
        "addi" => i_assemble!(ADDI),
        "addiw" => i_assemble!(ADDIW),
        "andi" => i_assemble!(ANDI),
        "ori" => i_assemble!(ORI),
        "xori" => i_assemble!(XORI),
        "slti" => i_assemble!(SLTI),
        "sltiu" => i_assemble!(SLTIU),
        "slli" => sh_assemble!(SLLI),
        "srai" => sh_assemble!(SRAI),
        "sraiw" => shw_assemble!(SRAIW),
        "srli" => sh_assemble!(SRLI),
        "srliw" => shw_assemble!(SRLIW),
        "slliw" => shw_assemble!(SLLIW),
        // register-register instructions
        "add" => r_assemble!(ADD),
        "addw" => r_assemble!(ADDW),
        "subw" => r_assemble!(SUBW),
        "and" => r_assemble!(AND),
        "sub" => r_assemble!(SUB),
        "or" => r_assemble!(OR),
        "xor" => r_assemble!(XOR),
        "sllw" => r_assemble!(SLLW),
        "srl" => r_assemble!(SRL),
        "sra" => r_assemble!(SRA),
        "srlw" => r_assemble!(SRLW),
        "sraw" => r_assemble!(SRAW),
        "sll" => r_assemble!(SLL),
        "slt" => r_assemble!(SLT),
        "sltu" => r_assemble!(SLTU),
        "mul" => r_assemble!(MUL),
        "mulh" => r_assemble!(MULH),
        "mulhsu" => r_assemble!(MULHSU),
        "mulhu" => r_assemble!(MULHU),
        "div" => r_assemble!(DIV),
        "divu" => r_assemble!(DIVU),
        "rem" => r_assemble!(REM),
        "remu" => r_assemble!(REMU),
        "mulw" => r_assemble!(MULW),
        "divw" => r_assemble!(DIVW),
        "divuw" => r_assemble!(DIVUW),
        "remw" => r_assemble!(REMW),
        "remuw" => r_assemble!(REMUW),
        // load instructions
        "lb" => l_assemble!(LB),
        "lbu" => l_assemble!(LBU),
        "lhu" => l_assemble!(LHU),
        "lw" => l_assemble!(LW),
        "lwu" => l_assemble!(LWU),
        "lh" => l_assemble!(LH),
        // store instructions
        "ld" => l_assemble!(LD),
        "sd" => s_assemble!(SD),
        "sw" => s_assemble!(SW),
        "sh" => s_assemble!(SH),
        "sb" => s_assemble!(SB),
        // branch instructions
        "blt" => b_assemble!(BLT),
        "beq" => b_assemble!(BEQ),
        "bne" => b_assemble!(BNE),
        "bge" => b_assemble!(BGE),
        "bgeu" => b_assemble!(BGEU),
        "bltu" => b_assemble!(BLTU),
        "jalr" => {
            if operands.len() != 2 {
                Err("jalr instruction requires 2 operands".to_owned())
            } else {
                let (base, offset) = parse_address_expression(operands[1])?;
                Ok(Instruction::JALR(
                    IRegister::from_string(operands[0])?,
                    base,
                    IImmediate::from_val(offset),
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
        "lui" => {
            if operands.len() != 2 {
                Err("lui instruction requires 2 operands".to_owned())
            } else {
                let int: u32 = parse_int(operands[1])? as u32;
                if int & 0xFFF != 0 {
                    Err("auipc immediate must be divisible by 0x1000".to_owned())
                } else {
                    Ok(Instruction::LUI(IRegister::from_string(operands[0])?, int))
                }
            }
        }
        "auipc" => {
            if operands.len() != 2 {
                Err("auipc instruction requires 2 operands".to_owned())
            } else {
                let int: u32 = parse_int(operands[1])? as u32;
                if int & 0xFFF != 0 {
                    Err("auipc immediate must be divisible by 0x1000".to_owned())
                } else {
                    Ok(Instruction::AUIPC(
                        IRegister::from_string(operands[0])?,
                        int,
                    ))
                }
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
        "fence.tso" => {
            if operands.len() != 2 {
                Err("fence.tso instruction requires 2 operands".to_owned())
            } else {
                let ops = parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                if ops != (parse_fence_set("rw") | (parse_fence_set("rw") << 4)) {
                    Err("fence.tso should be rw,rw".to_owned())
                } else {
                    Ok(Instruction::FENCE(
                        // rd and rs1 are currently unused
                        IRegister::Zero,
                        IRegister::Zero,
                        ops,
                        0b1000, // tso fence
                    ))
                }
            }
        }
        _ => Err(format!("unknown mnemonic: {}", mnemonic)),
    }
}

/// Converts a string representing operations into a fence u8
pub fn parse_fence_set(s: &str) -> u8 {
    let mut x = 0;
    if s.contains("w") {
        x |= 0b1;
    }
    if s.contains("r") {
        x |= 0b10;
    }
    if s.contains("o") {
        x |= 0b100;
    }
    if s.contains("i") {
        x |= 0b1000;
    }
    x
}

/// Disassembles an instruction.
pub fn disassemble_instruction(instruction: &Instruction) -> String {
    format!("{}", instruction)
}

/// Constructs an `Instruction` from it's machine code representation.
pub fn decode_instruction(instruction: u32) -> Result<Instruction, String> {
    let opcode = Opcode::from_int(instruction & 0b111_1111);

    let func3 = (instruction >> 12) & 0b111;
    let func7 = (instruction >> 25) & 0b111_1111;

    let rd = IRegister::from_int((instruction >> 7) & 0b1_1111);
    let rs1 = IRegister::from_int((instruction >> 15) & 0b1_1111);
    let rs2 = IRegister::from_int((instruction >> 20) & 0b1_1111);

    let i_immediate: IImmediate = IImmediate::from_u32(instruction);

    let s_immediate: SImmediate = SImmediate::from_u32(instruction);

    let u_immediate = instruction & (!0b1111_1111_1111);

    let b = (((instruction >> 7) & 0b1) << 11)
        | (((instruction >> 8) & 0b1111) << 1)
        | (((instruction >> 25) & 0b11_1111) << 5)
        | ((instruction >> 31) << 12);

    let b_immediate = ((b << 20_i32) >> 20) as i16;

    let shamt: Shamt = Shamt::from_u32(instruction);

    let shamtw: ShamtW = ShamtW::from_u32(instruction);

    let aq: bool = ((instruction >> 25) & 0b1) == 0b1;
    let rl: bool = ((instruction >> 26) & 0b1) == 0b1;

    match opcode {
        Opcode::Load => match func3 {
            0b000 => Ok(Instruction::LB(rd, rs1, i_immediate)),
            0b001 => Ok(Instruction::LH(rd, rs1, i_immediate)),
            0b010 => Ok(Instruction::LW(rd, rs1, i_immediate)),
            0b011 => Ok(Instruction::LD(rd, rs1, i_immediate)),
            0b100 => Ok(Instruction::LBU(rd, rs1, i_immediate)),
            0b101 => Ok(Instruction::LHU(rd, rs1, i_immediate)),
            0b110 => Ok(Instruction::LWU(rd, rs1, i_immediate)),
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
        Opcode::Op => match (func7, func3) {
            (0b000_0000, 0b000) => Ok(Instruction::ADD(rd, rs1, rs2)),
            (0b000_0000, 0b001) => Ok(Instruction::SLL(rd, rs1, rs2)),
            (0b000_0000, 0b010) => Ok(Instruction::SLT(rd, rs1, rs2)),
            (0b000_0000, 0b011) => Ok(Instruction::SLTU(rd, rs1, rs2)),
            (0b000_0000, 0b100) => Ok(Instruction::XOR(rd, rs1, rs2)),
            (0b000_0000, 0b101) => Ok(Instruction::SRL(rd, rs1, rs2)),
            (0b000_0000, 0b110) => Ok(Instruction::OR(rd, rs1, rs2)),
            (0b000_0000, 0b111) => Ok(Instruction::AND(rd, rs1, rs2)),
            (0b010_0000, 0b000) => Ok(Instruction::SUB(rd, rs1, rs2)),
            (0b010_0000, 0b101) => Ok(Instruction::SRA(rd, rs1, rs2)),
            (0b000_0001, 0b000) => Ok(Instruction::MUL(rd, rs1, rs2)),
            (0b000_0001, 0b001) => Ok(Instruction::MULH(rd, rs1, rs2)),
            (0b000_0001, 0b010) => Ok(Instruction::MULHSU(rd, rs1, rs2)),
            (0b000_0001, 0b011) => Ok(Instruction::MULHU(rd, rs1, rs2)),
            (0b000_0001, 0b100) => Ok(Instruction::DIV(rd, rs1, rs2)),
            (0b000_0001, 0b101) => Ok(Instruction::DIVU(rd, rs1, rs2)),
            (0b000_0001, 0b110) => Ok(Instruction::REM(rd, rs1, rs2)),
            (0b000_0001, 0b111) => Ok(Instruction::REMU(rd, rs1, rs2)),
            _ => Err(format!("unknown Op. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::Op32 => match (func3, func7) {
            (0b000, 0b000_0000) => Ok(Instruction::ADDW(rd, rs1, rs2)),
            (0b000, 0b000_0001) => Ok(Instruction::MULW(rd, rs1, rs2)),
            (0b000, 0b010_0000) => Ok(Instruction::SUBW(rd, rs1, rs2)),
            (0b001, 0b000_0000) => Ok(Instruction::SLLW(rd, rs1, rs2)),
            (0b100, 0b0000_001) => Ok(Instruction::DIVW(rd, rs1, rs2)),
            (0b101, 0b000_0000) => Ok(Instruction::SRLW(rd, rs1, rs2)),
            (0b101, 0b000_0001) => Ok(Instruction::DIVUW(rd, rs1, rs2)),
            (0b101, 0b010_0000) => Ok(Instruction::SRAW(rd, rs1, rs2)),
            (0b110, 0b000_0001) => Ok(Instruction::REMW(rd, rs1, rs2)),
            (0b111, 0b000_0001) => Ok(Instruction::REMUW(rd, rs1, rs2)),
            _ => Err(format!("unknown Op32. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm => match func3 {
            0b000 => Ok(Instruction::ADDI(rd, rs1, i_immediate)),
            // SLLi requires special handling because shamt uses the bottom bit of func7
            0b001 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SLLI(rd, rs1, shamt)),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b010 => Ok(Instruction::SLTI(rd, rs1, i_immediate)),
            0b011 => Ok(Instruction::SLTIU(rd, rs1, i_immediate)),
            0b100 => Ok(Instruction::XORI(rd, rs1, i_immediate)),
            // SRLI SRAI require special handling because shamt uses the bottom bit of func7
            0b101 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SRLI(rd, rs1, shamt)),
                0b010000_1 => Ok(Instruction::SRAI(rd, rs1, shamt)),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b110 => Ok(Instruction::ORI(rd, rs1, i_immediate)),
            0b111 => Ok(Instruction::ANDI(rd, rs1, i_immediate)),
            _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm32 => match func3 {
            0b000 => Ok(Instruction::ADDIW(rd, rs1, i_immediate)),
            0b001 => Ok(Instruction::SLLIW(rd, rs1, shamtw)),
            0b101 => match func7 {
                0b000_0000 => Ok(Instruction::SRLIW(rd, rs1, shamtw)),
                0b010_0000 => Ok(Instruction::SRAIW(rd, rs1, shamtw)),
                x => Err(format!("unknown OpImm32(101) func7: {}", x).to_owned()),
            },
            x => Err(format!("unkown OpImm32 func3: {}", x).to_owned()),
        },
        Opcode::Jalr => Ok(Instruction::JALR(rd, rs1, i_immediate)),
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
                    Err("reserved register fields not set to zero".to_owned())
                } else {
                    let fm = ((instruction >> 28) & 0b1111) as u8;
                    if fm != 0 && fm != 0b1000 {
                        Err(format!("reserved fence FM: {fm}").to_owned())
                    } else if fm == 0b1000 && ((instruction >> 20) & 0xFF) != 0b0011_0011 {
                        Err("fence.tso must be rw,rw".to_owned())
                    } else {
                        Ok(Instruction::FENCE(
                            rd,
                            rs1,
                            ((instruction >> 20) & 0xFF) as u8,
                            ((instruction >> 28) & 0b1111) as u8,
                        ))
                    }
                }
            }
            x => Err(format!("unknown fence func3: {x}")),
        },
        // shift func7 left 2 to clear aq,rl
        Opcode::AMO => match (func3, func7 >> 2) {
            (0b010, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.W expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRW(rd, rs1, aq, rl))
                }
            }
            (0b011, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.D expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRD(rd, rs1, aq, rl))
                }
            }
            (0b010, 0b00011) => Ok(Instruction::SCW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00011) => Ok(Instruction::SCD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00001) => Ok(Instruction::AMOSWAPW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00001) => Ok(Instruction::AMOSWAPD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00000) => Ok(Instruction::AMOADDW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00000) => Ok(Instruction::AMOADDD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b00100) => Ok(Instruction::AMOXORW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b00100) => Ok(Instruction::AMOXORD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b01100) => Ok(Instruction::AMOANDW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b01100) => Ok(Instruction::AMOANDD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b01000) => Ok(Instruction::AMOORW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b01000) => Ok(Instruction::AMOORD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b10000) => Ok(Instruction::AMOMINW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b10000) => Ok(Instruction::AMOMIND(rd, rs1, rs2, aq, rl)),
            (0b010, 0b10100) => Ok(Instruction::AMOMAXW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b10100) => Ok(Instruction::AMOMAXD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b11000) => Ok(Instruction::AMOMINUW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b11000) => Ok(Instruction::AMOMINUD(rd, rs1, rs2, aq, rl)),
            (0b010, 0b11100) => Ok(Instruction::AMOMAXUW(rd, rs1, rs2, aq, rl)),
            (0b011, 0b11100) => Ok(Instruction::AMOMAXUD(rd, rs1, rs2, aq, rl)),
            _ => Err(format!("unknown AMO. func3: {func3}, func7: {func7}")),
        },
        Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
    }
}
