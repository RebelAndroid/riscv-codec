use riscv_disassembler::cinstruction::{CInstruction, decode_compressed_instruction};
use riscv_disassembler::immediates::{
    CBImmediate, CDImmediate, CIImmediate, CJImmediate, CShamt, CWImmediate, CWideImmediate,
};
use riscv_disassembler::instruction::assemble_line;
use riscv_disassembler::register::{CFRegister, CIRegister, IRegister};

#[test]
fn add_4_immediate_stack_pointer() {
    // check assembler
    let i = assemble_line("c.addi4spn a0,280").unwrap().c();
    let expected = CInstruction::ADDI4SPN(CIRegister::A0, CWideImmediate::from_val(280));
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
    let expected = CInstruction::FLD(CFRegister::FA0, CIRegister::A1, CDImmediate::from_val(152));
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
    let expected = CInstruction::LW(
        CIRegister::A2,
        CIRegister::FramePointer,
        CWImmediate::from_val(0),
    );
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
    let expected = CInstruction::LD(CIRegister::A3, CIRegister::A4, CDImmediate::from_val(248));
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
    let expected = CInstruction::FSD(CFRegister::FS0, CIRegister::A5, CDImmediate::from_val(8));
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
    let expected = CInstruction::SW(CIRegister::A2, CIRegister::A2, CWImmediate::from_val(124));
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
    let expected = CInstruction::SD(CIRegister::A4, CIRegister::A3, CDImmediate::from_val(248));
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
    let expected = CInstruction::ADDI(IRegister::T1, CIImmediate::from_val(12));
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
    let expected = CInstruction::ADDIW(IRegister::S6, CIImmediate::from_val(31));
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
    let expected = CInstruction::LI(IRegister::T4, CIImmediate::from_val(-32));
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
    let expected = CInstruction::ADDI16SP(80);
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
    let expected = CInstruction::LUI(IRegister::S9, CIImmediate::from_val(24));
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
    let expected = CInstruction::SRLI(CIRegister::A2, CShamt::from_val(35));
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
    let expected = CInstruction::SRAI(CIRegister::FramePointer, CShamt::from_val(63));
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
    let expected = CInstruction::ANDI(CIRegister::S1, CIImmediate::from_val(-1));
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
        rd: CIRegister::A3,
        rs2: CIRegister::A0,
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
        rd: CIRegister::A1,
        rs2: CIRegister::A4,
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
        rd: CIRegister::A5,
        rs2: CIRegister::A2,
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
        rd: CIRegister::A5,
        rs2: CIRegister::A2,
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
        rd: CIRegister::A5,
        rs2: CIRegister::A2,
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
        rd: CIRegister::A5,
        rs2: CIRegister::A2,
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
    let expected = CInstruction::J(CJImmediate::from_val(72));
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
    let expected = CInstruction::J(CJImmediate::from_val(1538));
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
    let expected = CInstruction::J(CJImmediate::from_val(-2));
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
    let expected = CInstruction::J(CJImmediate::from_val(-216));
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
    let expected = CInstruction::BEQZ(CIRegister::A5, CBImmediate::from_val(72));
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
    let expected = CInstruction::BNEZ(CIRegister::A5, CBImmediate::from_val(-2));
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
    let expected = CInstruction::SLLI(IRegister::T6, CShamt::from_val(19));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_compressed_instruction(0x0fce).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}