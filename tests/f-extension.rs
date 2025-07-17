use riscv_disassembler::immediates::{IImmediate, SImmediate};
use riscv_disassembler::instruction::{
    Instruction, RoundingMode, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::{FRegister, IRegister};

#[test]
fn float_load_word() {
    // check assembler
    let i = assemble_line("flw fa0,64(a0)").unwrap().i();
    let expected = Instruction::FLW {
        dest: FRegister::FA0,
        base: IRegister::A0,
        offset: IImmediate::from_val(64),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x04052507).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_store_word() {
    // check assembler
    let i = assemble_line("fsw fs1,-1(a5)").unwrap().i();
    let expected = Instruction::FSW {
        base: IRegister::A5,
        src: FRegister::FS1,
        offset: SImmediate::from_val(-1),
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xfe97afa7).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_add() {
    // check assembler
    let i = assemble_line("fadd.s ft7,fa5,fs10").unwrap().i();
    let expected = Instruction::FADDS {
        dest: FRegister::FT7,
        src1: FRegister::FA5,
        src2: FRegister::FS10,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x01a7f3d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_subtract() {
    // check assembler
    let i = assemble_line("fsub.s.rtz ft0,ft8,fs0").unwrap().i();
    let expected = Instruction::FSUBS {
        dest: FRegister::FT0,
        src1: FRegister::FT8,
        src2: FRegister::FS0,
        rm: RoundingMode::RTZ,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x088e1053).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_multiply() {
    // check assembler
    let i = assemble_line("fmul.s.rmm fs1,fs9,ft11").unwrap().i();
    let expected = Instruction::FMULS {
        dest: FRegister::FS1,
        src1: FRegister::FS9,
        src2: FRegister::FT11,
        rm: RoundingMode::RMM,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x11fcc4d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_divide() {
    // check assembler
    let i = assemble_line("fdiv.s.rup fs6,fs10,ft2").unwrap().i();
    let expected = Instruction::FDIVS {
        dest: FRegister::FS6,
        src1: FRegister::FS10,
        src2: FRegister::FT2,
        rm: RoundingMode::RUP,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x182d3b53).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_sqrt() {
    // check assembler
    let i = assemble_line("fsqrt.s.rne ft3,fa3").unwrap().i();
    let expected = Instruction::FSQRTS {
        dest: FRegister::FT3,
        src: FRegister::FA3,
        rm: RoundingMode::RNE,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x580681d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_minimum() {
    // check assembler
    let i = assemble_line("fmin.s ft1,fs4,fa5").unwrap().i();
    let expected = Instruction::FMINS {
        dest: FRegister::FT1,
        src1: FRegister::FS4,
        src2: FRegister::FA5,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x28fa00d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_maximum() {
    // check assembler
    let i = assemble_line("fmax.s fa3,fs9,fs2").unwrap().i();
    let expected = Instruction::FMAXS {
        dest: FRegister::FA3,
        src1: FRegister::FS9,
        src2: FRegister::FS2,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x292c96d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_word_from_single() {
    // check assembler
    let i = assemble_line("fcvt.w.s.rup s1,fs2").unwrap().i();
    let expected = Instruction::FCVTWS {
        dest: IRegister::S1,
        src: FRegister::FS2,
        rm: RoundingMode::RUP,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc00934d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_word_from_single() {
    // check assembler
    let i = assemble_line("fcvt.wu.s.rmm sp,ft3").unwrap().i();
    let expected = Instruction::FCVTWUS {
        dest: IRegister::StackPointer,
        src: FRegister::FT3,
        rm: RoundingMode::RMM,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc011c153).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_move_x_from_word() {
    // check assembler
    let i = assemble_line("fmv.x.w s2,ft4").unwrap().i();
    let expected = Instruction::FMVXW {
        dest: IRegister::S2,
        src: FRegister::FT4,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xe0020953).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_equal_single() {
    // check assembler
    let i = assemble_line("feq.s a4,fs7,ft11").unwrap().i();
    let expected = Instruction::FEQS {
        dest: IRegister::A4,
        src1: FRegister::FS7,
        src2: FRegister::FT11,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xa1fba753).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_less_than_single() {
    // check assembler
    let i = assemble_line("flt.s s6,ft10,fa6").unwrap().i();
    let expected = Instruction::FLTS {
        dest: IRegister::S6,
        src1: FRegister::FT10,
        src2: FRegister::FA6,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xa10f1b53).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_less_equal_single() {
    // check assembler
    let i = assemble_line("fle.s s2,fs4,ft0").unwrap().i();
    let expected = Instruction::FLES {
        dest: IRegister::S2,
        src1: FRegister::FS4,
        src2: FRegister::FT0,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xa00a0953).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_class_single() {
    // check assembler
    let i = assemble_line("fclass.s tp,fs3").unwrap().i();
    let expected = Instruction::FCLASSS {
        dest: IRegister::ThreadPointer,
        src: FRegister::FS3,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xe0099253).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_word() {
    // check assembler
    let i = assemble_line("fcvt.s.w.rdn fa2,t4").unwrap().i();
    let expected = Instruction::FCVTSW {
        dest: FRegister::FA2,
        src: IRegister::T4,
        rm: RoundingMode::RDN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xd00ea653).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_word() {
    // check assembler
    let i = assemble_line("fcvt.s.wu.dyn fs4,t6").unwrap().i();
    let expected = Instruction::FCVTSWU {
        dest: FRegister::FS4,
        src: IRegister::T6,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xd01ffa53).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_move_word_from_x() {
    // check assembler
    let i = assemble_line("fmv.w.x fs3,t1").unwrap().i();
    let expected = Instruction::FMVWX {
        dest: FRegister::FS3,
        src: IRegister::T1,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xf00309d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_long_from_single() {
    // check assembler
    let i = assemble_line("fcvt.l.s s4,fa7").unwrap().i();
    let expected = Instruction::FCVTLS {
        dest: IRegister::S4,
        src: FRegister::FA7,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc028fa53).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_unsigned_long_from_single() {
    // check assembler
    let i = assemble_line("fcvt.lu.s t2,ft9").unwrap().i();
    let expected = Instruction::FCVTLUS {
        dest: IRegister::T2,
        src: FRegister::FT9,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc03ef3d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_long() {
    // check assembler
    let i = assemble_line("fcvt.s.l fs8,s2").unwrap().i();
    let expected = Instruction::FCVTSL {
        dest: FRegister::FS8,
        src: IRegister::S2,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xd0297c53).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_convert_single_from_unsigned_long() {
    // check assembler
    let i = assemble_line("fcvt.s.lu ft7,fp").unwrap().i();
    let expected = Instruction::FCVTSLU {
        dest: FRegister::FT7,
        src: IRegister::FramePointer,
        rm: RoundingMode::DYN,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xd03473d3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
