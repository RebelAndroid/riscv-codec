use std::fmt::{Display, Formatter};

use crate::{
    immediates::{
        BImmediate, C16SPImmediate, CBImmediate, CDImmediate, CDSPImmediate, CIImmediate,
        CJImmediate, CSDSPImmediate, CSWSPImmediate, CShamt, CWImmediate, CWSPImmediate,
        CWideImmediate, IImmediate, JImmediate, SImmediate, Shamt,
    },
    instruction::Instruction,
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
        imm: C16SPImmediate,
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

impl CInstruction {
    /// Decodes a u16 into a `CInstruction`.
    pub fn decode(instruction: u16) -> Result<Self, String> {
        let crs2 = CIRegister::try_from((instruction >> 2) & 0b111).unwrap();
        let cfrd = CFRegister::try_from((instruction >> 2) & 0b111).unwrap();

        let crs1 = CIRegister::from((instruction >> 7) & 0b111);

        let ciimmediate = CIImmediate::from_u16(instruction);

        let cshamt = CShamt::from_u16(instruction);

        let rd = IRegister::from_int(((instruction >> 7) & 0b1_1111) as u32);
        let frd = FRegister::try_from(((instruction >> 7) & 0b1_1111) as u32).unwrap();
        let rs2 = IRegister::from_int(((instruction >> 2) & 0b1_1111) as u32);
        let frs2 = FRegister::try_from(((instruction >> 2) & 0b1_1111) as u32).unwrap();

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
                        Ok(CInstruction::ADDI16SP {
                            imm: C16SPImmediate::from_u16(instruction),
                        })
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
            0b11 => {
                Err("attempting to decode larger instruction as though it were 16 bits".to_owned())
            }
            _ => unreachable!(),
        }
    }

    pub fn disassemble(instruction: &CInstruction) -> String {
        format!("{}", instruction)
    }

    /// Converts a compressed instruction into the corresponding 32-bit `Instruction`.
    ///
    /// Note that C.JALR does not have exactly the same effect as the corresponding JALR as described in the manual:
    /// > Strictly speaking, C.JALR does not expand exactly to a base RVI instruction as the value added to the PC to
    /// > form the link address is 2 rather than 4 as in the base ISA, but supporting both offsets of 2 and 4 bytes
    /// > is only a very minor change to the base microarchitecture.
    pub fn expand(&self) -> Instruction {
        match self {
            CInstruction::ADDI4SPN { dest, imm } => Instruction::ADDI {
                dest: dest.expand(),
                src: IRegister::StackPointer,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::FLD { .. } => todo!(), // needs unimplemented double extension
            CInstruction::LW { dest, base, offset } => Instruction::LW {
                dest: dest.expand(),
                base: base.expand(),
                offset: IImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::LD { dest, base, offset } => Instruction::LD {
                dest: dest.expand(),
                base: base.expand(),
                offset: IImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::FSD { .. } => todo!(), // needs unimplemented double extension
            CInstruction::SW { src, base, offset } => Instruction::SW {
                src: src.expand(),
                base: base.expand(),
                offset: SImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::SD { src, base, offset } => Instruction::SD {
                src: src.expand(),
                base: base.expand(),
                offset: SImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::ADDI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: *dest,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::ADDIW { dest, imm } => Instruction::ADDIW {
                dest: *dest,
                src: *dest,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::LI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: IRegister::Zero,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::ADDI16SP { imm } => Instruction::ADDI {
                dest: IRegister::StackPointer,
                src: IRegister::StackPointer,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::LUI { dest, imm } => Instruction::ADDI {
                dest: *dest,
                src: IRegister::Zero,
                imm: IImmediate::try_from(imm.val()).unwrap(),
            },
            CInstruction::SRLI { dest, shamt } => Instruction::SRLI {
                dest: dest.expand(),
                src: dest.expand(),
                shamt: Shamt::try_from(shamt.val()).unwrap(),
            },
            CInstruction::SRAI { dest, shamt } => Instruction::SRAI {
                dest: dest.expand(),
                src: dest.expand(),
                shamt: Shamt::try_from(shamt.val()).unwrap(),
            },
            CInstruction::ANDI { dest, imm } => Instruction::ANDI {
                dest: dest.expand(),
                src: dest.expand(),
                imm: IImmediate::try_from(imm.val()).unwrap(),
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
                offset: JImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::BEQZ { src, offset } => Instruction::BEQ {
                src1: src.expand(),
                src2: IRegister::Zero,
                offset: BImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::BNEZ { src, offset } => Instruction::BNE {
                src1: src.expand(),
                src2: IRegister::Zero,
                offset: BImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::SLLI { dest, shamt } => Instruction::SLLI {
                dest: *dest,
                src: *dest,
                shamt: Shamt::try_from(shamt.val()).unwrap(),
            },
            CInstruction::FLDSP { .. } => todo!(), // needs unimplemented double extension
            CInstruction::LWSP { dest, offset } => Instruction::LW {
                dest: *dest,
                base: IRegister::StackPointer,
                offset: IImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::LDSP { dest, offset } => Instruction::LD {
                dest: *dest,
                base: IRegister::StackPointer,
                offset: IImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::JR { src } => Instruction::JALR {
                dest: IRegister::Zero,
                base: *src,
                offset: IImmediate::try_from(0).unwrap(),
            },
            CInstruction::MV { dest, src } => Instruction::ADD {
                dest: *dest,
                src1: IRegister::Zero,
                src2: *src,
            },
            CInstruction::EBREAK => Instruction::EBREAK,
            CInstruction::JALR { src } => Instruction::JALR {
                dest: IRegister::ReturnAddress,
                base: *src,
                offset: IImmediate::try_from(0).unwrap(),
            },
            // CInstruction::JALR { .. } => todo!(), // not exactly the same as the expanded version (see manual)
            CInstruction::ADD { dest, src } => Instruction::ADD {
                dest: *dest,
                src1: *dest,
                src2: *src,
            },
            CInstruction::FSDSP { .. } => todo!(), // needs unimplemented double extension
            CInstruction::SWSP { src, offset } => Instruction::SW {
                src: *src,
                base: IRegister::StackPointer,
                offset: SImmediate::try_from(offset.val()).unwrap(),
            },
            CInstruction::SDSP { src, offset } => Instruction::SD {
                src: *src,
                base: IRegister::StackPointer,
                offset: SImmediate::try_from(offset.val()).unwrap(),
            },
        }
    }

    /// Encodes a `CInstruction` into a `u16`.
    pub fn encode(instruction: &CInstruction) -> u16 {
        match instruction {
            CInstruction::ADDI4SPN { dest, imm } => 0b000 << 13 | imm.to_u16() | dest.rs2(),
            CInstruction::FLD { dest, base, offset } => {
                0b001 << 13 | offset.to_u16() | base.rs1() | dest.rs2()
            }
            CInstruction::LW { dest, base, offset } => {
                0b010 << 13 | offset.to_u16() | base.rs1() | dest.rs2()
            }
            CInstruction::LD { dest, base, offset } => {
                0b011 << 13 | offset.to_u16() | base.rs1() | dest.rs2()
            }
            CInstruction::FSD { src, base, offset } => {
                0b101 << 13 | offset.to_u16() | base.rs1() | src.rs2()
            }
            CInstruction::SW { src, base, offset } => {
                0b0110 << 13 | offset.to_u16() | base.rs1() | src.rs2()
            }
            CInstruction::SD { src, base, offset } => {
                0b111 << 13 | offset.to_u16() | base.rs1() | src.rs2()
            }
            CInstruction::ADDI { dest, imm } => {
                0b000 << 13 | imm.to_u16() | dest.rd() as u16 | 0b01
            }
            CInstruction::ADDIW { dest, imm } => {
                0b001 << 13 | imm.to_u16() | dest.rd() as u16 | 0b01
            }
            CInstruction::LI { dest, imm } => 0b010 << 13 | imm.to_u16() | dest.rd() as u16 | 0b01,
            CInstruction::ADDI16SP { imm } => 0b011 << 13 | imm.to_u16() | 0b10 << 7 | 0b01,
            CInstruction::LUI { dest, imm } => 0b011 << 13 | imm.to_u16() | dest.rd() as u16 | 0b01,
            CInstruction::SRLI { dest, shamt } => {
                0b100 << 13 | shamt.to_u16() | 0b00 << 10 | dest.rs1() | 0b01
            }
            CInstruction::SRAI { dest, shamt } => {
                0b100 << 13 | shamt.to_u16() | 0b01 << 10 | dest.rs1() | 0b01
            }
            CInstruction::ANDI { dest, imm } => {
                0b100 << 13 | imm.to_u16() | 0b10 << 10 | dest.rs1() | 0b01
            }
            CInstruction::SUB { dest, src } => {
                0b100 << 13 | 0b11 << 10 | dest.rs1() | 0b00 << 5 | src.rs2() | 0b01
            }
            CInstruction::XOR { dest, src } => {
                0b100 << 13 | 0b11 << 10 | dest.rs1() | 0b01 << 5 | src.rs2() | 0b01
            }
            CInstruction::OR { dest, src } => {
                0b100 << 13 | 0b11 << 10 | dest.rs1() | 0b10 << 5 | src.rs2() | 0b01
            }
            CInstruction::AND { dest, src } => {
                0b100 << 13 | 0b11 << 10 | dest.rs1() | 0b11 << 5 | src.rs2() | 0b01
            }
            CInstruction::SUBW { dest, src } => {
                0b100 << 13 | 1 << 12 | 0b11 << 10 | dest.rs1() | 0b00 << 5 | src.rs2() | 0b01
            }
            CInstruction::ADDW { dest, src } => {
                0b100 << 13 | 1 << 12 | 0b11 << 10 | dest.rs1() | 0b01 << 5 | src.rs2() | 0b01
            }
            CInstruction::J { offset } => 0b101 << 13 | offset.to_u16() | 0b01,
            CInstruction::BEQZ { src, offset } => 0b110 << 13 | offset.to_u16() | src.rs1() | 0b01,
            CInstruction::BNEZ { src, offset } => 0b111 << 13 | offset.to_u16() | src.rs1() | 0b01,
            CInstruction::SLLI { dest, shamt } => {
                0b000 << 13 | shamt.to_u16() | dest.rd() as u16 | 0b10
            }
            CInstruction::FLDSP { dest, offset } => {
                0b001 << 13 | offset.to_u16() | dest.rd() as u16 | 0b10
            }
            CInstruction::LWSP { dest, offset } => {
                0b010 << 13 | offset.to_u16() | dest.rd() as u16 | 0b10
            }
            CInstruction::LDSP { dest, offset } => {
                0b011 << 13 | offset.to_u16() | dest.rd() as u16 | 0b10
            }
            CInstruction::JR { src } => 0b100 << 13 | src.rd() as u16 | 0b10,
            CInstruction::MV { dest, src } => {
                0b100 << 13 | dest.rd() as u16 | (src.rd() >> 5) as u16 | 0b10
            }
            CInstruction::EBREAK => 0b100 << 13 | 0b1 << 12 | 0b10,
            CInstruction::JALR { src } => 0b100 << 13 | 0b1 << 12 | src.rd() as u16 | 0b10,
            CInstruction::ADD { dest, src } => {
                0b100 << 13 | 0b1 << 12 | dest.rd() as u16 | (src.rd() >> 5) as u16 | 0b10
            }
            CInstruction::FSDSP { src, offset } => {
                0b101 << 13 | offset.to_u16() | (src.rd() >> 5) as u16 | 0b10
            }
            CInstruction::SWSP { src, offset } => {
                0b110 << 13 | offset.to_u16() | (src.rd() >> 5) as u16 | 0b10
            }
            CInstruction::SDSP { src, offset } => {
                0b111 << 13 | offset.to_u16() | (src.rd() >> 5) as u16 | 0b10
            }
        }
    }
}
