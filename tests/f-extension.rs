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

#[test]
fn float_minimum() {
    // check assembler
    let i = assemble_line("fmin.s ft1,fs4,fa5").unwrap();
    let expected = Instruction::FMINS(FRegister::FT1, FRegister::FS4, FRegister::FA5);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x28fa00d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_maximum() {
    // check assembler
    let i = assemble_line("fmax.s fa3,fs9,fs2").unwrap();
    let expected = Instruction::FMAXS(FRegister::FA3, FRegister::FS9, FRegister::FS2);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x292c96d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_word_from_single() {
    // check assembler
    let i = assemble_line("fcvt.w.s.rup s1,fs2").unwrap();
    let expected = Instruction::FCVTWS(IRegister::S1, FRegister::FS2, RoundingMode::RUP);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xc00934d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_word_from_single() {
    // check assembler
    let i = assemble_line("fcvt.wu.s.rmm sp,ft3").unwrap();
    let expected = Instruction::FCVTWUS(IRegister::StackPointer, FRegister::FT3, RoundingMode::RMM);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xc011c153).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_move_x_from_word() {
    // check assembler
    let i = assemble_line("fmv.x.w s2,ft4").unwrap();
    let expected = Instruction::FMVXW(IRegister::S2, FRegister::FT4);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xe0020953).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_equal_single() {
    // check assembler
    let i = assemble_line("feq.s a4,fs7,ft11").unwrap();
    let expected = Instruction::FEQS(IRegister::A4, FRegister::FS7, FRegister::FT11);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xa1fba753).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_less_than_single() {
    // check assembler
    let i = assemble_line("flt.s s6,ft10,fa6").unwrap();
    let expected = Instruction::FLTS(IRegister::S6, FRegister::FT10, FRegister::FA6);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xa10f1b53).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_less_equal_single() {
    // check assembler
    let i = assemble_line("fle.s s2,fs4,ft0").unwrap();
    let expected = Instruction::FLES(IRegister::S2, FRegister::FS4, FRegister::FT0);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xa00a0953).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_class_single() {
    // check assembler
    let i = assemble_line("fclass.s tp,fs3").unwrap();
    let expected = Instruction::FCLASSS(IRegister::ThreadPointer, FRegister::FS3);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xe0099253).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_word() {
    // check assembler
    let i = assemble_line("fcvt.s.w.rdn fa2,t4").unwrap();
    let expected = Instruction::FCVTSW(FRegister::FA2, IRegister::T4, RoundingMode::RDN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xd00ea653).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_word() {
    // check assembler
    let i = assemble_line("fcvt.s.wu.dyn fs4,t6").unwrap();
    let expected = Instruction::FCVTSWU(FRegister::FS4, IRegister::T6, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xd01ffa53).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_move_word_from_x() {
    // check assembler
    let i = assemble_line("fmv.w.x fs3,t1").unwrap();
    let expected = Instruction::FMVWX(FRegister::FS3, IRegister::T1);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xf00309d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_long_from_single() {
    // check assembler
    let i = assemble_line("fcvt.l.s s4,fa7").unwrap();
    let expected = Instruction::FCVTLS(IRegister::S4, FRegister::FA7, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xc028fa53).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_long_from_single() {
    // check assembler
    let i = assemble_line("fcvt.lu.s t2,ft9").unwrap();
    let expected = Instruction::FCVTLUS(IRegister::T2, FRegister::FT9, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xc03ef3d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}


#[test]
fn float_convert_single_from_long() {
    // check assembler
    let i = assemble_line("fcvt.s.l fs8,s2").unwrap();
    let expected = Instruction::FCVTSL(FRegister::FS8, IRegister::S2, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xd0297c53).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_long() {
    // check assembler
    let i = assemble_line("fcvt.s.lu ft7,fp").unwrap();
    let expected = Instruction::FCVTSLU(FRegister::FT7, IRegister::FramePointer, RoundingMode::DYN);
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xd03473d3).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}