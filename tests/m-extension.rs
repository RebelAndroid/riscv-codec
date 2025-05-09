use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn multiply() {
    // check assembler
    let i = assemble_line("mul a0,a1,a0").unwrap();
    assert_eq!(
        i,
        Instruction::MUL(IRegister::A0, IRegister::A1, IRegister::A0)
    );

    // check decoder
    let i2 = decode_instruction(0x02A58533).unwrap();
    assert_eq!(
        i2,
        Instruction::MUL(IRegister::A0, IRegister::A1, IRegister::A0),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high() {
    // check assembler
    let i = assemble_line("mulh t3,t4,s8").unwrap();
    assert_eq!(
        i,
        Instruction::MULH(IRegister::T3, IRegister::T4, IRegister::S8)
    );

    // check decoder
    let i2 = decode_instruction(0x038e9e33).unwrap();
    assert_eq!(
        i2,
        Instruction::MULH(IRegister::T3, IRegister::T4, IRegister::S8),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_signed_unsigned() {
    // check assembler
    let i = assemble_line("mulhsu t0,a1,s3").unwrap();
    assert_eq!(
        i,
        Instruction::MULHSU(IRegister::T0, IRegister::A1, IRegister::S3)
    );

    // check decoder
    let i2 = decode_instruction(0x0335a2b3).unwrap();
    assert_eq!(
        i2,
        Instruction::MULHSU(IRegister::T0, IRegister::A1, IRegister::S3),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn multiply_high_unsigned() {
    // check assembler
    let i = assemble_line("mulhu ra,t5,s11").unwrap();
    assert_eq!(
        i,
        Instruction::MULHU(IRegister::ReturnAddress, IRegister::T5, IRegister::S11)
    );

    // check decoder
    let i2 = decode_instruction(0x03bf30b3).unwrap();
    assert_eq!(
        i2,
        Instruction::MULHU(IRegister::ReturnAddress, IRegister::T5, IRegister::S11),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn divide() {
    // check assembler
    let i = assemble_line("div t2,a2,s2").unwrap();
    assert_eq!(
        i,
        Instruction::DIV(IRegister::T2, IRegister::A2, IRegister::S2)
    );

    // check decoder
    let i2 = decode_instruction(0x032643b3).unwrap();
    assert_eq!(
        i2,
        Instruction::DIV(IRegister::T2, IRegister::A2, IRegister::S2),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned() {
    // check assembler
    let i = assemble_line("divu t3,t1,a3").unwrap();
    assert_eq!(
        i,
        Instruction::DIVU(IRegister::T3, IRegister::T1, IRegister::A3)
    );

    // check decoder
    let i2 = decode_instruction(0x02d35e33).unwrap();
    assert_eq!(
        i2,
        Instruction::DIVU(IRegister::T3, IRegister::T1, IRegister::A3),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn remainder() {
    // check assembler
    let i = assemble_line("rem s5,a5,t6").unwrap();
    assert_eq!(
        i,
        Instruction::REM(IRegister::S5, IRegister::A5, IRegister::T6)
    );

    // check decoder
    let i2 = decode_instruction(0x03f7eab3).unwrap();
    assert_eq!(
        i2,
        Instruction::REM(IRegister::S5, IRegister::A5, IRegister::T6),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned() {
    // check assembler
    let i = assemble_line("remu a4,s4,t4").unwrap();
    assert_eq!(
        i,
        Instruction::REMU(IRegister::A4, IRegister::S4, IRegister::T4)
    );

    // check decoder
    let i2 = decode_instruction(0x03da7733).unwrap();
    assert_eq!(
        i2,
        Instruction::REMU(IRegister::A4, IRegister::S4, IRegister::T4),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn multiply_word() {
    // check assembler
    let i = assemble_line("mulw sp,t4,a2").unwrap();
    assert_eq!(
        i,
        Instruction::MULW(IRegister::StackPointer, IRegister::T4, IRegister::A2)
    );

    // check decoder
    let i2 = decode_instruction(0x02ce813b).unwrap();
    assert_eq!(
        i2,
        Instruction::MULW(IRegister::StackPointer, IRegister::T4, IRegister::A2),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn divide_word() {
    // check assembler
    let i = assemble_line("divw t1,a3,s6").unwrap();
    assert_eq!(
        i,
        Instruction::DIVW(IRegister::T1, IRegister::A3, IRegister::S6)
    );

    // check decoder
    let i2 = decode_instruction(0x0366c33b).unwrap();
    assert_eq!(
        i2,
        Instruction::DIVW(IRegister::T1, IRegister::A3, IRegister::S6),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn divide_unsigned_word() {
    // check assembler
    let i = assemble_line("divuw t6,a0,fp").unwrap();
    assert_eq!(
        i,
        Instruction::DIVUW(IRegister::T6, IRegister::A0, IRegister::FramePointer)
    );

    // check decoder
    let i2 = decode_instruction(0x02855fbb).unwrap();
    assert_eq!(
        i2,
        Instruction::DIVUW(IRegister::T6, IRegister::A0, IRegister::FramePointer),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn remainder_word() {
    // check assembler
    let i = assemble_line("remw fp,t3,a2").unwrap();
    assert_eq!(
        i,
        Instruction::REMW(IRegister::FramePointer, IRegister::T3, IRegister::A2)
    );

    // check decoder
    let i2 = decode_instruction(0x02ce643b).unwrap();
    assert_eq!(
        i2,
        Instruction::REMW(IRegister::FramePointer, IRegister::T3, IRegister::A2),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn remainder_unsigned_word() {
    // check assembler
    let i = assemble_line("remuw a1,t4,a5").unwrap();
    assert_eq!(
        i,
        Instruction::REMUW(IRegister::A1, IRegister::T4, IRegister::A5)
    );

    // check decoder
    let i2 = decode_instruction(0x02fef5bb).unwrap();
    assert_eq!(
        i2,
        Instruction::REMUW(IRegister::A1, IRegister::T4, IRegister::A5),
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}
