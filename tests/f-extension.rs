use riscv_disassembler::immediates::{IImmediate, SImmediate};
use riscv_disassembler::instruction::{
    Instruction, RoundingMode, assemble_line, decode_instruction, disassemble_instruction, encode_instruction,
};
use riscv_disassembler::register::{FRegister, IRegister};

#[test]
fn float_load_word() {
    let expected = Instruction::FLW {
        dest: FRegister::FA0,
        base: IRegister::A0,
        offset: IImmediate::try_from(64).unwrap(),
    };
    let bin = 0x04052507;

    // check assembler
    let i = assemble_line("flw fa0,64(a0)").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_store_word() {
    let expected = Instruction::FSW {
        base: IRegister::A5,
        src: FRegister::FS1,
        offset: SImmediate::try_from(-1).unwrap(),
    };
    let bin = 0xfe97afa7;

    // check assembler
    let i = assemble_line("fsw fs1,-1(a5)").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_add() {
    let expected = Instruction::FADDS {
        dest: FRegister::FT7,
        src1: FRegister::FA5,
        src2: FRegister::FS10,
        rm: RoundingMode::DYN,
    };
    let bin = 0x01a7f3d3;

    // check assembler
    let i = assemble_line("fadd.s ft7,fa5,fs10").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_subtract() {
    let expected = Instruction::FSUBS {
        dest: FRegister::FT0,
        src1: FRegister::FT8,
        src2: FRegister::FS0,
        rm: RoundingMode::RTZ,
    };
    let bin = 0x088e1053;

    // check assembler
    let i = assemble_line("fsub.s.rtz ft0,ft8,fs0").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_multiply() {
    let expected = Instruction::FMULS {
        dest: FRegister::FS1,
        src1: FRegister::FS9,
        src2: FRegister::FT11,
        rm: RoundingMode::RMM,
    };
    let bin = 0x11fcc4d3;

    // check assembler
    let i = assemble_line("fmul.s.rmm fs1,fs9,ft11").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_divide() {
    let expected = Instruction::FDIVS {
        dest: FRegister::FS6,
        src1: FRegister::FS10,
        src2: FRegister::FT2,
        rm: RoundingMode::RUP,
    };
    let bin = 0x182d3b53;

    // check assembler
    let i = assemble_line("fdiv.s.rup fs6,fs10,ft2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_sqrt() {
    let expected = Instruction::FSQRTS {
        dest: FRegister::FT3,
        src: FRegister::FA3,
        rm: RoundingMode::RNE,
    };
    let bin = 0x580681d3;

    // check assembler
    let i = assemble_line("fsqrt.s.rne ft3,fa3").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_minimum() {
    let expected = Instruction::FMINS {
        dest: FRegister::FT1,
        src1: FRegister::FS4,
        src2: FRegister::FA5,
    };
    let bin = 0x28fa00d3;

    // check assembler
    let i = assemble_line("fmin.s ft1,fs4,fa5").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_maximum() {
    let expected = Instruction::FMAXS {
        dest: FRegister::FA3,
        src1: FRegister::FS9,
        src2: FRegister::FS2,
    };
    let bin = 0x292c96d3;

    // check assembler
    let i = assemble_line("fmax.s fa3,fs9,fs2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_word_from_single() {
    let expected = Instruction::FCVTWS {
        dest: IRegister::S1,
        src: FRegister::FS2,
        rm: RoundingMode::RUP,
    };
    let bin = 0xc00934d3;

    // check assembler
    let i = assemble_line("fcvt.w.s.rup s1,fs2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_word_from_single() {
    let expected = Instruction::FCVTWUS {
        dest: IRegister::StackPointer,
        src: FRegister::FT3,
        rm: RoundingMode::RMM,
    };
    let bin = 0xc011c153;

    // check assembler
    let i = assemble_line("fcvt.wu.s.rmm sp,ft3").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_move_x_from_word() {
    let expected = Instruction::FMVXW {
        dest: IRegister::S2,
        src: FRegister::FT4,
    };
    let bin = 0xe0020953;

    // check assembler
    let i = assemble_line("fmv.x.w s2,ft4").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_equal_single() {
    let expected = Instruction::FEQS {
        dest: IRegister::A4,
        src1: FRegister::FS7,
        src2: FRegister::FT11,
    };
    let bin = 0xa1fba753;

    // check assembler
    let i = assemble_line("feq.s a4,fs7,ft11").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_less_than_single() {
    let expected = Instruction::FLTS {
        dest: IRegister::S6,
        src1: FRegister::FT10,
        src2: FRegister::FA6,
    };
    let bin = 0xa10f1b53;

    // check assembler
    let i = assemble_line("flt.s s6,ft10,fa6").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin, "\ngot: {:b} \nexp: {:b}", b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_less_equal_single() {
    let expected = Instruction::FLES {
        dest: IRegister::S2,
        src1: FRegister::FS4,
        src2: FRegister::FT0,
    };
    let bin = 0xa00a0953;

    // check assembler
    let i = assemble_line("fle.s s2,fs4,ft0").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_class_single() {
    let expected = Instruction::FCLASSS {
        dest: IRegister::ThreadPointer,
        src: FRegister::FS3,
    };
    let bin = 0xe0099253;

    // check assembler
    let i = assemble_line("fclass.s tp,fs3").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_word() {
    let expected = Instruction::FCVTSW {
        dest: FRegister::FA2,
        src: IRegister::T4,
        rm: RoundingMode::RDN,
    };
    let bin = 0xd00ea653;

    // check assembler
    let i = assemble_line("fcvt.s.w.rdn fa2,t4").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_word() {
    let expected = Instruction::FCVTSWU {
        dest: FRegister::FS4,
        src: IRegister::T6,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd01ffa53;

    // check assembler
    let i = assemble_line("fcvt.s.wu.dyn fs4,t6").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_move_word_from_x() {
    let expected = Instruction::FMVWX {
        dest: FRegister::FS3,
        src: IRegister::T1,
    };
    let bin = 0xf00309d3;

    // check assembler
    let i = assemble_line("fmv.w.x fs3,t1").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_long_from_single() {
    let expected = Instruction::FCVTLS {
        dest: IRegister::S4,
        src: FRegister::FA7,
        rm: RoundingMode::DYN,
    };
    let bin = 0xc028fa53;

    // check assembler
    let i = assemble_line("fcvt.l.s s4,fa7").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_long_from_single() {
    let expected = Instruction::FCVTLUS {
        dest: IRegister::T2,
        src: FRegister::FT9,
        rm: RoundingMode::DYN,
    };
    let bin = 0xc03ef3d3;

    // check assembler
    let i = assemble_line("fcvt.lu.s t2,ft9").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_long() {
    let expected = Instruction::FCVTSL {
        dest: FRegister::FS8,
        src: IRegister::S2,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd0297c53;

    // check assembler
    let i = assemble_line("fcvt.s.l fs8,s2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_long() {
    let expected = Instruction::FCVTSLU {
        dest: FRegister::FT7,
        src: IRegister::FramePointer,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd03473d3;

    // check assembler
    let i = assemble_line("fcvt.s.lu ft7,fp").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}