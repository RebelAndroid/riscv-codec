use riscv_disassembler::immediates::{IImmediate, SImmediate};
use riscv_disassembler::instruction::{
    assemble_line, decode_instruction, disassemble_instruction, Instruction, RoundingMode
};
use riscv_disassembler::register::{IRegister, FRegister};

#[test]
fn float_load_word() {
    // check assembler
    let i = assemble_line("flw fa0,64(a0)").unwrap();
    let expected = Instruction::FLW(FRegister::FA0, IRegister::A0, IImmediate::from_val(64));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x04052507).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_store_word() {
    // check assembler
    let i = assemble_line("fsw fs1,-1(a5)").unwrap();
    let expected = Instruction::FSW(IRegister::A5, FRegister::FS1, SImmediate::from_val(-1));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xfe97afa7).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_add() {
    // check assembler
    let i = assemble_line("fadd.s ft7,fa5,fs10").unwrap();
    let expected = Instruction::FADDS(FRegister::FT7, FRegister::FA5, FRegister::FS10, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x01a7f3d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_subtract() {
    // check assembler
    let i = assemble_line("fsub.s.rtz ft0,ft8,fs0").unwrap();
    let expected = Instruction::FSUBS(FRegister::FT0, FRegister::FT8, FRegister::FS0, RoundingMode::RTZ);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x088e1053).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_multiply() {
    // check assembler
    let i = assemble_line("fmul.s.rmm fs1,fs9,ft11").unwrap();
    let expected = Instruction::FMULS(FRegister::FS1, FRegister::FS9, FRegister::FT11, RoundingMode::RMM);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x11fcc4d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_divide() {
    // check assembler
    let i = assemble_line("fdiv.s.rup fs6,fs10,ft2").unwrap();
    let expected = Instruction::FDIVS(FRegister::FS6, FRegister::FS10, FRegister::FT2, RoundingMode::RUP);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x182d3b53).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_sqrt() {
    // check assembler
    let i = assemble_line("fsqrt.s.rne ft3,fa3").unwrap();
    let expected = Instruction::FSQRTS(FRegister::FT3, FRegister::FA3, RoundingMode::RNE);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x580681d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}