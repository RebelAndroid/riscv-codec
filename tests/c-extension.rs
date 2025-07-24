use riscv_codec::assembly::assemble_line;
use riscv_codec::cinstruction::{
    CInstruction,
};
use riscv_codec::immediates::{
    C16SPImmediate, CBImmediate, CDImmediate, CDSPImmediate, CIImmediate, CJImmediate, CSDSPImmediate, CSWSPImmediate, CShamt, CWImmediate, CWSPImmediate, CWideImmediate
};
use riscv_codec::register::{CFRegister, CIRegister, FRegister, IRegister};

#[test]
fn add_4_immediate_stack_pointer() {
    let expected = CInstruction::ADDI4SPN {
        dest: CIRegister::A0,
        imm: CWideImmediate::try_from(280).unwrap(),
    };
    let bin = 0x0a28;

    // check assembler
    let i = assemble_line("c.addi4spn a0,280").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_load_double() {
    let expected = CInstruction::FLD {
        dest: CFRegister::FA0,
        base: CIRegister::A1,
        offset: CDImmediate::try_from(152).unwrap(),
    };
    let bin = 0x2dc8;

    // check assembler
    let i = assemble_line("c.fld fa0,152(a1)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_word() {
    let expected = CInstruction::LW {
        dest: CIRegister::A2,
        base: CIRegister::FramePointer,
        offset: CWImmediate::try_from(0).unwrap(),
    };
    let bin = 0x4010;

    // check assembler
    let i = assemble_line("c.lw a2,0(fp)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_doubleword() {
    let expected = CInstruction::LD {
        dest: CIRegister::A3,
        base: CIRegister::A4,
        offset: CDImmediate::try_from(248).unwrap(),
    };
    let bin = 0x7f74;

    // check assembler
    let i = assemble_line("c.ld a3,248(a4)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_store_double() {
    let expected = CInstruction::FSD {
        src: CFRegister::FS0,
        base: CIRegister::A5,
        offset: CDImmediate::try_from(8).unwrap(),
    };
    let bin = 0xa780;

    // check assembler
    let i = assemble_line("c.fsd fs0,8(a5)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_word() {
    let expected = CInstruction::SW {
        src: CIRegister::A2,
        base: CIRegister::A2,
        offset: CWImmediate::try_from(124).unwrap(),
    };
    let bin = 0xde70;

    // check assembler
    let i = assemble_line("c.sw a2,124(a2)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_double() {
    let expected = CInstruction::SD {
        src: CIRegister::A4,
        base: CIRegister::A3,
        offset: CDImmediate::try_from(248).unwrap(),
    };
    let bin = 0xfef8;

    // check assembler
    let i = assemble_line("c.sd a4,248(a3)").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate() {
    let expected = CInstruction::ADDI {
        dest: IRegister::T1,
        imm: CIImmediate::try_from(12).unwrap(),
    };
    let bin = 0x0331;
    // check assembler
    let i = assemble_line("c.addi t1,12").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate_word() {
    let expected = CInstruction::ADDIW {
        dest: IRegister::S6,
        imm: CIImmediate::try_from(31).unwrap(),
    };
    let bin = 0x2b7d;

    // check assembler
    let i = assemble_line("c.addiw s6,31").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_immediate() {
    let expected = CInstruction::LI {
        dest: IRegister::T4,
        imm: CIImmediate::try_from(-32).unwrap(),
    };
    let bin = 0x5e81;

    // check assembler
    let i = assemble_line("c.li t4,-32").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_16_immediate_stack_pointer() {
    let expected = CInstruction::ADDI16SP { imm: C16SPImmediate::try_from(80).unwrap() };
    let bin = 0x6161;

    // check assembler
    let i = assemble_line("c.addi16sp 80").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_upper_immediate() {
    let expected = CInstruction::LUI {
        dest: IRegister::S9,
        imm: CIImmediate::try_from(24).unwrap(),
    };
    let bin = 0x6ce1;

    // check assembler
    let i = assemble_line("c.lui s9,24").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical_immediate() {
    let expected = CInstruction::SRLI {
        dest: CIRegister::A2,
        shamt: CShamt::try_from(35).unwrap(),
    };
    let bin = 0x920d;

    // check assembler
    let i = assemble_line("c.srli a2,35").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic_immediate() {
    let expected = CInstruction::SRAI {
        dest: CIRegister::FramePointer,
        shamt: CShamt::try_from(63).unwrap(),
    };
    let bin = 0x947d;

    // check assembler
    let i = assemble_line("c.srai fp,63").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn and_immediate() {
    let expected = CInstruction::ANDI {
        dest: CIRegister::S1,
        imm: CIImmediate::try_from(-1).unwrap(),
    };
    let bin = 0x98fd;

    // check assembler
    let i = assemble_line("c.andi s1,-1").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn subtract() {
    let expected = CInstruction::SUB {
        dest: CIRegister::A3,
        src: CIRegister::A0,
    };
    let bin = 0x8e89;

    // check assembler
    let i = assemble_line("c.sub a3,a0").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn exclusive_or() {
    let expected = CInstruction::XOR {
        dest: CIRegister::A1,
        src: CIRegister::A4,
    };
    let bin = 0x8db9;

    // check assembler
    let i = assemble_line("c.xor a1,a4").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn or() {
    let expected = CInstruction::OR {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    let bin = 0x8fd1;

    // check assembler
    let i = assemble_line("c.or a5,a2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn and() {
    let expected = CInstruction::AND {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    let bin = 0x8ff1;
    // check assembler
    let i = assemble_line("c.and a5,a2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn subtract_word() {
    let expected = CInstruction::SUBW {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    let bin = 0x9f91;

    // check assembler
    let i = assemble_line("c.subw a5,a2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add_word() {
    let expected = CInstruction::ADDW {
        dest: CIRegister::A5,
        src: CIRegister::A2,
    };
    let bin = 0x9fb1;

    // check assembler
    let i = assemble_line("c.addw a5,a2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(0x9fb1).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump() {
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(72).unwrap(),
    };
    let bin = 0xa0a1;

    // check assembler
    let i = assemble_line("c.j 72").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(0xa0a1).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

// extra tests for jump because of the complexity of the immediate
#[test]
fn jump2() {
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(1538).unwrap(),
    };
    let bin = 0xa509;

    // check assembler
    let i = assemble_line("c.j 1538").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump3() {
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(-2).unwrap(),
    };
    let bin = 0xbffd;
    // check assembler
    let i = assemble_line("c.j -2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump4() {
    let expected = CInstruction::J {
        offset: CJImmediate::try_from(-216).unwrap(),
    };
    let bin = 0xb725;

    // check assembler
    let i = assemble_line("c.j -216").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn branch_equal_zero() {
    let expected = CInstruction::BEQZ {
        src: CIRegister::A5,
        offset: CBImmediate::try_from(72).unwrap(),
    };
    let bin = 0xc7a1;

    // check assembler
    let i = assemble_line("c.beqz a5, 72").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn branch_not_equal_zero() {
    let expected = CInstruction::BNEZ {
        src: CIRegister::A5,
        offset: CBImmediate::try_from(-2).unwrap(),
    };
    let bin = 0xfffd;

    // check assembler
    let i = assemble_line("c.bnez a5, -2").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(0xfffd).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn shfit_left_logical_immediate() {
    let expected = CInstruction::SLLI {
        dest: IRegister::T6,
        shamt: CShamt::try_from(19).unwrap(),
    };
    let bin = 0x0fce;

    // check assembler
    let i = assemble_line("c.slli t6, 19").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_load_double_stack_pointer() {
    let expected = CInstruction::FLDSP {
        dest: FRegister::FT8,
        offset: CDSPImmediate::try_from(504).unwrap(),
    };
    let bin = 0x3e7e;

    // check assembler
    let i = assemble_line("c.fldsp ft8, 504").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_word_stack_pointer() {
    let expected = CInstruction::LWSP {
        dest: IRegister::S1,
        offset: CWSPImmediate::try_from(200).unwrap(),
    };
    let bin = 0x44ae;

    // check assembler
    let i = assemble_line("c.lwsp s1, 200").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn load_double_stack_pointer() {
    let expected = CInstruction::LDSP {
        dest: IRegister::FramePointer,
        offset: CDSPImmediate::try_from(400).unwrap(),
    };
    let bin = 0x645a;

    // check assembler
    let i = assemble_line("c.ldsp fp, 400").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump_register() {
    let expected = CInstruction::JR { src: IRegister::T0 };
    let bin = 0x8282;

    // check assembler
    let i = assemble_line("c.jr t0").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

// this test is called "mv" rather than "move" becaue move is keyword in Rust
#[test]
fn mv() {
    let expected = CInstruction::MV {
        dest: IRegister::A7,
        src: IRegister::T3,
    };
    let bin = 0x88f2;

    // check assembler
    let i = assemble_line("c.mv a7, t3").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn environment_break() {
    let expected = CInstruction::EBREAK;
    let bin = 0x9002;

    // check assembler
    let i = assemble_line("c.ebreak").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link_register() {
    let expected = CInstruction::JALR { src: IRegister::S5 };
    let bin = 0x9a82;

    // check assembler
    let i = assemble_line("c.jalr s5").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn add() {
    let expected = CInstruction::ADD {
        dest: IRegister::T0,
        src: IRegister::S9,
    };
    let bin = 0x92e6;

    // check assembler
    let i = assemble_line("c.add t0, s9").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn float_store_double_stack_pointer() {
    let expected = CInstruction::FSDSP {
        src: FRegister::FA2,
        offset: CSDSPImmediate::try_from(136).unwrap(),
    };
    let bin = 0xa532;

    // check assembler
    let i = assemble_line("c.fsdsp fa2, 136").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_word_stack_pointer() {
    let expected = CInstruction::SWSP {
        src: IRegister::S7,
        offset: CSWSPImmediate::try_from(56).unwrap(),
    };
    let bin = 0xdc5e;

    // check assembler
    let i = assemble_line("c.swsp s7, 56").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}

#[test]
fn store_double_stack_pointer() {
    let expected = CInstruction::SDSP {
        src: IRegister::GlobalPointer,
        offset: CSDSPImmediate::try_from(112).unwrap(),
    };
    let bin = 0xf88e;

    // check assembler
    let i = assemble_line("c.sdsp gp, 112").unwrap().c();
    assert_eq!(i, expected);

    // check decoder
    let i2 = CInstruction::decode(0xf88e).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = CInstruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    println!("{}", CInstruction::disassemble(&i));
    let i3 = assemble_line(&CInstruction::disassemble(&i)).unwrap().c();
    assert_eq!(i, i3);
}
