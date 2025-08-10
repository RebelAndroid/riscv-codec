use crate::immediates::{
    BImmediate, CSR, CSRImmediate, JImmediate, SImmediate, Shamt, ShamtW, UImmediate,
};
use crate::register::{FRegister, IRegister};
use crate::{immediates::IImmediate, opcode::Opcode};
use alloc::borrow::ToOwned;
use alloc::fmt::{Display, Formatter};
use alloc::format;
use alloc::string::String;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), alloc::fmt::Error> {
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
    Lui {
        dest: IRegister,
        imm: UImmediate,
    },
    /// Add upper immediate to PC
    Auipc {
        dest: IRegister,
        imm: UImmediate,
    },
    /// Jump and Link
    Jal {
        dest: IRegister,
        offset: JImmediate,
    },
    /// Jump and Link Register
    Jalr {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    Beq {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    Bne {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    Blt {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    Bge {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    Bltu {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    Bgeu {
        src1: IRegister,
        src2: IRegister,
        offset: BImmediate,
    },
    /// Load Byte
    Lb {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Halfword
    Lh {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Word
    Lw {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Byte Unsigned
    Lbu {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Halfword Unsigned
    Lhu {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Store Byte
    Sb {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Store Halfword
    Sh {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Store Word
    Sw {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    Addi {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    Slti {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    Sltiu {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    Xori {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    Ori {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    Andi {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    /// Left Shift Immediate
    Slli {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    /// Logical Right Shift Immediate
    Srli {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    /// Arithmetic Right Shift Immediate
    Srai {
        dest: IRegister,
        src: IRegister,
        shamt: Shamt,
    },
    Add {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Sub {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Left Shift
    Sll {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Slt {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Sltu {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Xor {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Logical Right Shift Immediate
    Srl {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Arithmetic Right Shift Immediate
    Sra {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Or {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    And {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    Fence {
        rd: IRegister,
        rs1: IRegister,
        ops: u8,
        fm: u8,
    },
    Ecall,
    Ebreak,
    //
    // Instructions Added In RV64I
    //
    /// Load Word Unsigned
    Lwu {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Load Doubleword
    Ld {
        dest: IRegister,
        base: IRegister,
        offset: IImmediate,
    },
    /// Store Doubleword
    Sd {
        src: IRegister,
        base: IRegister,
        offset: SImmediate,
    },
    /// Add Immediate (word)
    Addiw {
        dest: IRegister,
        src: IRegister,
        imm: IImmediate,
    },
    /// Left Shift Immediate (word)
    Slliw {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Logical Right Shift Immediate (word)
    Srliw {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Arithmetic Right Shift Immediate (word)
    Sraiw {
        dest: IRegister,
        src: IRegister,
        shamt: ShamtW,
    },
    /// Add (word)
    Addw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Subtract (word)
    Subw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Left Shift (word)
    Sllw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Logical Right Shift (word)
    Srlw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Arithmetic Right Shift (word)
    Sraw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    //
    // Instructions In M Extension
    //
    /// Multiply
    Mul {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply (High bits)
    Mulh {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Signed-Unsigned (High bits)
    Mulhsu {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Unsigned (High)
    Mulhu {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide
    Div {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide (Unsigned)
    Divu {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder
    Rem {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder (Unsigned)
    Remu {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Multiply Word
    Mulw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide Word
    Divw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Divide Unsigned Word
    Divuw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder Word
    Remw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    /// Remainder Unsigned Word
    Remuw {
        dest: IRegister,
        src1: IRegister,
        src2: IRegister,
    },
    //
    // Instructions In A Extension
    //
    /// Load Reserved Word
    // rd, rs1, ac, rl
    LrW {
        dest: IRegister,
        addr: IRegister,
        aq: bool,
        rl: bool,
    },
    ScW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoswapW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoaddW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoxorW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoandW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoorW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmominW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmomaxW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmominuW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmomaxuW {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    //
    LrD {
        dest: IRegister,
        addr: IRegister,
        aq: bool,
        rl: bool,
    },
    ScD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoswapD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoaddD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoxorD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoandD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmoorD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmominD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmomaxD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmominuD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    AmomaxuD {
        dest: IRegister,
        addr: IRegister,
        src: IRegister,
        aq: bool,
        rl: bool,
    },
    //
    // Instructions in F Extension
    //
    Flw {
        dest: FRegister,
        base: IRegister,
        offset: IImmediate,
    },
    Fsw {
        base: IRegister,
        src: FRegister,
        offset: SImmediate,
    },
    FmaddS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FmsubS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FnmsubS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FnmaddS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FaddS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FsubS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FmulS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FdivS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FsqrtS {
        dest: FRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FsgnjS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FsgnjnS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FsgnjxS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FminS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FmaxS {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FcvtWS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FcvtWuS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FmvXW {
        dest: IRegister,
        src: FRegister,
    },
    FeqS {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FltS {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FleS {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FclassS {
        dest: IRegister,
        src: FRegister,
    },
    FcvtSW {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FcvtSWu {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FmvWX {
        dest: FRegister,
        src: IRegister,
    },
    //
    // Instructions in F Extension (RV64)
    //
    FcvtLS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FcvtLuS {
        dest: IRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FcvtSL {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FcvtSLu {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    //
    // Instructions in Zicsr Extension
    //
    Csrrw {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    Csrrs {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    Csrrc {
        dest: IRegister,
        src: IRegister,
        csr: CSR,
    },
    Csrrwi {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    Csrrsi {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    Csrrci {
        dest: IRegister,
        imm: CSRImmediate,
        csr: CSR,
    },
    //
    // Instructions in Zifencei Extension
    //
    FenceI,
    //
    // Instructions in D Extension
    //
    Fld {
        dest: FRegister,
        base: IRegister,
        offset: IImmediate,
    },
    Fsd {
        src: FRegister,
        base: IRegister,
        offset: SImmediate,
    },
    FmaddD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FmsubD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FnmaddD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FnmsubD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        src3: FRegister,
        rm: RoundingMode,
    },
    FaddD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FsubD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FmulD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FdivD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
        rm: RoundingMode,
    },
    FsqrtD {
        dest: FRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FsgnjD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FsgnjnD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FsgnjxD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FminD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FmaxD {
        dest: FRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FcvtSD {
        dest: FRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FcvtDS {
        dest: FRegister,
        src: FRegister,
        rm: RoundingMode,
    },
    FeqD {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FltD {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FleD {
        dest: IRegister,
        src1: FRegister,
        src2: FRegister,
    },
    FclassD {
        dest: IRegister,
        src1: FRegister,
    },
    FcvtWD {
        dest: IRegister,
        src1: FRegister,
        rm: RoundingMode,
    },
    FcvtWuD {
        dest: IRegister,
        src1: FRegister,
        rm: RoundingMode,
    },
    FcvtDW {
        dest: FRegister,
        src1: IRegister,
        rm: RoundingMode,
    },
    FcvtDWu {
        dest: FRegister,
        src1: IRegister,
        rm: RoundingMode,
    },
    FcvtLD {
        dest: IRegister,
        src1: FRegister,
        rm: RoundingMode,
    },
    FcvtLuD {
        dest: IRegister,
        src1: FRegister,
        rm: RoundingMode,
    },
    FmvXD {
        dest: IRegister,
        src: FRegister,
    },
    FcvtDL {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FcvtDLu {
        dest: FRegister,
        src: IRegister,
        rm: RoundingMode,
    },
    FmvDX {
        dest: FRegister,
        src: IRegister,
    },
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), alloc::fmt::Error> {
        match self {
            Instruction::Lui { dest, imm } => write!(f, "lui {dest},{imm}"),
            Instruction::Auipc { dest, imm } => write!(f, "auipc {dest},{imm}"),
            Instruction::Jal { dest, offset } => write!(f, "jal {dest},{offset}"),
            Instruction::Jalr { dest, base, offset } => write!(f, "jalr {dest},{offset}({base})"),
            Instruction::Beq { src1, src2, offset } => write!(f, "beq {src1},{src2},{offset}"),
            Instruction::Bne { src1, src2, offset } => write!(f, "bne {src1},{src2},{offset}"),
            Instruction::Blt { src1, src2, offset } => write!(f, "blt {src1},{src2},{offset}"),
            Instruction::Bge { src1, src2, offset } => write!(f, "bge {src1},{src2},{offset}"),
            Instruction::Bltu { src1, src2, offset } => write!(f, "bltu {src1},{src2},{offset}"),
            Instruction::Bgeu { src1, src2, offset } => write!(f, "bgeu {src1},{src2},{offset}"),
            Instruction::Lb { dest, base, offset } => write!(f, "lb {dest},{offset}({base})"),
            Instruction::Lh { dest, base, offset } => write!(f, "lh {dest},{offset}({base})"),
            Instruction::Lw { dest, base, offset } => write!(f, "lw {dest},{offset}({base})"),
            Instruction::Lbu { dest, base, offset } => write!(f, "lbu {dest},{offset}({base})"),
            Instruction::Lhu { dest, base, offset } => write!(f, "lhu {dest},{offset}({base})"),
            Instruction::Sb { src, base, offset } => write!(f, "sb {src},{offset}({base})"),
            Instruction::Sh { src, base, offset } => write!(f, "sh {src},{offset}({base})"),
            Instruction::Sw { src, base, offset } => write!(f, "sw {src},{offset}({base})"),
            Instruction::Addi { dest, src, imm } => write!(f, "addi {dest},{src},{imm}"),
            Instruction::Slti { dest, src, imm } => write!(f, "slti {dest},{src},{imm}"),
            Instruction::Sltiu { dest, src, imm } => write!(f, "sltiu {dest},{src},{imm}"),
            Instruction::Xori { dest, src, imm } => write!(f, "xori {dest},{src},{imm}"),
            Instruction::Ori { dest, src, imm } => write!(f, "ori {dest},{src},{imm}"),
            Instruction::Andi { dest, src, imm } => write!(f, "andi {dest},{src},{imm}"),
            Instruction::Slli { dest, src, shamt } => write!(f, "slli {dest},{src},{shamt}"),
            Instruction::Srli { dest, src, shamt } => write!(f, "srli {dest},{src},{shamt}"),
            Instruction::Srai { dest, src, shamt } => write!(f, "srai {dest},{src},{shamt}"),
            Instruction::Add { dest, src1, src2 } => write!(f, "add {dest},{src1},{src2}"),
            Instruction::Sub { dest, src1, src2 } => write!(f, "sub {dest},{src1},{src2}"),
            Instruction::Sll { dest, src1, src2 } => write!(f, "sll {dest},{src1},{src2}"),
            Instruction::Slt { dest, src1, src2 } => write!(f, "slt {dest},{src1},{src2}"),
            Instruction::Sltu { dest, src1, src2 } => write!(f, "sltu {dest},{src1},{src2}"),
            Instruction::Xor { dest, src1, src2 } => write!(f, "xor {dest},{src1},{src2}"),
            Instruction::Srl { dest, src1, src2 } => write!(f, "srl {dest},{src1},{src2}"),
            Instruction::Sra { dest, src1, src2 } => write!(f, "sra {dest},{src1},{src2}"),
            Instruction::Or { dest, src1, src2 } => write!(f, "or {dest},{src1},{src2}"),
            Instruction::And { dest, src1, src2 } => write!(f, "and {dest},{src1},{src2}"),
            Instruction::Fence { .. } => write!(f, "{}", self.fmt_fence()),
            Instruction::Ecall => write!(f, "ecall"),
            Instruction::Ebreak => write!(f, "ebreak"),
            Instruction::Lwu { dest, base, offset } => write!(f, "lwu {dest},{offset}({base})"),
            Instruction::Ld { dest, base, offset } => write!(f, "ld {dest},{offset}({base})"),
            Instruction::Sd { src, base, offset } => write!(f, "sd {src},{offset}({base})"),
            Instruction::Addiw { dest, src, imm } => write!(f, "addiw {dest},{src},{imm}"),
            Instruction::Slliw { dest, src, shamt } => write!(f, "slliw {dest},{src},{shamt}"),
            Instruction::Srliw { dest, src, shamt } => write!(f, "srliw {dest},{src},{shamt}"),
            Instruction::Sraiw { dest, src, shamt } => write!(f, "sraiw {dest},{src},{shamt}"),
            Instruction::Addw { dest, src1, src2 } => write!(f, "addw {dest},{src1},{src2}"),
            Instruction::Subw { dest, src1, src2 } => write!(f, "subw {dest},{src1},{src2}"),
            Instruction::Sllw { dest, src1, src2 } => write!(f, "sllw {dest},{src1},{src2}"),
            Instruction::Srlw { dest, src1, src2 } => write!(f, "srlw {dest},{src1},{src2}"),
            Instruction::Sraw { dest, src1, src2 } => write!(f, "sraw {dest},{src1},{src2}"),
            Instruction::Mul { dest, src1, src2 } => write!(f, "mul {dest},{src1},{src2}"),
            Instruction::Mulh { dest, src1, src2 } => write!(f, "mulh {dest},{src1},{src2}"),
            Instruction::Mulhsu { dest, src1, src2 } => write!(f, "mulhsu {dest},{src1},{src2}"),
            Instruction::Mulhu { dest, src1, src2 } => write!(f, "mulhu {dest},{src1},{src2}"),
            Instruction::Div { dest, src1, src2 } => write!(f, "div {dest},{src1},{src2}"),
            Instruction::Divu { dest, src1, src2 } => write!(f, "divu {dest},{src1},{src2}"),
            Instruction::Rem { dest, src1, src2 } => write!(f, "rem {dest},{src1},{src2}"),
            Instruction::Remu { dest, src1, src2 } => write!(f, "remu {dest},{src1},{src2}"),
            Instruction::Mulw { dest, src1, src2 } => write!(f, "mulw {dest},{src1},{src2}"),
            Instruction::Divw { dest, src1, src2 } => write!(f, "divw {dest},{src1},{src2}"),
            Instruction::Divuw { dest, src1, src2 } => write!(f, "divuw {dest},{src1},{src2}"),
            Instruction::Remw { dest, src1, src2 } => write!(f, "remw {dest},{src1},{src2}"),
            Instruction::Remuw { dest, src1, src2 } => write!(f, "remuw {dest},{src1},{src2}"),
            Instruction::LrW { dest, addr, aq, rl } => {
                write!(f, "lr.w{} {dest},{addr}", aq_rl_suffix(aq, rl))
            }
            Instruction::ScW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "sc.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoswapW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoswap.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoaddW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoadd.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoxorW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoxor.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoandW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoand.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoorW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoor.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmominW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomin.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmomaxW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomax.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmominuW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amominu.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmomaxuW {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomaxu.w{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::LrD { dest, addr, aq, rl } => {
                write!(f, "lr.d{} {dest},{addr}", aq_rl_suffix(aq, rl))
            }
            Instruction::ScD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "sc.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoswapD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoswap.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoaddD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoadd.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoxorD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoxor.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoandD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoand.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmoorD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amoor.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmominD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomin.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmomaxD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomax.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmominuD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amominu.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::AmomaxuD {
                dest,
                addr,
                src,
                aq,
                rl,
            } => {
                write!(f, "amomaxu.d{} {dest},{addr},{src}", aq_rl_suffix(aq, rl))
            }
            Instruction::Flw { dest, base, offset } => write!(f, "flw {dest},{offset}({base})"),
            Instruction::Fsw { base, src, offset } => write!(f, "fsw {src},{offset}({base})"),
            Instruction::FmaddS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fmadd.s {dest},{src1},{src2},{src3},{rm}")
            }
            Instruction::FmsubS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fmsub.s {dest},{src1},{src2},{src3},{rm}")
            }
            Instruction::FnmsubS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fnmsub.s {dest},{src1},{src2},{src3},{rm}")
            }
            Instruction::FnmaddS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                write!(f, "fnmadd.s {dest},{src1},{src2},{src3},{rm}")
            }
            Instruction::FaddS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fadd.s {dest},{src1},{src2},{rm}"),
            Instruction::FsubS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fsub.s {dest},{src1},{src2},{rm}"),
            Instruction::FmulS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fmul.s {dest},{src1},{src2},{rm}"),
            Instruction::FdivS {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fdiv.s {dest},{src1},{src2},{rm}"),
            Instruction::FsqrtS { dest, src, rm } => write!(f, "fsqrt.s {dest},{src},{rm}"),
            Instruction::FsgnjS { dest, src1, src2 } => write!(f, "fsgnj.s {dest},{src1},{src2}"),
            Instruction::FsgnjnS { dest, src1, src2 } => write!(f, "fsgnjn.s {dest},{src1},{src2}"),
            Instruction::FsgnjxS { dest, src1, src2 } => write!(f, "fsgnjx.s {dest},{src1},{src2}"),
            Instruction::FminS { dest, src1, src2 } => write!(f, "fmin.s {dest},{src1},{src2}"),
            Instruction::FmaxS { dest, src1, src2 } => write!(f, "fmax.s {dest},{src1},{src2}"),
            Instruction::FcvtWS { dest, src, rm } => write!(f, "fcvt.w.s {dest},{src},{rm}"),
            Instruction::FcvtWuS { dest, src, rm } => write!(f, "fcvt.wu.s {dest},{src},{rm}"),
            Instruction::FmvXW { dest, src } => write!(f, "fmv.x.w {dest},{src}"),
            Instruction::FeqS { dest, src1, src2 } => write!(f, "feq.s {dest},{src1},{src2}"),
            Instruction::FltS { dest, src1, src2 } => write!(f, "flt.s {dest},{src1},{src2}"),
            Instruction::FleS { dest, src1, src2 } => write!(f, "fle.s {dest},{src1},{src2}"),
            Instruction::FclassS { dest, src } => write!(f, "fclass.s {dest},{src}"),
            Instruction::FcvtSW { dest, src, rm } => write!(f, "fcvt.s.w {dest},{src},{rm}"),
            Instruction::FcvtSWu { dest, src, rm } => write!(f, "fcvt.s.wu {dest},{src},{rm}"),
            Instruction::FmvWX { dest, src } => write!(f, "fmv.w.x {dest},{src}"),
            Instruction::FcvtLS { dest, src, rm } => write!(f, "fcvt.l.s {dest},{src},{rm}"),
            Instruction::FcvtLuS { dest, src, rm } => write!(f, "fcvt.lu.s {dest},{src},{rm}"),
            Instruction::FcvtSL { dest, src, rm } => write!(f, "fcvt.s.l {dest},{src},{rm}"),
            Instruction::FcvtSLu { dest, src, rm } => write!(f, "fcvt.s.lu {dest},{src},{rm}"),
            Instruction::Csrrw { dest, src, csr } => write!(f, "csrrw {dest},{csr},{src}"),
            Instruction::Csrrs { dest, src, csr } => write!(f, "csrrs {dest},{csr},{src}"),
            Instruction::Csrrc { dest, src, csr } => write!(f, "csrrc {dest},{csr},{src}"),
            Instruction::Csrrwi { dest, imm, csr } => write!(f, "csrrwi {dest},{csr},{imm}"),
            Instruction::Csrrsi { dest, imm, csr } => write!(f, "csrrsi {dest},{csr},{imm}"),
            Instruction::Csrrci { dest, imm, csr } => write!(f, "csrrci {dest},{csr},{imm}"),
            Instruction::FenceI => write!(f, "fence.i"),
            Instruction::Fld { dest, base, offset } => write!(f, "fld {dest},{offset}({base})"),
            Instruction::Fsd { src, base, offset } => write!(f, "fsd {src},{offset}({base})"),
            Instruction::FmaddD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => write!(f, "fmadd.d {dest},{src1},{src2},{src3},{rm}"),
            Instruction::FmsubD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => write!(f, "fmsub.d {dest},{src1},{src2},{src3},{rm}"),
            Instruction::FnmaddD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => write!(f, "fnmadd.d {dest},{src1},{src2},{src3},{rm}"),
            Instruction::FnmsubD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => write!(f, "fnmsub.d {dest},{src1},{src2},{src3},{rm}"),
            Instruction::FaddD {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fadd.d {dest},{src1},{src2},{rm}"),
            Instruction::FsubD {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fsub.d {dest},{src1},{src2},{rm}"),
            Instruction::FmulD {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fmul.d {dest},{src1},{src2},{rm}"),
            Instruction::FdivD {
                dest,
                src1,
                src2,
                rm,
            } => write!(f, "fdiv.d {dest},{src1},{src2},{rm}"),
            Instruction::FsqrtD {
                dest,
                src: src1,
                rm,
            } => write!(f, "fsqrt.d {dest},{src1},{rm}"),
            Instruction::FsgnjD { dest, src1, src2 } => write!(f, "fsgnj.d {dest},{src1},{src2}"),
            Instruction::FsgnjnD { dest, src1, src2 } => write!(f, "fsgnjn.d {dest},{src1},{src2}"),
            Instruction::FsgnjxD { dest, src1, src2 } => write!(f, "fsgnjx.d {dest},{src1},{src2}"),
            Instruction::FminD { dest, src1, src2 } => write!(f, "fmin.d {dest},{src1},{src2}"),
            Instruction::FmaxD { dest, src1, src2 } => write!(f, "fmax.d {dest},{src1},{src2}"),
            Instruction::FcvtSD { dest, src, rm } => write!(f, "fcvt.s.d {dest},{src},{rm}"),
            Instruction::FcvtDS { dest, src, rm } => write!(f, "fcvt.d.s {dest},{src},{rm}"),
            Instruction::FeqD { dest, src1, src2 } => write!(f, "feq.d {dest},{src1},{src2}"),
            Instruction::FltD { dest, src1, src2 } => write!(f, "flt.d {dest},{src1},{src2}"),
            Instruction::FleD { dest, src1, src2 } => write!(f, "fle.d {dest},{src1},{src2}"),
            Instruction::FclassD { dest, src1 } => write!(f, "fclass.d {dest},{src1}"),
            Instruction::FcvtWD { dest, src1, rm } => write!(f, "fcvt.w.d {dest},{src1},{rm}"),
            Instruction::FcvtWuD { dest, src1, rm } => write!(f, "fcvt.wu.d {dest},{src1},{rm}"),
            Instruction::FcvtDW { dest, src1, rm } => write!(f, "fcvt.d.w {dest},{src1},{rm}"),
            Instruction::FcvtDWu { dest, src1, rm } => write!(f, "fcvt.d.wu {dest},{src1},{rm}"),
            Instruction::FcvtLD { dest, src1, rm } => write!(f, "fcvt.l.d {dest},{src1},{rm}"),
            Instruction::FcvtLuD { dest, src1, rm } => write!(f, "fcvt.lu.d {dest},{src1},{rm}"),
            Instruction::FmvXD { dest, src } => write!(f, "fmv.x.d {dest},{src}"),
            Instruction::FcvtDL {
                dest,
                src: src1,
                rm,
            } => write!(f, "fcvt.d.l {dest},{src1},{rm}"),
            Instruction::FcvtDLu {
                dest,
                src: src1,
                rm,
            } => write!(f, "fcvt.d.lu {dest},{src1},{rm}"),
            Instruction::FmvDX { dest, src } => write!(f, "fmv.d.x {dest},{src}"),
        }
    }
}

impl Instruction {
    fn fmt_fence(&self) -> String {
        if let Instruction::Fence {
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

    /// Constructs an `Instruction` from it's machine code representation.
    pub fn decode(instruction: u32) -> Result<Instruction, String> {
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
                0b000 => Ok(Instruction::Lb {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b001 => Ok(Instruction::Lh {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b010 => Ok(Instruction::Lw {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b011 => Ok(Instruction::Ld {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b100 => Ok(Instruction::Lbu {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b101 => Ok(Instruction::Lhu {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b110 => Ok(Instruction::Lwu {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b111 => Err("Invalid load func3".to_owned()),
                _ => unreachable!(),
            },
            Opcode::Auipc => Ok(Instruction::Auipc {
                dest: rd,
                imm: u_immediate,
            }),
            Opcode::Store => match func3 {
                0b000 => Ok(Instruction::Sb {
                    src: rs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                0b001 => Ok(Instruction::Sh {
                    src: rs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                0b010 => Ok(Instruction::Sw {
                    src: rs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                0b011 => Ok(Instruction::Sd {
                    src: rs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                x => Err(format!("invalid store func3: {}", x)),
            },
            Opcode::Lui => Ok(Instruction::Lui {
                dest: rd,
                imm: u_immediate,
            }),
            Opcode::Op => match (func7, func3) {
                (0b000_0000, 0b000) => Ok(Instruction::Add {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b001) => Ok(Instruction::Sll {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b010) => Ok(Instruction::Slt {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b011) => Ok(Instruction::Sltu {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b100) => Ok(Instruction::Xor {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b101) => Ok(Instruction::Srl {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b110) => Ok(Instruction::Or {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0000, 0b111) => Ok(Instruction::And {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b010_0000, 0b000) => Ok(Instruction::Sub {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b010_0000, 0b101) => Ok(Instruction::Sra {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b000) => Ok(Instruction::Mul {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b001) => Ok(Instruction::Mulh {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b010) => Ok(Instruction::Mulhsu {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b011) => Ok(Instruction::Mulhu {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b100) => Ok(Instruction::Div {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b101) => Ok(Instruction::Divu {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b110) => Ok(Instruction::Rem {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000_0001, 0b111) => Ok(Instruction::Remu {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                _ => Err(format!("unknown Op. func3: {}, func7: {}", func3, func7)),
            },
            Opcode::Op32 => match (func3, func7) {
                (0b000, 0b000_0000) => Ok(Instruction::Addw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000, 0b000_0001) => Ok(Instruction::Mulw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b000, 0b010_0000) => Ok(Instruction::Subw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b001, 0b000_0000) => Ok(Instruction::Sllw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b100, 0b0000_001) => Ok(Instruction::Divw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b101, 0b000_0000) => Ok(Instruction::Srlw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b101, 0b000_0001) => Ok(Instruction::Divuw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b101, 0b010_0000) => Ok(Instruction::Sraw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b110, 0b000_0001) => Ok(Instruction::Remw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                (0b111, 0b000_0001) => Ok(Instruction::Remuw {
                    dest: rd,
                    src1: rs1,
                    src2: rs2,
                }),
                _ => Err(format!("unknown Op32. func3: {}, func7: {}", func3, func7)),
            },
            Opcode::OpImm => match func3 {
                0b000 => Ok(Instruction::Addi {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                // SLLi requires special handling because shamt uses the bottom bit of func7
                0b001 => match func7 | 0b1 {
                    0b000000_1 => Ok(Instruction::Slli {
                        dest: rd,
                        src: rs1,
                        shamt,
                    }),
                    _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
                },
                0b010 => Ok(Instruction::Slti {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                0b011 => Ok(Instruction::Sltiu {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                0b100 => Ok(Instruction::Xori {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                // SRLI SRAI require special handling because shamt uses the bottom bit of func7
                0b101 => match func7 | 0b1 {
                    0b000000_1 => Ok(Instruction::Srli {
                        dest: rd,
                        src: rs1,
                        shamt,
                    }),
                    0b010000_1 => Ok(Instruction::Srai {
                        dest: rd,
                        src: rs1,
                        shamt,
                    }),
                    _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
                },
                0b110 => Ok(Instruction::Ori {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                0b111 => Ok(Instruction::Andi {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                _ => Err(format!("unknown OpImm. func3: {}, func7: {}", func3, func7)),
            },
            Opcode::OpImm32 => match func3 {
                0b000 => Ok(Instruction::Addiw {
                    dest: rd,
                    src: rs1,
                    imm: i_immediate,
                }),
                0b001 => match func7 {
                    0b000_0000 => Ok(Instruction::Slliw {
                        dest: rd,
                        src: rs1,
                        shamt: shamtw,
                    }),
                    x => Err(format!("unknown OpImm32(001) func7: {}", x).to_owned()),
                },
                0b101 => match func7 {
                    0b000_0000 => Ok(Instruction::Srliw {
                        dest: rd,
                        src: rs1,
                        shamt: shamtw,
                    }),
                    0b010_0000 => Ok(Instruction::Sraiw {
                        dest: rd,
                        src: rs1,
                        shamt: shamtw,
                    }),
                    x => Err(format!("unknown OpImm32(101) func7: {}", x).to_owned()),
                },
                x => Err(format!("unkown OpImm32 func3: {}", x).to_owned()),
            },
            Opcode::Jalr => match func3 {
                0b000 => Ok(Instruction::Jalr {
                    dest: rd,
                    base: rs1,
                    offset: i_immediate,
                }),
                x => Err(format!("unknown Jalr func3: {}", x)),
            },
            Opcode::Jal => Ok(Instruction::Jal {
                dest: rd,
                offset: JImmediate::from_u32(instruction),
            }),
            Opcode::Branch => match func3 {
                0b000 => Ok(Instruction::Beq {
                    src1: rs1,
                    src2: rs2,
                    offset: b_immediate,
                }),
                0b001 => Ok(Instruction::Bne {
                    src1: rs1,
                    src2: rs2,
                    offset: b_immediate,
                }),
                0b100 => Ok(Instruction::Blt {
                    src1: rs1,
                    src2: rs2,
                    offset: b_immediate,
                }),
                0b101 => Ok(Instruction::Bge {
                    src1: rs1,
                    src2: rs2,
                    offset: b_immediate,
                }),
                0b110 => Ok(Instruction::Bltu {
                    src1: rs1,
                    src2: rs2,
                    offset: b_immediate,
                }),
                0b111 => Ok(Instruction::Bgeu {
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
                            Ok(Instruction::Fence {
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
                            Ok(Instruction::FenceI)
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
                        Ok(Instruction::LrW {
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
                        Ok(Instruction::LrD {
                            dest: rd,
                            addr: rs1,
                            aq,
                            rl,
                        })
                    }
                }
                (0b010, 0b00011) => Ok(Instruction::ScW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b00011) => Ok(Instruction::ScD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b00001) => Ok(Instruction::AmoswapW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b00001) => Ok(Instruction::AmoswapD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b00000) => Ok(Instruction::AmoaddW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b00000) => Ok(Instruction::AmoaddD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b00100) => Ok(Instruction::AmoxorW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b00100) => Ok(Instruction::AmoxorD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b01100) => Ok(Instruction::AmoandW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b01100) => Ok(Instruction::AmoandD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b01000) => Ok(Instruction::AmoorW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b01000) => Ok(Instruction::AmoorD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b10000) => Ok(Instruction::AmominW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b10000) => Ok(Instruction::AmominD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b10100) => Ok(Instruction::AmomaxW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b10100) => Ok(Instruction::AmomaxD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b11000) => Ok(Instruction::AmominuW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b11000) => Ok(Instruction::AmominuD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b010, 0b11100) => Ok(Instruction::AmomaxuW {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                (0b011, 0b11100) => Ok(Instruction::AmomaxuD {
                    dest: rd,
                    addr: rs1,
                    src: rs2,
                    aq,
                    rl,
                }),
                _ => Err(format!("unknown AMO. func3: {func3}, func7: {func7}")),
            },
            Opcode::LoadFp => match func3 {
                0b010 => Ok(Instruction::Flw {
                    dest: frd,
                    base: rs1,
                    offset: i_immediate,
                }),
                0b011 => Ok(Instruction::Fld {
                    dest: frd,
                    base: rs1,
                    offset: i_immediate,
                }),
                _ => Err(format!("unknown func3: {func3} in opcode LoadFp")),
            },
            Opcode::StoreFp => match func3 {
                0b010 => Ok(Instruction::Fsw {
                    src: frs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                0b011 => Ok(Instruction::Fsd {
                    src: frs2,
                    base: rs1,
                    offset: s_immediate,
                }),
                _ => Err(format!("unknown func3: {func3} in opcode StoreFp")),
            },
            Opcode::OpFp => match func7 {
                0b000_0000 => Ok(Instruction::FaddS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_0001 => Ok(Instruction::FaddD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_0100 => Ok(Instruction::FsubS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_0101 => Ok(Instruction::FsubD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_1000 => Ok(Instruction::FmulS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_1001 => Ok(Instruction::FmulD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_1100 => Ok(Instruction::FdivS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b000_1101 => Ok(Instruction::FdivD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b010_0000 => {
                    if instruction >> 20 & 0b11111 != 0b0_0001 {
                        Err("expected rs2=0b0_0001 in OpFp func7=0b010_0000".to_owned())
                    } else {
                        Ok(Instruction::FcvtSD {
                            dest: frd,
                            src: frs1,
                            rm: RoundingMode::from_int(func3)?,
                        })
                    }
                }
                0b010_0001 => {
                    if instruction >> 20 & 0b11111 != 0b0_0000 {
                        Err("expected rs2=0b0_0000 in OpFp func7=0b010_0001".to_owned())
                    } else {
                        Ok(Instruction::FcvtDS {
                            dest: frd,
                            src: frs1,
                            rm: RoundingMode::from_int(func3)?,
                        })
                    }
                }
                0b010_1100 => {
                    if instruction >> 20 & 0b11111 != 0b0_0000 {
                        Err("expected rs2=0b0_0000 in OpFp func7=0b010_1100".to_owned())
                    } else {
                        Ok(Instruction::FsqrtS {
                            dest: frd,
                            src: frs1,
                            rm: RoundingMode::from_int(func3)?,
                        })
                    }
                }
                0b010_1101 => {
                    if instruction >> 20 & 0b11111 != 0b0_0000 {
                        Err("expected rs2=0b0_0000 in OpFp func7=0b010_1101".to_owned())
                    } else {
                        Ok(Instruction::FsqrtD {
                            dest: frd,
                            src: frs1,
                            rm: RoundingMode::from_int(func3)?,
                        })
                    }
                },
                0b001_0000 => match func3 {
                    0b000 => Ok(Instruction::FsgnjS {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FsgnjnS {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b010 => Ok(Instruction::FsgnjxS {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0000 func3: {}", x)),
                },
                0b001_0001 => match func3 {
                    0b000 => Ok(Instruction::FsgnjD {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FsgnjnD {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b010 => Ok(Instruction::FsgnjxD {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0001 func3: {}", x)),
                },
                0b001_0100 => match func3 {
                    0b000 => Ok(Instruction::FminS {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FmaxS {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0100 func3: {}", x)),
                },
                0b001_0101 => match func3 {
                    0b000 => Ok(Instruction::FminD {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FmaxD {
                        dest: frd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0101 func3: {}", x)),
                },
                0b101_0000 => match func3 {
                    0b000 => Ok(Instruction::FleS {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FltS {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b010 => Ok(Instruction::FeqS {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b101_0000 func3: {}", x)),
                },
                0b101_0001 => match func3 {
                    0b000 => Ok(Instruction::FleD {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b001 => Ok(Instruction::FltD {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    0b010 => Ok(Instruction::FeqD {
                        dest: rd,
                        src1: frs1,
                        src2: frs2,
                    }),
                    x => Err(format!("unknown OpFp func7=0b101_0000 func3: {}", x)),
                },
                0b110_0000 => match (instruction >> 20) & 0b1_1111 {
                    0b0_0000 => Ok(Instruction::FcvtWS {
                        dest: rd,
                        src: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0001 => Ok(Instruction::FcvtWuS {
                        dest: rd,
                        src: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0010 => Ok(Instruction::FcvtLS {
                        dest: rd,
                        src: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0011 => Ok(Instruction::FcvtLuS {
                        dest: rd,
                        src: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
                },
                0b110_0001 => match (instruction >> 20) & 0b1_1111 {
                    0b0_0000 => Ok(Instruction::FcvtWD {
                        dest: rd,
                        src1: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0001 => Ok(Instruction::FcvtWuD {
                        dest: rd,
                        src1: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0010 => Ok(Instruction::FcvtLD {
                        dest: rd,
                        src1: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0011 => Ok(Instruction::FcvtLuD {
                        dest: rd,
                        src1: frs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
                },
                0b110_1000 => match (instruction >> 20) & 0b1_1111 {
                    0b0_0000 => Ok(Instruction::FcvtSW {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0001 => Ok(Instruction::FcvtSWu {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0010 => Ok(Instruction::FcvtSL {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0011 => Ok(Instruction::FcvtSLu {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0100 rs2: {}", x)),
                },
                0b110_1001 => match (instruction >> 20) & 0b1_1111 {
                    0b0_0000 => Ok(Instruction::FcvtDW {
                        dest: frd,
                        src1: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0001 => Ok(Instruction::FcvtDWu {
                        dest: frd,
                        src1: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0010 => Ok(Instruction::FcvtDL {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    0b0_0011 => Ok(Instruction::FcvtDLu {
                        dest: frd,
                        src: rs1,
                        rm: RoundingMode::from_int(func3)?,
                    }),
                    x => Err(format!("unknown OpFp func7=0b001_0101 rs2: {}", x)),
                },
                0b111_0000 => {
                    if (instruction >> 20) & 0b1_1111 == 0 {
                        if func3 == 0 {
                            Ok(Instruction::FmvXW {
                                dest: rd,
                                src: frs1,
                            })
                        } else if func3 == 1 {
                            Ok(Instruction::FclassS {
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
                0b111_0001 => {
                    if (instruction >> 20) & 0b1_1111 == 0 {
                        if func3 == 0 {
                            Ok(Instruction::FmvXD {
                                dest: rd,
                                src: frs1,
                            })
                        } else if func3 == 1 {
                            Ok(Instruction::FclassD {
                                dest: rd,
                                src1: frs1,
                            })
                        } else {
                            Err(format!(
                                "unknown OpFp func7=0b111_0001 rs2=0 func3: {}",
                                func3
                            ))
                        }
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_0001 unknown rs2: {} and func3: {}",
                            (instruction >> 20) & 0b1_1111,
                            func3
                        ))
                    }
                }
                0b111_1000 => {
                    if (instruction >> 20) & 0b1_1111 == 0 {
                        if func3 == 0 {
                            Ok(Instruction::FmvWX {
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
                0b111_1001 => {
                    if (instruction >> 20) & 0b1_1111 == 0 {
                        if func3 == 0 {
                            Ok(Instruction::FmvDX {
                                dest: frd,
                                src: rs1,
                            })
                        } else {
                            Err(format!(
                                "unknown OpFp func7=0b111_1001 rs2=0 func3: {}",
                                func3
                            ))
                        }
                    } else {
                        Err(format!(
                            "unknown OpFp func7=0b111_0001 unknown rs2: {} and func3: {}",
                            (instruction >> 20) & 0b1_1111,
                            func3
                        ))
                    }
                }
                x => Err(format!("Unknown OpFp func7: {x}")),
            },
            Opcode::Reserved => Err("instruction uses reserved opcode".to_owned()),
            Opcode::Madd => match func7 & 0b11 {
                0b00 => Ok(Instruction::FmaddS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b01 => Ok(Instruction::FmaddD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                _ => Err(format!(
                    "Fmadd unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                )),
            },
            Opcode::Msub => match func7 & 0b11 {
                0b00 => Ok(Instruction::FmsubS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b01 => Ok(Instruction::FmsubD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                _ => Err(format!(
                    "Fmsub unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                )),
            },
            Opcode::Nmsub => match func7 & 0b11 {
                0b00 => Ok(Instruction::FnmsubS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b01 => Ok(Instruction::FnmsubD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                _ => Err(format!(
                    "Fnmsub unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                )),
            },
            Opcode::Nmadd => match func7 & 0b11 {
                0b00 => Ok(Instruction::FnmaddS {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                0b01 => Ok(Instruction::FnmaddD {
                    dest: frd,
                    src1: frs1,
                    src2: frs2,
                    src3: frs3,
                    rm: RoundingMode::from_int(func3)?,
                }),
                _ => Err(format!(
                    "Fmadd unknown lower 2 bits of func7: {}",
                    func7 & 0b11
                )),
            },
            Opcode::System => match func3 {
                0b000 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
                0b001 => Ok(Instruction::Csrrw {
                    dest: rd,
                    src: rs1,
                    csr: CSR::from_u32(instruction),
                }),
                0b010 => Ok(Instruction::Csrrs {
                    dest: rd,
                    src: rs1,
                    csr: CSR::from_u32(instruction),
                }),
                0b011 => Ok(Instruction::Csrrc {
                    dest: rd,
                    src: rs1,
                    csr: CSR::from_u32(instruction),
                }),
                0b100 => Err("Reserved func3 in Opcode SYSTEM".to_owned()),
                0b101 => Ok(Instruction::Csrrwi {
                    dest: rd,
                    imm: CSRImmediate::from_u32(instruction),
                    csr: CSR::from_u32(instruction),
                }),
                0b110 => Ok(Instruction::Csrrsi {
                    dest: rd,
                    imm: CSRImmediate::from_u32(instruction),
                    csr: CSR::from_u32(instruction),
                }),
                0b111 => Ok(Instruction::Csrrci {
                    dest: rd,
                    imm: CSRImmediate::from_u32(instruction),
                    csr: CSR::from_u32(instruction),
                }),
                _ => unreachable!(),
            },
        }
    }

    pub fn encode(instruction: &Instruction) -> u32 {
        match instruction {
            Instruction::Lui { dest, imm } => imm.to_u32() | dest.rd() | 0b0110111,
            Instruction::Auipc { dest, imm } => imm.to_u32() | dest.rd() | 0b0010111,
            Instruction::Jal { dest, offset } => offset.to_u32() | dest.rd() | 0b1101111,
            Instruction::Jalr { dest, base, offset } => {
                offset.to_u32() | base.rs1() | dest.rd() | 0b1100111
            }
            Instruction::Beq { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b000 << 12 | 0b1100011
            }
            Instruction::Bne { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b001 << 12 | 0b1100011
            }
            Instruction::Blt { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b100 << 12 | 0b1100011
            }
            Instruction::Bge { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b101 << 12 | 0b1100011
            }
            Instruction::Bltu { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b110 << 12 | 0b1100011
            }
            Instruction::Bgeu { src1, src2, offset } => {
                offset.to_u32() | src2.rs2() | src1.rs1() | 0b111 << 12 | 0b1100011
            }
            Instruction::Lb { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b000 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Lh { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b001 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Lw { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b010 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Lbu { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b100 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Lhu { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b101 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Sb { src, base, offset } => {
                offset.to_u32() | src.rs2() | base.rs1() | 0b000 << 12 | 0b0100011
            }
            Instruction::Sh { src, base, offset } => {
                offset.to_u32() | src.rs2() | base.rs1() | 0b001 << 12 | 0b0100011
            }
            Instruction::Sw { src, base, offset } => {
                offset.to_u32() | src.rs2() | base.rs1() | 0b010 << 12 | 0b0100011
            }
            Instruction::Addi { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b000 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Slti { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b010 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Sltiu { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b011 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Xori { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b100 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Ori { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b110 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Andi { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b111 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Slli { dest, src, shamt } => {
                shamt.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Srli { dest, src, shamt } => {
                shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Srai { dest, src, shamt } => {
                0b0100000 << 25 | shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0010011
            }
            Instruction::Add { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Sub { dest, src1, src2 } => {
                0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Sll { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Slt { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Sltu { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b011 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Xor { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Srl { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Sra { dest, src1, src2 } => {
                0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b0101 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Or { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0110011
            }
            Instruction::And { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Fence { rd, rs1, ops, fm } => {
                (*fm as u32) << 28 | (*ops as u32) << 20 | rs1.rs1() | rd.rd() | 0b0001111
            }
            Instruction::Ecall => 0b1110011,
            Instruction::Ebreak => 0b1 << 20 | 0b1110011,
            Instruction::Lwu { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b110 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Ld { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b011 << 12 | dest.rd() | 0b0000011
            }
            Instruction::Sd { src, base, offset } => {
                offset.to_u32() | src.rs2() | base.rs1() | 0b011 << 12 | 0b0100011
            }
            Instruction::Addiw { dest, src, imm } => {
                imm.to_u32() | src.rs1() | 0b000 << 12 | dest.rd() | 0b0011011
            }
            Instruction::Slliw { dest, src, shamt } => {
                shamt.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b0011011
            }
            Instruction::Srliw { dest, src, shamt } => {
                shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0011011
            }
            Instruction::Sraiw { dest, src, shamt } => {
                0b0100000 << 25 | shamt.to_u32() | src.rs1() | 0b101 << 12 | dest.rd() | 0b0011011
            }
            Instruction::Addw { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Subw { dest, src1, src2 } => {
                0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Sllw { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Srlw { dest, src1, src2 } => {
                src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Sraw { dest, src1, src2 } => {
                0b0100000 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Mul { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Mulh { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Mulhsu { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Mulhu { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b011 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Div { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Divu { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Rem { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Remu { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0110011
            }
            Instruction::Mulw { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Divw { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b100 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Divuw { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b101 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Remw { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b110 << 12 | dest.rd() | 0b0111011
            }
            Instruction::Remuw { dest, src1, src2 } => {
                0b0000001 << 25 | src2.rs2() | src1.rs1() | 0b111 << 12 | dest.rd() | 0b0111011
            }
            Instruction::LrW { dest, addr, aq, rl } => {
                0b00010 << 27
                    | aqb(*aq)
                    | rlb(*rl)
                    | addr.rs1()
                    | 0b010 << 12
                    | dest.rd()
                    | 0b0101111
            }
            Instruction::ScW {
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
            Instruction::AmoswapW {
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
            Instruction::AmoaddW {
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
            Instruction::AmoxorW {
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
            Instruction::AmoandW {
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
            Instruction::AmoorW {
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
            Instruction::AmominW {
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
            Instruction::AmomaxW {
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
            Instruction::AmominuW {
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
            Instruction::AmomaxuW {
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
            Instruction::LrD { dest, addr, aq, rl } => {
                0b00010 << 27
                    | aqb(*aq)
                    | rlb(*rl)
                    | addr.rs1()
                    | 0b011 << 12
                    | dest.rd()
                    | 0b0101111
            }
            Instruction::ScD {
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
            Instruction::AmoswapD {
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
            Instruction::AmoaddD {
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
            Instruction::AmoxorD {
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
            Instruction::AmoandD {
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
            Instruction::AmoorD {
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
            Instruction::AmominD {
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
            Instruction::AmomaxD {
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
            Instruction::AmominuD {
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
            Instruction::AmomaxuD {
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
            Instruction::Flw { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b010 << 12 | dest.rd() | 0b0000111
            }
            Instruction::Fsw { base, src, offset } => {
                offset.to_u32() | src.rs2() | base.rs1() | 0b010 << 12 | 0b0100111
            }
            Instruction::FmaddS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1000011,
            Instruction::FmsubS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1000111,
            Instruction::FnmsubS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1001011,
            Instruction::FnmaddS {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => src3.rs3() | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1001111,
            Instruction::FaddS {
                dest,
                src1,
                src2,
                rm,
            } => 0b0000000 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FsubS {
                dest,
                src1,
                src2,
                rm,
            } => 0b0000100 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FmulS {
                dest,
                src1,
                src2,
                rm,
            } => 0b0001000 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FdivS {
                dest,
                src1,
                src2,
                rm,
            } => 0b0001100 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FsqrtS { dest, src, rm } => {
                0b0101100 << 25 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FsgnjS { dest, src1, src2 } => {
                0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FsgnjnS { dest, src1, src2 } => {
                0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FsgnjxS { dest, src1, src2 } => {
                0b0010000 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FminS { dest, src1, src2 } => {
                0b0010100 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FmaxS { dest, src1, src2 } => {
                0b0010100 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FcvtWS { dest, src, rm } => {
                0b1100000 << 25 | 0b00000 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtWuS { dest, src, rm } => {
                0b1100000 << 25 | 0b00001 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FmvXW { dest, src } => 0b1110000 << 25 | src.rs1() | dest.rd() | 0b1010011,
            Instruction::FeqS { dest, src1, src2 } => {
                0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FltS { dest, src1, src2 } => {
                0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FleS { dest, src1, src2 } => {
                0b1010000 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FclassS { dest, src } => {
                0b1110000 << 25 | src.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FcvtSW { dest, src, rm } => {
                0b1101000 << 25 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtSWu { dest, src, rm } => {
                0b1101000 << 25 | 0b00001 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FmvWX { dest, src } => 0b1111000 << 25 | src.rs1() | dest.rd() | 0b1010011,
            Instruction::FcvtLS { dest, src, rm } => {
                0b1100000 << 25 | 0b00010 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtLuS { dest, src, rm } => {
                0b1100000 << 25 | 0b00011 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtSL { dest, src, rm } => {
                0b1101000 << 25 | 0b00010 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtSLu { dest, src, rm } => {
                0b1101000 << 25 | 0b00011 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::Csrrw { dest, src, csr } => {
                csr.to_u32() | src.rs1() | 0b001 << 12 | dest.rd() | 0b1110011
            }
            Instruction::Csrrs { dest, src, csr } => {
                csr.to_u32() | src.rs1() | 0b010 << 12 | dest.rd() | 0b1110011
            }
            Instruction::Csrrc { dest, src, csr } => {
                csr.to_u32() | src.rs1() | 0b011 << 12 | dest.rd() | 0b1110011
            }
            Instruction::Csrrwi { dest, imm, csr } => {
                csr.to_u32() | imm.to_u32() | 0b101 << 12 | dest.rd() | 0b1110011
            }
            Instruction::Csrrsi { dest, imm, csr } => {
                csr.to_u32() | imm.to_u32() | 0b110 << 12 | dest.rd() | 0b1110011
            }
            Instruction::Csrrci { dest, imm, csr } => {
                csr.to_u32() | imm.to_u32() | 0b111 << 12 | dest.rd() | 0b1110011
            }
            Instruction::FenceI => 0b001 << 12 | 0b0001111,
            Instruction::Fld { dest, base, offset } => {
                offset.to_u32() | base.rs1() | 0b011 << 12 | dest.rd() | 0b0000111
            }
            Instruction::Fsd { src, base, offset } => {
                offset.to_u32() | base.rs1() | 0b011 << 12 | src.rs2() | 0b0100111
            }
            Instruction::FmaddD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                src3.rs3()
                    | 0b01 << 25
                    | src2.rs2()
                    | src1.rs1()
                    | rm.to_u32()
                    | dest.rd()
                    | 0b1000011
            }
            Instruction::FmsubD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                src3.rs3()
                    | 0b01 << 25
                    | src2.rs2()
                    | src1.rs1()
                    | rm.to_u32()
                    | dest.rd()
                    | 0b1000111
            }
            Instruction::FnmaddD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                src3.rs3()
                    | 0b01 << 25
                    | src2.rs2()
                    | src1.rs1()
                    | rm.to_u32()
                    | dest.rd()
                    | 0b1001111
            }
            Instruction::FnmsubD {
                dest,
                src1,
                src2,
                src3,
                rm,
            } => {
                src3.rs3()
                    | 0b01 << 25
                    | src2.rs2()
                    | src1.rs1()
                    | rm.to_u32()
                    | dest.rd()
                    | 0b1001011
            }
            Instruction::FaddD {
                dest,
                src1,
                src2,
                rm,
            } => 0b0000001 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FsubD {
                dest,
                src1,
                src2,
                rm,
            } => 0b0000101 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FmulD {
                dest,
                src1,
                src2,
                rm,
            } => 0b0001001 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FdivD {
                dest,
                src1,
                src2,
                rm,
            } => 0b0001101 << 25 | src2.rs2() | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FsqrtD {
                dest,
                src: src1,
                rm,
            } => 0b0101101 << 25 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FsgnjD { dest, src1, src2 } => {
                0b0010001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FsgnjnD { dest, src1, src2 } => {
                0b0010001 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FsgnjxD { dest, src1, src2 } => {
                0b0010001 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FminD { dest, src1, src2 } => {
                0b0010101 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FmaxD { dest, src1, src2 } => {
                0b0010101 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FcvtSD { dest, src, rm } => {
                0b0100000 << 25 | 0b00001 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtDS { dest, src, rm } => {
                0b0100001 << 25 | 0b00000 << 20 | src.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FeqD { dest, src1, src2 } => {
                0b1010001 << 25 | src2.rs2() | src1.rs1() | 0b010 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FltD { dest, src1, src2 } => {
                0b1010001 << 25 | src2.rs2() | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FleD { dest, src1, src2 } => {
                0b1010001 << 25 | src2.rs2() | src1.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FclassD { dest, src1 } => {
                0b1110001 << 25 | 0b00000 << 20 | src1.rs1() | 0b001 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FcvtWD { dest, src1, rm } => {
                0b1100001 << 25 | 0b00000 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtWuD { dest, src1, rm } => {
                0b1100001 << 25 | 0b00001 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtDW { dest, src1, rm } => {
                0b1101001 << 25 | 0b00000 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtDWu { dest, src1, rm } => {
                0b1101001 << 25 | 0b00001 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtLD { dest, src1, rm } => {
                0b1100001 << 25 | 0b00010 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FcvtLuD { dest, src1, rm } => {
                0b1100001 << 25 | 0b00011 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011
            }
            Instruction::FmvXD { dest, src } => {
                0b1110001 << 25 | 0b00000 << 20 | src.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
            Instruction::FcvtDL {
                dest,
                src: src1,
                rm,
            } => 0b1101001 << 25 | 0b00010 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FcvtDLu {
                dest,
                src: src1,
                rm,
            } => 0b1101001 << 25 | 0b00011 << 20 | src1.rs1() | rm.to_u32() | dest.rd() | 0b1010011,
            Instruction::FmvDX { dest, src } => {
                0b1111001 << 25 | 0b00000 << 20 | src.rs1() | 0b000 << 12 | dest.rd() | 0b1010011
            }
        }
    }
}

/// Disassembles an instruction.
pub fn disassemble_instruction(instruction: &Instruction) -> String {
    format!("{}", instruction)
}
