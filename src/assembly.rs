use alloc::borrow::ToOwned;
use alloc::{format, vec};
use alloc::string::String;
use alloc::vec::Vec;
use riscv_codec_proc_macros::{
    amo_assemble, b_assemble, ci_assemble, cr_assemble, fr_assemble, i_assemble, l_assemble,
    r_assemble, s_assemble, sh_assemble, shw_assemble,
};

use crate::immediates::*;
use crate::instruction::RoundingMode;
use crate::register::{CFRegister, CIRegister, FRegister, IRegister};
use crate::{cinstruction::CInstruction, instruction::Instruction};

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

fn parse_address_expression_compressed(str: &str) -> Result<(CIRegister, i64), String> {
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

/// Converts a string representing operations into a fence u8
fn parse_fence_set(s: &str) -> u8 {
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

    if mnemonics[0] == "c" {
        if mnemonics.len() == 1 {
            Err("compressed instruction must be specified".to_owned())
        } else {
            compressed_assemble(&mnemonics[1..], operands).map(AssemblyResult::C)
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

fn compressed_assemble(mnemonics: &[&str], operands: Vec<&str>) -> Result<CInstruction, String> {
    match mnemonics[0] {
        "addi4spn" => {
            if operands.len() != 2 {
                Err("c.addi4spn requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::ADDI4SPN {
                    dest: CIRegister::try_from(operands[0])?,
                    imm: CWideImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "fld" => {
            if operands.len() != 2 {
                Err("c.fld requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::FLD {
                    dest: CFRegister::try_from(operands[0])?,
                    base,
                    offset: CDImmediate::try_from(imm)?,
                })
            }
        }
        "lw" => {
            if operands.len() != 2 {
                Err("c.lw requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::LW {
                    dest: CIRegister::try_from(operands[0])?,
                    base,
                    offset: CWImmediate::try_from(imm)?,
                })
            }
        }
        "ld" => {
            if operands.len() != 2 {
                Err("c.ld requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::LD {
                    dest: CIRegister::try_from(operands[0])?,
                    base,
                    offset: CDImmediate::try_from(imm)?,
                })
            }
        }
        "fsd" => {
            if operands.len() != 2 {
                Err("c.fsd requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::FSD {
                    src: CFRegister::try_from(operands[0])?,
                    base,
                    offset: CDImmediate::try_from(imm)?,
                })
            }
        }
        "sw" => {
            if operands.len() != 2 {
                Err("c.sw requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::SW {
                    src: CIRegister::try_from(operands[0])?,
                    base,
                    offset: CWImmediate::try_from(imm)?,
                })
            }
        }
        "sd" => {
            if operands.len() != 2 {
                Err("c.sd requires 2 operands".to_owned())
            } else {
                let (base, imm) = parse_address_expression_compressed(operands[1])?;
                Ok(CInstruction::SD {
                    src: CIRegister::try_from(operands[0])?,
                    base,
                    offset: CDImmediate::try_from(imm)?,
                })
            }
        }
        "addi" => ci_assemble!(ADDI),
        "addiw" => ci_assemble!(ADDIW),
        "li" => ci_assemble!(LI),
        "addi16sp" => {
            if operands.len() != 1 {
                Err("c.addi16sp requires 1 operands".to_owned())
            } else {
                let i = parse_int(operands[0])?;

                Ok(CInstruction::ADDI16SP {
                    imm: C16SPImmediate::try_from(i)?,
                })
            }
        }
        "lui" => ci_assemble!(LUI),
        "srli" => {
            if operands.len() != 2 {
                Err("c.srli requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SRLI {
                    dest: CIRegister::try_from(operands[0])?,
                    shamt: CShamt::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "srai" => {
            if operands.len() != 2 {
                Err("c.srai requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SRAI {
                    dest: CIRegister::try_from(operands[0])?,
                    shamt: CShamt::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "andi" => {
            if operands.len() != 2 {
                Err("c.andi requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::ANDI {
                    dest: CIRegister::try_from(operands[0])?,
                    imm: CIImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "sub" => cr_assemble!(SUB),
        "xor" => cr_assemble!(XOR),
        "or" => cr_assemble!(OR),
        "and" => cr_assemble!(AND),
        "subw" => cr_assemble!(SUBW),
        "addw" => cr_assemble!(ADDW),
        "j" => {
            if operands.len() != 1 {
                Err("c.j requires 1 operand".to_owned())
            } else {
                Ok(CInstruction::J {
                    offset: CJImmediate::try_from(parse_int(operands[0])?)?,
                })
            }
        }
        "beqz" => {
            if operands.len() != 2 {
                Err("c.beqz requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::BEQZ {
                    src: CIRegister::try_from(operands[0])?,
                    offset: CBImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "bnez" => {
            if operands.len() != 2 {
                Err("c.bne requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::BNEZ {
                    src: CIRegister::try_from(operands[0])?,
                    offset: CBImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "slli" => {
            if operands.len() != 2 {
                Err("c.slli requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SLLI {
                    dest: IRegister::from_string(operands[0])?,
                    shamt: CShamt::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "fldsp" => {
            if operands.len() != 2 {
                Err("c.fldsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::FLDSP {
                    dest: FRegister::try_from(operands[0])?,
                    offset: CDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "ldsp" => {
            if operands.len() != 2 {
                Err("c.ldsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::LDSP {
                    dest: IRegister::from_string(operands[0])?,
                    offset: CDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "lwsp" => {
            if operands.len() != 2 {
                Err("c.lwsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::LWSP {
                    dest: IRegister::from_string(operands[0])?,
                    offset: CWSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "jr" => {
            if operands.len() != 1 {
                Err("c.jr requires 1 operand".to_owned())
            } else {
                Ok(CInstruction::JR {
                    src: IRegister::from_string(operands[0])?,
                })
            }
        }
        "jalr" => {
            if operands.len() != 1 {
                Err("c.jalr requires 1 operand".to_owned())
            } else {
                Ok(CInstruction::JALR {
                    src: IRegister::from_string(operands[0])?,
                })
            }
        }
        "ebreak" => {
            if operands.len() != 0 {
                Err("c.jr requires 0 operands".to_owned())
            } else {
                Ok(CInstruction::EBREAK)
            }
        }
        "add" => {
            if operands.len() != 2 {
                Err("c.add requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::ADD {
                    dest: IRegister::from_string(operands[0])?,
                    src: IRegister::from_string(operands[1])?,
                })
            }
        }
        "fsdsp" => {
            if operands.len() != 2 {
                Err("c.fsdsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::FSDSP {
                    src: FRegister::try_from(operands[0])?,
                    offset: CSDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "swsp" => {
            if operands.len() != 2 {
                Err("c.swsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SWSP {
                    src: IRegister::from_string(operands[0])?,
                    offset: CSWSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "sdsp" => {
            if operands.len() != 2 {
                Err("c.sdsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SDSP {
                    src: IRegister::from_string(operands[0])?,
                    offset: CSDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "mv" => {
            if operands.len() != 2 {
                Err("c.mv requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::MV {
                    dest: IRegister::from_string(operands[0])?,
                    src: IRegister::from_string(operands[1])?,
                })
            }
        }
        _ => Err(format!(
            "unknown compressed instruction mnemonic: {}",
            mnemonics[0]
        )),
    }
}
