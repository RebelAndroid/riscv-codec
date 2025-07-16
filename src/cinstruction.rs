use std::fmt::{Display, Formatter};

use proc_macros::{ci_assemble, cr_assemble};

use crate::{
    immediates::{
        BImmediate, CBImmediate, CDImmediate, CDSPImmediate, CIImmediate, CJImmediate,
        CSDSPImmediate, CSWSPImmediate, CShamt, CWImmediate, CWSPImmediate, CWideImmediate,
        IImmediate, JImmediate, SImmediate, Shamt,
    },
    instruction::{Instruction, parse_address_expression_compressed, parse_int},
    register::{CFRegister, CIRegister, FRegister, IRegister},
};

#[derive(Debug, PartialEq)]
pub enum CInstruction {
    //
    // Instructions in C extension
    //
    ADDI4SPN {
        dest: CIRegister,
        imm: CWideImmediate,
    },
    FLD {
        dest: CFRegister,
        base: CIRegister,
        offset: CDImmediate,
    },
    LW {
        dest: CIRegister,
        base: CIRegister,
        offset: CWImmediate,
    },
    LD {
        dest: CIRegister,
        base: CIRegister,
        offset: CDImmediate,
    },
    FSD {
        src: CFRegister,
        base: CIRegister,
        offset: CDImmediate,
    },
    SW {
        src: CIRegister,
        base: CIRegister,
        offset: CWImmediate,
    },
    SD {
        src: CIRegister,
        base: CIRegister,
        offset: CDImmediate,
    },
    ADDI {
        dest: IRegister,
        imm: CIImmediate,
    },
    ADDIW {
        dest: IRegister,
        imm: CIImmediate,
    },
    LI {
        dest: IRegister,
        imm: CIImmediate,
    },
    ADDI16SP {
        imm: i16,
    },
    LUI {
        dest: IRegister,
        imm: CIImmediate,
    },
    SRLI {
        dest: CIRegister,
        shamt: CShamt,
    },
    SRAI {
        dest: CIRegister,
        shamt: CShamt,
    },
    ANDI {
        dest: CIRegister,
        imm: CIImmediate,
    },
    SUB {
        dest: CIRegister,
        src: CIRegister,
    },
    XOR {
        dest: CIRegister,
        src: CIRegister,
    },
    OR {
        dest: CIRegister,
        src: CIRegister,
    },
    AND {
        dest: CIRegister,
        src: CIRegister,
    },
    SUBW {
        dest: CIRegister,
        src: CIRegister,
    },
    ADDW {
        dest: CIRegister,
        src: CIRegister,
    },
    J {
        offset: CJImmediate,
    },
    BEQZ {
        src: CIRegister,
        offset: CBImmediate,
    },
    BNEZ {
        src: CIRegister,
        offset: CBImmediate,
    },
    SLLI {
        dest: IRegister,
        shamt: CShamt,
    },
    FLDSP {
        dest: FRegister,
        offset: CDSPImmediate,
    },
    LWSP {
        dest: IRegister,
        offset: CWSPImmediate,
    },
    LDSP {
        dest: IRegister,
        offset: CDSPImmediate,
    },
    JR {
        src: IRegister,
    },
    MV {
        dest: IRegister,
        src: IRegister,
    },
    EBREAK,
    JALR {
        src: IRegister,
    },
    ADD {
        dest: IRegister,
        src: IRegister,
    },
    FSDSP {
        src: FRegister,
        offset: CSDSPImmediate,
    },
    SWSP {
        src: IRegister,
        offset: CSWSPImmediate,
    },
    SDSP {
        src: IRegister,
        offset: CSDSPImmediate,
    },
}

impl Display for CInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CInstruction::ADDI4SPN { dest, imm } => write!(f, "c.addi4spn {dest},{imm}"),
            CInstruction::FLD {
                dest: rd,
                base,
                offset,
            } => write!(f, "c.fld {rd},{offset}({base})"),
            CInstruction::LW {
                dest: rd,
                base,
                offset,
            } => write!(f, "c.lw {rd},{offset}({base})"),
            CInstruction::LD {
                dest: rd,
                base,
                offset,
            } => write!(f, "c.ld {rd},{offset}({base})"),
            CInstruction::FSD { src, base, offset } => write!(f, "c.fsd {src},{offset}({base})"),
            CInstruction::SW { src, base, offset } => write!(f, "c.sw {src},{offset}({base})"),
            CInstruction::SD { src, base, offset } => write!(f, "c.sd {src},{offset}({base})"),
            CInstruction::ADDI { dest, imm } => write!(f, "c.addi {dest},{imm}"),
            CInstruction::ADDIW { dest, imm } => write!(f, "c.addiw {dest},{imm}"),
            CInstruction::LI { dest, imm } => write!(f, "c.li {dest},{imm}"),
            CInstruction::ADDI16SP { imm } => write!(f, "c.addi16sp {imm}"),
            CInstruction::LUI { dest, imm } => write!(f, "c.lui {dest},{imm}"),
            CInstruction::SRLI { dest, shamt } => write!(f, "c.srli {dest},{shamt}"),
            CInstruction::SRAI { dest, shamt } => write!(f, "c.srai {dest},{shamt}"),
            CInstruction::ANDI { dest, imm } => write!(f, "c.andi {dest},{imm}"),
            CInstruction::SUB { dest: rd, src: rs2 } => write!(f, "c.sub {rd},{rs2}"),
            CInstruction::XOR { dest: rd, src: rs2 } => write!(f, "c.xor {rd},{rs2}"),
            CInstruction::OR { dest: rd, src: rs2 } => write!(f, "c.or {rd},{rs2}"),
            CInstruction::AND { dest: rd, src: rs2 } => write!(f, "c.and {rd},{rs2}"),
            CInstruction::SUBW { dest: rd, src: rs2 } => write!(f, "c.subw {rd},{rs2}"),
            CInstruction::ADDW { dest: rd, src: rs2 } => write!(f, "c.addw {rd},{rs2}"),
            CInstruction::J { offset } => write!(f, "c.j {offset}"),
            CInstruction::BEQZ { src, offset } => write!(f, "c.beqz {src},{offset}"),
            CInstruction::BNEZ { src, offset } => write!(f, "c.bnez {src},{offset}"),
            CInstruction::SLLI { dest, shamt } => write!(f, "c.slli {dest},{shamt}"),
            CInstruction::FLDSP { dest, offset } => write!(f, "c.fldsp {dest},{offset}"),
            CInstruction::LWSP { dest, offset } => write!(f, "c.lwsp {dest},{offset}"),
            CInstruction::LDSP { dest, offset } => write!(f, "c.ldsp {dest},{offset}"),
            CInstruction::JR { src } => write!(f, "c.jr {src}"),
            CInstruction::MV { dest, src } => write!(f, "c.mv {dest},{src}"),
            CInstruction::EBREAK => write!(f, "c.ebreak"),
            CInstruction::JALR { src } => write!(f, "c.jalr {src}"),
            CInstruction::ADD { dest: rd, src: rs2 } => write!(f, "c.add {rd},{rs2}"),
            CInstruction::FSDSP { src, offset } => write!(f, "c.fsdsp {src},{offset}"),
            CInstruction::SWSP { src, offset } => write!(f, "c.swsp {src},{offset}"),
            CInstruction::SDSP { src, offset } => write!(f, "c.sdsp {src},{offset}"),
        }
    }
}

pub fn decode_compressed_instruction(instruction: u16) -> Result<CInstruction, String> {
    let crs2 = CIRegister::from_int((instruction >> 2) & 0b111);
    let cfrd = CFRegister::from_int((instruction >> 2) & 0b111);

    let crs1 = CIRegister::from_int((instruction >> 7) & 0b111);
    // let frs1 = FRegister::from_int_compressed((instruction >> 7) & 0b111);

    let ciimmediate = CIImmediate::from_u16(instruction);

    let cshamt = CShamt::from_u16(instruction);

    let rd = IRegister::from_int(((instruction >> 7) & 0b1_1111) as u32);
    let frd = FRegister::from_int(((instruction >> 7) & 0b1_1111) as u32);
    let rs2 = IRegister::from_int(((instruction >> 2) & 0b1_1111) as u32);
    let frs2 = FRegister::from_int(((instruction >> 2) & 0b1_1111) as u32);

    match instruction & 0b11 {
        0b00 => match instruction >> 13 {
            0b000 => {
                let imm = CWideImmediate::from_u16(instruction);
                if imm.val() == 0 {
                    Err("compressed illegal instruction".to_owned())
                } else {
                    Ok(CInstruction::ADDI4SPN { dest: crs2, imm })
                }
            }
            0b001 => Ok(CInstruction::FLD {
                dest: cfrd,
                base: crs1,
                offset: CDImmediate::from_u16(instruction),
            }),
            0b010 => Ok(CInstruction::LW {
                dest: crs2,
                base: crs1,
                offset: CWImmediate::from_u16(instruction),
            }),
            0b011 => Ok(CInstruction::LD {
                dest: crs2,
                base: crs1,
                offset: CDImmediate::from_u16(instruction),
            }),
            0b100 => Err("reserved opcode in C instruction".to_owned()),
            0b101 => Ok(CInstruction::FSD {
                src: cfrd,
                base: crs1,
                offset: CDImmediate::from_u16(instruction),
            }),
            0b110 => Ok(CInstruction::SW {
                src: crs2,
                base: crs1,
                offset: CWImmediate::from_u16(instruction),
            }),
            0b111 => Ok(CInstruction::SD {
                src: crs2,
                base: crs1,
                offset: CDImmediate::from_u16(instruction),
            }),
            _ => unreachable!(),
        },
        0b01 => match instruction >> 13 {
            0b000 => Ok(CInstruction::ADDI {
                dest: rd,
                imm: ciimmediate,
            }),
            0b001 => Ok(CInstruction::ADDIW {
                dest: rd,
                imm: ciimmediate,
            }),
            0b010 => Ok(CInstruction::LI {
                dest: rd,
                imm: ciimmediate,
            }),
            0b011 => {
                if (instruction >> 7) & 0b111 == 2 {
                    let a = (instruction >> 2) & 0b1;
                    let b = (instruction >> 3) & 0b11;
                    let c = (instruction >> 5) & 0b1;
                    let d = (instruction >> 6) & 0b1;
                    let e = (instruction >> 12) & 0b1;
                    println!("a: {a}, b: {b}, c: {c}, d: {d}, e: {e}");
                    let i = ((d << 4) | (a << 5) | (c << 6) | (b << 7) | (e << 9)) as i16;
                    println!("i: {i}");
                    let i2 = (i << 6) >> 6;
                    Ok(CInstruction::ADDI16SP { imm: i2 })
                } else {
                    Ok(CInstruction::LUI {
                        dest: rd,
                        imm: ciimmediate,
                    })
                }
            }
            0b100 => match (instruction >> 10) & 0b11 {
                0b00 => Ok(CInstruction::SRLI {
                    dest: crs1,
                    shamt: cshamt,
                }),
                0b01 => Ok(CInstruction::SRAI {
                    dest: crs1,
                    shamt: cshamt,
                }),
                0b10 => Ok(CInstruction::ANDI {
                    dest: crs1,
                    imm: ciimmediate,
                }),
                0b11 => match ((instruction >> 5) & 0b11, (instruction >> 12) & 0b1) {
                    (0b00, 0b0) => Ok(CInstruction::SUB {
                        dest: crs1,
                        src: crs2,
                    }),
                    (0b01, 0b0) => Ok(CInstruction::XOR {
                        dest: crs1,
                        src: crs2,
                    }),
                    (0b10, 0b0) => Ok(CInstruction::OR {
                        dest: crs1,
                        src: crs2,
                    }),
                    (0b11, 0b0) => Ok(CInstruction::AND {
                        dest: crs1,
                        src: crs2,
                    }),
                    (0b00, 0b1) => Ok(CInstruction::SUBW {
                        dest: crs1,
                        src: crs2,
                    }),
                    (0b01, 0b1) => Ok(CInstruction::ADDW {
                        dest: crs1,
                        src: crs2,
                    }),
                    _ => Err("Reserved instruction".to_owned()),
                },
                _ => unreachable!(),
            },
            0b101 => Ok(CInstruction::J {
                offset: CJImmediate::from_u16(instruction),
            }),
            0b110 => Ok(CInstruction::BEQZ {
                src: crs1,
                offset: CBImmediate::from_u16(instruction),
            }),
            0b111 => Ok(CInstruction::BNEZ {
                src: crs1,
                offset: CBImmediate::from_u16(instruction),
            }),
            _ => unreachable!(),
        },
        0b10 => match instruction >> 13 {
            0b000 => Ok(CInstruction::SLLI {
                dest: rd,
                shamt: cshamt,
            }),
            0b001 => Ok(CInstruction::FLDSP {
                dest: frd,
                offset: CDSPImmediate::from_u16(instruction),
            }),
            0b010 => Ok(CInstruction::LWSP {
                dest: rd,
                offset: CWSPImmediate::from_u16(instruction),
            }),
            0b011 => Ok(CInstruction::LDSP {
                dest: rd,
                offset: CDSPImmediate::from_u16(instruction),
            }),
            0b100 => {
                match (
                    (instruction >> 12) & 0b1,
                    (instruction >> 7) & 0b1_1111,
                    (instruction >> 2) & 0b1_1111,
                ) {
                    (0, _, 0) => Ok(CInstruction::JR { src: rd }),
                    (0, _, _) => Ok(CInstruction::MV { dest: rd, src: rs2 }),
                    (1, 0, 0) => Ok(CInstruction::EBREAK),
                    (1, _, 0) => Ok(CInstruction::JALR { src: rd }),
                    (1, _, _) => Ok(CInstruction::ADD { dest: rd, src: rs2 }),
                    _ => unreachable!(),
                }
            }
            0b101 => Ok(CInstruction::FSDSP {
                src: frs2,
                offset: CSDSPImmediate::from_u16(instruction),
            }),
            0b110 => Ok(CInstruction::SWSP {
                src: rs2,
                offset: CSWSPImmediate::from_u16(instruction),
            }),
            0b111 => Ok(CInstruction::SDSP {
                src: rs2,
                offset: CSDSPImmediate::from_u16(instruction),
            }),
            _ => unreachable!(),
        },
        0b11 => Err("attempting to decode larger instruction as though it were 16 bits".to_owned()),
        _ => unreachable!(),
    }
}

impl CInstruction {
    pub fn disassemble(instruction: &CInstruction) -> String {
        format!("{}", instruction)
    }

    pub fn assemble_line(mnemonics: &[&str], operands: Vec<&str>) -> Result<CInstruction, String> {
        match mnemonics[0] {
            "addi4spn" => {
                if operands.len() != 2 {
                    Err("c.addi4spn requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::ADDI4SPN {
                        dest: CIRegister::from_string(operands[0])?,
                        imm: CWideImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "fld" => {
                if operands.len() != 2 {
                    Err("c.fld requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::FLD {
                        dest: CFRegister::from_string(operands[0])?,
                        base,
                        offset: CDImmediate::from_val(imm),
                    })
                }
            }
            "lw" => {
                if operands.len() != 2 {
                    Err("c.lw requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::LW {
                        dest: CIRegister::from_string(operands[0])?,
                        base,
                        offset: CWImmediate::from_val(imm),
                    })
                }
            }
            "ld" => {
                if operands.len() != 2 {
                    Err("c.ld requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::LD {
                        dest: CIRegister::from_string(operands[0])?,
                        base,
                        offset: CDImmediate::from_val(imm),
                    })
                }
            }
            "fsd" => {
                if operands.len() != 2 {
                    Err("c.fsd requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::FSD {
                        src: CFRegister::from_string(operands[0])?,
                        base,
                        offset: CDImmediate::from_val(imm),
                    })
                }
            }
            "sw" => {
                if operands.len() != 2 {
                    Err("c.sw requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::SW {
                        src: CIRegister::from_string(operands[0])?,
                        base,
                        offset: CWImmediate::from_val(imm),
                    })
                }
            }
            "sd" => {
                if operands.len() != 2 {
                    Err("c.sd requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::SD {
                        src: CIRegister::from_string(operands[0])?,
                        base,
                        offset: CDImmediate::from_val(imm),
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
                    if i > 2i64.pow(9) - 1 || i < -2i64.pow(9) {
                        panic!("attempted to construct out of range CWImmediate")
                    }
                    if i % 16 != 0 {
                        panic!("attempted to construct non multiple of 4 CWImmediate")
                    }
                    Ok(CInstruction::ADDI16SP { imm: i as i16 })
                }
            }
            "lui" => ci_assemble!(LUI),
            "srli" => {
                if operands.len() != 2 {
                    Err("c.srli requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SRLI {
                        dest: CIRegister::from_string(operands[0])?,
                        shamt: CShamt::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "srai" => {
                if operands.len() != 2 {
                    Err("c.srai requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SRAI {
                        dest: CIRegister::from_string(operands[0])?,
                        shamt: CShamt::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "andi" => {
                if operands.len() != 2 {
                    Err("c.andi requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::ANDI {
                        dest: CIRegister::from_string(operands[0])?,
                        imm: CIImmediate::from_val(parse_int(operands[1])?),
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
                        offset: CJImmediate::from_val(parse_int(operands[0])?),
                    })
                }
            }
            "beqz" => {
                if operands.len() != 2 {
                    Err("c.beqz requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::BEQZ {
                        src: CIRegister::from_string(operands[0])?,
                        offset: CBImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "bnez" => {
                if operands.len() != 2 {
                    Err("c.bne requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::BNEZ {
                        src: CIRegister::from_string(operands[0])?,
                        offset: CBImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "slli" => {
                if operands.len() != 2 {
                    Err("c.slli requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SLLI {
                        dest: IRegister::from_string(operands[0])?,
                        shamt: CShamt::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "fldsp" => {
                if operands.len() != 2 {
                    Err("c.fldsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::FLDSP {
                        dest: FRegister::from_string(operands[0])?,
                        offset: CDSPImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "ldsp" => {
                if operands.len() != 2 {
                    Err("c.ldsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::LDSP {
                        dest: IRegister::from_string(operands[0])?,
                        offset: CDSPImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "lwsp" => {
                if operands.len() != 2 {
                    Err("c.lwsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::LWSP {
                        dest: IRegister::from_string(operands[0])?,
                        offset: CWSPImmediate::from_val(parse_int(operands[1])?),
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
                        src: FRegister::from_string(operands[0])?,
                        offset: CSDSPImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "swsp" => {
                if operands.len() != 2 {
                    Err("c.swsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SWSP {
                        src: IRegister::from_string(operands[0])?,
                        offset: CSWSPImmediate::from_val(parse_int(operands[1])?),
                    })
                }
            }
            "sdsp" => {
                if operands.len() != 2 {
                    Err("c.sdsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SDSP {
                        src: IRegister::from_string(operands[0])?,
                        offset: CSDSPImmediate::from_val(parse_int(operands[1])?),
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

    // expands a compressed instruction to its 32 bit form
    pub fn expand(&self) -> Instruction {
        match self {
            CInstruction::ADDI4SPN { dest, imm } => Instruction::ADDI {
                dest: dest.expand(),
                src: IRegister::StackPointer,
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::FLD { .. } => todo!(), // needs unimplemented double extension
            CInstruction::LW { dest, base, offset } => Instruction::LW {
                dest: dest.expand(),
                base: base.expand(),
                offset: IImmediate::from_val(offset.val()),
            },
            CInstruction::LD { dest, base, offset } => Instruction::LD {
                dest: dest.expand(),
                base: base.expand(),
                offset: IImmediate::from_val(offset.val()),
            },
            CInstruction::FSD { .. } => todo!(), // needs unimplemented double extension
            CInstruction::SW { src, base, offset } => Instruction::SW {
                src: src.expand(),
                base: base.expand(),
                offset: SImmediate::from_val(offset.val()),
            },
            CInstruction::SD { src, base, offset } => Instruction::SD {
                src: src.expand(),
                base: base.expand(),
                offset: SImmediate::from_val(offset.val()),
            },
            CInstruction::ADDI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: *dest,
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::ADDIW { dest, imm } => Instruction::ADDIW {
                dest: *dest,
                src: *dest,
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::LI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: IRegister::Zero,
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::ADDI16SP { imm } => Instruction::ADDI {
                dest: IRegister::StackPointer,
                src: IRegister::StackPointer,
                imm: IImmediate::from_val(*imm as i64),
            },
            CInstruction::LUI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: IRegister::Zero,
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::SRLI { dest, shamt } => Instruction::SRLI {
                dest: dest.expand(),
                src: dest.expand(),
                shamt: Shamt::from_val(shamt.val()),
            },
            CInstruction::SRAI { dest, shamt } => Instruction::SRAI {
                dest: dest.expand(),
                src: dest.expand(),
                shamt: Shamt::from_val(shamt.val()),
            },
            CInstruction::ANDI { dest, imm } => Instruction::ANDI {
                dest: dest.expand(),
                src: dest.expand(),
                imm: IImmediate::from_val(imm.val()),
            },
            CInstruction::SUB { dest, src } => Instruction::SUB {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::XOR { dest, src } => Instruction::XOR {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::OR { dest, src } => Instruction::OR {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::AND { dest, src } => Instruction::AND {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::SUBW { dest, src } => Instruction::SUBW {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::ADDW { dest, src } => Instruction::ADDW {
                dest: dest.expand(),
                src1: dest.expand(),
                src2: src.expand(),
            },
            CInstruction::J { offset } => Instruction::JAL {
                dest: IRegister::Zero,
                offset: JImmediate::from_val(offset.val()),
            },
            CInstruction::BEQZ { src, offset } => Instruction::BEQ {
                src1: src.expand(),
                src2: IRegister::Zero,
                offset: BImmediate::from_val(offset.val()),
            },
            CInstruction::BNEZ { src, offset } => Instruction::BNE {
                src1: src.expand(),
                src2: IRegister::Zero,
                offset: BImmediate::from_val(offset.val()),
            },
            CInstruction::SLLI { dest, shamt } => Instruction::SLLI {
                dest: *dest,
                src: *dest,
                shamt: Shamt::from_val(shamt.val()),
            },
            CInstruction::FLDSP { .. } => todo!(), // needs unimplemented double extension
            CInstruction::LWSP { dest, offset } => Instruction::LW {
                dest: *dest,
                base: IRegister::StackPointer,
                offset: IImmediate::from_val(offset.val()),
            },
            CInstruction::LDSP { dest, offset } => Instruction::LD {
                dest: *dest,
                base: IRegister::StackPointer,
                offset: IImmediate::from_val(offset.val()),
            },
            CInstruction::JR { src } => Instruction::JALR {
                dest: IRegister::Zero,
                base: *src,
                offset: IImmediate::from_val(0),
            },
            CInstruction::MV { dest, src } => Instruction::ADD {
                dest: *dest,
                src1: IRegister::Zero,
                src2: *src,
            },
            CInstruction::EBREAK => Instruction::EBREAK,
            // CInstruction::JALR(rs1) => Instruction::JALR(IRegister::ReturnAddress, *rs1, IImmediate::from_val(0)),
            CInstruction::JALR { .. } => todo!(), // not exactly the same as the expanded version (see manual)
            CInstruction::ADD { dest, src } => Instruction::ADD {
                dest: *dest,
                src1: *dest,
                src2: *src,
            },
            CInstruction::FSDSP { .. } => todo!(), // needs unimplemented double extension
            CInstruction::SWSP { src, offset } => Instruction::SW {
                src: *src,
                base: IRegister::StackPointer,
                offset: SImmediate::from_val(offset.val()),
            },
            CInstruction::SDSP { src, offset } => Instruction::SD {
                src: *src,
                base: IRegister::StackPointer,
                offset: SImmediate::from_val(offset.val()),
            },
        }
    }
}
