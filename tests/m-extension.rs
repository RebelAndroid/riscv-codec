use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn multiply() {
    // check assembler
    let i = assemble_line("mul a0,a1,a0").unwrap().i();
    let expeted = Instruction::MUL{dest: IRegister::A0, src1: IRegister::A1, src2: IRegister::A0};
    assert_eq!(
        i,
        expeted
    );

    // check decoder
    let i2 = decode_instruction(0x02A58533).unwrap();
    assert_eq!(
        i2,
        expeted
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high() {
    // check assembler
    let i = assemble_line("mulh t3,t4,s8").unwrap().i();
    let expected = Instruction::MULH{dest: IRegister::T3, src1: IRegister::T4, src2: IRegister::S8};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x038e9e33).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_signed_unsigned() {
    // check assembler
    let i = assemble_line("mulhsu t0,a1,s3").unwrap().i();
    let expected = Instruction::MULHSU{dest: IRegister::T0, src1: IRegister::A1, src2: IRegister::S3};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x0335a2b3).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_unsigned() {
    // check assembler
    let i = assemble_line("mulhu ra,t5,s11").unwrap().i();
    let expected = Instruction::MULHU{dest: IRegister::ReturnAddress, src1: IRegister::T5, src2: IRegister::S11};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x03bf30b3).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide() {
    // check assembler
    let i = assemble_line("div t2,a2,s2").unwrap().i();
    let expected = Instruction::DIV{dest: IRegister::T2, src1: IRegister::A2, src2: IRegister::S2};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x032643b3).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned() {
    // check assembler
    let i = assemble_line("divu t3,t1,a3").unwrap().i();
    let expected = Instruction::DIVU{dest: IRegister::T3, src1: IRegister::T1, src2: IRegister::A3};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x02d35e33).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder() {
    // check assembler
    let i = assemble_line("rem s5,a5,t6").unwrap().i();
    let expected = Instruction::REM{dest: IRegister::S5, src1: IRegister::A5, src2: IRegister::T6};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x03f7eab3).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned() {
    // check assembler
    let i = assemble_line("remu a4,s4,t4").unwrap().i();
    let expected = Instruction::REMU{dest: IRegister::A4, src1: IRegister::S4, src2: IRegister::T4};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x03da7733).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn multiply_word() {
    // check assembler
    let i = assemble_line("mulw sp,t4,a2").unwrap().i();
    let expected = Instruction::MULW{dest: IRegister::StackPointer, src1: IRegister::T4, src2: IRegister::A2};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x02ce813b).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_word() {
    // check assembler
    let i = assemble_line("divw t1,a3,s6").unwrap().i();
    let expected = Instruction::DIVW{dest: IRegister::T1, src1: IRegister::A3, src2: IRegister::S6};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x0366c33b).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned_word() {
    // check assembler
    let i = assemble_line("divuw t6,a0,fp").unwrap().i();
    let expected = Instruction::DIVUW{dest: IRegister::T6, src1: IRegister::A0, src2: IRegister::FramePointer};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x02855fbb).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_word() {
    // check assembler
    let i = assemble_line("remw fp,t3,a2").unwrap().i();
    let expected = Instruction::REMW{dest: IRegister::FramePointer, src1: IRegister::T3, src2: IRegister::A2};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x02ce643b).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned_word() {
    // check assembler
    let i = assemble_line("remuw a1,t4,a5").unwrap().i();
    let expected = Instruction::REMUW{dest: IRegister::A1, src1: IRegister::T4, src2: IRegister::A5};
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x02fef5bb).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
