use riscv_disassembler::cinstruction::{CInstruction, decode_compressed_instruction};
use riscv_disassembler::immediates::{
    CBImmediate, CDImmediate, CDSPImmediate, CIImmediate, CJImmediate, CSDSPImmediate,
    CSWSPImmediate, CShamt, CWImmediate, CWSPImmediate, CWideImmediate,
};
use riscv_disassembler::instruction::assemble_line;
use riscv_disassembler::register::{CFRegister, CIRegister, FRegister, IRegister};

#[test]
fn add_4_immediate_stack_pointer() {
    // check assembler
    let i = assemble_line("c.addi4spn a0,280").unwrap().c();
    let expected = CInstruction::ADDI4SPN {
        dest: CIRegister::A0,
        imm: CWideImmediate::try_from(280).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x0a28).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_load_double() {
    // check assembler
    let i = assemble_line("c.fld fa0,152(a1)").unwrap().c();
    let expected = CInstruction::FLD {
        dest: CFRegister::FA0,
        base: CIRegister::A1,
        offset: CDImmediate::try_from(152).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x2dc8).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_word() {
    // check assembler
    let i = assemble_line("c.lw a2,0(fp)").unwrap().c();
    let expected = CInstruction::LW {
        dest: CIRegister::A2,
        base: CIRegister::FramePointer,
        offset: CWImmediate::try_from(0).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x4010).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_doubleword() {
    // check assembler
    let i = assemble_line("c.ld a3,248(a4)").unwrap().c();
    let expected = CInstruction::LD {
        dest: CIRegister::A3,
        base: CIRegister::A4,
        offset: CDImmediate::try_from(248).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x7f74).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_store_double() {
    // check assembler
    let i = assemble_line("c.fsd fs0,8(a5)").unwrap().c();
    let expected = CInstruction::FSD {
        src: CFRegister::FS0,
        base: CIRegister::A5,
        offset: CDImmediate::try_from(8).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xa780).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_word() {
    // check assembler
    let i = assemble_line("c.sw a2,124(a2)").unwrap().c();
    let expected = CInstruction::SW {
        src: CIRegister::A2,
        base: CIRegister::A2,
        offset: CWImmediate::try_from(124).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xde70).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_double() {
    // check assembler
    let i = assemble_line("c.sd a4,248(a3)").unwrap().c();
    let expected = CInstruction::SD {
        src: CIRegister::A4,
        base: CIRegister::A3,
        offset: CDImmediate::try_from(248).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xfef8).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate() {
    // check assembler
    let i = assemble_line("c.addi t1,12").unwrap().c();
    let expected = CInstruction::ADDI {
        dest: IRegister::T1,
        imm: CIImmediate::try_from(12).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x0331).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate_word() {
    // check assembler
    let i = assemble_line("c.addiw s6,31").unwrap().c();
    let expected = CInstruction::ADDIW {
        dest: IRegister::S6,
        imm: CIImmediate::try_from(31).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x2b7d).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_immediate() {
    // check assembler
    let i = assemble_line("c.li t4,-32").unwrap().c();
    let expected = CInstruction::LI {
        dest: IRegister::T4,
        imm: CIImmediate::try_from(-32).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x5e81).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_16_immediate_stack_pointer() {
    // check assembler
    let i = assemble_line("c.addi16sp 80").unwrap().c();
    let expected = CInstruction::ADDI16SP { imm: 80 };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x6161).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_upper_immediate() {
    // check assembler
    let i = assemble_line("c.lui s9,24").unwrap().c();
    let expected = CInstruction::LUI {
        dest: IRegister::S9,
        imm: CIImmediate::try_from(24).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x6ce1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical_immediate() {
    // check assembler
    let i = assemble_line("c.srli a2,35").unwrap().c();
    let expected = CInstruction::SRLI {
        dest: CIRegister::A2,
        shamt: CShamt::try_from(35).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x920d).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic_immediate() {
    // check assembler
    let i = assemble_line("c.srai fp,63").unwrap().c();
    let expected = CInstruction::SRAI {
        dest: CIRegister::FramePointer,
        shamt: CShamt::try_from(63).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x947d).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn and_immediate() {
    // check assembler
    let i = assemble_line("c.andi s1,-1").unwrap().c();
    let expected = CInstruction::ANDI {
        dest: CIRegister::S1,
        imm: CIImmediate::try_from(-1).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x98fd).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn subtract() {
    // check assembler
    let i = assemble_line("c.sub a3,a0").unwrap().c();
    let expected = CInstruction::SUB {
        dest: CIRegister::A3,
        src: CIRegister::A0,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x8e89).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn exclusive_or() {
    // check assembler
    let i = assemble_line("c.xor a1,a4").unwrap().c();
    let expected = CInstruction::XOR {
        dest: CIRegister::A1,
        src: CIRegister::A4,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x8db9).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn or() {
    // check assembler
    let i = assemble_line("c.or a5,a2").unwrap().c();
    let expected = CInstruction::OR {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x8fd1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn and() {
    // check assembler
    let i = assemble_line("c.and a5,a2").unwrap().c();
    let expected = CInstruction::AND {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x8ff1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn subtract_word() {
    // check assembler
    let i = assemble_line("c.subw a5,a2").unwrap().c();
    let expected = CInstruction::SUBW {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x9f91).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_word() {
    // check assembler
    let i = assemble_line("c.addw a5,a2").unwrap().c();
    let expected = CInstruction::ADDW {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x9fb1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump() {
    // check assembler
    let i = assemble_line("c.j 72").unwrap().c();
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(72).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xa0a1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

// extra tests for jump because of the complexity of the immediate
#[test]
fn jump2() {
    // check assembler
    let i = assemble_line("c.j 1538").unwrap().c();
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(1538).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xa509).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump3() {
    // check assembler
    let i = assemble_line("c.j -2").unwrap().c();
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(-2).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xbffd).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump4() {
    // check assembler
    let i = assemble_line("c.j -216").unwrap().c();
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(-216).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xb725).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn branch_equal_zero() {
    // check assembler
    let i = assemble_line("c.beqz a5, 72").unwrap().c();
    let expected = CInstruction::BEQZ {
        src: CIRegister::A5,
        offset: CBImmediate::try_from(72).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xc7a1).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn branch_not_equal_zero() {
    // check assembler
    let i = assemble_line("c.bnez a5, -2").unwrap().c();
    let expected = CInstruction::BNEZ {
        src: CIRegister::A5,
        offset: CBImmediate::try_from(-2).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xfffd).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shfit_left_logical_immediate() {
    // check assembler
    let i = assemble_line("c.slli t6, 19").unwrap().c();
    let expected = CInstruction::SLLI {
        dest: IRegister::T6,
        shamt: CShamt::try_from(19).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x0fce).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_load_double_stack_pointer() {
    // check assembler
    let i = assemble_line("c.fldsp ft8, 504").unwrap().c();
    let expected = CInstruction::FLDSP{dest: FRegister::FT8, offset: CDSPImmediate::try_from(504).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x3e7e).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_word_stack_pointer() {
    // check assembler
    let i = assemble_line("c.lwsp s1, 200").unwrap().c();
    let expected = CInstruction::LWSP{dest: IRegister::S1, offset: CWSPImmediate::try_from(200).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x44ae).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_double_stack_pointer() {
    // check assembler
    let i = assemble_line("c.ldsp fp, 400").unwrap().c();
    let expected = CInstruction::LDSP{dest: IRegister::FramePointer, offset: CDSPImmediate::try_from(400).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x645a).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump_register() {
    // check assembler
    let i = assemble_line("c.jr t0").unwrap().c();
    let expected = CInstruction::JR { src: IRegister::T0 };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x8282).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

// this test is called "mv" rather than "move" becaue move is keyword in Rust
#[test]
fn mv() {
    // check assembler
    let i = assemble_line("c.mv a7, t3").unwrap().c();
    let expected = CInstruction::MV {
        dest: IRegister::A7,
        src: IRegister::T3,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x88f2).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn environment_break() {
    // check assembler
    let i = assemble_line("c.ebreak").unwrap().c();
    let expected = CInstruction::EBREAK;
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x9002).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link_register() {
    // check assembler
    let i = assemble_line("c.jalr s5").unwrap().c();
    let expected = CInstruction::JALR { src: IRegister::S5 };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x9a82).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add() {
    // check assembler
    let i = assemble_line("c.add t0, s9").unwrap().c();
    let expected = CInstruction::ADD {
        dest: IRegister::T0,
        src: IRegister::S9,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x92e6).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_store_double_stack_pointer() {
    // check assembler
    let i = assemble_line("c.fsdsp fa2, 136").unwrap().c();
    let expected = CInstruction::FSDSP {
        src: FRegister::FA2,
        offset: CSDSPImmediate::try_from(136).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xa532).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_word_stack_pointer() {
    // check assembler
    let i = assemble_line("c.swsp s7, 56").unwrap().c();
    let expected = CInstruction::SWSP {
        src: IRegister::S7,
        offset: CSWSPImmediate::try_from(56).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xdc5e).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_double_stack_pointer() {
    // check assembler
    let i = assemble_line("c.sdsp gp, 112").unwrap().c();
    let expected = CInstruction::SDSP {
        src: IRegister::GlobalPointer,
        offset: CSDSPImmediate::try_from(112).unwrap(),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0xf88e).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}
