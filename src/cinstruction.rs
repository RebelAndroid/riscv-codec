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
    ADDI4SPN(CIRegister, CWideImmediate),
    FLD(CFRegister, CIRegister, CDImmediate),
    LW(CIRegister, CIRegister, CWImmediate),
    LD(CIRegister, CIRegister, CDImmediate),
    FSD(CFRegister, CIRegister, CDImmediate),
    SW(CIRegister, CIRegister, CWImmediate),
    SD(CIRegister, CIRegister, CDImmediate),
    ADDI(IRegister, CIImmediate),
    ADDIW(IRegister, CIImmediate),
    LI(IRegister, CIImmediate),
    ADDI16SP(i16),
    LUI(IRegister, CIImmediate),
    SRLI(CIRegister, CShamt),
    SRAI(CIRegister, CShamt),
    ANDI(CIRegister, CIImmediate),
    SUB { rd: CIRegister, rs2: CIRegister },
    XOR { rd: CIRegister, rs2: CIRegister },
    OR { rd: CIRegister, rs2: CIRegister },
    AND { rd: CIRegister, rs2: CIRegister },
    SUBW { rd: CIRegister, rs2: CIRegister },
    ADDW { rd: CIRegister, rs2: CIRegister },
    J(CJImmediate),
    BEQZ(CIRegister, CBImmediate),
    BNEZ(CIRegister, CBImmediate),
    SLLI(IRegister, CShamt),
    FLDSP(FRegister, CDSPImmediate),
    LWSP(IRegister, CWSPImmediate),
    LDSP(IRegister, CDSPImmediate),
    JR(IRegister),
    MV { rd: IRegister, rs2: IRegister },
    EBREAK,
    JALR(IRegister),
    ADD { rd: IRegister, rs2: IRegister },
    FSDSP(FRegister, CSDSPImmediate),
    SWSP(IRegister, CSWSPImmediate),
    SDSP(IRegister, CSDSPImmediate),
}

impl Display for CInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CInstruction::ADDI4SPN(rd, imm) => write!(f, "c.addi4spn {rd},{imm}"),
            CInstruction::FLD(rd, rs1, imm) => write!(f, "c.fld {rd},{imm}({rs1})"),
            CInstruction::LW(rd, rs1, imm) => write!(f, "c.lw {rd},{imm}({rs1})"),
            CInstruction::LD(rd, rs1, imm) => write!(f, "c.ld {rd},{imm}({rs1})"),
            CInstruction::FSD(rs2, rs1, imm) => write!(f, "c.fsd {rs2},{imm}({rs1})"),
            CInstruction::SW(rs2, rs1, imm) => write!(f, "c.sw {rs2},{imm}({rs1})"),
            CInstruction::SD(rs2, rs1, imm) => write!(f, "c.sd {rs2},{imm}({rs1})"),
            CInstruction::ADDI(rd, imm) => write!(f, "c.addi {rd},{imm}"),
            CInstruction::ADDIW(rd, imm) => write!(f, "c.addiw {rd},{imm}"),
            CInstruction::LI(rd, imm) => write!(f, "c.li {rd},{imm}"),
            CInstruction::ADDI16SP(imm) => write!(f, "c.addi16sp {imm}"),
            CInstruction::LUI(rd, imm) => write!(f, "c.lui {rd},{imm}"),
            CInstruction::SRLI(rd, imm) => write!(f, "c.srli {rd},{imm}"),
            CInstruction::SRAI(rd, imm) => write!(f, "c.srai {rd},{imm}"),
            CInstruction::ANDI(rd, imm) => write!(f, "c.andi {rd},{imm}"),
            CInstruction::SUB { rd, rs2 } => write!(f, "c.sub {rd},{rs2}"),
            CInstruction::XOR { rd, rs2 } => write!(f, "c.xor {rd},{rs2}"),
            CInstruction::OR { rd, rs2 } => write!(f, "c.or {rd},{rs2}"),
            CInstruction::AND { rd, rs2 } => write!(f, "c.and {rd},{rs2}"),
            CInstruction::SUBW { rd, rs2 } => write!(f, "c.subw {rd},{rs2}"),
            CInstruction::ADDW { rd, rs2 } => write!(f, "c.addw {rd},{rs2}"),
            CInstruction::J(imm) => write!(f, "c.j {imm}"),
            CInstruction::BEQZ(rd, imm) => write!(f, "c.beqz {rd},{imm}"),
            CInstruction::BNEZ(rd, imm) => write!(f, "c.bnez {rd},{imm}"),
            CInstruction::SLLI(rd, imm) => write!(f, "c.slli {rd},{imm}"),
            CInstruction::FLDSP(rd, imm) => write!(f, "c.fldsp {rd},{imm}"),
            CInstruction::LWSP(rd, imm) => write!(f, "c.lwsp {rd},{imm}"),
            CInstruction::LDSP(rd, imm) => write!(f, "c.ldsp {rd},{imm}"),
            CInstruction::JR(rd) => write!(f, "c.jr {rd}"),
            CInstruction::MV { rd, rs2 } => write!(f, "c.mv {rd},{rs2}"),
            CInstruction::EBREAK => write!(f, "c.ebreak"),
            CInstruction::JALR(rs1) => write!(f, "c.jalr {rs1}"),
            CInstruction::ADD { rd, rs2 } => write!(f, "c.add {rd},{rs2}"),
            CInstruction::FSDSP(frd, imm) => write!(f, "c.fsdsp {frd},{imm}"),
            CInstruction::SWSP(rd, imm) => write!(f, "c.swsp {rd},{imm}"),
            CInstruction::SDSP(rd, imm) => write!(f, "c.sdsp {rd},{imm}"),
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
                    Ok(CInstruction::ADDI4SPN(crs2, imm))
                }
            }
            0b001 => Ok(CInstruction::FLD(
                cfrd,
                crs1,
                CDImmediate::from_u16(instruction),
            )),
            0b010 => Ok(CInstruction::LW(
                crs2,
                crs1,
                CWImmediate::from_u16(instruction),
            )),
            0b011 => Ok(CInstruction::LD(
                crs2,
                crs1,
                CDImmediate::from_u16(instruction),
            )),
            0b100 => Err("reserved opcode in C instruction".to_owned()),
            0b101 => Ok(CInstruction::FSD(
                cfrd,
                crs1,
                CDImmediate::from_u16(instruction),
            )),
            0b110 => Ok(CInstruction::SW(
                crs2,
                crs1,
                CWImmediate::from_u16(instruction),
            )),
            0b111 => Ok(CInstruction::SD(
                crs2,
                crs1,
                CDImmediate::from_u16(instruction),
            )),
            _ => unreachable!(),
        },
        0b01 => match instruction >> 13 {
            0b000 => Ok(CInstruction::ADDI(rd, ciimmediate)),
            0b001 => Ok(CInstruction::ADDIW(rd, ciimmediate)),
            0b010 => Ok(CInstruction::LI(rd, ciimmediate)),
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
                    Ok(CInstruction::ADDI16SP(i2))
                } else {
                    Ok(CInstruction::LUI(rd, ciimmediate))
                }
            }
            0b100 => match (instruction >> 10) & 0b11 {
                0b00 => Ok(CInstruction::SRLI(crs1, cshamt)),
                0b01 => Ok(CInstruction::SRAI(crs1, cshamt)),
                0b10 => Ok(CInstruction::ANDI(crs1, ciimmediate)),
                0b11 => match ((instruction >> 5) & 0b11, (instruction >> 12) & 0b1) {
                    (0b00, 0b0) => Ok(CInstruction::SUB {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    (0b01, 0b0) => Ok(CInstruction::XOR {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    (0b10, 0b0) => Ok(CInstruction::OR {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    (0b11, 0b0) => Ok(CInstruction::AND {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    (0b00, 0b1) => Ok(CInstruction::SUBW {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    (0b01, 0b1) => Ok(CInstruction::ADDW {
                        rd: crs1,
                        rs2: crs2,
                    }),
                    _ => Err("Reserved instruction".to_owned()),
                },
                _ => unreachable!(),
            },
            0b101 => Ok(CInstruction::J(CJImmediate::from_u16(instruction))),
            0b110 => Ok(CInstruction::BEQZ(crs1, CBImmediate::from_u16(instruction))),
            0b111 => Ok(CInstruction::BNEZ(crs1, CBImmediate::from_u16(instruction))),
            _ => unreachable!(),
        },
        0b10 => match instruction >> 13 {
            0b000 => Ok(CInstruction::SLLI(rd, cshamt)),
            0b001 => Ok(CInstruction::FLDSP(
                frd,
                CDSPImmediate::from_u16(instruction),
            )),
            0b010 => Ok(CInstruction::LWSP(rd, CWSPImmediate::from_u16(instruction))),
            0b011 => Ok(CInstruction::LDSP(rd, CDSPImmediate::from_u16(instruction))),
            0b100 => {
                match (
                    (instruction >> 12) & 0b1,
                    (instruction >> 7) & 0b1_1111,
                    (instruction >> 2) & 0b1_1111,
                ) {
                    (0, _, 0) => Ok(CInstruction::JR(rd)),
                    (0, _, _) => Ok(CInstruction::MV { rd, rs2 }),
                    (1, 0, 0) => Ok(CInstruction::EBREAK),
                    (1, _, 0) => Ok(CInstruction::JALR(rd)),
                    (1, _, _) => Ok(CInstruction::ADD { rd, rs2 }),
                    _ => unreachable!(),
                }
            }
            0b101 => Ok(CInstruction::FSDSP(
                frs2,
                CSDSPImmediate::from_u16(instruction),
            )),
            0b110 => Ok(CInstruction::SWSP(
                rs2,
                CSWSPImmediate::from_u16(instruction),
            )),
            0b111 => Ok(CInstruction::SDSP(
                rs2,
                CSDSPImmediate::from_u16(instruction),
            )),
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
                    Ok(CInstruction::ADDI4SPN(
                        CIRegister::from_string(operands[0])?,
                        CWideImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "fld" => {
                if operands.len() != 2 {
                    Err("c.fld requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::FLD(
                        CFRegister::from_string(operands[0])?,
                        base,
                        CDImmediate::from_val(imm),
                    ))
                }
            }
            "lw" => {
                if operands.len() != 2 {
                    Err("c.lw requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::LW(
                        CIRegister::from_string(operands[0])?,
                        base,
                        CWImmediate::from_val(imm),
                    ))
                }
            }
            "ld" => {
                if operands.len() != 2 {
                    Err("c.ld requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::LD(
                        CIRegister::from_string(operands[0])?,
                        base,
                        CDImmediate::from_val(imm),
                    ))
                }
            }
            "fsd" => {
                if operands.len() != 2 {
                    Err("c.fsd requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::FSD(
                        CFRegister::from_string(operands[0])?,
                        base,
                        CDImmediate::from_val(imm),
                    ))
                }
            }
            "sw" => {
                if operands.len() != 2 {
                    Err("c.sw requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::SW(
                        CIRegister::from_string(operands[0])?,
                        base,
                        CWImmediate::from_val(imm),
                    ))
                }
            }
            "sd" => {
                if operands.len() != 2 {
                    Err("c.sd requires 2 operands".to_owned())
                } else {
                    let (base, imm) = parse_address_expression_compressed(operands[1])?;
                    Ok(CInstruction::SD(
                        CIRegister::from_string(operands[0])?,
                        base,
                        CDImmediate::from_val(imm),
                    ))
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
                    Ok(CInstruction::ADDI16SP(i as i16))
                }
            }
            "lui" => ci_assemble!(LUI),
            "srli" => {
                if operands.len() != 2 {
                    Err("c.srli requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SRLI(
                        CIRegister::from_string(operands[0])?,
                        CShamt::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "srai" => {
                if operands.len() != 2 {
                    Err("c.srai requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SRAI(
                        CIRegister::from_string(operands[0])?,
                        CShamt::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "andi" => {
                if operands.len() != 2 {
                    Err("c.andi requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::ANDI(
                        CIRegister::from_string(operands[0])?,
                        CIImmediate::from_val(parse_int(operands[1])?),
                    ))
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
                    Ok(CInstruction::J(CJImmediate::from_val(parse_int(
                        operands[0],
                    )?)))
                }
            }
            "beqz" => {
                if operands.len() != 2 {
                    Err("c.beqz requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::BEQZ(
                        CIRegister::from_string(operands[0])?,
                        CBImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "bnez" => {
                if operands.len() != 2 {
                    Err("c.bne requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::BNEZ(
                        CIRegister::from_string(operands[0])?,
                        CBImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "slli" => {
                if operands.len() != 2 {
                    Err("c.slli requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SLLI(
                        IRegister::from_string(operands[0])?,
                        CShamt::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "fldsp" => {
                if operands.len() != 2 {
                    Err("c.fldsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::FLDSP(
                        FRegister::from_string(operands[0])?,
                        CDSPImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "ldsp" => {
                if operands.len() != 2 {
                    Err("c.ldsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::LDSP(
                        IRegister::from_string(operands[0])?,
                        CDSPImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "lwsp" => {
                if operands.len() != 2 {
                    Err("c.lwsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::LWSP(
                        IRegister::from_string(operands[0])?,
                        CWSPImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "jr" => {
                if operands.len() != 1 {
                    Err("c.jr requires 1 operand".to_owned())
                } else {
                    Ok(CInstruction::JR(IRegister::from_string(operands[0])?))
                }
            }
            "jalr" => {
                if operands.len() != 1 {
                    Err("c.jalr requires 1 operand".to_owned())
                } else {
                    Ok(CInstruction::JALR(IRegister::from_string(operands[0])?))
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
                        rd: IRegister::from_string(operands[0])?,
                        rs2: IRegister::from_string(operands[1])?,
                    })
                }
            }
            "fsdsp" => {
                if operands.len() != 2 {
                    Err("c.fsdsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::FSDSP(
                        FRegister::from_string(operands[0])?,
                        CSDSPImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "swsp" => {
                if operands.len() != 2 {
                    Err("c.swsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SWSP(
                        IRegister::from_string(operands[0])?,
                        CSWSPImmediate::from_val(parse_int(operands[1])?),
                    ))
                }
            }
            "sdsp" => {
                if operands.len() != 2 {
                    Err("c.sdsp requires 2 operands".to_owned())
                } else {
                    Ok(CInstruction::SDSP(
                        IRegister::from_string(operands[0])?,
                        CSDSPImmediate::from_val(parse_int(operands[1])?),
                    ))
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
            CInstruction::ADDI4SPN(rd, imm) => Instruction::ADDI(
                rd.expand(),
                IRegister::StackPointer,
                IImmediate::from_val(imm.val()),
            ),
            CInstruction::FLD(_, _, _) => todo!(), // needs unimplemented double extension
            CInstruction::LW(rd, rs1, imm) => {
                Instruction::LW(rd.expand(), rs1.expand(), IImmediate::from_val(imm.val()))
            }
            CInstruction::LD(rd, rs1, imm) => {
                Instruction::LD(rd.expand(), rs1.expand(), IImmediate::from_val(imm.val()))
            }
            CInstruction::FSD(_, _, _) => todo!(), // needs unimplemented double extension
            CInstruction::SW(rs2, rs1, imm) => {
                Instruction::SW(rs1.expand(), rs2.expand(), SImmediate::from_val(imm.val()))
            }
            CInstruction::SD(rs2, rs1, imm) => {
                Instruction::SD(rs1.expand(), rs2.expand(), SImmediate::from_val(imm.val()))
            }
            CInstruction::ADDI(rd, imm) => {
                Instruction::ADDI(*rd, *rd, IImmediate::from_val(imm.val()))
            }
            CInstruction::ADDIW(rd, imm) => {
                Instruction::ADDIW(*rd, *rd, IImmediate::from_val(imm.val()))
            }
            CInstruction::LI(rd, imm) => {
                Instruction::ADDI(*rd, IRegister::Zero, IImmediate::from_val(imm.val()))
            }
            CInstruction::ADDI16SP(imm) => Instruction::ADDI(
                IRegister::StackPointer,
                IRegister::StackPointer,
                IImmediate::from_val(*imm as i64),
            ),
            CInstruction::LUI(rd, imm) => {
                Instruction::ADDI(*rd, IRegister::Zero, IImmediate::from_val(imm.val()))
            }
            CInstruction::SRLI(rd, shamt) => {
                Instruction::SRLI(rd.expand(), rd.expand(), Shamt::from_val(shamt.val()))
            }
            CInstruction::SRAI(rd, shamt) => {
                Instruction::SRAI(rd.expand(), rd.expand(), Shamt::from_val(shamt.val()))
            }
            CInstruction::ANDI(rd, imm) => {
                Instruction::ANDI(rd.expand(), rd.expand(), IImmediate::from_val(imm.val()))
            }
            CInstruction::SUB { rd, rs2 } => {
                Instruction::SUB(rd.expand(), rd.expand(), rs2.expand())
            }
            CInstruction::XOR { rd, rs2 } => {
                Instruction::XOR(rd.expand(), rd.expand(), rs2.expand())
            }
            CInstruction::OR { rd, rs2 } => Instruction::OR(rd.expand(), rd.expand(), rs2.expand()),
            CInstruction::AND { rd, rs2 } => {
                Instruction::AND(rd.expand(), rd.expand(), rs2.expand())
            }
            CInstruction::SUBW { rd, rs2 } => {
                Instruction::SUBW(rd.expand(), rd.expand(), rs2.expand())
            }
            CInstruction::ADDW { rd, rs2 } => {
                Instruction::ADDW(rd.expand(), rd.expand(), rs2.expand())
            }
            CInstruction::J(imm) => {
                Instruction::JAL(IRegister::Zero, JImmediate::from_val(imm.val()))
            }
            CInstruction::BEQZ(rs1, imm) => Instruction::BEQ(
                rs1.expand(),
                IRegister::Zero,
                BImmediate::from_val(imm.val()),
            ),
            CInstruction::BNEZ(rs1, imm) => Instruction::BNE(
                rs1.expand(),
                IRegister::Zero,
                BImmediate::from_val(imm.val()),
            ),
            CInstruction::SLLI(rd, shamt) => {
                Instruction::SLLI(*rd, *rd, Shamt::from_val(shamt.val()))
            }
            CInstruction::FLDSP(_, _) => todo!(), // needs unimplemented double extension
            CInstruction::LWSP(rd, imm) => Instruction::LW(
                *rd,
                IRegister::StackPointer,
                IImmediate::from_val(imm.val()),
            ),
            CInstruction::LDSP(rd, imm) => Instruction::LD(
                *rd,
                IRegister::StackPointer,
                IImmediate::from_val(imm.val()),
            ),
            CInstruction::JR(rs1) => {
                Instruction::JALR(IRegister::Zero, *rs1, IImmediate::from_val(0))
            }
            CInstruction::MV { rd, rs2 } => Instruction::ADD(*rd, IRegister::Zero, *rs2),
            CInstruction::EBREAK => Instruction::EBREAK,
            // CInstruction::JALR(rs1) => Instruction::JALR(IRegister::ReturnAddress, *rs1, IImmediate::from_val(0)),
            CInstruction::JALR(_) => todo!(), // not exactly the same as the expanded version (see manual)
            CInstruction::ADD { rd, rs2 } => Instruction::ADD(*rd, *rd, *rs2),
            CInstruction::FSDSP(_, _) => todo!(), // needs unimplemented double extension
            CInstruction::SWSP(rs2, imm) => Instruction::SW(
                *rs2,
                IRegister::StackPointer,
                SImmediate::from_val(imm.val()),
            ),
            CInstruction::SDSP(rs2, imm) => Instruction::SD(
                *rs2,
                IRegister::StackPointer,
                SImmediate::from_val(imm.val()),
            ),
        }
    }
}
