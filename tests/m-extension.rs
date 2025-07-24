use riscv_codec::assembly::assemble_line;
use riscv_codec::instruction::{Instruction, disassemble_instruction};
use riscv_codec::register::IRegister;

#[test]
fn multiply() {
    let expected = Instruction::MUL {
        dest: IRegister::A0,
        src1: IRegister::A1,
        src2: IRegister::A0,
    };
    let bin = 0x02A58533;

    // check assembler
    let i = assemble_line("mul a0,a1,a0").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high() {
    let expected = Instruction::MULH {
        dest: IRegister::T3,
        src1: IRegister::T4,
        src2: IRegister::S8,
    };
    let bin = 0x038e9e33;

    // check assembler
    let i = assemble_line("mulh t3,t4,s8").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_signed_unsigned() {
    let expected = Instruction::MULHSU {
        dest: IRegister::T0,
        src1: IRegister::A1,
        src2: IRegister::S3,
    };
    let bin = 0x0335a2b3;

    // check assembler
    let i = assemble_line("mulhsu t0,a1,s3").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_unsigned() {
    let expected = Instruction::MULHU {
        dest: IRegister::ReturnAddress,
        src1: IRegister::T5,
        src2: IRegister::S11,
    };
    let bin = 0x03bf30b3;

    // check assembler
    let i = assemble_line("mulhu ra,t5,s11").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide() {
    let expected = Instruction::DIV {
        dest: IRegister::T2,
        src1: IRegister::A2,
        src2: IRegister::S2,
    };
    let bin = 0x032643b3;

    // check assembler
    let i = assemble_line("div t2,a2,s2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned() {
    let expected = Instruction::DIVU {
        dest: IRegister::T3,
        src1: IRegister::T1,
        src2: IRegister::A3,
    };
    let bin = 0x02d35e33;

    // check assembler
    let i = assemble_line("divu t3,t1,a3").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder() {
    let expected = Instruction::REM {
        dest: IRegister::S5,
        src1: IRegister::A5,
        src2: IRegister::T6,
    };
    let bin = 0x03f7eab3;

    // check assembler
    let i = assemble_line("rem s5,a5,t6").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned() {
    let expected = Instruction::REMU {
        dest: IRegister::A4,
        src1: IRegister::S4,
        src2: IRegister::T4,
    };
    let bin = 0x03da7733;

    // check assembler
    let i = assemble_line("remu a4,s4,t4").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_word() {
    let expected = Instruction::MULW {
        dest: IRegister::StackPointer,
        src1: IRegister::T4,
        src2: IRegister::A2,
    };
    let bin = 0x02ce813b;

    // check assembler
    let i = assemble_line("mulw sp,t4,a2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_word() {
    let expected = Instruction::DIVW {
        dest: IRegister::T1,
        src1: IRegister::A3,
        src2: IRegister::S6,
    };
    let bin = 0x0366c33b;

    // check assembler
    let i = assemble_line("divw t1,a3,s6").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned_word() {
    let expected = Instruction::DIVUW {
        dest: IRegister::T6,
        src1: IRegister::A0,
        src2: IRegister::FramePointer,
    };
    let bin = 0x02855fbb;

    // check assembler
    let i = assemble_line("divuw t6,a0,fp").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_word() {
    let expected = Instruction::REMW {
        dest: IRegister::FramePointer,
        src1: IRegister::T3,
        src2: IRegister::A2,
    };
    let bin = 0x02ce643b;

    // check assembler
    let i = assemble_line("remw fp,t3,a2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned_word() {
    let expected = Instruction::REMUW {
        dest: IRegister::A1,
        src1: IRegister::T4,
        src2: IRegister::A5,
    };
    let bin = 0x02fef5bb;

    // check assembler
    let i = assemble_line("remuw a1,t4,a5").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
