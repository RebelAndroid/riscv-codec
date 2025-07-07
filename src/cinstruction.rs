use std::fmt::{Display, Formatter};

use proc_macros::ci_assemble;

use crate::{
    immediates::{CBImmediate, CDImmediate, CIImmediate, CShamt, CWImmediate, CWideImmediate},
    instruction::{parse_address_expression_compressed, parse_int},
    register::{CFRegister, CIRegister, IRegister},
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
    FLW(CFRegister, CIRegister, CWImmediate),
    ADDI(IRegister, CIImmediate),
    ADDIW(IRegister, CIImmediate),
    LI(IRegister, CIImmediate),
    ADDI16SP(i16),
    LUI(IRegister, CIImmediate),
    SRLI(CIRegister, CShamt),
    SRAI(CIRegister, CShamt),
    ANDI(CIRegister, CIImmediate),
    SUB(CIRegister, CIRegister),
    XOR(CIRegister, CIRegister),
    OR(CIRegister, CIRegister),
    AND(CIRegister, CIRegister),
    SUBW(CIRegister, CIRegister),
    ADDW(CIRegister, CIRegister),
    J(u16),
    BEQZ(CIRegister, CBImmediate),
    BNEZ(CIRegister, CBImmediate),
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
            CInstruction::FLW(frd, frs1, imm) => write!(f, "c.flw {frd},{imm}({frs1})"),
            CInstruction::ADDI(rd, imm) => write!(f, "c.addi {rd},{imm}"),
            CInstruction::ADDIW(rd, imm) => write!(f, "c.addiw {rd},{imm}"),
            CInstruction::LI(rd, imm) => write!(f, "c.li {rd},{imm}"),
            CInstruction::ADDI16SP(imm) => write!(f, "c.addi16sp {imm}"),
            CInstruction::LUI(rd, imm) => write!(f, "c.lui {rd},{imm}"),
            CInstruction::SRLI(rd, imm) => write!(f, "c.srli {rd},{imm}"),
            CInstruction::SRAI(rd, imm) => write!(f, "c.srai {rd},{imm}"),
            CInstruction::ANDI(rd, imm) => write!(f, "c.andi {rd},{imm}"),
            CInstruction::SUB(rd, rs2) => write!(f, "c.sub {rd},{rs2}"),
            CInstruction::XOR(rd, rs2) => write!(f, "c.xor {rd},{rs2}"),
            CInstruction::OR(rd, rs2) => write!(f, "c.or {rd},{rs2}"),
            CInstruction::AND(rd, rs2) => write!(f, "c.and {rd},{rs2}"),
            CInstruction::SUBW(rd, rs2) => write!(f, "c.subw {rd},{rs2}"),
            CInstruction::ADDW(rd, rs2) => write!(f, "c.addw {rd},{rs2}"),
            CInstruction::J(_) => todo!(),
            CInstruction::BEQZ(rd, imm) => write!(f, "c.beqz {rd},{imm}"),
            CInstruction::BNEZ(rd, imm) => write!(f, "c.bnez {rd},{imm}"),
        }
    }
}

pub fn decode_compressed_instruction(instruction: u16) -> Result<CInstruction, String> {
    let crd = CIRegister::from_int((instruction >> 2) & 0b111);
    let cfrd = CFRegister::from_int((instruction >> 2) & 0b111);

    let crs1 = CIRegister::from_int((instruction >> 7) & 0b111);
    // let frs1 = FRegister::from_int_compressed((instruction >> 7) & 0b111);

    let ciimmediate = CIImmediate::from_u16(instruction);

    let cshamt = CShamt::from_u16(instruction);

    let rd = IRegister::from_int(((instruction >> 7) & 0b1_1111) as u32);
    match instruction & 0b11 {
        0b00 => match instruction >> 13 {
            0b000 => {
                let imm = CWideImmediate::from_u16(instruction);
                if imm.val() == 0 {
                    Err("compressed illegal instruction".to_owned())
                } else {
                    Ok(CInstruction::ADDI4SPN(crd, imm))
                }
            }
            0b001 => Ok(CInstruction::FLD(
                cfrd,
                crs1,
                CDImmediate::from_u16(instruction),
            )),
            0b010 => Ok(CInstruction::LW(
                crd,
                crs1,
                CWImmediate::from_u16(instruction),
            )),
            0b011 => Ok(CInstruction::LD(
                crd,
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
                crd,
                crs1,
                CWImmediate::from_u16(instruction),
            )),
            0b111 => Ok(CInstruction::SD(
                crd,
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
            0b100 => Ok(CInstruction::SRLI(crs1, cshamt)),
            0b101 => todo!(),
            0b110 => todo!(),
            0b111 => todo!(),
            _ => unreachable!(),
        },
        0b10 => todo!(),
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
                    if i > 2i64.pow(9) - 1 || i < -2i64.pow(9)  {
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
                if operands.len() != 2{
                    Err("c.srli requires 2 operands".to_owned())
                }else {
                    Ok(CInstruction::SRLI(CIRegister::from_string(operands[0])?, CShamt::from_val(parse_int(operands[1])?)))
                }
            }
            _ => Err(format!(
                "unknown compressed instruction mnemonic: {}",
                mnemonics[0]
            )),
        }
    }
}
