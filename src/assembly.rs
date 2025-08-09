use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use riscv_codec_proc_macros::{
    amo_assemble, b_assemble, ci_assemble, cr_assemble, fr_assemble, fr3_assemble, i_assemble,
    l_assemble, r_assemble, s_assemble, sh_assemble, shw_assemble,
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
            let r = IRegister::try_from(y)?;
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
            "addi" => i_assemble!(Addi),
            "addiw" => i_assemble!(Addiw),
            "andi" => i_assemble!(Andi),
            "ori" => i_assemble!(Ori),
            "xori" => i_assemble!(Xori),
            "slti" => i_assemble!(Slti),
            "sltiu" => i_assemble!(Sltiu),
            "slli" => sh_assemble!(Slli),
            "srai" => sh_assemble!(Srai),
            "sraiw" => shw_assemble!(Sraiw),
            "srli" => sh_assemble!(Srli),
            "srliw" => shw_assemble!(Srliw),
            "slliw" => shw_assemble!(Slliw),
            // register-register instructions
            "add" => r_assemble!(Add),
            "addw" => r_assemble!(Addw),
            "subw" => r_assemble!(Subw),
            "and" => r_assemble!(And),
            "sub" => r_assemble!(Sub),
            "or" => r_assemble!(Or),
            "xor" => r_assemble!(Xor),
            "sllw" => r_assemble!(Sllw),
            "srl" => r_assemble!(Srl),
            "sra" => r_assemble!(Sra),
            "srlw" => r_assemble!(Srlw),
            "sraw" => r_assemble!(Sraw),
            "sll" => r_assemble!(Sll),
            "slt" => r_assemble!(Slt),
            "sltu" => r_assemble!(Sltu),
            "mul" => r_assemble!(Mul),
            "mulh" => r_assemble!(Mulh),
            "mulhsu" => r_assemble!(Mulhsu),
            "mulhu" => r_assemble!(Mulhu),
            "div" => r_assemble!(Div),
            "divu" => r_assemble!(Divu),
            "rem" => r_assemble!(Rem),
            "remu" => r_assemble!(Remu),
            "mulw" => r_assemble!(Mulw),
            "divw" => r_assemble!(Divw),
            "divuw" => r_assemble!(Divuw),
            "remw" => r_assemble!(Remw),
            "remuw" => r_assemble!(Remuw),
            // load instructions
            "lb" => l_assemble!(Lb),
            "lbu" => l_assemble!(Lbu),
            "lhu" => l_assemble!(Lhu),
            "lw" => l_assemble!(Lw),
            "lwu" => l_assemble!(Lwu),
            "lh" => l_assemble!(Lh),
            // store instructions
            "ld" => l_assemble!(Ld),
            "sd" => s_assemble!(Sd),
            "sw" => s_assemble!(Sw),
            "sh" => s_assemble!(Sh),
            "sb" => s_assemble!(Sb),
            // branch instructions
            "blt" => b_assemble!(Blt),
            "beq" => b_assemble!(Beq),
            "bne" => b_assemble!(Bne),
            "bge" => b_assemble!(Bge),
            "bgeu" => b_assemble!(Bgeu),
            "bltu" => b_assemble!(Bltu),
            "jalr" => {
                if operands.len() != 2 {
                    Err("jalr instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::Jalr {
                        dest: IRegister::try_from(operands[0])?,
                        base,
                        offset: IImmediate::try_from(offset)?,
                    })
                }
            }
            "jal" => {
                if operands.len() != 2 {
                    Err("jal instruction requires 2 operands".to_owned())
                } else {
                    Ok(Instruction::Jal {
                        dest: IRegister::try_from(operands[0])?,
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
                        Ok(Instruction::Lui {
                            dest: IRegister::try_from(operands[0])?,
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
                        Ok(Instruction::Auipc {
                            dest: IRegister::try_from(operands[0])?,
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
                        Ok(Instruction::Fence {
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
                            Ok(Instruction::Fence {
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
                            dest: IRegister::try_from(operands[0])?,
                            addr: IRegister::try_from(operands[1])?,
                            aq: false,
                            rl: false,
                        })
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD {
                            dest: IRegister::try_from(operands[0])?,
                            addr: IRegister::try_from(operands[1])?,
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
                            dest: IRegister::try_from(operands[0])?,
                            addr: IRegister::try_from(operands[1])?,
                            aq,
                            rl,
                        })
                    } else if mnemonics[1] == "d" {
                        Ok(Instruction::LRD {
                            dest: IRegister::try_from(operands[0])?,
                            addr: IRegister::try_from(operands[1])?,
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
                let rm = if operands.len() == 2 {
                    RoundingMode::DYN
                } else if operands.len() == 3 {
                    RoundingMode::from_str(operands[2])?
                } else {
                    return Err("fsqrt instruction requires 2 or 3 operands".to_owned());
                };

                match mnemonics.get(1) {
                    Some(&"s") => Ok(Instruction::FsqrtS {
                        dest: FRegister::try_from(operands[0])?,
                        src: FRegister::try_from(operands[1])?,
                        rm,
                    }),
                    Some(&"d") => Ok(Instruction::FsqrtD {
                        dest: FRegister::try_from(operands[0])?,
                        src: FRegister::try_from(operands[1])?,
                        rm,
                    }),
                    Some(_) => Err("fsqrt instructions requires prefix {s,d}".to_owned()),
                    None => Err("fsqrt instructions requires prefix {s,d}".to_owned()),
                }
            }
            "fadd" => fr_assemble!(Fadd true),
            "fsub" => fr_assemble!(Fsub true),
            "fmul" => fr_assemble!(Fmul true),
            "fdiv" => fr_assemble!(Fdiv true),
            "fmin" => fr_assemble!(Fmin false),
            "fmax" => fr_assemble!(Fmax false),
            "fsgnj" => fr_assemble!(Fsgnj false),
            "fsgnjx" => fr_assemble!(Fsgnjx false),
            "fsgnjn" => fr_assemble!(Fsgnjn false),
            "fmadd" => fr3_assemble!(Fmadd),
            "fmsub" => fr3_assemble!(Fmsub),
            "fnmsub" => fr3_assemble!(Fnmsub),
            "fnmadd" => fr3_assemble!(Fnmadd),
            "fcvt" => {
                let rm = if operands.len() == 2 {
                    RoundingMode::DYN
                } else if operands.len() == 3 {
                    RoundingMode::from_str(operands[2])?
                } else {
                    return Err("fcvt instruction requires 2 or 3 operands".to_owned());
                };

                if mnemonics.len() == 3 {
                    // default rounding mode
                    match (mnemonics[1], mnemonics[2]) {
                        ("w", "s") => Ok(Instruction::FcvtWS {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("wu", "s") => Ok(Instruction::FcvtWuS {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("s", "w") => Ok(Instruction::FcvtSW {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("s", "wu") => Ok(Instruction::FcvtSWu {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("l", "s") => Ok(Instruction::FCVTLS {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("lu", "s") => Ok(Instruction::FCVTLUS {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("s", "l") => Ok(Instruction::FCVTSL {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("s", "lu") => Ok(Instruction::FCVTSLU {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("w", "d") => Ok(Instruction::FcvtWD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("wu", "d") => Ok(Instruction::FcvtWuD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("d", "w") => Ok(Instruction::FcvtDW {
                            dest: FRegister::try_from(operands[0])?,
                            src1: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("d", "wu") => Ok(Instruction::FcvtDWu {
                            dest: FRegister::try_from(operands[0])?,
                            src1: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("l", "d") => Ok(Instruction::FcvtLD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("lu", "d") => Ok(Instruction::FcvtLuD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("d", "l") => Ok(Instruction::FcvtDL {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("d", "lu") => Ok(Instruction::FcvtDLu {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("s", "d") => Ok(Instruction::FcvtSD {
                            dest: FRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        ("d", "s") => Ok(Instruction::FcvtDS {
                            dest: FRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                            rm,
                        }),
                        _ => Err("invalid fcvt suffixes".to_owned()),
                    }
                } else {
                    Err("fcvt should have 2 suffixes".to_owned())
                }
            }
            "fmv" => {
                if operands.len() != 2 {
                    Err("fmv requires 2 operands".to_owned())
                } else if mnemonics.len() == 3 {
                    match (mnemonics[1], mnemonics[2]) {
                        ("x", "w") => Ok(Instruction::FmvXW {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                        }),
                        ("w", "x") => Ok(Instruction::FmvWX {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
                        }),
                        ("x", "d") => Ok(Instruction::FmvXD {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                        }),
                        ("d", "x") => Ok(Instruction::FmvDX {
                            dest: FRegister::try_from(operands[0])?,
                            src: IRegister::try_from(operands[1])?,
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
                        "s" => Ok(Instruction::FeqS {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => Ok(Instruction::FeqD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
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
                        "s" => Ok(Instruction::FltS {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => Ok(Instruction::FltD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
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
                        "s" => Ok(Instruction::FleS {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
                        "d" => Ok(Instruction::FleD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                            src2: FRegister::try_from(operands[2])?,
                        }),
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
                        "s" => Ok(Instruction::FclassS {
                            dest: IRegister::try_from(operands[0])?,
                            src: FRegister::try_from(operands[1])?,
                        }),
                        "d" => Ok(Instruction::FclassD {
                            dest: IRegister::try_from(operands[0])?,
                            src1: FRegister::try_from(operands[1])?,
                        }),
                        "q" => todo!(),
                        "h" => todo!(),
                        _ => Err("fclass requires a suffix {s,d}".to_owned()),
                    }
                } else {
                    Err("fclass requires a suffix {s,d}".to_owned())
                }
            }
            "csrrw" => {
                if operands.len() != 3 {
                    Err("csrrw requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRW {
                        dest: IRegister::try_from(operands[0])?,
                        src: IRegister::try_from(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrs" => {
                if operands.len() != 3 {
                    Err("csrrs requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRS {
                        dest: IRegister::try_from(operands[0])?,
                        src: IRegister::try_from(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrc" => {
                if operands.len() != 3 {
                    Err("csrrc requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRC {
                        dest: IRegister::try_from(operands[0])?,
                        src: IRegister::try_from(operands[2])?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "csrrwi" => {
                if operands.len() != 3 {
                    Err("csrrwi requires 3 operands".to_owned())
                } else {
                    Ok(Instruction::CSRRWI {
                        dest: IRegister::try_from(operands[0])?,
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
                        dest: IRegister::try_from(operands[0])?,
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
                        dest: IRegister::try_from(operands[0])?,
                        imm: CSRImmediate::try_from(parse_int(operands[2])?)?,
                        csr: CSR::try_from(parse_int(operands[1])?)?,
                    })
                }
            }
            "fld" => {
                if operands.len() != 2 {
                    Err("fld instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::Fld {
                        dest: FRegister::try_from(operands[0])?,
                        base,
                        offset: IImmediate::try_from(offset)?,
                    })
                }
            }
            "fsd" => {
                if operands.len() != 2 {
                    Err("fsd instruction requires 2 operands".to_owned())
                } else {
                    let (base, offset) = parse_address_expression(operands[1])?;
                    Ok(Instruction::Fsd {
                        base,
                        src: FRegister::try_from(operands[0])?,
                        offset: SImmediate::try_from(offset)?,
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
                    dest: IRegister::try_from(operands[0])?,
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
                    dest: IRegister::try_from(operands[0])?,
                    offset: CDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "lwsp" => {
            if operands.len() != 2 {
                Err("c.lwsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::LWSP {
                    dest: IRegister::try_from(operands[0])?,
                    offset: CWSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "jr" => {
            if operands.len() != 1 {
                Err("c.jr requires 1 operand".to_owned())
            } else {
                Ok(CInstruction::JR {
                    src: IRegister::try_from(operands[0])?,
                })
            }
        }
        "jalr" => {
            if operands.len() != 1 {
                Err("c.jalr requires 1 operand".to_owned())
            } else {
                Ok(CInstruction::JALR {
                    src: IRegister::try_from(operands[0])?,
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
                    dest: IRegister::try_from(operands[0])?,
                    src: IRegister::try_from(operands[1])?,
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
                    src: IRegister::try_from(operands[0])?,
                    offset: CSWSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "sdsp" => {
            if operands.len() != 2 {
                Err("c.sdsp requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::SDSP {
                    src: IRegister::try_from(operands[0])?,
                    offset: CSDSPImmediate::try_from(parse_int(operands[1])?)?,
                })
            }
        }
        "mv" => {
            if operands.len() != 2 {
                Err("c.mv requires 2 operands".to_owned())
            } else {
                Ok(CInstruction::MV {
                    dest: IRegister::try_from(operands[0])?,
                    src: IRegister::try_from(operands[1])?,
                })
            }
        }
        _ => Err(format!(
            "unknown compressed instruction mnemonic: {}",
            mnemonics[0]
        )),
    }
}
