use crate::cinstruction::CInstruction;
use crate::immediates::{
    BImmediate, CSR, CSRImmediate, JImmediate, SImmediate, Shamt, ShamtW, UImmediate,
};
use crate::register::{CIRegister, FRegister, IRegister};
use crate::{immediates::IImmediate, opcode::Opcode};
use std::fmt::{Display, Formatter};

use proc_macros::{
    amo_assemble, b_assemble, fr_assemble, i_assemble, l_assemble, r_assemble, s_assemble,
    sh_assemble, shw_assemble,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoundingMode {
    /// round to nearest, ties to even
    RNE = 0b000,
    /// round towards zero
    RTZ = 0b001,
    /// round down
    RDN = 0b010,
    /// round up
    RUP = 0b011,
    /// round to nearest, ties to max magnitude
    RMM = 0b100,
    /// use rounding mode in fcsr
    DYN = 0b111,
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RoundingMode::RNE => write!(f, "rne"),
            RoundingMode::RTZ => write!(f, "rtz"),
            RoundingMode::RDN => write!(f, "rdn"),
            RoundingMode::RUP => write!(f, "rup"),
            RoundingMode::RMM => write!(f, "rmm"),
            RoundingMode::DYN => write!(f, "dyn"),
        }
    }
}

impl RoundingMode {
    pub fn from_int(x: u32) -> Result<RoundingMode, String> {
        match x {
            0b000 => Ok(RoundingMode::RNE),
            0b001 => Ok(RoundingMode::RTZ),
            0b010 => Ok(RoundingMode::RDN),
            0b011 => Ok(RoundingMode::RUP),
            0b100 => Ok(RoundingMode::RMM),
            0b111 => Ok(RoundingMode::DYN),
            _ => Err("attempted to create invalid rounding mode".to_owned()),
        }
    }
    pub fn from_str(x: &str) -> Result<RoundingMode, String> {
        match x {
            "rne" => Ok(RoundingMode::RNE),
            "rtz" => Ok(RoundingMode::RTZ),
            "rdn" => Ok(RoundingMode::RDN),
            "rup" => Ok(RoundingMode::RUP),
            "rmm" => Ok(RoundingMode::RMM),
            "dyn" => Ok(RoundingMode::DYN),
            _ => Err("attempted to create invalid rounding mode".to_owned()),
        }
    }

    pub fn to_u32(self) -> u32 {
        return (self as u32) << 12;
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    //
    // Instructions from RV32I
    //
    /// Load upper immediate
    LUI {
        dest: IRegister,
        imm: UImmediate,
    },
    /// Add upper immediate to PC
    AUIPC {
        dest: IRegister,
        imm: UImmediate,
    },
    /// Jump and Link
    JAL {
        dest: IRegister,
        offset: JImmediate,
    },
    /// Jump and Link Register
    JALR {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    BEQ {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    BNE {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    BLT {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    BGE {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    BLTU {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    BGEU {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    /// Load Byte
    LB {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Halfword
    LH {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Word
    LW {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Byte Unsigned
    LBU {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Halfword Unsigned
    LHU {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Store Byte
    SB {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Store Halfword
    SH {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Store Word
    SW {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    ADDI {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    SLTI {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    SLTIU {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    XORI {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    ORI {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    ANDI {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    /// Left Shift Immediate
    SLLI {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    /// Logical Right Shift Immediate
    SRLI {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    /// Arithmetic Right Shift Immediate
    SRAI {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    ADD {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    SUB {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Left Shift
    SLL {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Branch if Equal
    SLT {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    SLTU {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    XOR {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Logical Right Shift Immediate
    SRL {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Arithmetic Right Shift Immediate
    SRA {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    OR {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    AND {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    FENCE {
        rd: IRegister,
        rs1: IRegister,
        ops: u8,
        fm: u8,
    },
    ECALL,
    EBREAK,
    //
    // Instructions Added In RV64I
    //
    /// Load Word Unsigned
    LWU {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Doubleword
    LD {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Store Doubleword
    SD {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Add Immediate (word)
    ADDIW {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    /// Left Shift Immediate (word)
    SLLIW {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Logical Right Shift Immediate (word)
    SRLIW {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Arithmetic Right Shift Immediate (word)
    SRAIW {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Add (word)
    ADDW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Subtract (word)
    SUBW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Left Shift (word)
    SLLW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Logical Right Shift (word)
    SRLW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Arithmetic Right Shift (word)
    SRAW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    //
    // Instructions In M Extension
    //
    /// Multiply
    MUL {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply (High bits)
    MULH {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Signed-Unsigned (High bits)
    MULHSU {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Unsigned (High)
    MULHU {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide
    DIV {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide (Unsigned)
    DIVU {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder
    REM {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder (Unsigned)
    REMU {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Word
    MULW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide Word
    DIVW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide Unsigned Word
    DIVUW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder Word
    REMW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder Unsigned Word
    REMUW {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    //
    // Instructions In A Extension
    //
    /// Load Reserved Word
    // rd, rs1, ac, rl
    LRW {
        dest: IRegister,
        addr: IRegister,
        aq: bool,
        rl: bool,
    },
    SCW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOSWAPW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOADDW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOXORW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOANDW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOORW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMINW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMAXW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMINUW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMAXUW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    //
    LRD {
        dest: IRegister,
        addr: IRegister,
        aq: bool,
        rl: bool,
    },
    SCD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOSWAPD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOADDD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOXORD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOANDD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOORD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMIND {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMAXD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMINUD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AMOMAXUD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    //
    // Instructions in F Extension
    //
    FLW {
        dest: FRegister,
        base: IRegister,
        offset: IImmediate,
    },
    FSW {
        base: IRegister,
        src: FRegister,
        offset: SImmediate,
    },
    FMADDS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FMSUBS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FNMSUBS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FNMADDS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FADDS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FSUBS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FMULS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FDIVS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FSQRTS {
        dest: FRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FSGNJS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FSGNJNS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FSGNJXS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FMINS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FMAXS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FCVTWS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FCVTWUS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FMVXW {
        dest: IRegister,
        src: FRegister,
    },
    FEQS {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FLTS {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FLES {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FCLASSS {
        dest: IRegister,
        src: FRegister,
    },
    FCVTSW {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FCVTSWU {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FMVWX {
        dest: FRegister,
        src: IRegister,
    },
    //
    // Instructions in F Extension (RV64)
    //
    FCVTLS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FCVTLUS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FCVTSL {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FCVTSLU {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    //
    // Instructions in Zicsr Extension
    //
    CSRRW {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    CSRRS {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    CSRRC {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    CSRRWI {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    CSRRSI {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    CSRRCI {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    //
    // Instructions in Zifencei Extension
    //
    FENCEI,
}

fn aq_rl_suffix(aq: &bool, rl: &bool) -> &'static str {
    match (aq, rl) {
        (true, true) => ".aqrl",
        (true, false) => ".aq",
        (false, true) => ".rl",
        (false, false) => "",
    }
}

/// puts the aquire bit in the correct location
fn aqb(aq: bool) -> u32 {
    if aq { 1 << 26 } else { 0 }
}

/// puts the release bit in the correct location
fn rlb(rl: bool) -> u32 {
    if rl { 1 << 25 } else { 0 }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::LUI { dest, imm } => write!(f, "lui {dest},{imm}"),
            Instruction::AUIPC { dest, imm } => write!(f, "auipc {dest},{imm}"),
            Instruction::JAL { dest, offset } => write!(f, "jal {dest},{offset}"),
            Instruction::JALR { dest, base, offset } => write!(f, "jalr {dest},{offset}({base})"),
            Instruction::BEQ { src1, src2, offset } => write!(f, "beq {src1},{src2},{offset}"),
            Instruction::BNE { src1, src2, offset } => write!(f, "bne {src1},{src2},{offset}"),
            Instruction::BLT { src1, src2, offset } => write!(f, "blt {src1},{src2},{offset}"),
            Instruction::BGE { src1, src2, offset } => write!(f, "bge {src1},{src2},{offset}"),
            Instruction::BLTU { src1, src2, offset } => write!(f, "bltu {src1},{src2},{offset}"),
            Instruction::BGEU { src1, src2, offset } => write!(f, "bgeu {src1},{src2},{offset}"),
            Instruction::LB { dest, base, offset } => write!(f, "lb {dest},{offset}({base})"),
            Instruction::LH { dest, base, offset } => write!(f, "lh {dest},{offset}({base})"),
            Instruction::LW { dest, base, offset } => write!(f, "lw {dest},{offset}({base})"),
            Instruction::LBU { dest, base, offset } => write!(f, "lbu {dest},{offset}({base})"),
            Instruction::LHU { dest, base, offset } => write!(f, "lhu {dest},{offset}({base})"),
            Instruction::SB { src, base, offset } => write!(f, "sb {src},{offset}({base})"),
            Instruction::SH { src, base, offset } => write!(f, "sh {src},{offset}({base})"),
            Instruction::SW { src, base, offset } => write!(f, "sw {src},{offset}({base})"),
            Instruction::ADDI { dest, src, imm } => write!(f, "addi {dest},{src},{imm}"),
            Instruction::SLTI { dest, src, imm } => write!(f, "slti {dest},{src},{imm}"),
            Instruction::SLTIU { dest, src, imm } => write!(f, "sltiu {dest},{src},{imm}"),
            Instruction::XORI { dest, src, imm } => write!(f, "xori {dest},{src},{imm}"),
            Instruction::ORI { dest, src, imm } => write!(f, "ori {dest},{src},{imm}"),
            Instruction::ANDI { dest, src, imm } => write!(f, "andi {dest},{src},{imm}"),
            Instruction::SLLI { dest, src, shamt } => write!(f, "slli {dest},{src},{shamt}"),
            Instruction::SRLI { dest, src, shamt } => write!(f, "srli {dest},{src},{shamt}"),
            Instruction::SRAI { dest, src, shamt } => write!(f, "srai {dest},{src},{shamt}"),
            Instruction::ADD { dest, src1, src2 } => write!(f, "add {dest},{src1},{src2}"),
            Instruction::SUB { dest, src1, src2 } => write!(f, "sub {dest},{src1},{src2}"),
            Instruction::SLL { dest, src1, src2 } => write!(f, "sll {dest},{src1},{src2}"),
            Instruction::SLT { dest, src1, src2 } => write!(f, "slt {dest},{src1},{src2}"),
            Instruction::SLTU { dest, src1, src2 } => write!(f, "sltu {dest},{src1},{src2}"),
            Instruction::XOR { dest, src1, src2 } => write!(f, "xor {dest},{src1},{src2}"),
            Instruction::SRL { dest, src1, src2 } => write!(f, "srl {dest},{src1},{src2}"),
            Instruction::SRA { dest, src1, src2 } => write!(f, "sra {dest},{src1},{src2}"),
            Instruction::OR { dest, src1, src2 } => write!(f, "or {dest},{src1},{src2}"),
            Instruction::AND { dest, src1, src2 } => write!(f, "and {dest},{src1},{src2}"),
            Instruction::FENCE { .. } => write!(f, "{}", self.fmt_fence()),
            Instruction::ECALL => write!(f, "ecall"),
            Instruction::EBREAK => write!(f, "ebreak"),
            Instruction::LWU { dest, base, offset } => write!(f, "lwu {dest},{offset}({base})"),
            Instruction::LD { dest, base, offset } => write!(f, "ld {dest},{offset}({base})"),
            Instruction::SD { src, base, offset } => write!(f, "sd {src},{offset}({base})"),
            Instruction::ADDIW { dest, src, imm } => write!(f, "addiw {dest},{src},{imm}"),
            Instruction::SLLIW { dest, src, shamt } => write!(f, "slliw {dest},{src},{shamt}"),
            Instruction::SRLIW { dest, src, shamt } => write!(f, "srliw {dest},{src},{shamt}"),
            Instruction::SRAIW { dest, src, shamt } => write!(f, "sraiw {dest},{src},{shamt}"),
            Instruction::ADDW { dest, src1, src2 } => write!(f, "addw {dest},{src1},{src2}"),
            Instruction::SUBW { dest, src1, src2 } => write!(f, "subw {dest},{src1},{src2}"),
            Instruction::SLLW { dest, src1, src2 } => write!(f, "sllw {dest},{src1},{src2}"),
            Instruction::SRLW { dest, src1, src2 } => write!(f, "srlw {dest},{src1},{src2}"),
            Instruction::SRAW { dest, src1, src2 } => write!(f, "sraw {dest},{src1},{src2}"),
            Instruction::MUL { dest, src1, src2 } => write!(f, "mul {dest},{src1},{src2}"),
            Instruction::MULH { dest, src1, src2 } => write!(f, "mulh {dest},{src1},{src2}"),
            Instruction::MULHSU { dest, src1, src2 } => write!(f, "mulhsu {dest},{src1},{src2}"),
            Instruction::MULHU { dest, src1, src2 } => write!(f, "mulhu {dest},{src1},{src2}"),
            Instruction::DIV { dest, src1, src2 } => write!(f, "div {dest},{src1},{src2}"),
            Instruction::DIVU { dest, src1, src2 } => write!(f, "divu {dest},{src1},{src2}"),
            Instruction::REM { dest, src1, src2 } => write!(f, "rem {dest},{src1},{src2}"),
            Instruction::REMU { dest, src1, src2 } => write!(f, "remu {dest},{src1},{src2}"),
            Instruction::MULW { dest, src1, src2 } => write!(f, "mulw {dest},{src1},{src2}"),
            Instruction::DIVW { dest, src1, src2 } => write!(f, "divw {dest},{src1},{src2}"),
            Instruction::DIVUW { dest, src1, src2 } => write!(f, "divuw {dest},{src1},{src2}"),
            Instruction::REMW { dest, src1, src2 } => write!(f, "remw {dest},{src1},{src2}"),
            Instruction::REMUW { dest, src1, src2 } => write!(f, "remuw {dest},{src1},{src2}"),
            Instruction::LRW { dest, addr, aq, rl } => {
                write!(f, "lr.w{} {dest},{addr}", aq_rl_suffix(aq, rl))
            }
            Instruction::SCW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "sc.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOSWAPW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoswap.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOADDW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoadd.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOXORW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoxor.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOANDW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoand.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOORW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoor.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }

            Instruction::AMOMINW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomin.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomax.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMINUW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amominu.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXUW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomaxu.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::LRD { dest, addr, aq, rl } => {
                write!(f, "lr.d{} {dest},{addr}", aq_rl_suffix(aq, rl))
            }
            Instruction::SCD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "sc.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOSWAPD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoswap.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOADDD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoadd.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOXORD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoxor.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOANDD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoand.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOORD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoor.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMIND {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomin.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomax.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMINUD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amominu.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AMOMAXUD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomaxu.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::FLW { dest, base, offset } => write!(f, "flw {dest},{offset}({base})"),
            Instruction::FSW { base, src, offset } => write!(f, "fsw {src},{offset}({base})"),
            Instruction::FMADDS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fmadd.s.{rm} {dest},{src1},{src2},{src3}")
            }
            Instruction::FMSUBS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fmsub.s.{rm} {dest},{src1},{src2},{src3}")
            }
            Instruction::FNMSUBS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fnmsub.s.{rm} {dest},{src1},{src2},{src3}")
            }
            Instruction::FNMADDS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fnmadd.s.{rm} {dest},{src1},{src2},{src3}")
            }
            Instruction::FADDS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fadd.s.{rm} {dest},{src1},{src2}"),
            Instruction::FSUBS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fsub.s.{rm} {dest},{src1},{src2}"),
            Instruction::FMULS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fmul.s.{rm} {dest},{src1},{src2}"),
            Instruction::FDIVS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fdiv.s.{rm} {dest},{src1},{src2}"),
            Instruction::FSQRTS { dest, src, rm } => write!(f, "fsqrt.s.{rm} {dest},{src}"),
            Instruction::FSGNJS { dest, src1, src2 } => write!(f, "fsgnj.s {dest},{src1},{src2}"),
            Instruction::FSGNJNS { dest, src1, src2 } => write!(f, "fsgnjn.s {dest},{src1},{src2}"),
            Instruction::FSGNJXS { dest, src1, src2 } => write!(f, "fsgnjx.s {dest},{src1},{src2}"),
            Instruction::FMINS { dest, src1, src2 } => write!(f, "fmin.s {dest},{src1},{src2}"),
            Instruction::FMAXS { dest, src1, src2 } => write!(f, "fmax.s {dest},{src1},{src2}"),
            Instruction::FCVTWS { dest, src, rm } => write!(f, "fcvt.w.s.{rm} {dest},{src}"),
            Instruction::FCVTWUS { dest, src, rm } => write!(f, "fcvt.wu.s.{rm} {dest},{src}"),
            Instruction::FMVXW { dest, src } => write!(f, "fmv.x.w {dest},{src}"),
            Instruction::FEQS { dest, src1, src2 } => write!(f, "feq.s {dest},{src1},{src2}"),
            Instruction::FLTS { dest, src1, src2 } => write!(f, "flt.s {dest},{src1},{src2}"),
            Instruction::FLES { dest, src1, src2 } => write!(f, "fle.s {dest},{src1},{src2}"),
            Instruction::FCLASSS { dest, src } => write!(f, "fclass.s {dest},{src}"),
            Instruction::FCVTSW { dest, src, rm } => write!(f, "fcvt.s.w.{rm} {dest},{src}"),
            Instruction::FCVTSWU { dest, src, rm } => write!(f, "fcvt.s.wu.{rm} {dest},{src}"),
            Instruction::FMVWX { dest, src } => write!(f, "fmv.w.x {dest},{src}"),
            Instruction::FCVTLS { dest, src, rm } => write!(f, "fcvt.l.s.{rm} {dest},{src}"),
            Instruction::FCVTLUS { dest, src, rm } => write!(f, "fcvt.lu.s.{rm} {dest},{src}"),
            Instruction::FCVTSL { dest, src, rm } => write!(f, "fcvt.s.l.{rm} {dest},{src}"),
            Instruction::FCVTSLU { dest, src, rm } => write!(f, "fcvt.s.lu.{rm} {dest},{src}"),
            Instruction::CSRRW { dest, src, csr } => write!(f, "csrrw {dest},{csr},{src}"),
            Instruction::CSRRS { dest, src, csr } => write!(f, "csrrs {dest},{csr},{src}"),
            Instruction::CSRRC { dest, src, csr } => write!(f, "csrrc {dest},{csr},{src}"),
            Instruction::CSRRWI { dest, imm, csr } => write!(f, "csrrwi {dest},{csr},{imm}"),
            Instruction::CSRRSI { dest, imm, csr } => write!(f, "csrrsi {dest},{csr},{imm}"),
            Instruction::CSRRCI { dest, imm, csr } => write!(f, "csrrci {dest},{csr},{imm}"),
            Instruction::FENCEI => write!(f, "fence.i"),
        }
    }
}

impl Instruction {
    fn fmt_fence(&self) -> String {
        if let Instruction::FENCE {
            rd: _,
            rs1: _,
            ops,
            fm,
        } = *self
        {
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

pub fn parse_int(str: &str) -> Result<i64, String> {
    match str.parse::<i64>() {
        Ok(e) => Ok(e),
        Err(_) => Err(format!("unable to parse int:{str}").to_owned()),
    }
}

pub fn parse_address_expression(str: &str) -> Result<(IRegister, i64), String> {
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

pub fn parse_address_expression_compressed(str: &str) -> Result<(CIRegister, i64), String> {
    let (offset, register): (&str, &str) = if let Some(x) = str.split_once("(") {
        x
    } else {
        panic!("no (");
    };
    match register.strip_suffix(")") {
        Some(y) => {
            let r = CIRegister::try_from(y)?;
            let i = parse_int(offset)?;
            Ok((r, i))
        }
        _ => Err("Address expression should end in a )".to_owned()),
    }
}

#[derive(Debug, PartialEq)]
pub enum AssemblyResult {
    I(Instruction),
    C(CInstruction),
}
impl AssemblyResult {
    pub fn c(self) -> CInstruction {
        match self {
            AssemblyResult::I(_) => panic!("c called on regular instruction"),
            AssemblyResult::C(cinstruction) => cinstruction,
        }
    }
    pub fn i(self) -> Instruction {
        match self {
            AssemblyResult::I(instruction) => instruction,
            AssemblyResult::C(_) => panic!("i called on compressed instruction"),
        }
    }
}

/// Constructs an `Instruction` from a line of assembly.
pub fn assemble_line(line: &str) -> Result<AssemblyResult, String> {
    let (mnemonic, operands): (&str, &str) = if let Some(x) = line.split_once(" ") {
        x
    } else {
        (line, "")
    };

    let mnemonics: Vec<&str> = mnemonic.split(".").collect();

    let operands: Vec<&str> = if operands.is_empty() {
        vec![]
    } else {
        operands.split(',').collect()
    };
    let operands: Vec<&str> = operands
        .iter()
        .map(|operand| operand.to_owned().trim())
        .collect();

    println!("operands: {:?}", operands);
    println!("mnemonics: {:?}", mnemonics);

    if mnemonics[0] == "c" {
        if mnemonics.len() == 1 {
            Err("compressed instruction must be specified".to_owned())
        } else {
            CInstruction::assemble_line(&mnemonics[1..], operands).map(AssemblyResult::C)
        }
    } else {
        let x = match mnemonics[0] {
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
                    Ok(Instruction::JALR {
                        dest: IRegister::from_string(operands[0])?,
                        base,
                        offset: IImmediate::try_from(offset)?,
                    })
                }
            }
            "jal" => {
                if operands.len() != 2 {
                    Err("jal instruction requires 2 operands".to_owned())
                } else {
                    Ok(Instruction::JAL {
                        dest: IRegister::from_string(operands[0])?,
                        offset: JImmediate::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "lui" => {
                if operands.len() != 2 {
                    Err("lui instruction requires 2 operands".to_owned())
                } else {
                    let int: i64 = parse_int(operands[1])?;
                    if int > 2i64.pow(19) - 1 || int < -2i64.pow(19) {
                        Err("UImmediate out of range".to_owned())
                    } else {
                        Ok(Instruction::LUI {
                            dest: IRegister::from_string(operands[0])?,
                            imm: UImmediate::try_from(int)?,
                        })
                    }
                }
            }
            "auipc" => {
                if operands.len() != 2 {
                    Err("auipc instruction requires 2 operands".to_owned())
                } else {
                    let int: i64 = parse_int(operands[1])?;
                    if int > 2i64.pow(19) - 1 || int < -2i64.pow(19) {
                        Err("UImmediate out of range".to_owned())
                    } else {
                        Ok(Instruction::AUIPC {
                            dest: IRegister::from_string(operands[0])?,
                            imm: UImmediate::try_from(int)?,
                        })
                    }
                }
            }
            "fence" => {
                if mnemonics.len() == 1 {
                    if operands.len() != 2 {
                        Err("fence instruction requires 2 operands".to_owned())
                    } else {
                        let ops =
                            parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                        Ok(Instruction::FENCE {
                            // rd and rs1 are currently unused
                            rd: IRegister::Zero,
                            rs1: IRegister::Zero,
                            ops,
                            fm: 0, //fm field, always zero for a non-tso fence
                        })
                    }
                } else if mnemonics[1] == "tso" {
                    if operands.len() != 2 {
                        Err("fence.tso instruction requires 2 operands".to_owned())
                    } else {
                        let ops =
                            parse_fence_set(operands[1]) | (parse_fence_set(operands[0]) << 4);
                        if ops != (parse_fence_set("rw") | (parse_fence_set("rw") << 4)) {
                            Err("fence.tso should be rw,rw".to_owned())
                        } else {
                            Ok(Instruction::FENCE {
                                // rd and rs1 are currently unused
                                rd: IRegister::Zero,
                                rs1: IRegister::Zero,
                                ops,
                                fm: 0b1000, // tso fence
                            })
                        }
                    }
                } else if mnemonics[1] == "i" {
                    if operands.len() != 0 {
                        Err("fence.i requires 0 operands".to_owned())
                    } else {
                        Ok(Instruction::FENCEI)
                    }
                } else {
                    Err("invalid fence".to_owned())
                }
            }
            // LR can't use `amo_assemble!` because it only has two operands
            "lr" => {
                if mnemonics.len() == 1 {
                    Err("lr must have size (w/d)".to_owned())
                } else if mnemonics.len() == 2 {
                    if mnemonics[1] == "w" {
                        Ok(Instruction::LRW {
                            dest: IRegister::from_string(operands[0])?,
                            addr: IRegister::from_string(operands[1])?,
                            aq: false,
                            rl: false,
                        })
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD {
                            dest: IRegister::from_string(operands[0])?,
                            addr: IRegister::from_string(operands[1])?,
                            aq: false,
                            rl: false,
                        })
                    } else {
                        Err("size of lr isntruction must be word (w) or doubleword (d)".to_owned())
                    }
                } else if mnemonics.len() == 3 {
                    let (aq, rl) = match mnemonics[2] {
                        "" => (false, false),
                        "aq" => (true, false),
                        "rl" => (false, true),
                        "aqrl" => (true, true),
                        _ => return Err("ordering should be (aq)(rl)".to_owned()),
                    };
                    if mnemonics[1] == "w" {
                        Ok(Instruction::LRW {
                            dest: IRegister::from_string(operands[0])?,
                            addr: IRegister::from_string(operands[1])?,
                            aq,
                            rl,
                        })
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD {
                            dest: IRegister::from_string(operands[0])?,
                            addr: IRegister::from_string(operands[1])?,
                            aq,
                            rl,
                        })
                    } else {
                        Err("size of lr isntruction must be word (w) or doubleword (d)".to_owned())
                    }
                } else {
                    Err(
                        "lr instruction has too many suffixes, expected lr.size.ordering"
                            .to_owned(),
                    )
                }
            }
            "sc" => amo_assemble!(SC),
            "amoswap" => amo_assemble!(AMOSWAP),
            "amoadd" => amo_assemble!(AMOADD),
            "amoxor" => amo_assemble!(AMOXOR),
            "amoand" => amo_assemble!(AMOAND),
            "amoor" => amo_assemble!(AMOOR),
            "amomin" => amo_assemble!(AMOMIN),
            "amomax" => amo_assemble!(AMOMAX),
            "amominu" => amo_assemble!(AMOMINU),
            "amomaxu" => amo_assemble!(AMOMAXU),
            "flw" => {
                if operands.len() != 2 {
                    println!("{:?}", operands);
                    Err("flw instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::FLW {
                        dest: FRegister::try_from(operands[0])?,
                        base,
                        offset: IImmediate::try_from(offset)?,
                    })
                }
            }
            "fsw" => {
                if operands.len() != 2 {
                    println!("{:?}", operands);
                    Err("fsw instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::FSW {
                        base,
                        src: FRegister::try_from(operands[0])?,
                        offset: SImmediate::try_from(offset)?,
                    })
                }
            }
            "fsqrt" => {
                if operands.len() != 2 {
                    Err("fsqrt instruction requires 2 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FSQRTS {
                        dest: FRegister::try_from(operands[0])?,
                        src: FRegister::try_from(operands[1])?,
                        rm: RoundingMode::DYN,
                    })
                } else if mnemonics.len() == 3 {
                    Ok(Instruction::FSQRTS {
                        dest: FRegister::try_from(operands[0])?,
                        src: FRegister::try_from(operands[1])?,
                        rm: RoundingMode::from_str(mnemonics[2])?,
                    })
                } else {
                    Err("fsqrt instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fadd" => fr_assemble!(FADD),
            "fsub" => fr_assemble!(FSUB),
            "fmul" => fr_assemble!(FMUL),
            "fdiv" => fr_assemble!(FDIV),
            "fmin" => {
                if operands.len() != 3 {
                    Err("fmin instruction requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FMINS {
                        dest: FRegister::try_from(operands[0])?,
                        src1: FRegister::try_from(operands[1])?,
                        src2: FRegister::try_from(operands[2])?,
                    })
                } else {
                    Err("fmin instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fmax" => {
                if operands.len() != 3 {
                    Err("fmax instruction requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    Ok(Instruction::FMAXS {
                        dest: FRegister::try_from(operands[0])?,
                        src1: FRegister::try_from(operands[1])?,
                        src2: FRegister::try_from(operands[2])?,
                    })
                } else {
                    Err("fmax instruction requires a suffix {s,d}".to_owned())
                }
            }
            "fcvt" => {
                if operands.len() != 2 {
                    Err("fcvt requires 3 operands".to_owned())
                } else if mnemonics.len() == 3 {
                    // default rounding mode
                    match (mnemonics[1], mnemonics[2]) {
                        ("w", "s") => Ok(Instruction::FCVTWS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("wu", "s") => Ok(Instruction::FCVTWUS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("s", "w") => Ok(Instruction::FCVTSW {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("s", "wu") => Ok(Instruction::FCVTSWU {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("l", "s") => Ok(Instruction::FCVTLS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("lu", "s") => Ok(Instruction::FCVTLUS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("s", "l") => Ok(Instruction::FCVTSL {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        ("s", "lu") => Ok(Instruction::FCVTSLU {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::DYN,
                        }),
                        _ => Err("invalid fcvt suffixes".to_owned()),
                    }
                } else if mnemonics.len() == 4 {
                    match (mnemonics[1], mnemonics[2]) {
                        ("w", "s") => Ok(Instruction::FCVTWS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("wu", "s") => Ok(Instruction::FCVTWUS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("s", "w") => Ok(Instruction::FCVTSW {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("s", "wu") => Ok(Instruction::FCVTSWU {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("l", "s") => Ok(Instruction::FCVTLS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("lu", "s") => Ok(Instruction::FCVTLUS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("s", "l") => Ok(Instruction::FCVTSL {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        ("s", "lu") => Ok(Instruction::FCVTSLU {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                            rm: RoundingMode::from_str(mnemonics[3])?,
                        }),
                        _ => Err("invalid fcvt suffixes".to_owned()),
                    }
                } else {
                    Err("fcvt should have 2 or 3 suffixes".to_owned())
                }
            }
            "fmv" => {
                if operands.len() != 2 {
                    Err("fmv requires 2 operands".to_owned())
                } else if mnemonics.len() == 3 {
                    match (mnemonics[1], mnemonics[2]) {
                        ("x", "w") => Ok(Instruction::FMVXW {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                        }),
                        ("w", "x") => Ok(Instruction::FMVWX {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::from_string(operands[1])?,
                        }),
                        _ => Err("invalid fmv suffixes".to_owned()),
                    }
                } else {
                    Err("fmv requires 2 suffixes".to_owned())
                }
            }
            "feq" => {
                if operands.len() != 3 {
                    Err("feq requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FEQS {
                            dest: IRegister::from_string(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("feq requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("feq requires a suffix {s,d}".to_owned())
                }
            }
            "flt" => {
                if operands.len() != 3 {
                    Err("flt requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FLTS {
                            dest: IRegister::from_string(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("flt requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("flt requires a suffix {s,d}".to_owned())
                }
            }
            "fle" => {
                if operands.len() != 3 {
                    Err("fle requires 3 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FLES {
                            dest: IRegister::from_string(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("fle requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("fle requires a suffix {s,d}".to_owned())
                }
            }
            "fclass" => {
                if operands.len() != 2 {
                    Err("fclass requires 2 operands".to_owned())
                } else if mnemonics.len() == 2 {
                    match mnemonics[1] {
                        "s" => Ok(Instruction::FCLASSS {
                            dest: IRegister::from_string(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                        }),
                        "d" => todo!(),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("fle requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("fle requires a suffix {s,d}".to_owned())
                }
            }
            "csrrw" => {
                if operands.len() != 3 {
                    Err("csrrw requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRW {
                        dest: IRegister::from_string(operands[0])?,
                        src: IRegister::from_string(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrs" => {
                if operands.len() != 3 {
                    Err("csrrs requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRS {
                        dest: IRegister::from_string(operands[0])?,
                        src: IRegister::from_string(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrc" => {
                if operands.len() != 3 {
                    Err("csrrc requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRC {
                        dest: IRegister::from_string(operands[0])?,
                        src: IRegister::from_string(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrwi" => {
                if operands.len() != 3 {
                    Err("csrrwi requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRWI {
                        dest: IRegister::from_string(operands[0])?,
                        imm: CSRImmediate::try_from(parse_int(operands[2])?)?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrsi" => {
                if operands.len() != 3 {
                    Err("csrrsi requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRSI {
                        dest: IRegister::from_string(operands[0])?,
                        imm: CSRImmediate::try_from(parse_int(operands[2])?)?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrci" => {
                if operands.len() != 3 {
                    Err("csrrci requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRCI {
                        dest: IRegister::from_string(operands[0])?,
                        imm: CSRImmediate::try_from(parse_int(operands[2])?)?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            _ => Err(format!("unknown mnemonic: {}", mnemonic)),
        };
        x.map(AssemblyResult::I)
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

    let frd = FRegister::try_from((instruction >> 7) & 0b1_1111).unwrap();
    let frs1 = FRegister::try_from((instruction >> 15) & 0b1_1111).unwrap();
    let frs2 = FRegister::try_from((instruction >> 20) & 0b1_1111).unwrap();
    let frs3 = FRegister::try_from((instruction >> 27) & 0b1_1111).unwrap();

    let i_immediate: IImmediate = IImmediate::from_u32(instruction);

    let s_immediate: SImmediate = SImmediate::from_u32(instruction);

    let u_immediate = UImmediate::from_u32(instruction);

    let b_immediate = BImmediate::from_u32(instruction);

    let shamt: Shamt = Shamt::from_u32(instruction);

    let shamtw: ShamtW = ShamtW::from_u32(instruction);

    // aq is bit 26, rl is bit 25
    let aq: bool = ((instruction >> 26) & 0b1) == 0b1;
    let rl: bool = ((instruction >> 25) & 0b1) == 0b1;

    match opcode {
        Opcode::Load => match func3 {
            0b000 => Ok(Instruction::LB {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b001 => Ok(Instruction::LH {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b010 => Ok(Instruction::LW {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b011 => Ok(Instruction::LD {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b100 => Ok(Instruction::LBU {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b101 => Ok(Instruction::LHU {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b110 => Ok(Instruction::LWU {
                dest: rd,
                base: rs1,
                offset: i_immediate,
            }),
            0b111 => Err("Invalid load func3".to_owned()),
            _ => unreachable!(),
        },
        Opcode::Auipc => Ok(Instruction::AUIPC {
            dest: rd,
            imm: u_immediate,
        }),
        Opcode::Store => match func3 {
            0b000 => Ok(Instruction::SB {
                src: rs2,
                base: rs1,
                offset: s_immediate,
            }),
            0b001 => Ok(Instruction::SH {
                src: rs2,
                base: rs1,
                offset: s_immediate,
            }),
            0b010 => Ok(Instruction::SW {
                src: rs2,
                base: rs1,
                offset: s_immediate,
            }),
            0b011 => Ok(Instruction::SD {
                src: rs2,
                base: rs1,
                offset: s_immediate,
            }),
            x => Err(format!("invalid store func3: {}", x)),
        },
        Opcode::Lui => Ok(Instruction::LUI {
            dest: rd,
            imm: u_immediate,
        }),
        Opcode::Op => match (func7, func3) {
            (0b000_0000, 0b000) => Ok(Instruction::ADD {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b001) => Ok(Instruction::SLL {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b010) => Ok(Instruction::SLT {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b011) => Ok(Instruction::SLTU {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b100) => Ok(Instruction::XOR {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b101) => Ok(Instruction::SRL {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b110) => Ok(Instruction::OR {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0000, 0b111) => Ok(Instruction::AND {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b010_0000, 0b000) => Ok(Instruction::SUB {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b010_0000, 0b101) => Ok(Instruction::SRA {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b000) => Ok(Instruction::MUL {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b001) => Ok(Instruction::MULH {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b010) => Ok(Instruction::MULHSU {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b011) => Ok(Instruction::MULHU {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b100) => Ok(Instruction::DIV {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b101) => Ok(Instruction::DIVU {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b110) => Ok(Instruction::REM {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000_0001, 0b111) => Ok(Instruction::REMU {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            _ => Err(format!("unknown Op. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::Op32 => match (func3, func7) {
            (0b000, 0b000_0000) => Ok(Instruction::ADDW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000, 0b000_0001) => Ok(Instruction::MULW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b000, 0b010_0000) => Ok(Instruction::SUBW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b001, 0b000_0000) => Ok(Instruction::SLLW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b100, 0b0000_001) => Ok(Instruction::DIVW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b101, 0b000_0000) => Ok(Instruction::SRLW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b101, 0b000_0001) => Ok(Instruction::DIVUW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b101, 0b010_0000) => Ok(Instruction::SRAW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b110, 0b000_0001) => Ok(Instruction::REMW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            (0b111, 0b000_0001) => Ok(Instruction::REMUW {
                dest: rd,
                src1: rs1,
                src2: rs2,
            }),
            _ => Err(format!("unknown Op32. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm => match func3 {
            0b000 => Ok(Instruction::ADDI {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            // SLLi requires special handling because shamt uses the bottom bit of func7
            0b001 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SLLI {
                    dest: rd,
                    src: rs1,
                    shamt,
                }),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b010 => Ok(Instruction::SLTI {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            0b011 => Ok(Instruction::SLTIU {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            0b100 => Ok(Instruction::XORI {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            // SRLI SRAI require special handling because shamt uses the bottom bit of func7
            0b101 => match func7 | 0b1 {
                0b000000_1 => Ok(Instruction::SRLI {
                    dest: rd,
                    src: rs1,
                    shamt,
                }),
                0b010000_1 => Ok(Instruction::SRAI {
                    dest: rd,
                    src: rs1,
                    shamt,
                }),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            0b110 => Ok(Instruction::ORI {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            0b111 => Ok(Instruction::ANDI {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
        },
        Opcode::OpImm32 => match func3 {
            0b000 => Ok(Instruction::ADDIW {
                dest: rd,
                src: rs1,
                imm: i_immediate,
            }),
            0b001 => Ok(Instruction::SLLIW {
                dest: rd,
                src: rs1,
                shamt: shamtw,
            }),
            0b101 => match func7 {
                0b000_0000 => Ok(Instruction::SRLIW {
                    dest: rd,
                    src: rs1,
                    shamt: shamtw,
                }),
                0b010_0000 => Ok(Instruction::SRAIW {
                    dest: rd,
                    src: rs1,
                    shamt: shamtw,
                }),
                x => Err(format!("unknown OpImm32(101) func7: {}", x).to_owned()),
            },
            x => Err(format!("unkown OpImm32 func3: {}", x).to_owned()),
        },
        Opcode::Jalr => Ok(Instruction::JALR {
            dest: rd,
            base: rs1,
            offset: i_immediate,
        }),
        Opcode::Jal => Ok(Instruction::JAL {
            dest: rd,
            offset: JImmediate::from_u32(instruction),
        }),
        Opcode::Branch => match func3 {
            0b000 => Ok(Instruction::BEQ {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
            0b001 => Ok(Instruction::BNE {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
            0b100 => Ok(Instruction::BLT {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
            0b101 => Ok(Instruction::BGE {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
            0b110 => Ok(Instruction::BLTU {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
            0b111 => Ok(Instruction::BGEU {
                src1: rs1,
                src2: rs2,
                offset: b_immediate,
            }),
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
                        Ok(Instruction::FENCE {
                            rd,
                            rs1,
                            ops: ((instruction >> 20) & 0xFF) as u8,
                            fm: ((instruction >> 28) & 0b1111) as u8,
                        })
                    }
                }
            }
            0b001 => {
                if rd != IRegister::Zero || rs1 != IRegister::Zero {
                    // technicially, we are supposed to ignore these fields
                    Err("reserved register fields not set to zero".to_owned())
                } else {
                    let func12 = instruction >> 20;
                    if func12 != 0 {
                        Err("reserved register fields not set to zero".to_owned())
                    } else {
                        Ok(Instruction::FENCEI)
                    }
                }
            }
            x => Err(format!("unknown fence func3: {x}")),
        },
        Opcode::AMO => match (func3, func7 >> 2) {
            (0b010, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.W expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRW {
                        dest: rd,
                        addr: rs1,
                        aq,
                        rl,
                    })
                }
            }
            (0b011, 0b00010) => {
                if rs2 != IRegister::Zero {
                    Err("LR.D expects rs2 to be 0".to_owned())
                } else {
                    Ok(Instruction::LRD {
                        dest: rd,
                        addr: rs1,
                        aq,
                        rl,
                    })
                }
            }
            (0b010, 0b00011) => Ok(Instruction::SCW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b00011) => Ok(Instruction::SCD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b00001) => Ok(Instruction::AMOSWAPW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b00001) => Ok(Instruction::AMOSWAPD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b00000) => Ok(Instruction::AMOADDW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b00000) => Ok(Instruction::AMOADDD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b00100) => Ok(Instruction::AMOXORW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b00100) => Ok(Instruction::AMOXORD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b01100) => Ok(Instruction::AMOANDW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b01100) => Ok(Instruction::AMOANDD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b01000) => Ok(Instruction::AMOORW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b01000) => Ok(Instruction::AMOORD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b10000) => Ok(Instruction::AMOMINW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b10000) => Ok(Instruction::AMOMIND {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b10100) => Ok(Instruction::AMOMAXW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b10100) => Ok(Instruction::AMOMAXD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b11000) => Ok(Instruction::AMOMINUW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b11000) => Ok(Instruction::AMOMINUD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b010, 0b11100) => Ok(Instruction::AMOMAXUW {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            (0b011, 0b11100) => Ok(Instruction::AMOMAXUD {
                dest: rd,
                addr: rs1,
                src: rs2,
                aq,
                rl,
            }),
            _ => Err(format!("unknown AMO. func3: {func3}, func7: {func7}")),
        },
        Opcode::LoadFp => {
            println!("{i_immediate}, {:b}", instruction);
            if func3 == 0b010 {
                Ok(Instruction::FLW {
                    dest: frd,
                    base: rs1,
                    offset: i_immediate,
                })
            } else {
                Err(format!("unknown func3: {func3} in opcode LoadFp"))
            }
        }
        Opcode::StoreFp => {
            if func3 == 0b010 {
                Ok(Instruction::FSW {
                    base: rs1,
                    src: frs2,
                    offset: s_immediate,
                })
            } else {
                Err(format!("unknown func3: {func3} in opcode LoadFp"))
            }
        }
        Opcode::OpFp => match func7 {
            0b000_0000 => Ok(Instruction::FADDS {
                dest: frd,
                src1: frs1,
                src2: frs2,
                rm: RoundingMode::from_int(func3)?,
            }),
            0b000_0100 => Ok(Instruction::FSUBS {
                dest: frd,
                src1: frs1,
                src2: frs2,
                rm: RoundingMode::from_int(func3)?,
            }),
            0b000_1000 => Ok(Instruction::FMULS {
                dest: frd,
                src1: frs1,
                src2: frs2,
                rm: RoundingMode::from_int(func3)?,
            }),
            0b000_1100 => Ok(Instruction::FDIVS {
                dest: frd,
                src1: frs1,
                src2: frs2,
                rm: RoundingMode::from_int(func3)?,
            }),
            0b010_1100 => Ok(Instruction::FSQRTS {
                dest: frd,
                src: frs1,
                rm: RoundingMode::from_int(func3)?,
            }),
            0b001_0000 => match func3 {
                0b000 => Ok(Instruction::FSGNJS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                }),
                0b001 => Ok(Instruction::FSGNJNS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                }),
                0b010 => Ok(Instruction::FSGNJXS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                }),
                x => Err(format!("unknown OpFp func7=0b001_0000 func3: {}", x)),
            },
            0b001_0100 => match func3 {
                0b000 => Ok(Instruction::FMINS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                }),
                0b001 => Ok(Instruction::FMAXS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                }),
                x => Err(format!("unknown OpFp func7=0b001_0100 func3: {}", x)),
            },
            0b101_0000 => match func3 {
                0b000 => Ok(Instruction::FLES {
                    dest: rd,
                    src1: frs1,
                    src2: frs2,
                }),
                0b001 => Ok(Instruction::FLTS {
                    dest: rd,
                    src1: frs1,
                    src2: frs2,
                }),
                0b010 => Ok(Instruction::FEQS {
                    dest: rd,
                    src1: frs1,
                    src2: frs2,
                }),
                x => Err(format!("unknown OpFp func7=0b101_0000 func3: {}", x)),
            },
            0b110_0000 => match (instruction >> 20) & 0b1_1111 {
                0b0_0000 => Ok(Instruction::FCVTWS {
                    dest: rd,
                    src: frs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0001 => Ok(Instruction::FCVTWUS {
                    dest: rd,
                    src: frs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0010 => Ok(Instruction::FCVTLS {
                    dest: rd,
                    src: frs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0011 => Ok(Instruction::FCVTLUS {
                    dest: rd,
                    src: frs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
            },
            0b110_1000 => match (instruction >> 20) & 0b1_1111 {
                0b0_0000 => Ok(Instruction::FCVTSW {
                    dest: frd,
                    src: rs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0001 => Ok(Instruction::FCVTSWU {
                    dest: frd,
                    src: rs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0010 => Ok(Instruction::FCVTSL {
                    dest: frd,
                    src: rs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b0_0011 => Ok(Instruction::FCVTSLU {
                    dest: frd,
                    src: rs1,
                    rm: RoundingMode::from_int(func3)?,
                }),
                x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
            },
            0b111_0000 => {
                if (instruction >> 20) & 0b1_1111 == 0 {
                    if func3 == 0 {
                        Ok(Instruction::FMVXW {
                            dest: rd,
                            src: frs1,
                        })
                    } else if func3 == 1 {
                        Ok(Instruction::FCLASSS {
                            dest: rd,
                            src: frs1,
                        })
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_0000 rs2=0 func3: {}",
                            func3
                        ))
                    }
                } else {
                    Err(format!(
                        "unknown OpFp func7=0b111_0000 unknown rs2: {} and func3: {}",
                        (instruction >> 20) & 0b1_1111,
                        func3
                    ))
                }
            }
            0b111_1000 => {
                if (instruction >> 20) & 0b1_1111 == 0 {
                    if func3 == 0 {
                        Ok(Instruction::FMVWX {
                            dest: frd,
                            src: rs1,
                        })
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_1000 rs2=0 func3: {}",
                            func3
                        ))
                    }
                } else {
                    Err(format!(
                        "unknown OpFp func7=0b111_0000 unknown rs2: {} and func3: {}",
                        (instruction >> 20) & 0b1_1111,
                        func3
                    ))
                }
            }
            x => Err(format!("Unknown OpFp func7: {x}")),
        },
        Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
        Opcode::Madd => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FMADDS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                })
            } else {
                Err(format!(
                    "FMADD unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Msub => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FMSUBS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                })
            } else {
                Err(format!(
                    "FMSUB unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Nmsub => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FNMSUBS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                })
            } else {
                Err(format!(
                    "FMNSUB unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::Nmadd => {
            if func7 & 0b11 == 0 {
                Ok(Instruction::FNMADDS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                })
            } else {
                Err(format!(
                    "FNMADD unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                ))
            }
        }
        Opcode::System => match func3 {
            0b000 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
            0b001 => Ok(Instruction::CSRRW {
                dest: rd,
                src: rs1,
                csr: CSR::from_u32(instruction),
            }),
            0b010 => Ok(Instruction::CSRRS {
                dest: rd,
                src: rs1,
                csr: CSR::from_u32(instruction),
            }),
            0b011 => Ok(Instruction::CSRRC {
                dest: rd,
                src: rs1,
                csr: CSR::from_u32(instruction),
            }),
            0b100 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
            0b101 => Ok(Instruction::CSRRWI {
                dest: rd,
                imm: CSRImmediate::from_u32(instruction),
                csr: CSR::from_u32(instruction),
            }),
            0b110 => Ok(Instruction::CSRRSI {
                dest: rd,
                imm: CSRImmediate::from_u32(instruction),
                csr: CSR::from_u32(instruction),
            }),
            0b111 => Ok(Instruction::CSRRCI {
                dest: rd,
                imm: CSRImmediate::from_u32(instruction),
                csr: CSR::from_u32(instruction),
            }),
            _ => unreachable!(),
        },
    }
}

pub fn encode_instruction(instruction: &Instruction) -> u32 {
    match instruction {
        Instruction::LUI { dest, imm } => imm.to_u32() | dest.rd() | 0b0110111,
        Instruction::AUIPC { dest, imm } => imm.to_u32() | dest.rd() | 0b0010111,
        Instruction::JAL { dest, offset } => offset.to_u32() | dest.rd() | 0b1101111,
        Instruction::JALR { dest, base, offset } => {
            offset.to_u32() | base.rs1() | dest.rd() | 0b1100111
        }
        Instruction::BEQ { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b000 << 12 | 0b1100011
        }
        Instruction::BNE { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b001 << 12 | 0b1100011
        }
        Instruction::BLT { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b100 << 12 | 0b1100011
        }
        Instruction::BGE { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b101 << 12 | 0b1100011
        }
        Instruction::BLTU { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b110 << 12 | 0b1100011
        }
        Instruction::BGEU { src1, src2, offset } => {
            offset.to_u32() | src2.rs2() | src1.rs1() | 0b111 << 12 | 0b1100011
        }
        Instruction::LB { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b000 << 12 | dest.rd() | 0b0000011
        }
        Instruction::LH { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b001 << 12 | dest.rd() | 0b0000011
        }
        Instruction::LW { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b010 << 12 | dest.rd() | 0b0000011
        }
        Instruction::LBU { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b100 << 12 | dest.rd() | 0b0000011
        }
        Instruction::LHU { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b101 << 12 | dest.rd() | 0b0000011
        }
        Instruction::SB { src, base, offset } => {
            offset.to_u32() | src.rs2() | base.rs1() | 0b000 << 12 | 0b0100011
        }
        Instruction::SH { src, base, offset } => {
            offset.to_u32() | src.rs2() | base.rs1() | 0b001 << 12 | 0b0100011
        }
        Instruction::SW { src, base, offset } => {
            offset.to_u32() | src.rs2() | base.rs1() | 0b010 << 12 | 0b0100011
        }
        Instruction::ADDI { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b000 << 12 | dest.rd() | 0b0010011
        }
        Instruction::SLTI { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b010 << 12 | dest.rd() | 0b0010011
        }
        Instruction::SLTIU { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b011 << 12 | dest.rd() | 0b0010011
        }
        Instruction::XORI { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b100 << 12 | dest.rd() | 0b0010011
        }
        Instruction::ORI { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b110 << 12 | dest.rd() | 0b0010011
        }
        Instruction::ANDI { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b111 << 12 | dest.rd() | 0b0010011
        }
        Instruction::SLLI { dest, src, shamt } => {
            shamt.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b0010011
        }
        Instruction::SRLI { dest, src, shamt } => {
            shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0010011
        }
        Instruction::SRAI { dest, src, shamt } => {
            0b0100000 << 25 | shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0010011
        }
        Instruction::ADD { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SUB { dest, src1, src2 } => {
            0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SLL { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SLT { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SLTU { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b011 << 12 | dest.rd() | 0b0110011
        }
        Instruction::XOR { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SRL { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0110011
        }
        Instruction::SRA { dest, src1, src2 } => {
            0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b0101 << 12 | dest.rd() | 0b0110011
        }
        Instruction::OR { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0110011
        }
        Instruction::AND { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0110011
        }
        Instruction::FENCE { rd, rs1, ops, fm } => todo!(),
        Instruction::ECALL => 0b1110011,
        Instruction::EBREAK => 0b1 << 20 | 0b1110011,
        Instruction::LWU { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b110 << 12 | dest.rd() | 0b0000011
        }
        Instruction::LD { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b011 << 12 | dest.rd() | 0b0000011
        }
        Instruction::SD { src, base, offset } => {
            offset.to_u32() | src.rs2() | base.rs1() | 0b011 << 12 | 0b0100011
        }
        Instruction::ADDIW { dest, src, imm } => {
            imm.to_u32() | src.rs1() | 0b000 << 12 | dest.rd() | 0b0011011
        }
        Instruction::SLLIW { dest, src, shamt } => {
            shamt.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b0011011
        }
        Instruction::SRLIW { dest, src, shamt } => {
            shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0011011
        }
        Instruction::SRAIW { dest, src, shamt } => {
            0b0100000 << 25 | shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0011011
        }
        Instruction::ADDW { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
        }
        Instruction::SUBW { dest, src1, src2 } => {
            0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
        }
        Instruction::SLLW { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0111011
        }
        Instruction::SRLW { dest, src1, src2 } => {
            src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
        }
        Instruction::SRAW { dest, src1, src2 } => {
            0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
        }
        Instruction::MUL { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
        }
        Instruction::MULH { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0110011
        }
        Instruction::MULHSU { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b0110011
        }
        Instruction::MULHU { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b011 << 12 | dest.rd() | 0b0110011
        }
        Instruction::DIV { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0110011
        }
        Instruction::DIVU { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0110011
        }
        Instruction::REM { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0110011
        }
        Instruction::REMU { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0110011
        }
        Instruction::MULW { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
        }
        Instruction::DIVW { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0111011
        }
        Instruction::DIVUW { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
        }
        Instruction::REMW { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0111011
        }
        Instruction::REMUW { dest, src1, src2 } => {
            0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0111011
        }
        Instruction::LRW { dest, addr, aq, rl } => {
            0b00010 << 27 | aqb(*aq) | rlb(*rl) | addr.rs1() | 0b010 << 12 | dest.rd() | 0b0101111
        }
        Instruction::SCW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00011 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOSWAPW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00001 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOADDW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOXORW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOANDW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b01100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOORW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b01000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMINW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b10000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMAXW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b10100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMINUW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b11000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMAXUW {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b11100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b010 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::LRD { dest, addr, aq, rl } => {
            0b00010 << 27 | aqb(*aq) | rlb(*rl) | addr.rs1() | 0b011 << 12 | dest.rd() | 0b0101111
        }
        Instruction::SCD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00011 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOSWAPD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00001 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOADDD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOXORD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b00100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOANDD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b01100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOORD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b01000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMIND {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b10000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMAXD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b10100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMINUD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b11000 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::AMOMAXUD {
            dest,
            addr,
            src,
            aq,
            rl,
        } => {
            0b11100 << 27
                | aqb(*aq)
                | rlb(*rl)
                | src.rs2()
                | addr.rs1()
                | 0b011 << 12
                | dest.rd()
                | 0b0101111
        }
        Instruction::FLW { dest, base, offset } => {
            offset.to_u32() | base.rs1() | 0b010 << 12 | dest.rd() | 0b0000111
        }
        Instruction::FSW { base, src, offset } => {
            offset.to_u32() | src.rs2() | base.rs1() | 0b010 << 12 | 0b0100111
        }
        Instruction::FMADDS {
            dest,
            src1,
            src2,
            src3,
            rm,
        } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1000011,
        Instruction::FMSUBS {
            dest,
            src1,
            src2,
            src3,
            rm,
        } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1000111,
        Instruction::FNMSUBS {
            dest,
            src1,
            src2,
            src3,
            rm,
        } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1001011,
        Instruction::FNMADDS {
            dest,
            src1,
            src2,
            src3,
            rm,
        } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1001111,
        Instruction::FADDS {
            dest,
            src1,
            src2,
            rm,
        } => 0b0000000 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
        Instruction::FSUBS {
            dest,
            src1,
            src2,
            rm,
        } => 0b0000100 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
        Instruction::FMULS {
            dest,
            src1,
            src2,
            rm,
        } => 0b0001000 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
        Instruction::FDIVS {
            dest,
            src1,
            src2,
            rm,
        } => 0b0001100 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
        Instruction::FSQRTS { dest, src, rm } => {
            0b0101100 << 25 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FSGNJS { dest, src1, src2 } => {
            0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
        }

        Instruction::FSGNJNS { dest, src1, src2 } => {
            0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FSGNJXS { dest, src1, src2 } => {
            0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FMINS { dest, src1, src2 } => {
            0b0010100 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FMAXS { dest, src1, src2 } => {
            0b0010100 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FCVTWS { dest, src, rm } => {
            0b1100000 << 25 | 0b00000 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FCVTWUS { dest, src, rm } => {
            0b1100000 << 25 | 0b00001 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FMVXW { dest, src } => 0b1110000 << 25 | src.rs1() | dest.rd() | 0b1010011,
        Instruction::FEQS { dest, src1, src2 } => {
            0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FLTS { dest, src1, src2 } => {
            0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FLES { dest, src1, src2 } => {
            0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FCLASSS { dest, src } => {
            0b1110000 << 25 | src.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
        }
        Instruction::FCVTSW { dest, src, rm } => {
            0b1101000 << 25 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FCVTSWU { dest, src, rm } => {
            0b1101000 << 25 | 0b00001 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FMVWX { dest, src } => 0b1111000 << 25 | src.rs1() | dest.rd() | 0b1010011,
        Instruction::FCVTLS { dest, src, rm } => {
            0b1100000 << 25 | 0b00010 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FCVTLUS { dest, src, rm } => {
            0b1100000 << 25 | 0b00011 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FCVTSL { dest, src, rm } => {
            0b1101000 << 25 | 0b00010 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::FCVTSLU { dest, src, rm } => {
            0b1101000 << 25 | 0b00011 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
        }
        Instruction::CSRRW { dest, src, csr } => {
            csr.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b1110011
        }
        Instruction::CSRRS { dest, src, csr } => {
            csr.to_u32() | src.rs1() | 0b010 << 12 | dest.rd() | 0b1110011
        }
        Instruction::CSRRC { dest, src, csr } => {
            csr.to_u32() | src.rs1() | 0b011 << 12 | dest.rd() | 0b1110011
        }
        Instruction::CSRRWI { dest, imm, csr } => {
            csr.to_u32() | imm.to_u32() | 0b101 << 12 | dest.rd() | 0b1110011
        }
        Instruction::CSRRSI { dest, imm, csr } => {
            csr.to_u32() | imm.to_u32() | 0b110 << 12 | dest.rd() | 0b1110011
        }
        Instruction::CSRRCI { dest, imm, csr } => {
            csr.to_u32() | imm.to_u32() | 0b111 << 12 | dest.rd() | 0b1110011
        }
        Instruction::FENCEI => 0b001 << 12 | 0b0001111,
    }
}
